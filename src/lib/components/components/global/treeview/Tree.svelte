<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import type { TreeItemData } from './TreeItemData';
    import TreeItem from "./TreeItem.svelte";

    export let items: TreeItemData[];

    const dispatch = createEventDispatcher();

    function handleClick(event: any) {
        dispatch('click', event.detail);
    }

    function trackHierarchy() {

    }

</script>

<div class="flex flex-col w-full min-h-[180px] overflow-auto">
    {#each items as item}
        <TreeItem item={item} level={0} on:click={handleClick}/>
    {/each}
</div>
