<script lang="ts">
    import Icon from '@iconify/svelte';
    import Button from '$lib/components/ui/button/button.svelte';
    import * as Tooltip from "$lib/components/ui/tooltip";
    import type { ToolbarItemInterface, ToolbarToggleType } from "./Toolbar";

    export let item: ToolbarItemInterface;
    let toggle: ToolbarToggleType;
    export let status: boolean = false;

    $: {
        toggle = item as ToolbarToggleType;
        status = toggle.status;
    };
</script>

<div class="flex flex-row items-center gap-1">
    <Tooltip.Root openDelay={200}>
        <Tooltip.Trigger>
            {#if status}
            <Button variant="ghost" class="cursor-default" on:click={() => {toggle.buttonOff.action(); status = false}}>
                <Icon icon={toggle.buttonOn.icon} width="20px"/>
            </Button>
            {:else}
            <Button variant="ghost" class="cursor-default" on:click={() => {toggle.buttonOn.action(); status = true}}>
                <Icon icon={toggle.buttonOff.icon} width="20px"/>
            </Button>
            {/if}
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p>{status ? toggle.buttonOn.tooltip : toggle.buttonOff.tooltip}</p>
        </Tooltip.Content>
    </Tooltip.Root>
</div>