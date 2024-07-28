<script lang="ts">
  import Icon from "@iconify/svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  import CreateFolderForms from "$lib/components/forms/folder/CreateFolderForms.svelte";
  import CreateProjectForms from "$lib/components/forms/project/CreateProjectForms.svelte";
  import CreateModuleForms from "$lib/components/forms/module/CreateModuleForms.svelte";
  import ProjectForm from "$lib/components/forms/project/ProjectForm.svelte";
  import FolderForm from "$lib/components/forms/folder/FolderForm.svelte";
  import { goIn } from "$lib/stores/PanelView";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
  import { getIconFromTreeItemType } from "$lib/utils/getIconFromTreeItemType";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js"; 
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import type { TreeItem } from "$lib/components/structs/Tree";
  
  export let currentItem: TreeItem;
  export let treeHistory: TreeItem[];

  let projectFormDialog: boolean = false;
  let folderFormDialog: boolean = false;
  let moduleFormDialog: boolean = false;
  
  let editDialog: boolean = false;
  
</script>

<div class="h-full px-1 flex flex-col">
{#if currentItem}  
  {#if currentItem.itemType === "repository"}
    <div></div>
  {:else if currentItem.itemType === "project"}
    <ProjectForm bind:openDialog={editDialog} project={currentItem} on:deleted on:updated/>
  {:else if currentItem.itemType === "folder"}
    <FolderForm bind:openDialog={editDialog} folder={currentItem} on:deleted on:updated/>
  {/if}
    <div class="py-1 px-2 text-sm">
      <Breadcrumb.Root class="py-1">
        <Breadcrumb.List>
          {#each treeHistory as hist, index}
          <Breadcrumb.Item>
            <button on:click={() => {goIn(hist)}}>
              {hist.name}
            </button>
          </Breadcrumb.Item>
          {#if index < treeHistory.length - 1}
          <Breadcrumb.Separator />
          {/if}
          {/each}
        </Breadcrumb.List>
      </Breadcrumb.Root>
      <div class="flex flex-row items-center border-b-[1px]">
        <Icon icon={getIconFromTreeItemType(currentItem, true)} width="20px"/>
        <h1 class="text-lg font-bold py-1 pl-2">{currentItem.name}</h1>
        <p class="text-sm pl-2 font-light">/ {currentItem.itemType}</p>
        <div class="grow flex flex-row-reverse p-1 gap-1">
          <DropdownMenu.Root closeOnItemClick closeOnOutsideClick>
            <DropdownMenu.Trigger>
                <Tooltip.Root openDelay={200}>
                    <Tooltip.Trigger>
                        <Button variant="secondary" size="sm" class="cursor-default">
                            <Icon icon="gravity-ui:circle-plus" width="20px"/>
                        </Button>
                    </Tooltip.Trigger>
                    <Tooltip.Content>
                        <p>New...</p>
                    </Tooltip.Content>
                </Tooltip.Root>
            </DropdownMenu.Trigger>
            <DropdownMenu.Content>
              {#if currentItem.itemType !== "module"}
                <DropdownMenu.Item on:click={() => {projectFormDialog = true}} class="min-w-[150px]">
                  <Icon icon="gravity-ui:folder-fill" width="15px"/>
                  <p class="pl-3">New Project</p>
                </DropdownMenu.Item>
                {#if currentItem.itemType !== "repository"}
                <DropdownMenu.Item on:click={() => {folderFormDialog = true}} class="min-w-[150px]">
                  <Icon icon="gravity-ui:folder" width="15px"/>
                  <p class="pl-3">New Folder</p>
                </DropdownMenu.Item>
                <DropdownMenu.Item on:click={() => {moduleFormDialog = true}} class="min-w-[150px]">
                  <Icon icon="gravity-ui:layout-header-cells-large-fill" width="15px"/>
                  <p class="pl-3">New Module</p>
                </DropdownMenu.Item>
                {/if}
              {/if}
            </DropdownMenu.Content>
          </DropdownMenu.Root>
          <Tooltip.Root openDelay={200}>
            <Tooltip.Trigger>
              <Button variant="secondary" size="sm" on:click={() => {editDialog = true}}>
                <Icon icon="gravity-ui:pencil-to-line" width="20px"/>
              </Button>
            </Tooltip.Trigger>
            <Tooltip.Content>
              <p>Edit</p>
            </Tooltip.Content>
          </Tooltip.Root>
        </div>
        <CreateProjectForms bind:openDialog={projectFormDialog} selectedParent={currentItem}/>
        <CreateFolderForms bind:openDialog={folderFormDialog} selectedParent={currentItem}/>
        <CreateModuleForms bind:openDialog={moduleFormDialog} selectedParent={currentItem}/>
      </div>
    </div>
      {#if currentItem.children.length > 0}
      <ScrollArea class="grow pl-2">
        {#each currentItem.children??[] as child}
        <button class="flex items-center py-2 hover:bg-slate-200 px-4 w-full" on:click={() => goIn(child)}>
          <Icon icon={getIconFromTreeItemType(child)} width="20px"/>
          <span class="px-2">{child.name}</span>
        </button>
        {/each}
      </ScrollArea>
      {:else}
        {#if currentItem}
          <div class="w-full h-full grow flex flex-col items-center justify-center text-slate-400 pb-[100px] rounded-lg">
            <Icon icon={getIconFromTreeItemType(currentItem, true)} width="50px"/>
            <h1 class="text-xl font-semibold my-1">EMPTY {currentItem.itemType?.toUpperCase()??""}</h1>
          </div>
        {/if}
      {/if}
  {/if}
</div>
