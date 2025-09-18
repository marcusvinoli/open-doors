<script lang="ts">
    import ToolbarItem from "./ToolbarItem.svelte";
    
    import { toolbar } from "$lib/stores/Toolbar.svelte";

    import type { ToolbarItemType } from "./Toolbar";
    
    let toolbarItems: ToolbarItemType[] = $derived(toolbar().items);

</script>

{#if toolbarItems.length > 0}
    <div class="h-10 p-1 bg-slate-100 flex align-center text-neutral-700">
        {#each toolbarItems as item}
            <ToolbarItem {item} />
        {/each}
    </div>
{/if}
