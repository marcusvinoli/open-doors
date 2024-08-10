<script lang="ts">
    import type { ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType, ToolbarToggleType } from "$lib/components/global/toolbar/Toolbar";
    import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar";
    import { beforeUpdate, onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { confirm } from '@tauri-apps/api/dialog';
    import { addTab, updateActiveTab } from "$lib/stores/Tabs";
    import { get } from "svelte/store";
    import { page } from "$app/stores";
    import { repository } from "$lib/stores/Repository";
    import * as Resizable from "$lib/components/ui/resizable";
    import ObjectEditor from "$lib/components/global/object_editor/ObjectEditor.svelte";
    import type { Link, Object, ObjectView } from "$lib/components/structs/Object";
    import IndexTree from "$lib/components/global/indextree/IndexTree.svelte";
    import ObjectExplorer from "$lib/components/global/object_explorer/ObjectExplorer.svelte";
    import { createDraftObject, createObject, deleteObject, readDraftObjects, readModule, readModuleFromPath, readObjects } from "$lib/controllers/Module";
    import type { Module } from "$lib/components/structs/Module";
    import TemplateForm from "$lib/components/forms/module/TemplateForm.svelte";
    import { loadRepository, reloadRepository } from "$lib/controllers/Repository";
    import { pageState } from "./store";
    import type { View } from "$lib/components/global/object_explorer/viewStructs";
    import { defaultView } from "$lib/components/global/object_explorer/viewMethods";
    import ToolbarToggle from "$lib/components/global/toolbar/ToolbarToggle.svelte";
    import ToolbarButton from "$lib/components/global/toolbar/ToolbarButton.svelte";
    
    let selectedObject: ObjectView | null = null;
    let objects: ObjectView[] = [];
    let module: Module;

    let editPanelFlag: boolean = false;
    let treePanelFlag: boolean = false;
    let templateFlag: boolean = false;
    let editModeFlag: boolean = true;
    let updateModuleFlag: boolean = false;
    let tabKey: string = "";
    let view: View = defaultView();

    function loadHomeToolbar() {
        clearToolbar();
    
        let homeButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Home",
            icon: "gravity-ui:house",
            action: () => {
                goto("/home")
            },
        }

        let showTree: ToolbarButtonType = {
            type: "button",
            tooltip: "Show/Hide tree panel",
            icon: "gravity-ui:layout-header-side-content",
            action: () => {
                treePanelFlag = !treePanelFlag;
            },
        }

        let toggleEditMode: ToolbarButtonType = {
            type: "button",
            tooltip: "Show/Hide tree panel",
            icon: "gravity-ui:layout-header-side-content",
            action: () => {
                editModeFlag = !editModeFlag;
            },
        }

        let templateManager: ToolbarButtonType = {
            type: "button",
            tooltip: "Template",
            icon: "gravity-ui:layout-columns-3",
            action: () => {
                templateFlag = !templateFlag;
            }
        }
        
        let newButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New...",
            icon: "gravity-ui:circle-plus",
            action: () => {},
        }
    
        let newBaselineButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Baseline",
            icon: "gravity-ui:tag",
            action: () => {},
        }
    
        let newObjectButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Object",
            icon: "gravity-ui:square-chart-bar",
            action: () => {
                if(!editPanelFlag) {
                    selectedObject = null;
                    editPanelFlag = true;
                }
            },
        }

        let readOnlyModeButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Read Only",
            icon: "lucide:pencil-off",
            action: () => {
                editModeFlag = false;
            },
        }

        let editModeButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Edit",
            icon: "lucide:pencil",
            action: () => {
                editModeFlag = true;
            },
        }

        let viewModeButton: ToolbarToggleType = {
            type: "toggle",
            buttonOn: editModeButton,
            buttonOff: readOnlyModeButton,
            status: editModeFlag,
        }
        
        let creationGroup: ToolbarDropdownType = {
            button: newButton,
            items: [
                {
                    items: [
                        newObjectButton,
                    ],
                    type: "buttonsGroup",
                },
                {
                    items: [
                        newBaselineButton,
                    ],
                    type: "buttonsGroup",
                }
            ],
            type: "dropdown",
        }
    
        let navigationGroup: ToolbarGroupType = {
            items: [homeButton],
            type: "buttonsGroup"
        }
    
        let newGroup: ToolbarGroupType = {
            items: [creationGroup],
            type: "buttonsGroup"
        }

        let viewGrouplView: ToolbarGroupType = {
            items: [showTree, viewModeButton],
            type: "buttonsGroup"
        }

        let templateButton: ToolbarGroupType = {
            items: [templateManager],
            type: "buttonsGroup"
        }
    
        addToolbarItem(navigationGroup);
        addToolbarItem(newGroup);
        addToolbarItem(viewGrouplView);
        addToolbarItem(templateButton);

    }

    function handleObjectCreation(event: any) {
        let obj = event.detail.obj.object;
        createObject(module.path, obj)
            .then(() => {
                selectedObject = null;
                editPanelFlag = false;
                updateModuleFlag = true;
                load(module.path);
            })
    }
    
    function handleObjectDraftCreation(event: any) {
        let obj = event.detail.obj.object;
        createDraftObject(module.path, obj)
            .then(() => {
                editPanelFlag = false;
            })
            .finally(() => {
                selectedObject = null;
                updateModuleFlag = true;
                load(module.path);
            })
        
    }
    
    async function handleObjectExclusion(event: any) {
        let obj = event.detail.obj.object;
        const confirmed = await confirm('Do you really want to delete this Object?', 'Deleting object ' + module.manifest.prefix + module.manifest.separator + obj.id );
        if (!confirmed) {
            return;
        }
        deleteObject(module.path, obj.id)
            .then(() => {
                editPanelFlag = false;
            })
            .finally(() => {
                selectedObject = null;
                updateModuleFlag = true;
                load(module.path);
            })
    }
    
    function handleCloseEditPanel(event: any) {
        editPanelFlag = false;
    }

    function handleObjectSelect(event: any) {
        selectedObject = event.detail.object;
        editPanelFlag = true;
    }
    
    function scrollIntoView(event: any) {
        const el = document.getElementById("row-" + event.detail.path);
	    const ov = document.getElementById("scroll-table");
		if (!el) return;
		if (!ov) return;

        const offset = 50; // Ajuste o valor do offset conforme a altura do seu cabeçalho

        ov.scrollTo({
            top: el.offsetTop - offset,
            behavior: 'smooth'
        });

        document.querySelectorAll('.flash').forEach(element => {
            element.classList.remove('flash');
        });

        el.classList.add('flash');
    }

    function compareLevels(a: string, b: string): number {
        const parseLevel = (level: string) => level.split(/[\.\-]/).map(part => isNaN(Number(part)) ? part : Number(part));
        
        const aParts = parseLevel(a);
        const bParts = parseLevel(b);
        
        const len = Math.max(aParts.length, bParts.length);
        for (let i = 0; i < len; i++) {
            if (aParts[i] === undefined) return -1;
            if (bParts[i] === undefined) return 1;
            
            if (typeof aParts[i] === 'number' && typeof bParts[i] === 'number') {
                if (aParts[i] !== bParts[i]) return (aParts[i] as number) - (bParts[i] as number);
            } else if (typeof aParts[i] === 'string' && typeof bParts[i] === 'string') {
                let aP = aParts[i] as string;
                let bP = bParts[i] as string;
                if (aParts[i] !== bParts[i]) return aP.localeCompare(bP);
            } else {
                return typeof aParts[i] === 'number' ? -1 : 1;
            }
        }
        return 0;
    }

    function sortItems(items: ObjectView[]): ObjectView[] {
        return items.sort((a, b) => compareLevels(a.object.level, b.object.level));
    }

    function getLinks(links: any, id: number) {
        let ret: Link[] = [];

        if (links[id]) {
            links[id].forEach((lk: Link) => {
                ret.push(lk);
            })
        }

        return ret;
    }

    async function load(modPath: string) {
        module = await readModuleFromPath(modPath);
        let retObjects = await readObjects(modPath);
        let retDraftObjects = await readDraftObjects(modPath);
        let newObjects: ObjectView[] = [];

        retObjects.forEach((obj) => {
            let dob = {
                object: obj,
                isDraft: false,
                hasChanges: false,
                inboundLinks: getLinks(module.inboundLinks, obj.id),
            }
            newObjects.push(dob);
        });
        
        retDraftObjects.forEach((dobj) => {
            let index = newObjects.findIndex((ob) => {return (ob.object.id === dobj.id)});
            let dob = {
                object: dobj,
                isDraft: true,
                hasChanges: false,
                inboundLinks: getLinks(module.inboundLinks, dobj.id),
            }

            if (index < 0) {
                newObjects.push(dob);
            } else {
                newObjects[index] = dob;
            }
        });

        objects = sortItems(newObjects);
    }

    function generateKey(input: string): string {
        const sanitized = input.toLowerCase().replace(/[^a-z0-9]/g, '');
        const truncated = sanitized.length > 30 ? sanitized.substring(0, 30) : sanitized;
        return truncated;
    }
    
    $: {
        const { mod, version } = $page.params;
        load(mod);
        tabKey = generateKey(`${mod.substring($repository?.tree.path.length)}-${version}`);
        
        const savedState = pageState.getPageState(tabKey);
        if (savedState) {
            ({ scrollX, scrollY, selectedObject, editPanelFlag, view } = savedState);
            window.scrollTo(savedState.scrollX, savedState.scrollY);
        }
        console.log("Gotcha");
    }
    
    onMount(() => {
        const params = get(page).params;
        const url: string = $page.url.pathname;
        const name: string = params.mod.substring($repository?.tree.path.length);
        const version: string = params.version;
        loadRepository();
        loadHomeToolbar();
        addTab(name, "gravity-ui:layout-header-cells-large-fill", url, version);
        load(params.mod);       
        const savedState = pageState.getPageState(tabKey);
        if (savedState) {
            ({ scrollX, scrollY, selectedObject, editPanelFlag } = savedState);
            window.scrollTo(savedState.scrollX, savedState.scrollY);
        }
    })
    
    beforeUpdate(() => {
        const state = {
            scrollX: window.scrollX,
            scrollY: window.scrollY,
            selectedObject,
            editPanelFlag,
            view,
        };
        pageState.setPageState(tabKey, state);
    });

</script>

<div class="bg-slate-50 h-full py-1">
    {#if module}
    <TemplateForm bind:module={module} bind:openDialog={templateFlag}/>
    {/if}
    <Resizable.PaneGroup direction="horizontal">
        {#if treePanelFlag}
        <Resizable.Pane defaultSize={20} collapsible order={1}>
            <IndexTree items={objects} on:click={scrollIntoView}/>
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        {/if}
        <Resizable.Pane order={2}>
            {#if module}
            <ObjectExplorer bind:view={view} bind:module={module} bind:objects={objects} bind:editMode={editModeFlag} on:click={handleObjectSelect}/>
            {/if}
        </Resizable.Pane>
        {#if editPanelFlag && editModeFlag}
        <Resizable.Handle withHandle/>
        <Resizable.Pane class="h-full" defaultSize={50} order={3}>
            {#if module}
            <ObjectEditor bind:template={module.template} objv={selectedObject} on:close={handleCloseEditPanel} on:saveDraft={handleObjectDraftCreation} on:save={handleObjectCreation} on:delete={handleObjectExclusion}/>
            {/if}
        </Resizable.Pane>
        {/if}
    </Resizable.PaneGroup>
    <div class="flash"></div>
</div>
