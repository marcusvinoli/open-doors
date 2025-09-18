<script lang="ts">
    import ToolbarButton from "./ToolbarButton.svelte";
    import ToolbarDropdown from "./ToolbarDropdown.svelte";
    import ToolbarGroup from "./ToolbarGroup.svelte";
    import ToolbarToggle from "./ToolbarToggle.svelte";
    
    import type { ToolbarItemType } from "./Toolbar";

    let {
        item 
    } : { 
        item: ToolbarItemType 
    } = $props();

</script>

{#if item.type === 'button'}
    <ToolbarButton button={item} />
{:else if item.type === 'dropdown'}
    <ToolbarDropdown dropdown={item} />
{:else if item.type === 'toggle'}
    <ToolbarToggle toggle={item} />
{:else if item.type === 'group'}
    <ToolbarGroup group={item} />
{/if}
