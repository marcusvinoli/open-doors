<script lang="ts">
    import { onMount } from 'svelte';
    import Icon from '@iconify/svelte';
    import Tree from "$lib/components/global/treeview/TreeView.svelte";
    import * as Resizable from "$lib/components/ui/resizable";
    import type { TabData } from "$lib/components/global/tabs/TabData";
    import { newTab, repository } from '../store';
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import CreateProjectForms from "$lib/components/forms/CreateProjectForms.svelte"
    import { goto } from '$app/navigation';
    import { reloadRepository } from '$lib/controllers/Repository';
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import PanelView from '$lib/components/global/panelview/PanelView.svelte';
    import ToolbarButton from '$lib/components/global/toolbar/ToolbarButton.svelte';
    import type { ToolbarGroupType, ToolbarButtonType, ToolbarDropdownType, ToolbarItemInterface } from '$lib/components/global/toolbar/Toolbar';
    import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar";
    import ToolbarDropdown from '$lib/components/global/toolbar/ToolbarDropdown.svelte';
    import ToolBar from '$lib/components/global/toolbar/Toolbar.svelte';

    let newProjectDialog = false;

    function openNewProjectDialog() {
        newProjectDialog = true;
    }

    function goHome() {
        goto("/home");
    }

    function updateTreeView(event: any) {
        //readStructure(event.detail.item);
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
            action: () => {
                openNewProjectDialog()
            },
        }

        let newFolderButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Folder",
            icon: "gravity-ui:folder-open",
            action: () => {
                console.log("Create a new folder")
            },
        }

        let newModuleButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Module",
            icon: "gravity-ui:layout-header-cells-large-fill",
            action: () => {
                console.log("Create a new module")
            },
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
            items: [homeButton],
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
        let homeTab: TabData = {
            path: "/home",
            icon: "gravity-ui:house",
            title: "Home",
        };
        $newTab = homeTab;
    })

</script>

<div class="bg-slate-50 h-full py-1">
    <CreateProjectForms bind:openDialog={newProjectDialog} on:create={goHome}/>
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={20} minSize={5}>
            <ScrollArea class="h-full">
                <Tree treeItems={$repository?.structure} on:itemSelected={updateTreeView}/>
            </ScrollArea>
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        <Resizable.Pane minSize={5}>
            {#if $repository?.structure.children.length > 0}
                <div>Test</div>
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