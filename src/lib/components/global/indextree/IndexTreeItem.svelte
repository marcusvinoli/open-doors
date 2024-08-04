<script lang="ts">
    import Icon from '@iconify/svelte';
    import { createEventDispatcher } from 'svelte';
    import type { IndexTree } from "./IndexTree";

    export let item: IndexTree
    export let level;

    let tabLevel = level;

    function toggleItem(){
        item.opened = !item.opened;
    }

    const dispatch =  createEventDispatcher();

    function handleClick(event: any) {
        event.stopPropagation();
        dispatch('click', {path: item.path});
        toggleItem();
    }
</script>


<button class="flex items-center hover:bg-slate-200 px-2 w-full text-ellipsis text-sm font-light" on:click={handleClick}>
    <div class="flex">
        {#each {length: tabLevel} as _}
        <div class="tree-strips hover:border-r-slate-300 w-[10px] h-[30px]"></div>
        {/each}
    </div>
    <div class="flex text-left truncate p-1 items-center">
        {#if item.opened && item.header}
            <Icon icon="gravity-ui:chevron-down" width="10px" class="mr-1"/>
        {:else if !item.opened && item.header}
            <Icon icon="gravity-ui:chevron-right" width="10px" class="mr-1"/>
        {:else }
            <Icon icon="gravity-ui:minus" width="10px" class="mr-1"/>
        {/if}
        <p class="mr-1">{item.level}.</p>
        <p class="">{item.header}{item.content}</p>
    </div>
</button>

{#if (item.children) && item.opened}
    {#each item.children as child}
        <svelte:self item={child} level={level+1} on:click/>
    {/each}
{/if}