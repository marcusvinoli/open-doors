<script lang="ts">
    import Icon from '@iconify/svelte';
    import IndexTreeItem from './IndexTreeItem.svelte';

    import { cn } from '$lib/utils';

    import type { IndexItem } from '$lib/components/structs/IndexItem';
    
    let { 
        state = $bindable(),
        item,
        level,
        onclick,
        showDeletions,
        isLast = false,
    } : {
        state: Map<number, boolean>,
        item: IndexItem;
        level: number;
        isLast?: boolean;
        showDeletions?: boolean;
        onclick?: (id: number | string) => void;
    } = $props();

    let open = $derived(state.get(item.id) ?? false);

    function toggleItem(event: any){
        event.stopPropagation();
        open = !open;
        state.set(item.id, open);
    }

    function handleClick(event: any) {
        event.stopPropagation();
        if (!open) {
            toggleItem(event);
        }
        if (onclick) {
            onclick(item.id);
        }
    }

    function isHeader() : boolean {
        const headerFormat = /^\d+(?:\.\d+)*$/;
        return headerFormat.test(item.level);
    }

</script>

{#if !item.isDeleted || showDeletions}
    <div class="flex items-center hover:bg-slate-200 min-w-0 text-sm font-light select-none right-1" onclick={handleClick} role="button" tabindex="-1" onkeydown={()=>{}}>
        <div class="flex h-full">
            {#each {length: level} as _}
                <div class="block w-[12px] bg-pink-200"></div>
            {/each}
        </div>
        <div class="flex text-left p-0.5 w-full mr-1 min-w-0">
            {#if isHeader()}
                <button class="flex justify-center items-center min-w-[20px] ratio-square transition-all" onclick={toggleItem}>
                    <Icon icon="gravity-ui:chevron-right" width="12px" rotate={open ? 45 : 0} class="transition-all"/>
                </button>
                <p class={cn(
                    'truncate font-semibold select-none',
                    item.isDeleted ? 'line-through italic' : ''
                )}>
                    {item.level + ". " + item.headline}
                </p>
            {:else}
                <p class={cn(
                    'truncate ml-[20px] select-none',
                    item.isDeleted ? 'line-through italic' : ''
                )}>
                    {item.headline}
                </p>
            {/if}
        </div>
    </div>
{/if}

{#if (item.children) && open || (!item.isDeleted && showDeletions) }
    {#each item.children as child, i (child.id)}
        <IndexTreeItem item={child} level={level+1} onclick={onclick} state={state} isLast={item.children.length-1 === i}/>
    {/each}
{/if}
