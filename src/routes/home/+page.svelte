<script lang="ts">
    import Icon from '@iconify/svelte';
    import Tree from "$lib/components/global/treeview/TreeView.svelte";
    import PanelView from '$lib/components/global/panelview/PanelView.svelte';
    import CreateFolderForms from '$lib/components/forms/folder/CreateFolderForms.svelte';
    import CreateProjectForms from "$lib/components/forms/project/CreateProjectForms.svelte"
    import { goto } from '$app/navigation';
    import { Button } from "$lib/components/ui/button/index.js";
    import { onMount } from 'svelte';
    import { repository } from '$lib/stores/Repository';
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import { reloadRepository } from '$lib/controllers/Repository';
    import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar";
    import { treeHistory, goBack, goIn, currentItem } from '$lib/stores/PanelView';
    import * as Resizable from "$lib/components/ui/resizable";
    import type { TabData } from "$lib/components/global/tabs/TabData";
    import type { TreeItem } from '$lib/components/structs/Tree';
    import type { ToolbarGroupType, ToolbarButtonType, ToolbarDropdownType } from '$lib/components/global/toolbar/Toolbar';
    import CreateModuleForms from '$lib/components/forms/module/CreateModuleForms.svelte';
    import { addTab } from '$lib/stores/Tabs';
    import { loadAuthorInformation } from '$lib/controllers/User';


    let newProjectDialog: boolean = false;
    let newFolderDialog: boolean = false;
    let newModuleDialog: boolean = false;

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

    function updateTreeView(event: any) {
        let currentSelection = event.detail.item as TreeItem;
        goIn(currentSelection);
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
            action: openNewModuleDialog,
        }

        let newProjectButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Project",
            icon: "gravity-ui:folder-open-fill",
            action: () => {},
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

    onMount(() => {
        reloadRepository();
        loadHomeToolbar();
        loadAuthorInformation();
        addTab("Home", "gravity-ui:house", "/home");
    })

</script>

<div class="bg-slate-50 h-full py-1">
    <CreateProjectForms bind:openDialog={newProjectDialog} on:create={goHome} selectedParent={$repository?.tree}/>
    <CreateFolderForms bind:openDialog={newFolderDialog} on:create={goHome} selectedParent={$repository?.tree}/>
    <CreateModuleForms bind:openDialog={newModuleDialog} on:create={goHome} selectedParent={$repository?.tree}/>
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={20} minSize={5}>
            <ScrollArea class="h-full">
                <Tree treeItems={$repository?.tree} on:itemSelected={updateTreeView}/>
            </ScrollArea>
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        <Resizable.Pane minSize={5}>
        {#if $repository?.tree.children.length > 0}
            <div class="flex flex-col h-full text-sm">
                {#if $currentItem}
                    <PanelView currentItem={$currentItem} treeHistory={$treeHistory} on:deleted={goBack}/>
                {/if}
            </div>
        {:else}
            <div class="w-full flex flex-col text-center items-center text-slate-500 py-10">
            <div class="my-5">
                <Icon icon="gravity-ui:folder-exclamation" width="50px"/>
            </div>
            <h1 class="font-semibold ">It seems that there is no project on this Repository...</h1>
            <h2 class="font-regular pb-3">Let's create the first one!</h2>
            <Button on:click={openNewProjectDialog}>
                <Icon icon="gravity-ui:folder-plus" width="20px"/>
                <p class="pl-2">New Project</p>
            </Button>
            </div>
        {/if}
        </Resizable.Pane>
    </Resizable.PaneGroup> 
</div>
                        