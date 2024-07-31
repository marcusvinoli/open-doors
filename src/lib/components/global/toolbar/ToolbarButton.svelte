<script lang="ts">
    import Icon from '@iconify/svelte';
    import Button from '$lib/components/ui/button/button.svelte';
    import * as Tooltip from "$lib/components/ui/tooltip";
    import type { ToolbarItemInterface, ToolbarButtonType } from "./Toolbar";

    export let item: ToolbarItemInterface;
    let button: ToolbarButtonType;

    $: {button = item as ToolbarButtonType};
</script>

<div class="flex flex-row items-center gap-1">
    <Tooltip.Root openDelay={200}>
        <Tooltip.Trigger>
            <Button variant="ghost" class="cursor-default" on:click={button.action}>
                <Icon icon={button.icon} width="20px"/>
            </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p>{button.tooltip}</p>
        </Tooltip.Content>
    </Tooltip.Root>
</div>