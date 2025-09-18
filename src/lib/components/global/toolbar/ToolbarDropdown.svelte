<script lang="ts">
    import Icon from '@iconify/svelte';
    
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";

    import type { ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType, ToolbarItemType } from "./Toolbar";

    import ToolbarButton from './ToolbarButton.svelte';
    
    let { 
        dropdown 
    } : {
        dropdown: ToolbarDropdownType;
    } = $props();

    let isOpened: boolean = $state(false);

</script>

{#snippet dropdownButton(button: ToolbarButtonType)}
    <DropdownMenu.Item onclick={button.onclick} disabled={button.disabled}>
        <div class="flex flex-row items-center gap-2">
            {#if button.icon}
                {@const icon = typeof button.icon === 'function' ? button.icon() : button.icon}
                <Icon {icon} width="10px"/>
            {/if}
            <p class="text-sm">{button.tooltip}</p>
        </div>
    </DropdownMenu.Item>
{/snippet}

{#snippet dropdownGroup(group: ToolbarGroupType, lastItem?: boolean)}
    <DropdownMenu.Group>
        {#each group.items as item}
            {@render dropdownItem(item)}
        {/each}
    </DropdownMenu.Group>
    {#if !lastItem}
    <DropdownMenu.Separator class="m-1"/>
    {/if}
{/snippet}

{#snippet dropdownDropdown(dropdown: ToolbarDropdownType)}
    <DropdownMenu.Sub>
        <DropdownMenu.SubTrigger>
            <div class="flex flex-row items-center gap-2">
            {#if dropdown.button.icon}
                {@const icon = typeof dropdown.button.icon === 'function' ? dropdown.button.icon() : dropdown.button.icon}
                <Icon {icon} width="10px"/>
            {/if}
            <p class="text-sm">{dropdown.button.tooltip}</p>
        </div>
        </DropdownMenu.SubTrigger>
        <DropdownMenu.SubContent>
            {#each dropdown.items as item}
                {@render dropdownItem(item)}
            {/each}
        </DropdownMenu.SubContent>
    </DropdownMenu.Sub>
{/snippet}

{#snippet dropdownItem(item: ToolbarItemType, lastItem?: boolean)}
    {#if item.type === 'button'}
        {@render dropdownButton(item as ToolbarButtonType)}
    {:else if item.type === 'group'}
        {@render dropdownGroup(item as ToolbarGroupType, lastItem)}
    {:else if item.type === 'dropdown'}
        {@render dropdownDropdown(item as ToolbarDropdownType)}
    {/if}
{/snippet}

<DropdownMenu.Root bind:open={isOpened}>
    <DropdownMenu.Trigger>
        <ToolbarButton button={dropdown.button} class={(isOpened ? 'bg-slate-200' : '')}/>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
        {#each dropdown.items as item, i}
            {@render dropdownItem(item, (i === dropdown.items.length - 1))}
        {/each}
    </DropdownMenu.Content>
</DropdownMenu.Root>
