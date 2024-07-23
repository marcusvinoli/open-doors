<script lang="ts">
    import { toolbarItems } from "$lib/stores/Toolbar";
    import ToolbarGroup from "./ToolbarGroup.svelte";
</script>


{#if $toolbarItems.items.length > 0}
<div class="h-12 p-1 bg-slate-100 flex align-center text-neutral-700">
    {#each $toolbarItems.items as item}
        <ToolbarGroup group={item}/>
    {/each}
</div>
{/if}