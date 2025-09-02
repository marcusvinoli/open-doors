<script lang="ts">
    import Icon from "@iconify/svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import FolderForm from "$lib/components/forms/folder/FolderForm.svelte";
    import ModuleForm from "$lib/components/forms/module/ModuleForm.svelte";
    import ProjectForm from "$lib/components/forms/project/ProjectForm.svelte";
    import ModulePanelView from "./ModulePanelView.svelte";
    import CreateModuleForms from "$lib/components/forms/module/CreateModuleForms.svelte";
    import CreateFolderForms from "$lib/components/forms/folder/CreateFolderForms.svelte";
    import CreateProjectForms from "$lib/components/forms/project/CreateProjectForms.svelte";
    import BaselineForm from "$lib/components/forms/module/BaselineForm.svelte";
    
    import { goto } from "$app/navigation";
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import { encodePath } from "$lib/utils/path-handler";
    import { deleteModule } from "$lib/controllers/Module";
    import { reloadRepository } from "$lib/controllers/Repository";
    import { getIconFromTreeItemType } from "$lib/utils/tree-item-utils";
    import { currentItem as getCurrentItem, treeHistory as getTreeHistory, goTo, goBack } from "$lib/stores/PanelView.svelte";
    
    import type { TreeItem } from "$lib/components/structs/Tree";
    import type { Project } from "$lib/components/structs/Project";

    import * as Tooltip from "$lib/components/ui/tooltip";
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js"; 
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
   
    let {
        onitemupdate,
        onitemdelete,
        onitemcreate,
    } : {
        onitemupdate?: (treeItem: TreeItem) => void,
        onitemdelete?: (treeItem: TreeItem) => void,
        onitemcreate?: (treeItem: TreeItem) => void,
    } = $props();
    
    let projectFormDialog: boolean = $state(false);
    let folderFormDialog: boolean = $state(false);
    let moduleFormDialog: boolean = $state(false);
    let baselineDialog: boolean = $state(false);
    let editDialog: boolean = $state(false);
    let currentItem: TreeItem | null = $derived(getCurrentItem());
    let treeHistory: TreeItem[] = $derived(getTreeHistory());
    
    function handleItemSelection(item: any) {
        goTo(item);
    }

    function openCurrent() {
        goto("/module/" + encodePath(currentItem!.path))
    }

    function handleOpenBaseline(baseline: string) {
        goto("/module/" + encodePath(currentItem!.path) + "/baseline/" + baseline);
    }

    function handleFolderCreation(folder: TreeItem) {
        reloadRepository()
            .then(() => {
                goTo(folder);
            })
    }

    function handleFolderDeletion(folder: TreeItem) {
        reloadRepository()
            .then(() => {
                goBack();
            })
    }

    function handleFolderUpdate(folder: TreeItem) {
        reloadRepository()
            .then(() => {
                goTo(folder);
            })
    }

    function handleProjectCreation(project: Project) {
        reloadRepository()
            .then(() => {
                goTo(project.tree);
            })
    }

    function handleProjectUpdate(project: TreeItem) {
        reloadRepository()
            .then(() => {
                goTo(project);
            })
    }

    function handleProjectDeletion(project: TreeItem) {
        reloadRepository()
            .then(() => {
                goBack();
            })
    }

    function handleModuleCreation(module: any) {
        reloadRepository()
            .then(() => {
                goTo(currentItem!);
            })
    }

    function handleModuleUpdate(mod: TreeItem){
        reloadRepository()
            .then(() => {
                goTo(mod);
            })
    }

    function handleModuleDeletion(mod: TreeItem){
        deleteModule(mod)
            .then(() => {
                reloadRepository()
                    .then(() => {
                        goBack();
                    });
            })
            .catch(e => console.log(e))
    }

</script>

<div class="h-full px-1 flex flex-col">
    {#if currentItem}  
        {#if currentItem.itemType === "module"}
            <ModuleForm bind:openDialog={editDialog} moduleItem={currentItem} onmoduleupdate={handleModuleUpdate} onmoduledelete={handleModuleDeletion}/>
        {:else if currentItem.itemType === "project"}
            <ProjectForm bind:openDialog={editDialog} project={currentItem} onprojectupdate={handleProjectUpdate} onprojectdelete ={handleProjectDeletion}/>
        {:else if currentItem.itemType === "folder"}
            <FolderForm bind:openDialog={editDialog} folder={currentItem} onfolderupdate={handleFolderUpdate} onfolderdelete={handleFolderDeletion}/>
        {/if}
        <div class="py-1 text-sm border-b-[1px]">
            <Breadcrumb.Root class="py-1 px-2">
                <Breadcrumb.List>
                {#each treeHistory as hist, index}
                    <Breadcrumb.Item>
                        <button onclick={() => { handleItemSelection(hist) }}>
                        {hist?.name || ''}
                        </button>
                    </Breadcrumb.Item>
                    {#if index < treeHistory.length - 1}
                        <Breadcrumb.Separator />
                    {/if}
                {/each}
                </Breadcrumb.List>
            </Breadcrumb.Root>
            <div class="flex flex-row items-center px-2">
                <Icon icon={getIconFromTreeItemType(currentItem!, true)} width="20px"/>
                <h1 class="text-lg font-bold py-1 pl-2">{currentItem.name}</h1>
                <p class="text-sm pl-2 font-light">/ {currentItem.itemType}</p>
                <div class="grow flex flex-row-reverse p-1 gap-1">
                    <DropdownMenu.Root>
                        <DropdownMenu.Trigger>
                            <Tooltip.Provider> 
                            <Tooltip.Root delayDuration={200}>
                                <Tooltip.Trigger>
                                    <Button variant="secondary" size="sm" class="cursor-default">
                                        <Icon icon="gravity-ui:circle-plus" width="20px"/>
                                    </Button>
                                </Tooltip.Trigger>
                                <Tooltip.Content>
                                    <p>New...</p>
                                </Tooltip.Content>
                            </Tooltip.Root>
                            </Tooltip.Provider>
                        </DropdownMenu.Trigger>
                        <DropdownMenu.Content>
                        {#if currentItem.itemType !== 'module'}
                            <DropdownMenu.Item onclick={() => {projectFormDialog = true}} class="min-w-[150px]">
                            <Icon icon="gravity-ui:folder-fill" width="15px"/>
                            <p class="pl-3">New Project</p>
                            </DropdownMenu.Item>
                            {#if currentItem.itemType !== 'repository'}
                            <DropdownMenu.Item onclick={() => {folderFormDialog = true}} class="min-w-[150px]">
                            <Icon icon="gravity-ui:folder" width="15px"/>
                            <p class="pl-3">New Folder</p>
                            </DropdownMenu.Item>
                            <DropdownMenu.Item onclick={() => {moduleFormDialog = true}} class="min-w-[150px]">
                            <Icon icon="gravity-ui:layout-header-cells-large-fill" width="15px"/>
                            <p class="pl-3">New Module</p>
                            </DropdownMenu.Item>
                            {/if}
                        {:else}
                            <DropdownMenu.Item onclick={() => {baselineDialog = true}} class="min-w-[150px]">
                            <Icon icon="gravity-ui:tag" width="15px"/>
                            <p class="pl-3">New Baseline</p>
                            </DropdownMenu.Item>
                        {/if}
                        </DropdownMenu.Content>
                    </DropdownMenu.Root>
                    <Tooltip.Provider>
                        <Tooltip.Root delayDuration={200}>
                            <Tooltip.Trigger>
                                <Button variant="secondary" size="sm" onclick={() => {editDialog = true}}>
                                    <Icon icon="gravity-ui:pencil-to-line" width="20px"/>
                                </Button>
                            </Tooltip.Trigger>
                            <Tooltip.Content>
                                <p>Edit details</p>
                            </Tooltip.Content>
                        </Tooltip.Root>
                    </Tooltip.Provider>
                    {#if currentItem.itemType === 'module'}
                        <Tooltip.Provider>
                            <Tooltip.Root delayDuration={200}>
                                <Tooltip.Trigger>
                                    <Button variant="secondary" size="sm" onclick={openCurrent}>
                                    <Icon icon="gravity-ui:arrow-up-right-from-square" width="20px"/>
                                    </Button>
                                </Tooltip.Trigger>
                                <Tooltip.Content>
                                    <p>Check out</p>
                                </Tooltip.Content>
                            </Tooltip.Root>
                        </Tooltip.Provider>
                    {/if}
                </div>
                <CreateProjectForms bind:openDialog={projectFormDialog} currentParent={currentItem} oncreateproject={handleProjectCreation}/>
                <CreateFolderForms bind:openDialog={folderFormDialog} currentParent={currentItem} oncreatefolder={handleFolderCreation}/>
                <CreateModuleForms bind:openDialog={moduleFormDialog} currentParent={currentItem} oncreatedmodule={handleModuleCreation}/>
            </div>
        </div>
        {#if getCurrentItem()!.itemType === "module"}
            <ModulePanelView moduleTree={getCurrentItem()!} onopenbaseline={handleOpenBaseline}/>
        {:else}
            {#if currentItem.children.length??0 > 0}
            <ScrollArea class="grow">
                {#each currentItem?.children??[] as child}
                <button class="flex items-center py-2 hover:bg-slate-200 px-2 w-full" onclick={() => handleItemSelection(child)}>
                    <Icon icon={getIconFromTreeItemType(child)} width="20px"/>
                    <span class="px-2">{child.name}</span>
                </button>
                {/each}
            </ScrollArea>
            {:else}
                <div class="w-full h-full grow flex flex-col items-center justify-center text-slate-400 pb-[100px] rounded-lg">
                    <Icon icon={getIconFromTreeItemType(currentItem, true)} width="50px"/>
                    <h1 class="text-xl font-semibold my-1">EMPTY {currentItem.itemType?.toUpperCase()??""}</h1>
                </div>       
            {/if}
        {/if}
    {/if}
</div>
