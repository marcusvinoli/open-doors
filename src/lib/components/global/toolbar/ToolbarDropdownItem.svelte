<script lang="ts">
    import Icon from '@iconify/svelte';
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import type { ToolbarButtonType, ToolbarItemInterface } from "./Toolbar";
    
    interface Props {
        item: ToolbarItemInterface;
    }

    let { item }: Props = $props();
    let dpItem: ToolbarButtonType = item as ToolbarButtonType;
</script>

<DropdownMenu.Item onclick={dpItem.action} class="min-w-[150px]">
    <Icon icon={dpItem.icon} width="15px"/>
    <p class="pl-3">{dpItem.tooltip}</p>
</DropdownMenu.Item>     