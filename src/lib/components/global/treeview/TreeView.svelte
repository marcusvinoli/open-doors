<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import Tree from "./Tree.svelte";
    import type { TreeItemData } from "./TreeItemData";

    const dispatch = createEventDispatcher();

    export let treeItems: TreeItemData[] = []

    function handleClick(event: any) {
        console.log(event.detail);
    }

</script>

<div>
    <Tree items={treeItems} on:click={handleClick}/>
</div>
