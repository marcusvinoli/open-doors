<script lang="ts">
    import Icon from '@iconify/svelte';
    import Button from '$lib/components/ui/button/button.svelte';

    import * as Tooltip from "$lib/components/ui/tooltip";

    import type { ToolbarItemInterface, ToolbarToggleType } from "./Toolbar";

    let { 
        item, 
        status = $bindable(false)
    } : {
        item: ToolbarItemInterface;
        status?: boolean;
    } = $props();

    let toggle: ToolbarToggleType = $derived(item as ToolbarToggleType);

</script>

<div class="flex flex-row items-center gap-1">
    <Tooltip.Provider>
        <Tooltip.Root delayDuration={200}>
            <Tooltip.Trigger>
                {#if status}
                <Button variant="ghost" class="cursor-default" onclick={() => {toggle.buttonOff.action(); status = false}}>
                    <Icon icon={toggle.buttonOn.icon} width="20px"/>
                </Button>
                {:else}
                <Button variant="ghost" class="cursor-default" onclick={() => {toggle.buttonOn.action(); status = true}}>
                    <Icon icon={toggle.buttonOff.icon} width="20px"/>
                </Button>
                {/if}
            </Tooltip.Trigger>
            <Tooltip.Content>
                <p>{status ? toggle.buttonOn.tooltip : toggle.buttonOff.tooltip}</p>
            </Tooltip.Content>
        </Tooltip.Root>
    </Tooltip.Provider>
</div>
