<script lang="ts">
    import { Separator } from "$lib/components/ui/separator";
    import ToolbarButton from "./ToolbarButton.svelte";
    import ToolbarDropdown from "./ToolbarDropdown.svelte";
    import type { ToolbarGroupType } from "./Toolbar";

    export let group: ToolbarGroupType;
</script>

<div class="flex flex-row items-center gap-1">
    {#each group.items as item}
        {#if item.type === "button" }
            <ToolbarButton item={item} />
        {:else if item.type === "buttonsGroup"}
            <svelte:self group={item} />
        {:else if item.type === "dropdown"}
            <ToolbarDropdown dropdown={item} />
        {/if}
    {/each}
    <Separator orientation="vertical" class="mx-1"/>
</div>