<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import Icon from '@iconify/svelte';
    import type { TreeItemData } from "./TreeItemData";

    export let item: TreeItemData;
    export let level;

    let ticks = level;

    function toggleItem(){
        var newItem = item;
        
        if(newItem.attributes) {
            newItem.attributes.opened = !newItem.attributes.opened;
        } else {
            newItem.attributes = {
                opened: true,
                selected: false,
            };
        }
        item = newItem;
    }

    const dispatch =  createEventDispatcher();

    function handleClick(event: any) {
        event.stopPropagation();
        toggleItem();
        dispatch('click', {item: item});
    }
</script>

<button class="flex items-center hover:bg-slate-200 {item.attributes?.selected ? 'bg-slate-200' : ''} w-full text-ellipsis text-sm font-light" on:click={handleClick}>
    <div class="flex">
        {#each {length: ticks} as _}
        <div class="border-r-[1px] border-r-slate-400 w-[16px] h-[30px]"></div>
        {/each}
    </div>
    <div class="flex text-left truncate p-1">
        <icon class="pr-2 pl-1">
            {#if item.type === "repo"}
                <Icon icon="gravity-ui:database" width="18px"/>
            {:else if item.type === "project" && item.attributes?.opened}
                <Icon icon="gravity-ui:folder-open-fill" width="18px"/>
            {:else if item.type === "project" && !item.attributes?.opened}
                <Icon icon="gravity-ui:folder-fill" width="18px"/>
            {:else if item.type === "folder" && !item.attributes?.opened}
                <Icon icon="gravity-ui:folder" width="18px"/>
            {:else if item.type === "folder" && item.attributes?.opened}
                <Icon icon="gravity-ui:folder-open" width="18px"/>
            {:else if item.type === "module"}
                <Icon icon="gravity-ui:layout-header-cells-large-fill" width="18px"/>
            {:else}
                <Icon icon="gravity-ui:file" width="18px"/>
            {/if}
        </icon>
        <p class="{item.attributes?.selected ? 'font-bold' : ''}">{item.name}</p>
    </div>
</button>

{#if (item.children) && (item.children?.length) && item.attributes?.opened}
    {#each item.children as child}
        <svelte:self item={child} level={level+1} on:click/>
    {/each}
{/if}
