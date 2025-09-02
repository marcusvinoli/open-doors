<script lang="ts">
    import ToolbarGroup from './ToolbarGroup.svelte';
    import ToolbarToggle from "./ToolbarToggle.svelte";
    import ToolbarButton from "./ToolbarButton.svelte";
    import ToolbarDropdown from "./ToolbarDropdown.svelte";

    import { Separator } from "$lib/components/ui/separator";
    
    import type { ToolbarGroupType } from "./Toolbar";

    interface Props {
        group: any;
    }

    let { group }: Props = $props();
</script>

<div class="flex flex-row items-center gap-1">
    {#each group.items as item}
        {#if item.type === "button"}
            <ToolbarButton item={item} />
        {:else if item.type === "buttonsGroup"}
            <ToolbarGroup group={item.items} />
        {:else if item.type === "dropdown"}
            <ToolbarDropdown dropdown={item} />
        {:else if item.type === "toggle"}
            <ToolbarToggle item={item} />
        {/if}
    {/each}
    <Separator orientation="vertical" class="mx-1"/>
</div>