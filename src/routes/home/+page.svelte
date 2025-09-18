<script lang="ts">
    import Icon from '@iconify/svelte';
    import Loading from '$lib/components/ui/loading/Loading.svelte';
    import TreeView from '$lib/components/global/treeview/TreeView.svelte';
    import PanelView from '$lib/components/global/panel_view/PanelView.svelte';
    import CreateFolderForms from '$lib/components/forms/folder/CreateFolderForms.svelte';
    import CreateModuleForms from '$lib/components/forms/module/CreateModuleForms.svelte';
    import CreateProjectForms from "$lib/components/forms/project/CreateProjectForms.svelte"

    import { app } from '$lib/stores/AppState.svelte';
    import { goto } from '$app/navigation';
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import { reloadRepository } from '$lib/controllers/Repository';
    import { getIconFromTreeItemType } from '$lib/utils/tree-item-utils';
    import { clearToolbar, setToolbar } from '$lib/stores/Toolbar.svelte';
    import { currentItem, goBack, goTo } from '$lib/stores/PanelView.svelte';
    
    import * as Resizable from "$lib/components/ui/resizable";
    
    import type { Module } from '$lib/components/structs/Module';
    import type { Project } from '$lib/components/structs/Project';
    import type { Toolbar } from '$lib/components/global/toolbar/Toolbar';
    import type { TreeItem } from '$lib/components/structs/Tree';

    import { load } from './+page';

    let newProjectDialog: boolean = $state(false);
    let newFolderDialog: boolean = $state(false);
    let newModuleDialog: boolean = $state(false);

    function openNewProjectDialog() {
        newProjectDialog = true;
    }

    function openNewFolderDialog() {
        newFolderDialog = true;
    }

    function openNewModuleDialog() {
        newModuleDialog = true;
    }

    function goHome() {
        goto("/home");
    }

    function onModuleCreation(mod: Module) {

    }

    function onFolderCreation(folderTreeItem: TreeItem) {
        reloadRepository()
        .then(() => {
            goTo(folderTreeItem);
        })
    }

    function onProjectCreation(project: Project) {
        reloadRepository()
        .then(() => {
            goTo(project.tree);
        })
    }

    function loadHomeToolbar() {
        const toolbar: Toolbar = {
            items: [
                {
                    type: 'group',
                    items: [
                        {
                            type: 'button',
                            tooltip: 'Home',
                            icon: 'gravity-ui:house',
                            onclick: () => {
                                goHome();
                            },
                            disabled: false,
                        },
                        {
                            type: 'button',
                            tooltip: 'Back',
                            icon: 'gravity-ui:arrow-left',
                            onclick: () => {
                                goBack()
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
                                icon: 'gravity-ui:circle-plus',
                                tooltip: 'New...',
                                disabled: false,
                            },
                            items: [
                                {
                                    type: 'button',
                                    tooltip: 'New Project',
                                    icon: 'gravity-ui:folder-open-fill',
                                    onclick: openNewProjectDialog,
                                    disabled: false,
                                },
                                {
                                    type: 'button',
                                    tooltip: 'New Folder',
                                    icon: 'gravity-ui:folder-open',
                                    onclick: openNewFolderDialog,
                                    disabled: false,
                                },
                                {
                                    type: 'button',
                                    tooltip: 'New Module',
                                    icon: 'gravity-ui:layout-header-cells-large-fill',
                                    onclick: openNewModuleDialog,
                                    disabled: false,
                                },
                            ]
                        },
                    ]
                }
            ]
        }
        setToolbar(toolbar);
    }
    
    let result: Promise<void> = load().then(() => {
        clearToolbar();
        loadHomeToolbar();
    });

</script>

{#await result}
    <div class="flex flex-col items-center justify-center bg-slate-50 text-slate-400 w-full h-full">
        <Loading />
        <h1 class="text-xl font-semibold my-1">LOADING</h1>
    </div>    
{:then _}
    <div class="bg-slate-50 h-full py-1">
        <CreateProjectForms bind:openDialog={newProjectDialog} currentParent={currentItem()} oncreateproject={onProjectCreation}/>
        <CreateFolderForms bind:openDialog={newFolderDialog} currentParent={currentItem()} oncreatefolder={onFolderCreation}/>
        <CreateModuleForms bind:openDialog={newModuleDialog} currentParent={currentItem()} oncreatedmodule={onModuleCreation}/> 
        <Resizable.PaneGroup direction="horizontal">
            <Resizable.Pane defaultSize={20} minSize={5}>
                <ScrollArea class="h-full">
                    <TreeView />            
                </ScrollArea>
            </Resizable.Pane>
            <Resizable.Handle withHandle/>
            <Resizable.Pane minSize={5}>
            {#if app.repository!.tree.children.length > 0}
                <div class="flex flex-col h-full text-sm">
                    <PanelView />
                </div>
            {:else}
                <div class="w-full h-full grow flex flex-col items-center justify-center text-slate-400 pb-[100px] rounded-lg">
                    <Icon icon={getIconFromTreeItemType(app.repository!.tree, true)} width="50px"/>
                    <h1 class="text-xl font-semibold my-1">EMPTY REPOSITORY</h1>
                </div>
            {/if}
            </Resizable.Pane>
        </Resizable.PaneGroup> 
    </div>
{:catch e}
    <div class="flex flex-col justify-center items-center w-full h-full text-slate-500">
        <Icon icon="mdi:dinosaur-pixel" width="50px"/>
        <h1 class="text-xl font-semibold my-1">OOPS! FAIL LOADING REPOSITORY...</h1>
        <div class="bg-red-100 border-red-900 rounded-sm text-red-800 mt-2 max-w-[80%] font-mono text-sm px-2 py-1 overflow-auto max-h-50">
            <div class="border-b-2 border-b-red-200">
                <p class="bold">Error Details:</p> 
            </div>
            <p>{e}</p>
        </div>
    </div>
{/await}
