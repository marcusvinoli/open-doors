<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from "$lib/components/ui/loading/Loading.svelte";
    import IndexTree from "$lib/components/global/index_tree/IndexTree.svelte";
    import ObjectForm from "$lib/components/forms/object/ObjectForm.svelte";
    import DynamicTable from "$lib/components/global/object_explorer/DynamicTable.svelte";
    import BaselineForm from "$lib/components/forms/module/BaselineForm.svelte";
    import AttributesForm from "$lib/components/forms/module/AttributesForm.svelte";

    import { app } from "$lib/stores/AppState.svelte";
    import { onMount, tick } from "svelte";
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
    import { setActiveTab } from "$lib/stores/Tabs.svelte";
    import { confirm, message } from '@tauri-apps/api/dialog';
    import { absolutePath, encodePath, relativePath } from "$lib/utils/path-handler";
    import { buildTreeIndex } from "$lib/utils/index-tree.utils";
    import { clearToolbar, setToolbar } from "$lib/stores/Toolbar.svelte";
    import { computeIndexLevelChild, computeIndexLevelSibilings, newObject } from "$lib/utils/object-utils";
    import { createBaseline, createDraftObject, createLink, createObject, deleteLink, deleteObject, exportCSV, exportXlsx, readObjects, restoreObject, updateTemplate, updateViews } from "$lib/controllers/Module";

    import * as Resizable from "$lib/components/ui/resizable";

    import { type View, defaultView } from "$lib/components/structs/View";
    import type { Link } from "$lib/components/structs/Link";
    import type { Task } from "$lib/components/structs/Task";
    import type { Module } from "$lib/components/structs/Module";
    import type { Object } from "$lib/components/structs/Object";
    import type { Template } from "$lib/components/structs/Template";
    import type { Baseline } from "$lib/components/structs/Baseline";
    import type { PageProps } from './$types';
    import type { IndexItem } from "$lib/components/structs/IndexItem";
    import type { Repository } from "$lib/components/structs/Repo";
    import type { ModuleState, Linker, ModuleFlags } from "$lib/components/structs/States";
    import type { Toolbar, ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType, ToolbarItemType, ToolbarToggleType } from "$lib/components/global/toolbar/Toolbar";
    import ViewsForm from "$lib/components/forms/module/ViewsForm.svelte";
    import ToolbarItem from "$lib/components/global/toolbar/ToolbarItem.svelte";
    import ToolbarButton from "$lib/components/global/toolbar/ToolbarButton.svelte";
    
    const OBJECT_TABLE_ID = 'object-table';
    const OBJECT_TABLE_CONTAINER_SUFFIX = '-container';
    const INDEX_TREE_ID = 'index-tree';

    let { data }: PageProps = $props();

    let repo: Repository | null = $derived(app.repository);
    let view: View = $state(defaultView);
    let module: Module | null = $state(null);
    let objects: Object[] = $state([]);
    let selectedObject: Object | null = $state(null);
    let linker: Linker | null = $state(app.linker);
    let tasks: Map<string, Task> = $state(app.currentModule?.tasks ?? new Map());
    let indexTree: IndexItem[] = $derived(buildTreeIndex([...objects]));
    let indexTreeState: Map<number, boolean> = $state(new Map());
    let context: Map<string, string> = $state(new Map());
    let objectsScroll: {x: number, y: number} = {x: 0, y: 0};
    let indexScroll: {x: number, y: number} = {x: 0, y: 0};
    let flags: ModuleFlags = $state({
        showNewBaselineDialog: false,
        showTemplateDialog: false,
        showRowsNumbering: false,
        showObjectDialog: false,
        showViewsDialog: false,
        showIndexPanel: false,
        showDeletions: false,
        showLinks: false,
        readOnly: false,
    });

    let previousPageKey: string | null = null;
    let currentPageKey: string = $derived(generateModuleStateKey(page.params.mod!, page.params.version ?? 'current'));

    $effect(() => {
        const mod: string = page.params.mod!;
        if (mod) {
            previousPageKey = generateModuleStateKey(mod, 'current');
            retrieveState(currentPageKey);
        }
    })
    
    $effect(() => {
        const url: string = page.url.pathname;
        setActiveTab(url);
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

    let toolbar: Toolbar = $state({
        items: [
            {
                type: 'group',
                items: [
                    {
                        type: 'button',
                        tooltip: 'Home',
                        icon: 'gravity-ui:house',
                        onclick: () => {
                            goto("/home")
                        },
                        disabled: false,
                    },
                    {
                        type: 'button',
                        tooltip: 'Show/Hide index panel',
                        icon: 'gravity-ui:layout-header-side-content',
                        onclick: () => {
                            flags.showIndexPanel = !flags.showIndexPanel;
                        },
                        disabled: false,
                    }
                ]
            },
            {
                type: 'group',
                items: [
                    {
                        type: 'dropdown',
                        button: {
                            type: 'button',
                            tooltip: 'New...',
                            icon: 'gravity-ui:circle-plus',
                            disabled: false,
                        },
                        items: [
                            {
                                type: 'button',
                                tooltip: 'New Object',
                                icon: 'gravity-ui:square-chart-bar',
                                onclick: () => {
                                    if (!flags.showObjectDialog) {
                                        if (objects.length === 0) {
                                            selectedObject = newObject(module?.template);
                                        } else {
                                            const lastObject = objects[objects.length - 1];
                                            let sibiling = computeIndexLevelSibilings(objects, lastObject.id as number);
                                            selectedObject = newObject(module?.template, sibiling.parentId, sibiling.indexLevel);
                                        }
                                        flags.showObjectDialog = true;
                                    }
                                },
                                get disabled() {
                                    return flags.readOnly;
                                }
                            },
                            {
                                type: 'button',
                                tooltip: 'New Baseline',
                                icon: 'gravity-ui:tag',
                                onclick: () => {
                                    flags.showNewBaselineDialog = !flags.showNewBaselineDialog;
                                },
                                disabled: false,
                            }
                        ]
                    },
                    {
                        type: 'toggle',
                        buttonTrue: {
                            type: 'button',
                            tooltip: 'Switch to Edit mode',
                            icon: 'ph:pencil-simple-slash-bold',
                            disabled: false,
                        },
                        buttonFalse: {
                            type: 'button',
                            tooltip: 'Switch to Read-Only mode',
                            icon: 'ph:pencil-simple-bold',
                            disabled: false,
                        },
                        get status() {
                            return flags.readOnly;
                        },
                        onchange: (status: boolean) => {
                            flags.readOnly = status;
                        }
                    }
                ]
            },
            {
                type: 'group',
                items: [
                    {
                        type: 'dropdown',
                        button: {
                            type: 'button',
                            tooltip: 'Views',
                            icon: 'gravity-ui:layout-list',
                            disabled: false,
                        },
                        items: [
                            {
                                type: 'group',
                                items: [
                                    {
                                        type: 'dropdown',
                                        button: {
                                            type: 'button',
                                            tooltip: 'Select View...',
                                            disabled: false,
                                        },
                                        get items() {
                                            let readOnlyView: ToolbarButtonType = {
                                                type: 'button',
                                                tooltip: 'Default View',
                                                disabled: false,
                                                onclick() {
                                                    view = defaultView
                                                }
                                            }
                                            let result: ToolbarItemType[] = [ readOnlyView ];
                                            module!.views.forEach(v => {
                                                const button: ToolbarButtonType = {
                                                    type: 'button',
                                                    tooltip: v.name,
                                                    disabled: false,
                                                    onclick() {
                                                        view = {...v};
                                                    }
                                                }
                                                result.push(button);
                                            })
                                            return result;
                                        }
                                    },
                                    {
                                        type: 'button',
                                        tooltip: 'View settings',
                                        disabled: false,
                                        onclick() {
                                            flags.showViewsDialog = true;
                                        },
                                    },
                                ],
                            },
                            // TODO: Reserved for Future Implementations
                            /*
                            {
                                type: 'group',
                                items: [
                                    {
                                        type: 'button',
                                        tooltip: 'Show or Hide deleted objects',
                                        onclick: () => {
                                            flags.showDeletions = !flags.showDeletions;
                                        },
                                        disabled: false,
                                    },
                                    {
                                        type: 'button',
                                        tooltip: 'Show or Hide deleted objects',
                                        onclick: () => {
                                            flags.showDeletions = !flags.showDeletions;
                                        },
                                        disabled: false,
                                    },
                                ],
                            },
                            */
                        ]
                    }, 
                   
                    {
                        type: 'button',
                        tooltip: 'Module Attributes',
                        icon: 'gravity-ui:rectangles-4',
                        onclick: () => {
                            flags.showTemplateDialog = !flags.showTemplateDialog;
                        },
                        disabled: false,
                    },
                    {
                        type: 'toggle',
                        buttonTrue: {
                            type: 'button',
                            tooltip: 'Hide deleted objects',
                            icon: 'gravity-ui:square-dashed-text',
                            disabled: false,
                        },
                        buttonFalse: {
                            type: 'button',
                            tooltip: 'Show deleted objects',
                            icon: 'gravity-ui:square-chart-bar',
                            disabled: false,
                        },
                        get status() { 
                            return flags.showDeletions; 
                        },
                        onchange: (status: boolean) => {
                            flags.showDeletions = status;
                        },
                    },
                    // TODO: Reserved for Future Implementations
                    /* {
                        type: 'toggle',
                        buttonTrue: {
                            type: 'button',
                            tooltip: 'Apply filter',
                            icon: 'gravity-ui:funnel',
                            disabled: false,
                        },
                        buttonFalse: {
                            type: 'button',
                            tooltip: 'Remove filter',
                            icon: 'gravity-ui:funnel-xmark',
                            disabled: false,
                        },
                        get status() { 
                            
                        },
                        onchange: (status: boolean) => {
                            
                        },
                    }, 
                    */
                ]
            },
            {
                type: 'group',
                items: [
                    {
                        type: 'dropdown',
                        button: {
                            type: 'button',
                            tooltip: 'Export...',
                            icon: 'gravity-ui:file-arrow-right-out',
                            onclick: () => {},
                            disabled: false,
                        },
                        items: [
                            {
                                type: 'button',
                                tooltip: 'Microsoft Excel (.xlsx)',
                                icon: 'ph:microsoft-excel-logo-fill',
                                onclick: () => {
                                    handleExportXLSX()
                                },
                                disabled: false,
                            },
                            {
                                type: 'button',
                                tooltip: 'Comma-Separeted Value (.csv)',
                                icon: 'ph:file-csv',
                                onclick: () => {
                                    handleExportCSV()
                                },
                                disabled: false,
                            }
                        ]
                    },
                ]
            },
        ]
    });

    function addModuleTask(task: Task) {
        tasks.set(task.id, task);
        app.currentModule!.tasks = new Map(tasks);
    }
    
    function removeModuleTask(id: string) {
        tasks.delete(id);
        app.currentModule!.tasks = new Map(tasks);
    }

    function addGlobalTask(task: Task) {
        const id = task.id + "_" + currentPageKey;
        app.tasks.set(id, task);
        app.tasks = new Map(app.tasks);
    }

    function removeGlobalTask(id: string) {
        app.tasks.delete(id + "_" + currentPageKey);
        app.tasks = new Map(app.tasks);
    }

    async function handleExportCSV() {
        if (!module) {
            return;
        }
        const taskId = 'csv_exporting';
        let csvTask: Task = {
            id: taskId,
            job: "CSV Exporter",
            status: "running",
            icon: "line-md:uploading-loop",
            tooltip: "Export " + module?.manifest.title + " to .CSV",
        };
        addGlobalTask(csvTask);
        exportCSV(module?.path)
            .then(() => {
                csvTask.status = "done";
            })
            .catch(() => {
                csvTask.status = "error";
            })
            .finally(() => {
                addGlobalTask(csvTask)
                tick().then(() => {
                    removeGlobalTask(taskId);
                })
            })
    }

    async function handleExportXLSX() {
        if (!module) {
            return;
        }
        const taskId= 'xlsx_exporting';
        let csvTask: Task = {
            id: taskId,
            job: "XLSX Exporter",
            status: "running",
            icon: "line-md:uploading-loop",
            tooltip: "Export " + module?.manifest.title + " to .XLSX",
        };
        addGlobalTask(csvTask);
        exportXlsx(module?.path)
            .then(() => {
                csvTask.status = "done";
            })
            .catch(() => {
                csvTask.status = "error";
            })
            .finally(() => {
                addGlobalTask(csvTask);
                tick().then(() => {
                    removeGlobalTask(taskId);
                })
            })
    }

    async function handleObjectCreation(obj: Object) {
        try {
            await createObject(module!.path, obj);
            selectedObject = null;
            flags.showObjectDialog = false;
            loadAllObjects(module!.path);
        } catch (err) {
            console.error(err);
        }
    }

    async function handleObjectDraftCreation(obj: Object) {
        try {
            const objs = await createDraftObject(module!.path, obj);
            selectedObject = null;
            flags.showObjectDialog = false;
            await loadAllObjects(module!.path);
        } catch (err) {
            console.error(err);
        }
    }

    async function handleObjectExclusion(obj: Object) {
        const confirmed = await confirm('Do you really want to delete this Object?', 'Deleting object ' + module!.manifest.prefix + module!.manifest.separator + obj.id);
        if (!confirmed) {
            selectedObject = null;
            flags.showObjectDialog = false;
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
                flags.showObjectDialog = false;
            })
    }

    async function handleObjectRestoring(obj: Object) {
        const confirmed = await confirm('Do you really want to restore this Object?', 'Restoring object ' + module!.manifest.prefix + module!.manifest.separator + obj.id);
        if (!confirmed) {
            return;
        }
        restoreObject(module!.path, obj.id)
            .then(() => {
                flags.showObjectDialog = true;
                loadAllObjects(module!.path);
            })
            .catch((err) => {
                console.error(err);
            })
            .finally(() => {
                selectedObject = null;
                flags.showObjectDialog = false;
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

    async function handleViewsUpdate(views: View[]) {
        updateViews(module!.path, views)
            .then(views => {
                module!.views = views as View[]
            })
            .catch(err => {
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
        flags.showObjectDialog = true;
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

    async function retrieveState(key: string | null) {
        if (!key) {
            return;
        }
        let res = app.modules.get(currentPageKey);
        if (!res) {
            return;      
        }
        flags = res.flags;
        module = res.module;
        objects = res.objects;
        view = res.currentView;
        selectedObject = res.currentObject;
        indexTree = res.indexTree.tree;
        indexTreeState = res.indexTree.state;
        context = res.context;
        tasks = res.tasks;
        await tick().then(() => {
            setScrollPosition(INDEX_TREE_ID, res.indexTree.scroll.x, res.indexTree.scroll.y);
            setScrollPosition(OBJECT_TABLE_ID + OBJECT_TABLE_CONTAINER_SUFFIX, res.scroll.x, res.scroll.y);
            if(page.url.hash) {
                const hash = page.url.hash.slice(1);
                handleScrollObjectsIntoView(hash);
            }
        })
    }

    function saveCurrentState(key: string | null) {
        if (!key) {
            return;
        }
        const moduleState: ModuleState = {
            flags,
            objects,
            module: module!,
            currentView: view,
            currentObject: selectedObject,
            indexTree: {
                tree: indexTree,
                state: indexTreeState,
                scroll: indexScroll,
            },
            filter: null,
            scroll: objectsScroll,
            context: context,
            tasks: tasks,
        };
        app.modules.set(key, {...moduleState});
    }

    function contextClick(item: string, id: number | string, arg?: any) {
        switch(item) {
            case 'properties': {
                selectedObject = objects.find(obj => (obj.id === id)) ?? null;
                flags.showObjectDialog = true;
                break;
            }
            case 'newObject': {
                selectedObject = newObject(module?.template);
                flags.showObjectDialog = true;
                break;
            }
            case 'newObjectAfter': { // Object on same level
                let indexes = computeIndexLevelSibilings(objects, id as number);
                selectedObject = newObject(module?.template, indexes.parentId, indexes.indexLevel);
                flags.showObjectDialog = true;
                break;
            }
            case 'newObjectBelow': { // Object as sub-level
                let indexes = computeIndexLevelChild(objects, id as number);
                selectedObject = newObject(module?.template, indexes.parentId, indexes.indexLevel);
                flags.showObjectDialog = true;
                break;
            }
            case 'createLink': {
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
            }
            case 'stablishLink': {
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
            }
            case 'stopLinking': {
                app.linker = linker = null;
                break;
            }
            case 'startMoving': {
                selectedObject = objects.find(obj => (obj.id === id)) ?? null;
                context.set('moving', id.toString());
                const taskId = 'moving_' + currentPageKey;
                const task: Task = {
                    id: taskId,
                    job: "Moving Object",
                    status: "running",
                    icon: "gravity-ui:arrow-up-arrow-down",
                    tooltip: "Moving object " + module?.manifest.prefix + module?.manifest.separator + id,
                };
                addModuleTask(task);
                break;
            }
            case 'moveAfter': {
                const movingObjectId: number = context.get('moving') ? Number.parseInt(context.get('moving')!) : -1;
                let movingObject: Object | null = objects.find(obj => (obj.id === movingObjectId)) ?? null;
                if (!movingObject) {
                    context.delete('moving');
                    removeModuleTask('moving_' + currentPageKey);
                    return;
                }
                let moveSibilings = computeIndexLevelSibilings(objects, id as number);
                movingObject.indexParentId = moveSibilings.parentId;
                movingObject.indexLevel = moveSibilings.indexLevel;
                handleObjectDraftCreation(movingObject!).then(() => {
                    context.delete('moving');
                    removeModuleTask('moving_' + currentPageKey);
                });
                break;
            }
            case 'moveBelow': {
                const movingObjectId: number = context.get('moving') ? Number.parseInt(context.get('moving')!) : -1;
                let movingObject: Object | null = objects.find(obj => (obj.id === movingObjectId)) ?? null;
                if (!movingObject || movingObject.id === id) {
                    context.delete('moving');
                    removeModuleTask('moving_' + currentPageKey);
                    return;
                }
                let moveChild = computeIndexLevelChild(objects, id as number);
                movingObject.indexParentId = moveChild.parentId;
                movingObject.indexLevel = moveChild.indexLevel;
                handleObjectDraftCreation(movingObject!).then(() => {
                    context.delete('moving');
                    removeModuleTask('moving_' + currentPageKey);
                });
                break;
            }
            case 'stopMoving': {
                selectedObject = null;
                context.delete('moving');
                const taskId = 'moving_' + currentPageKey;
                removeModuleTask('moving_' + currentPageKey);
                break;
            }
            default: 
                console.error('Unrecognized parameters: ', item, id, arg);
                break;
        }
    }

    onMount(() => {
        clearToolbar();
        setToolbar(toolbar);
        //loadToolbar();
    })

</script>

{#if module}
    {@const mod = module}
    <ObjectForm
        bind:openDialog={flags.showObjectDialog}
        object={selectedObject} 
        module={mod}
        onsave={handleObjectCreation}
        readOnly={flags.readOnly}
        onsavedraft={handleObjectDraftCreation}
        ondelete={handleObjectExclusion}
        onrestore={handleObjectRestoring}
        onlinkvisit={handleLinkVisit}
        onunlink={handleLinkDeletion}
    />
    <ViewsForm 
        bind:openDialog={flags.showViewsDialog}
        readOnly={flags.readOnly}
        module={mod}
        onviewsupdate={handleViewsUpdate}
    />
    <BaselineForm 
        bind:openDialog={flags.showNewBaselineDialog} 
        module={mod}
        onbaselinecreation={handleBaselineCreation}
    />
    <AttributesForm 
        bind:openDialog={flags.showTemplateDialog}
        module={mod}
        readOnly={flags.readOnly}
        ontemplateupdate={handleTemplateUpdate}
    />
{/if}
<div class="bg-slate-50 h-full py-1">
    <Resizable.PaneGroup direction="horizontal">
        {#if flags.showIndexPanel}
            <Resizable.Pane defaultSize={20} maxSize={40} collapsible order={1}>
                <IndexTree
                    id={INDEX_TREE_ID}
                    trees={indexTree} 
                    bind:state={indexTreeState}
                    showDeletions={flags.showDeletions}
                    onclick={handleScrollObjectsIntoView}
                    onscroll={handleIndexScrolling}
                />
            </Resizable.Pane>
            <Resizable.Handle withHandle/>
        {/if}
            <Resizable.Pane order={2} defaultSize={80}>
                {#await data}
                <div class="flex flex-col justify-center items-center w-full h-full text-slate-500">
                    <Loading/>
                    <h1 class="font-semibold">LOADING OBJECTS...</h1>
                </div>
                {:then}
                <DynamicTable
                    id={OBJECT_TABLE_ID}
                    module={module!}
                    objects={objects}
                    readOnly={flags.readOnly}
                    linker={linker}
                    oncontextclick={contextClick}
                    ondblclick={handleObjectSelection}
                    onscroll={handleObjectsScrolling}
                    showDeletions={flags.showDeletions}
                    context={context}
                    bind:view={view}
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
</div>
