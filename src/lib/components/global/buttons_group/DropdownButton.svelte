<script lang="ts">
    import Icon from '@iconify/svelte';
    import Button from '$lib/components/ui/button/button.svelte';
    import * as Tooltip from "$lib/components/ui/tooltip";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import type { ButtonInterface, ButtonDropdown } from "./ButtonGroup";
    import DropdownButtonItem from './DropdownButtonItem.svelte';
    
    export let dropdown: ButtonInterface;
    let dd: ButtonDropdown =  dropdown as ButtonDropdown;

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
        {#each dd.items as ddg, index}
            {#each ddg.items as item}
                <DropdownButtonItem item={item}/>         
            {/each}
            {#if index < dd.items.length - 1}
                <DropdownMenu.Separator />
            {/if}
        {/each}
    </DropdownMenu.Content>
</DropdownMenu.Root>