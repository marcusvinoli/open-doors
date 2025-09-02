<script lang="ts">
    import Tree from "./Tree.svelte";

    import { goTo } from "$lib/stores/PanelView.svelte";
    import { repository } from "$lib/stores/Repository.svelte";

    let repo = $derived(repository());

    function handleItemClick(item: any) {
        goTo(item);
    }

</script>

<div>
    {#if repo?.tree}
        <Tree onclick={handleItemClick}/>
    {/if}
</div>
