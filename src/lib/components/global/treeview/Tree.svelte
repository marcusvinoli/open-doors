<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import TreeItem from "./TreeItem.svelte";
    import type { TreeItem as TreeItemType } from '$lib/components/structs/Tree';

    export let structure: TreeItemType;

    const dispatch = createEventDispatcher();

    function handleClick(event: any) {
        dispatch('click', event.detail);
    }

</script>

<div class="flex flex-col w-full min-h-[180px] overflow-auto">
    {#if structure}
        <TreeItem item={structure} level={0} on:click={handleClick}/>
    {/if}
</div>
