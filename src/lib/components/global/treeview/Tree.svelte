<script lang="ts">
    import TreeItem from "./TreeItem.svelte";

    import { repository } from "$lib/stores/Repository.svelte";
    
    import type { TreeItem as TreeItemType } from '$lib/components/structs/Tree';
 
    let { 
        onclick,
    } : {
        onclick?: (item: any) => void,
    } = $props();

    let tree: TreeItemType | null = $derived(repository()?.tree);

    function handleClick(item: any) {
        if (onclick) {
            onclick(item)
        }
    }

</script>

<div class="flex flex-col w-full min-h-[180px] overflow-auto tree-content">
    {#if tree}
        <TreeItem item={tree} level={0} onclick={handleClick}/>
    {/if}
</div>

<!-- 
<style type="postcss">
    .tree-content:hover .tree-strips {
        @apply border-l-[1px];
    }
</style>
 -->