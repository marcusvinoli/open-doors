<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from "$lib/components/ui/loading/Loading.svelte";
    import IndexTree from "$lib/components/global/indextree/IndexTree.svelte";
    import ObjectForm from "$lib/components/forms/object/ObjectForm.svelte";
    import DynamicTable from "$lib/components/global/object_explorer/DynamicTable.svelte";
    //import ToolbarGroup from "$lib/components/global/toolbar/ToolbarGroup.svelte";
    import BaselineForm from "$lib/components/forms/module/BaselineForm.svelte";
    //import ToolbarButton from "$lib/components/global/toolbar/ToolbarButton.svelte";
    import AttributesForm from "$lib/components/forms/module/AttributesForm.svelte";
    //import ToolbarDropdown from "$lib/components/global/toolbar/ToolbarDropdown.svelte";

    import { app, disposeModule, loadModule } from "$lib/stores/AppState.svelte";
    import { tick } from "svelte";
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
    import { addTab, setActiveTab } from "$lib/stores/Tabs.svelte";
    import { confirm, message } from '@tauri-apps/api/dialog';
    import { absolutePath, encodePath, relativePath } from "$lib/utils/path-handler";
    import { buildTreeIndex } from "$lib/utils/index-tree.utils";
    import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar.svelte";
    import { computeIndexLevelChild, computeIndexLevelSibilings, newObject } from "$lib/utils/object-utils";
    import { createBaseline, createDraftObject, createLink, createObject, deleteLink, deleteObject, exportCSV, exportXlsx, readModuleFromPath, readObjects, restoreObject, updateTemplate } from "$lib/controllers/Module";

    import * as Resizable from "$lib/components/ui/resizable";

    import { type View, defaultView } from "$lib/components/structs/View";

    import type { Link } from "$lib/components/structs/Link";
    import type { Task } from "$lib/components/structs/Task";
    import type { Module } from "$lib/components/structs/Module";
    import type { Object } from "$lib/components/structs/Object";
    import type { Template } from "$lib/components/structs/Template";
    import type { Baseline } from "$lib/components/structs/Baseline";
    import type { IndexItem } from "$lib/components/structs/IndexItem";
    import type { Repository } from "$lib/components/structs/Repo";
    import type { ModuleState, Linker } from "$lib/components/structs/States";
    import type { ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType, ToolbarToggleType } from "$lib/components/global/toolbar/Toolbar";
    
    const OBJECT_TABLE_ID = 'object-table';
    const OBJECT_TABLE_CONTAINER_SUFFIX = '-container';
    const INDEX_TREE_ID = 'index-tree';

    let repo: Repository | null = $derived(app.repository);
    let moduleState: ModuleState | null = $derived(app.currentModule);

    let view: View = $state(defaultView);
    let module: Module | null = $state(null);
    let objects: Object[] = $state([]);
    let selectedObject: Object | null = $state(null);
    let indexTree: IndexItem[] = $derived(buildTreeIndex([...objects]));
    let indexTreeState: Map<number, boolean> = $state(new Map());
    let linker: Linker | null = $state(app.linker);

    let templateFlag: boolean = $state(false);
    let readOnlyFlag: boolean = $state(false);
    let treePanelFlag: boolean = $state(false);
    let showLinksFlag: boolean = $state(false);
    let objectFormFlag: boolean = $state(false);
    let newBaselineFlag: boolean = $state(false);
    let showDeletionsFlag: boolean = $state(false);
    let showRowNumberFlag: boolean = $state(false);

    let forceScroll: {x: number, y: number} = $state({x: 0, y: 0});
    let objectsScroll: {x: number, y: number} = {x: 0, y: 0};
    let indexScroll: {x: number, y: number} = {x: 0, y: 0};

    let previousPageKey: string | null = null;
    let currentPageKey: string = $derived(generateModuleStateKey(page.params.mod!, page.params.version ?? 'current'));

    $effect(() => {
        const {mod, version} = page.params;
        if (mod || version) {
            previousPageKey = generateModuleStateKey(mod!, version ?? 'current');
            retrieveState(currentPageKey);
        }
    })
    
    $effect(() => {
        const url: string = page.url.pathname
        setActiveTab(url);
    })

    $effect(() => {
        const hash = page.url.hash;
        if(hash && hash !== "") {
            handleScrollObjectsIntoView(hash.slice(1));
        }
    })

    $effect.pre(() => {
        const {mod, version} = page.params;
        if (mod || version) {
            saveCurrentState(previousPageKey);
        }
        return () => {
            saveCurrentState(previousPageKey);
        }
    })

    async function handleExportCSV() {
        if (!module) {
            return;
        }
        const taskID = 'csvExporter' + currentPageKey;
        let csvTask: Task = {
            job: "CSV Exporter",
            status: "running",
            icon: "line-md:uploading-loop",
            tooltip: "Export " + module?.manifest.title + " to .CSV",
        };
        app.tasks.set(taskID, csvTask);
        exportCSV(module?.path)
            .then(() => {
                csvTask.status = "done";
            })
            .catch(() => {
                csvTask.status = "error";
            })
            .finally(() => {
                app.tasks.set(taskID, csvTask)
                tick().then(() => {
                    app.tasks.delete(taskID)
                })
            })
    }

    async function handleExportXLSX() {
        if (!module) {
            return;
        }
        const taskID = 'xlsxExporter' + currentPageKey;
        let csvTask: Task = {
            job: "XLSX Exporter",
            status: "running",
            icon: "line-md:uploading-loop",
            tooltip: "Export " + module?.manifest.title + " to .XLSX",
        };
        app.tasks.set(taskID, csvTask);
        exportXlsx(module?.path)
            .then(() => {
                csvTask.status = "done";
            })
            .catch(() => {
                csvTask.status = "error";
            })
            .finally(() => {
                app.tasks.set(taskID, csvTask)
                tick().then(() => {
                    app.tasks.delete(taskID)
                })
            })
    }

    function loadToolbar() {
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

        let templateManager: ToolbarButtonType = {
            type: "button",
            tooltip: "Custom Attributes",
            icon: "gravity-ui:rectangles-4",
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
            action: () => {
                newBaselineFlag = !newBaselineFlag;
            },
        }

        let newObjectButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Object",
            icon: "gravity-ui:square-chart-bar",
            action: () => {
                if (!objectFormFlag) {
                    if (objects.length === 0) {
                        selectedObject = newObject(module?.template);
                    } else {
                        const lastObject = objects[objects.length - 1];
                        let sibiling = computeIndexLevelSibilings(objects, lastObject.id as number);
                        selectedObject = newObject(module?.template, sibiling.parentId, sibiling.indexLevel);
                    }
                    objectFormFlag = true;
                }
            },
        }

        let exportButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Export module...",
            icon: "gravity-ui:file-arrow-right-out",
            action: () => {},
        }

        let exportExcelButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Microsoft Excel (.xlsx)",
            icon: "ph:microsoft-excel-logo-fill",
            action: () => {
                handleExportXLSX()
            },
        }

        let exportCSVButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Comma-Separeted Value (.csv)",
            icon: "ph:file-csv",
            action: () => {
                handleExportCSV()
            },
        }

        let readOnlyModeButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Toggle Edit Mode",
            icon: "lucide:pencil-off",
            action: () => {
                readOnlyFlag = true;
            },
        }

        let editModeButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Toggle Read-Only Mode",
            icon: "lucide:pencil",
            action: () => {
                readOnlyFlag = false;
            },
        }

        /* let viewModeButton: ToolbarToggleType = {
            type: "toggle",
            buttonOn: editModeButton,
            buttonOff: readOnlyModeButton,
            status: readOnlyFlag,
        } */

        let showDeletionsButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Showing deletions",
            icon: "gravity-ui:square-dashed-text",
            action: () => {
                showDeletionsFlag = true;
            },
        }

        let dontShowDeletionsButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Show deletions",
            icon: "gravity-ui:square-chart-bar",
            action: () => {
                showDeletionsFlag = false;
            },
        }

        let deletionsModeButton: ToolbarToggleType = {
            type: "toggle",
            buttonOn: showDeletionsButton,
            buttonOff: dontShowDeletionsButton,
            status: showDeletionsFlag,
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

        let expGroup: ToolbarDropdownType = {
            button: exportButton,
            items: [
                {
                    items: [
                        exportExcelButton,
                    ],
                    type: "buttonsGroup",
                },
                {
                    items: [
                        exportCSVButton,
                    ],
                    type: "buttonsGroup",
                },
            ],
            type: "dropdown"
        }
    
        let navigationGroup: ToolbarGroupType = {
            items: [homeButton],
            type: "buttonsGroup"
        }
    
        let newGroup: ToolbarGroupType = {
            items: [creationGroup],
            type: "buttonsGroup"
        }
        
        let exportGroup: ToolbarGroupType = {
            items: [expGroup],
            type: "buttonsGroup"
        }

        let viewGrouplView: ToolbarGroupType = {
            items: [showTree, 
                //viewModeButton, 
                deletionsModeButton],
            type: "buttonsGroup"
        }

        let templateButton: ToolbarGroupType = {
            items: [templateManager],
            type: "buttonsGroup"
        }

        addToolbarItem(navigationGroup);
        addToolbarItem(newGroup);
        addToolbarItem(viewGrouplView);
        addToolbarItem(exportGroup);
        addToolbarItem(templateButton);
    }

    async function handleObjectCreation(obj: Object) {
        try {
            await createObject(module!.path, obj);
            selectedObject = null;
            objectFormFlag = false;
            loadAllObjects(module!.path);
        } catch (err) {
            console.error(err);
        }
    }

    async function handleObjectDraftCreation(obj: Object) {
        try {
            const objs = await createDraftObject(module!.path, obj);
            selectedObject = null;
            objectFormFlag = false;
            await loadAllObjects(module!.path);
        } catch (err) {
            console.error(err);
        }
    }

    async function handleObjectExclusion(obj: Object) {
        const confirmed = await confirm('Do you really want to delete this Object?', 'Deleting object ' + module!.manifest.prefix + module!.manifest.separator + obj.id);
        if (!confirmed) {
            selectedObject = null;
            objectFormFlag = false;
            return;
        }
        return deleteObject(module!.path, obj.id)
            .then(() => {
                loadAllObjects(module!.path);
            })
            .catch((err) => {
                console.error(err);
            })
            .finally(() => {
                selectedObject = null;
                objectFormFlag = false;
            })
    }

    async function handleObjectRestoring(obj: Object) {
        const confirmed = await confirm('Do you really want to restore this Object?', 'Restoring object ' + module!.manifest.prefix + module!.manifest.separator + obj.id);
        if (!confirmed) {
            return;
        }
        restoreObject(module!.path, obj.id)
            .then(() => {
                objectFormFlag = true;
                loadAllObjects(module!.path);
            })
            .catch((err) => {
                console.error(err);
            })
            .finally(() => {
                selectedObject = null;
                objectFormFlag = false;
            })
    }

    async function handleTemplateUpdate(template: Template) {
        updateTemplate(module!.path, template)
            .then(template => {
                module!.template = template as Template;
            })
            .catch((err) => {
                console.error(err);
            })
    }

    async function handleLinkCreation() {
        if (((linker?.from?.object === linker?.to?.object) 
            && (linker?.from?.path === linker?.to?.path))) {
            await message('You cannot link an object to itself', { title: 'Error', type: 'error' });
            return;
        }
        if (linker && (!linker?.from || !linker.to)) {
            return;
        }
        const originModulePath: string = absolutePath(repo!.tree.path, linker!.from!.path);
        return createLink(originModulePath, linker!.from!, linker!.to!)
            .then(() => {
                loadAllObjects(module!.path);
            })
    }

    async function handleLinkDeletion(object: Object, modulePath: string, link: Link) {
        const response = await confirm('Are you sure you want to delete this link?', { title: 'Confirm link deletion'});
        if (response) {
            const origin: Link = {
                object: object.id,
                path: relativePath(repo!.tree.path, modulePath),
                baseline: page.params.version ?? 'current',
            }
            return deleteLink(modulePath, origin, link)
                .then(() => {
                    loadAllObjects(module!.path);
                })
                .catch((e) => {
                    console.error(e);
                })
        }
    }

    async function handleLinkVisit(link: Link) {
        const path = "/module/" + encodePath(repo?.tree.path + "/" + link.path) + "#" + link.object;
        await tick().then(() => {
            handleScrollObjectsIntoView(link.object);
        });
        goto(path);
    }

    async function handleBaselineCreation(baseline: Baseline) {
        createBaseline(module!.path, baseline)
            .then((baselines) => {
                module!.baselines = baselines as Baseline[]
            })
            .catch((err) => {
                console.error(err);
            })
    }

    function handleObjectSelection(id: string | number) {
        selectedObject = objects.find(obj => obj.id === id) ?? null;
        objectFormFlag = true;
    }

    function handleScrollObjectsIntoView(id: string | number) {
        const el = document.getElementById("row-" + id.toString());
        const ov = document.getElementById(OBJECT_TABLE_ID + "-container");
        const hd = document.getElementById(OBJECT_TABLE_ID + "-header");
        
        if (!el) return;
        if (!ov) return;
        if (!hd) return;
        
        const offset = hd.offsetHeight;
        ov.scrollTo({
            top: el.offsetTop - offset,
            behavior: 'smooth'
        });
        
        document.querySelectorAll('.flash').forEach(element => {
            element.classList.remove('flash');
        });
        el.classList.add('flash');
    }

    function handleObjectsScrolling(e: any) {
        objectsScroll = {x: e.target.scrollTop ?? 0, y: e.target.scrollLeft ?? 0};
    }

    function handleIndexScrolling(e: any) {
        indexScroll = {x: e.target.scrollTop ?? 0, y: e.target.scrollLeft ?? 0};
    }

    function setScrollPosition(id: string, x: number, y: number) {
        const ov = document.getElementById(id);
        if (!ov) { return };
        ov.scrollTo({top: x, left: y, behavior: 'instant'})
    }

    async function loadAllObjects(modPath: string) {
        readObjects(modPath)
            .then(objs => {
                objects = [...objs as Object[]];
                if(selectedObject) {
                    selectedObject = objects.find(obj => obj.id === selectedObject?.id) ?? null;
                }
            })
    }

    function generateModuleStateKey(mod: string, version: string): string {
        const dataFormat = `${mod.substring(repo!.tree.path.length)}-${version}`;
        const sanitized = dataFormat.toLowerCase().replace(/[^a-z0-9]/g, '');
        const truncated = sanitized.length > 30 ? sanitized.substring(0, 30) : sanitized;
        return truncated;
    }

    function retrieveState(key: string | null) {
        if (!key) {
            return;
        }
        let res = app.modules.get(currentPageKey);
        if (!res) {
            return;      
        }
        newBaselineFlag = res.flags.showNewBaselineDialog;
        templateFlag = res.flags.showTemplateDialog;
        showRowNumberFlag = res.flags.showRowsNumbering;
        objectFormFlag = res.flags.showObjectDialog;
        treePanelFlag = res.flags.showIndexPanel;
        showDeletionsFlag = res.flags.showDeletions;
        showLinksFlag = res.flags.showLinks;
        readOnlyFlag = res.flags.readOnly;
        module = res.module;
        objects = res.objects;
        view = res.currentView;
        selectedObject = res.currentObject;
        indexTree = res.indexTree.tree;
        indexTreeState = res.indexTree.state;
        setScrollPosition(INDEX_TREE_ID, res.indexTree.scroll.x, res.indexTree.scroll.y);
        setScrollPosition(OBJECT_TABLE_ID + OBJECT_TABLE_CONTAINER_SUFFIX, res.scroll.x, res.scroll.y);
        forceScroll = {x: res.scroll.x, y: res.scroll.y};
    }

    function saveCurrentState(key: string | null) {
        if (!key) {
            return;
        }
        const moduleState: ModuleState = {
            flags: {
                showNewBaselineDialog: newBaselineFlag,
                showTemplateDialog: templateFlag,
                showRowsNumbering: showRowNumberFlag,
                showObjectDialog: objectFormFlag,
                showIndexPanel: treePanelFlag,
                showDeletions: showDeletionsFlag,
                showLinks: showLinksFlag,
                readOnly: readOnlyFlag,
            },
            module: module!,
            objects: objects,
            currentView: view,
            currentObject: selectedObject,
            indexTree: {
                tree: indexTree,
                state: indexTreeState,
                scroll: indexScroll,
            },
            filter: null,
            scroll: objectsScroll,
        };
        app.modules.set(key, {...moduleState});
    }

    async function loadPage() {
        let { mod, version } = page.params;
        const hash = page.url.hash;
        const url: string = page.url.pathname;
        const name: string = mod!.substring(repo!.tree.path.length);
        const baseline: string = version ?? "current";
        loadToolbar();
        return loadModule(mod!, version)
            .then(() => {
                addTab(name, "gravity-ui:layout-header-cells-large-fill", url, baseline, () => disposeModule(mod!, version));
                tick().then(() => {
                    retrieveState(currentPageKey);
                    if(hash && hash !== "") {
                        handleScrollObjectsIntoView(hash.slice(1));
                    }
                })
            })
    }

    function contextClick(item: string, id: number | string, arg?: any) {
        switch(item) {
            case 'properties':
                selectedObject = objects.find(obj => (obj.id == id)) ?? null;
                objectFormFlag = true;
                break;
            case 'newObject':
                selectedObject = newObject(module?.template);
                objectFormFlag = true;
                break;
            case 'newObjectAfter': // Object on same level
                let sibiling = computeIndexLevelSibilings(objects, id as number);
                selectedObject = newObject(module?.template, sibiling.parentId, sibiling.indexLevel);
                objectFormFlag = true;
                break;
            case 'newObjectBelow': // Object as sub-level
                let subItem = computeIndexLevelChild(objects, id as number);
                selectedObject = newObject(module?.template, subItem.parentId, subItem.indexLevel);
                objectFormFlag = true;
                break;
            case 'createLink': 
                let newLinker = {
                    from: {
                        path: relativePath(app.repository!.tree.path, module!.path),
                        object: Number.parseInt(id.toString()),
                        baseline: page.params.version ?? 'current',
                    },
                    to: null,
                }
                app.linker = linker = newLinker;
                break;
            case 'stablishLink':
                if (!linker) {
                    return;
                }
                let destination: Link = {
                    path: relativePath(app.repository!.tree.path, module!.path),
                    object: Number.parseInt(id.toString()),
                    baseline: page.params.version ?? 'current',
                };
                linker.to = destination;
                handleLinkCreation()
                    .then(() => {
                        if (linker) {
                            linker.to = null;
                        }
                    }
                );
                break;
            case 'stopLinking': 
                app.linker = linker = null;
                break;
            default: 
                console.log(item, id, arg);
                break;
        }
    }

    let ready = loadPage();

</script>

{#if module}
    <ObjectForm
        bind:openDialog={objectFormFlag}
        object={selectedObject} 
        module={module!}
        onsave={handleObjectCreation}
        onsavedraft={handleObjectDraftCreation}
        ondelete={handleObjectExclusion}
        onrestore={handleObjectRestoring}
        onlinkvisit={handleLinkVisit}
        onunlink={handleLinkDeletion}
    />
    <BaselineForm 
        bind:openDialog={newBaselineFlag} 
        module={module!}
        onbaselinecreation={handleBaselineCreation}
    />
    <AttributesForm 
        bind:openDialog={templateFlag}
        module={module!}
        ontemplateupdate={handleTemplateUpdate}
    />
{/if}
<div class="bg-slate-50 h-full py-1">
    {#key (page.params.mod, page.params.version)}
        <Resizable.PaneGroup direction="horizontal">
            {#if treePanelFlag}
            <Resizable.Pane defaultSize={20} maxSize={40} collapsible order={1}>
                <IndexTree
                id={INDEX_TREE_ID}
                bind:trees={indexTree} 
                bind:state={indexTreeState} 
                onclick={handleScrollObjectsIntoView}
                onscroll={handleIndexScrolling}
                />
            </Resizable.Pane>
            <Resizable.Handle withHandle/>
            {/if}
            <Resizable.Pane order={2} defaultSize={80}>
                {#await ready}
                <div class="flex flex-col justify-center items-center w-full h-full text-slate-500">
                    <Loading/>
                    <h1 class="font-semibold">LOADING OBJECTS...</h1>
                </div>
                {:then}
                <DynamicTable
                id={OBJECT_TABLE_ID}
                view={view} 
                module={module!} 
                objects={objects}
                readOnly={readOnlyFlag} 
                linker={linker}
                oncontextclick={contextClick}
                ondblclick={handleObjectSelection}
                onscroll={handleObjectsScrolling}
                scroll={forceScroll}
                showDeletions={showDeletionsFlag}
                bind:selectedObject={selectedObject} 
                />
                {:catch e}
                <div class="flex flex-col justify-center items-center w-full h-full text-slate-500">
                    <Icon icon="mdi:dinosaur-pixel" width="50px"/>
                    <h1 class="text-xl font-semibold my-1">OOPS! SOMETHING WENT WRONG...</h1>
                    <div class="bg-red-100 border-red-900 rounded-sm text-red-800 mt-2 max-w-[80%] font-mono text-sm px-2 py-1 overflow-auto max-h-50">
                        <div class="border-b-2 border-b-red-200">
                            <p class="bold">Error Details:</p> 
                        </div>
                        <p>{e}</p>
                    </div>
                </div>
                {/await}
            </Resizable.Pane>
        </Resizable.PaneGroup>
    {/key}
</div>
