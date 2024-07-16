<script lang="ts">
    import { beforeUpdate, createEventDispatcher, onMount } from 'svelte';
    import Tree from "./Tree.svelte";
    import type { TreeItem } from '$lib/components/structs/Tree';

    const dispatch = createEventDispatcher();

    export let treeItems: TreeItem;

    function handleClick(event: any) {
        event.stopPropagation();
        dispatch('itemSelected', event.detail);
    }

</script>

<div>
    {#if treeItems}
        <Tree structure={treeItems} on:click={handleClick}/>
    {/if}
</div>
