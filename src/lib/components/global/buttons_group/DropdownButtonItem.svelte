<script lang="ts">
    import Icon from '@iconify/svelte';
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import type { ButtonInterface, ButtonSimple } from "./ButtonGroup";
    
    interface Props {
        item: ButtonInterface;
    }

    let { item }: Props = $props();
    let dpItem: ButtonSimple = item as ButtonSimple;
</script>

<DropdownMenu.Item on:click={dpItem.action} class="min-w-[150px]">
    <Icon icon={dpItem.icon} width="15px"/>
    <p class="pl-3">{dpItem.tooltip}</p>
</DropdownMenu.Item>