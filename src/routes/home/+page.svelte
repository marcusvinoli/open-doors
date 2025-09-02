<script lang="ts">
    import Icon from '@iconify/svelte';
    import TreeView from '$lib/components/global/treeview/TreeView.svelte';
    import PanelView from '$lib/components/global/panel_view/PanelView.svelte';

    import CreateFolderForms from '$lib/components/forms/folder/CreateFolderForms.svelte';
    import CreateProjectForms from "$lib/components/forms/project/CreateProjectForms.svelte"
    import CreateModuleForms from '$lib/components/forms/module/CreateModuleForms.svelte';

    import { app } from '$lib/stores/AppState.svelte';
    import { goto } from '$app/navigation';
    import { currentItem, goBack, goTo } from '$lib/stores/PanelView.svelte';
    import { onMount } from 'svelte';
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import { reloadRepository } from '$lib/controllers/Repository';
    import { addTab, setActiveTab } from '$lib/stores/Tabs.svelte';
    import { loadAuthorInformation } from '$lib/controllers/User';
    import { getIconFromTreeItemType } from '$lib/utils/tree-item-utils';
    import { addToolbarItem, clearToolbar } from '$lib/stores/Toolbar.svelte';
    
    import * as Resizable from "$lib/components/ui/resizable";
    
    import type { Module } from '$lib/components/structs/Module';
    import type { Project } from '$lib/components/structs/Project';
    import type { TreeItem } from '$lib/components/structs/Tree';
    import type { ToolbarGroupType, ToolbarButtonType, ToolbarDropdownType } from '$lib/components/global/toolbar/Toolbar';
    import Loading from '$lib/components/ui/loading/Loading.svelte';

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
        clearToolbar();

        let homeButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Home",
            icon: "gravity-ui:house",
            action: () => {
                goHome()
            },
        }

        let backButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Back",
            icon: "gravity-ui:arrow-left",
            action: () => {
                goBack()
            },
        }

        let newButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New...",
            icon: "gravity-ui:circle-plus",
            action: () => {},
        }

        let newProjectButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Project",
            icon: "gravity-ui:folder-open-fill",
            action: openNewProjectDialog,
        }

        let newFolderButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Folder",
            icon: "gravity-ui:folder-open",
            action: openNewFolderDialog,
        }

        let newModuleButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Module",
            icon: "gravity-ui:layout-header-cells-large-fill",
            action: openNewModuleDialog,
        }

        let creationGroup: ToolbarDropdownType = {
            button: newButton,
            items: [
                {
                    items: [
                        newProjectButton, 
                        newFolderButton, 
                        newModuleButton
                    ],
                    type: "buttonsGroup",
                }
            ],
            type: "dropdown",
        }

        let navigationGroup: ToolbarGroupType = {
            items: [homeButton, backButton],
            type: "buttonsGroup"
        }

        let newGroup: ToolbarGroupType = {
            items: [creationGroup],
            type: "buttonsGroup"
        }

        addToolbarItem(navigationGroup);
        addToolbarItem(newGroup);
    }

    async function loadHomepage() : Promise<void> {
        return new Promise((resolve, reject) => {
            reloadRepository().then(() => loadAuthorInformation());
            addTab("Home", "gravity-ui:house", "/home");
            loadHomeToolbar();
            setActiveTab("/home");
            return resolve();
        })
    }

    onMount(() => {
        loadHomepage();
    })

    const result = loadHomepage();

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
