<script lang="ts">
    import Icon from '@iconify/svelte';
    import Button from '$lib/components/ui/button/button.svelte';
    
    import * as Tooltip from "$lib/components/ui/tooltip";
    
    import { cn } from '$lib/utils';

    import type { ToolbarButtonType } from "./Toolbar";

    let {
        button,
        class: customClass = "",
    } : {
        button: ToolbarButtonType;
        class?: string,
    } = $props();

</script>

<div class="flex flex-row items-center gap-1">
    <Tooltip.Provider>
        <Tooltip.Root>
            <Tooltip.Trigger>
                <Button variant="ghost" class={cn("cursor-default", customClass)} onclick={button.onclick} disabled={button.disabled}>
                    {#if button.icon}
                        {@const icon = typeof button.icon === 'function' ? button.icon() : button.icon}
                        <Icon {icon} width="18px"/>
                    {/if}
                    {#if button.label}
                        <span class="select-none">{button.label}</span>
                    {/if}
                </Button>
            </Tooltip.Trigger>
            <Tooltip.Content>
                <p>{button.tooltip ?? ''}</p>
            </Tooltip.Content>
        </Tooltip.Root>
    </Tooltip.Provider>
</div>
