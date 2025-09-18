<script lang="ts">
    import ToolbarButton from './ToolbarButton.svelte';

    import type { ToolbarButtonType, ToolbarToggleType } from './Toolbar';

    let { 
        toggle,
    } : {
        toggle: ToolbarToggleType;
    } = $props();
   
</script>

{#snippet button(button: ToolbarButtonType, status: boolean)}
    <ToolbarButton button={{
        type: 'button',
        icon: button.icon,
        label: button.label,
        tooltip: button.tooltip,
        disabled: button.disabled,
        onclick: () => {
            toggle.onchange(status)
        }
    }}/>
{/snippet}

{#if toggle.status}
    {@render button(toggle.buttonTrue, false)}
{:else}
    {@render button(toggle.buttonFalse, true)}
{/if}
