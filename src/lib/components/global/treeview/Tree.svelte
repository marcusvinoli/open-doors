<script lang="ts">
    import type { TreeItemState}  from './TreeViewState';
    import { convertToTreeItemState, updateTreeItemState } from './Utils';
    import { beforeUpdate, createEventDispatcher, onMount } from 'svelte';
    import TreeItem from "./TreeItem.svelte";
    import type { TreeItem as TreeItemType } from '$lib/components/structs/Tree';

    export let structure: TreeItemType;
    let tree: TreeItemState;

    const dispatch = createEventDispatcher();

    function handleClick(event: any) {
        dispatch('click', event.detail);
    }

    beforeUpdate(() => {
        if(!tree) {
            tree = convertToTreeItemState(structure);
        }
        tree = updateTreeItemState(structure, tree);
    })

</script>

<div class="flex flex-col w-full min-h-[180px] overflow-auto tree-content">
    {#if tree}
        <TreeItem item={tree} level={0} on:click={handleClick}/>
    {/if}
</div>

<!-- <style type="postcss">
    .tree-content:hover .tree-strips {
        @apply border-l-[1px];
    }
</style>
 -->