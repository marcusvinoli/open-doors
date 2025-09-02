<script lang="ts">
    import Icon from '@iconify/svelte';
    import TreeItem from "./TreeItem.svelte";
    
    import { getTreeState, setTreeState } from '$lib/stores/TreeState.svelte';

    import type { TreeItem as TreeItemType } from '$lib/components/structs/Tree';

    let { 
        level, 
        item,
        onclick,
    } : {
        level: any;
        item: TreeItemType;
        onclick?: (item: any) => void;
    } = $props();
    
    let tabLevel = level;
    let open = $state(getTreeState(item) || (level === 0)); // Ensures that repository is always start opened.

    function toggleItem(event: any){
        event.stopPropagation();
        open = !open;
        setTreeState(item, open);
    }

    function handleClick(event: any) {
        event.stopPropagation();
        if (onclick) {
            onclick(item)
        }
    }

    function handleDoubleClick(event: any) {
        toggleItem(event);
        handleClick(event);
    }
    
</script>

<div role="button" onclick={handleClick} ondblclick={handleDoubleClick} tabindex="-1" onkeydown={() => {}} class="cursor-default">
    <div class="flex items-center hover:bg-slate-200 px-2 w-full text-ellipsis text-sm font-light select-none">
        <div class="flex">
            {#each {length: tabLevel} as _}
            <div class="tree-strips hover:border-r-slate-300 w-[10px] h-[30px]"></div>
            {/each}
        </div>
        <div class="flex text-left truncate p-1 items-center">
            <button onclick={toggleItem}>
                {#if open && item.itemType != "module"}
                <Icon icon="gravity-ui:chevron-down" width="12px"/>
                {:else if !open  && item.itemType != "module"}
                <Icon icon="gravity-ui:chevron-right" width="12px"/>
                {:else}
                <div class="w-[12px]"></div>
                {/if}
            </button>
            <icon class="pr-2 pl-1">
                {#if item.itemType === "repository"}
                <Icon icon="gravity-ui:database" width="15px"/>
                {:else if item.itemType === "project" && open}
                <Icon icon="gravity-ui:folder-open-fill" width="15px"/>
                {:else if item.itemType === "project" && !open}
                <Icon icon="gravity-ui:folder-fill" width="15px"/>
                {:else if item.itemType === "folder" && open}
                <Icon icon="gravity-ui:folder-open" width="15px"/>
                {:else if item.itemType === "folder" && !open}
                <Icon icon="gravity-ui:folder" width="15px"/>
                {:else if item.itemType === "module"}
                <Icon icon="gravity-ui:layout-header-cells-large-fill" width="15px"/>
                {:else}
                <Icon icon="gravity-ui:file" width="15px"/>
                {/if}
            </icon>
            <span class="select-none">{item.name}</span>
        </div>
    </div>
</div>

{#if (item.children) && open}
    {#each item.children as child}
        <TreeItem item={child} level={level+1} onclick={onclick}/>
    {/each}
{/if}
