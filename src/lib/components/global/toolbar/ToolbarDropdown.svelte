<script lang="ts">
    import Icon from '@iconify/svelte';
    import Button from '$lib/components/ui/button/button.svelte';
    import * as Tooltip from "$lib/components/ui/tooltip";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import type { ToolbarDropdownType, ToolbarItemInterface } from "./Toolbar";
    import ToolbarDropdownItem from './ToolbarDropdownItem.svelte';
    
    export let dropdown: ToolbarItemInterface;
    let dd: ToolbarDropdownType =  dropdown as ToolbarDropdownType;

</script>

<DropdownMenu.Root closeOnItemClick closeOnOutsideClick>
    <DropdownMenu.Trigger>
        <Tooltip.Root openDelay={200}>
            <Tooltip.Trigger>
                <Button variant="ghost" class="cursor-default">
                    <Icon icon={dd.button.icon} width="20px"/>
                </Button>
            </Tooltip.Trigger>
            <Tooltip.Content>
                <p>{dd.button.tooltip}</p>
            </Tooltip.Content>
        </Tooltip.Root>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
        {#each dd.items as dropdownGroup, index}
            {#each dropdownGroup.items as item}
                <ToolbarDropdownItem item={item}/>         
            {/each}
            {#if index < dd.items.length - 1}
                <DropdownMenu.Separator />
            {/if}
        {/each}
    </DropdownMenu.Content>
  </DropdownMenu.Root>