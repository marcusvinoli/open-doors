<script lang="ts">
    import ToolbarGroup from "./ToolbarGroup.svelte";
    import { toolbarItems } from "$lib/stores/Toolbar";
    import type { ToolbarGroupType } from "./Toolbar";

    let tbItems: ToolbarGroupType[] = [];

    $: { tbItems = $toolbarItems; }

</script>


{#if tbItems.length > 0}
<div class="h-12 p-1 bg-slate-100 flex align-center text-neutral-700">
    {#each tbItems as item}
        <ToolbarGroup group={item}/>
    {/each}
</div>
{/if}