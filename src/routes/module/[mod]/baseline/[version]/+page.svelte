<script lang="ts">
    import type { ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType } from "$lib/components/global/toolbar/Toolbar";
    import PanelView from '$lib/components/global/panelview/PanelView.svelte';
    import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { activeTab, addTab } from "$lib/stores/Tabs";
    import { get } from "svelte/store";
    import { page } from "$app/stores";
    import { repository } from "$lib/stores/Repository";
    import * as Resizable from "$lib/components/ui/resizable";
    import ObjectEditor from "$lib/components/global/object_editor/ObjectEditor.svelte";
    import type { Object, ObjectView } from "$lib/components/structs/Object";
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import ObjectExplorer from "$lib/components/global/object_explorer/ObjectExplorer.svelte";
    import { createDraftObject, createObject, readDraftObjects, readModule, readModuleFromPath, readObjects } from "$lib/controllers/Module";
    import type { Module } from "$lib/components/structs/Module";

    let selectedObject: ObjectView | null = null;
    let objects: ObjectView[] = [];

    let module: Module;

    let editPanelFlag: boolean = false;
    let treePanelFlag: boolean = false;
    let editModeFlag: boolean = true;

    let updateModuleFlag: boolean = false;

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

        let treePanelView: ToolbarGroupType = {
            items: [showTree],
            type: "buttonsGroup"
        }
    
        addToolbarItem(navigationGroup);
        addToolbarItem(newGroup);
        addToolbarItem(treePanelView);
    }

    function handleObjectCreation(event: any) {
        let obj = event.detail.obj.object;
        createObject(module.path, obj)
            .then(() => {
                selectedObject = null;
                editPanelFlag = false;
                updateModuleFlag = true;
            })
    }
    
    function handleObjectDraftCreation(event: any) {
        let obj = event.detail.obj.object;
        createDraftObject(module.path, obj)
            .then(() => {
                selectedObject = null;
                editPanelFlag = false;
                updateModuleFlag = true;
            })
        
    }
    
    function handleObjectExclusion(event: any) {
        let obj = event.detail;
        /* deleteObject(module.path, obj)
            .then(() => {
                selectedObject = null;
                editPanelFlag = false;
            }) */
        
    }
    
    function handleCloseEditPanel(event: any) {
        editPanelFlag = false;
    }

    function handleObjectSelect(event: any) {
        selectedObject = event.detail.object;
        editPanelFlag = true;
    }

    async function loadObjects(modPath: string) {
        let retObjects = await readObjects(modPath);
        let retDraftObjects = await readDraftObjects(modPath);
        let newObjects: ObjectView[] = [];

        retObjects.forEach((obj) => {
            let dob = {
                object: obj,
                isDraft: false,
                hasChanges: false,
                links: [],
            }
            newObjects.push(dob);
        });
        
        retDraftObjects.forEach((dobj) => {
            let index = objects.findIndex((ob) => {return (ob.object.id === dobj.id)});
            let dob = {
                object: dobj,
                isDraft: true,
                hasChanges: false,
                links: [],
            }

            if (index < 0) {
                newObjects.push(dob);
            } else {
                newObjects[index] = dob;
            }
        });
        objects = newObjects;
    }
    
    $: {
        if (updateModuleFlag && module) {
            loadObjects(module.path);
            updateModuleFlag = false;
        }
    }
    
    onMount(() => {
        const params = get(page).params;
        const url: string = $page.url.pathname;
        const name: string = params.mod.substring($repository?.tree.path.length);
        const version: string = params.version;
        addTab(name, "gravity-ui:layout-header-cells-large-fill", url, version);
        readModuleFromPath(params.mod)
            .then((md) => {
                module = md;
                loadObjects(md.path);
            })
            .catch((err) => {
                console.log(err)
            })
        loadHomeToolbar();
    })
</script>

<div class="bg-slate-50 h-full py-1">
    <Resizable.PaneGroup direction="horizontal">
        {#if treePanelFlag}
        <Resizable.Pane defaultSize={20} collapsible order={1}>
            <div></div>
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        {/if}
        <Resizable.Pane order={2}>
            {#if module}
            <ObjectExplorer prefix={module.manifest.prefix} separator={module.manifest.separator} bind:objects={objects} editMode={editModeFlag} on:click={handleObjectSelect}/>
            {/if}
        </Resizable.Pane>
        {#if editPanelFlag}
        <Resizable.Handle withHandle/>
        <Resizable.Pane class="h-full" defaultSize={50} order={3}>
            <ObjectEditor bind:objv={selectedObject} on:close={handleCloseEditPanel} on:saveDraft={handleObjectDraftCreation} on:save={handleObjectCreation} on:delete={handleObjectExclusion}/>
        </Resizable.Pane>
        {/if}
    </Resizable.PaneGroup>
</div>
