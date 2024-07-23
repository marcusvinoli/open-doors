<script lang="ts">
    import Icon from '@iconify/svelte';
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import type { ToolbarButtonType, ToolbarItemInterface } from "./Toolbar";
    
    export let item: ToolbarItemInterface;
    let dpItem: ToolbarButtonType = item as ToolbarButtonType;
</script>

<DropdownMenu.Item on:click={dpItem.action} class="min-w-[150px]">
    <Icon icon={dpItem.icon} width="15px"/>
    <p class="pl-3">{dpItem.tooltip}</p>
</DropdownMenu.Item>     