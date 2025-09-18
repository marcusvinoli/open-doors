<script lang="ts">
    import ToolbarItem from "./ToolbarItem.svelte";

    import { Separator } from "$lib/components/ui/separator";
    
    import type { ToolbarGroupType } from "./Toolbar";

    let {
        group,
    } : {
        group: ToolbarGroupType;
    } = $props();

</script>

<div class="flex flex-row items-center gap-1">
    {#each group.items as item}
        <ToolbarItem {item} />
    {/each}
    <Separator orientation="vertical" class="mx-1"/>
</div>
