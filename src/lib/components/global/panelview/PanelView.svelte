<script lang="ts">
  import Icon from "@iconify/svelte";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js"; 
  import { getIconFromTreeItemType } from "$lib/utils/getIconFromTreeItemType";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
  import type { TreeItem } from "$lib/components/structs/Tree"; 
  import { beforeUpdate } from "svelte";
  
  export let itemsHist: TreeItem[] = [];
  let currentItem: TreeItem;

  function updateTreeView(currentSelection: TreeItem) {

  if (itemsHist.length === 0) {
    itemsHist.push(currentSelection);
    itemsHist = itemsHist;
    return;
  }
  
  if (itemsHist.includes(currentSelection) ) {
    while (itemsHist.pop() !== currentSelection) {}
    itemsHist.push(currentSelection);
    itemsHist = itemsHist;
    return;
  }
  
  let lastIndex = itemsHist.length - 1;
  if (itemsHist.at(lastIndex)?.children.includes(currentSelection)) {
    itemsHist.push(currentSelection);
    itemsHist = itemsHist;
    return
  }
}

  beforeUpdate(() => {
    if (itemsHist.length > 0) {
      currentItem = itemsHist[itemsHist.length - 1];
    }
    console.log(currentItem);
  })

</script>

<div class="h-full px-1 flex flex-col">
  {#if itemsHist.length > 0 && currentItem}
  <div class="p-3">
    <Breadcrumb.Root>
      <Breadcrumb.List>
      {#each itemsHist as hist, index}
        <Breadcrumb.Item>
          <button on:click={() => {updateTreeView(hist)}}>
            {hist.name}
          </button>
        </Breadcrumb.Item>
        {#if index < itemsHist.length - 1}
        <Breadcrumb.Separator />
        {/if}
      {/each}
      </Breadcrumb.List>
    </Breadcrumb.Root>
  </div>
  {#if currentItem.children.length > 0}
  <ScrollArea class="grow">
    {#each currentItem.children as child}
    <button class="flex items-center py-2 hover:bg-slate-200 px-4 w-full" on:click={() => updateTreeView(child)}>
      <Icon icon={getIconFromTreeItemType(child)} width="20px"/>
      <span class="px-2">{child.name}</span>
    </button>
    {/each}
  </ScrollArea>
  {:else}
  <div class="w-full h-full grow flex flex-col items-center justify-center text-slate-400 pb-[100px] bg-slate-200 rounded-lg">
    <Icon icon="gravity-ui:folder-open" width="50px"/>
    <h1 class="text-xl font-semibold my-1">EMPTY {currentItem.itemType?.toUpperCase()}</h1>
  </div>
  {/if}
  {/if}
</div>