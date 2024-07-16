<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import Icon from '@iconify/svelte';
    import type { TreeItemState } from '$lib/components/global/treeview/TreeViewState';

    export let level;
    export let item: TreeItemState
    
    let tabLevel = level;

    function toggleItem(){
        item.opened = !item.opened;
    }

    const dispatch =  createEventDispatcher();

    function handleClick(event: any) {
        event.stopPropagation();
        dispatch('click', {item: item});
        toggleItem();
        console.log(item);
    }
</script>

<button class="flex items-center hover:bg-slate-200 px-2 w-full text-ellipsis text-sm font-light" on:click={handleClick}>
    <div class="flex">
        {#each {length: tabLevel} as _}
        <div class="tree-strips hover:border-r-slate-300 w-[10px] h-[30px]"></div>
        {/each}
    </div>
    <div class="flex text-left truncate p-1 items-center">
        {#if item.opened}
            <Icon icon="gravity-ui:chevron-down" width="10px"/>
        {:else}
            <Icon icon="gravity-ui:chevron-right" width="10px"/>
        {/if}
        <icon class="pr-2 pl-1">
            {#if item.itemType === "repository"}
                <Icon icon="gravity-ui:database" width="15px"/>
            {:else if item.itemType === "project" && item.opened}
                <Icon icon="gravity-ui:folder-open-fill" width="15px"/>
            {:else if item.itemType === "project" && !item.opened}
                <Icon icon="gravity-ui:folder-fill" width="15px"/>
            {:else if item.itemType === "folder" && item.opened}
                <Icon icon="gravity-ui:folder-open" width="15px"/>
            {:else if item.itemType === "folder" && !item.opened}
                <Icon icon="gravity-ui:folder" width="15px"/>
            {:else if item.itemType === "module"}
                <Icon icon="gravity-ui:layout-header-cells-large-fill" width="15px"/>
            {:else}
                <Icon icon="gravity-ui:file" width="15px"/>
            {/if}
        </icon>
        <p class="">{item.name}</p>
    </div>
</button>

{#if (item.children) && item.opened}
    {#each item.children as child}
        <svelte:self item={child} level={level+1} on:click/>
    {/each}
{/if}
