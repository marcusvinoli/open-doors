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
            {#each {length: level-1} as _}
                <div class="w-[20px] border-r-1 border-r-slate-50 group-hover:border-r-slate-300"></div>
            {/each}
        </div>
        {#if level === 0}
            <div class="ml-2"></div>
        {:else}
            <div class="flex h-full">
                <div class="flex flex-col w-[20px]">
                    <div class="h-[50%] w-full border-r-1 border-r-slate-50 group-hover:border-r-slate-300"></div>
                    {#if !isLast}
                        <div class="h-[50%] w-full border-r-1 border-r-slate-50 group-hover:border-r-slate-300"></div>
                    {/if}
                </div>
                <div class="w-[10px]">
                    <!-- 
                    TODO: This element and the related "group-hover:" selector refers to hover effect on three.
                    <div class="border-b-1 h-[50%] border-b-slate-50 group-hover:border-b-slate-300"></div>
                    -->
                </div> 
            </div>
        {/if}
        <div class="flex text-left truncate items-center py-0.5 w-full mr-1 min-w-0">
            {#if isHeader()}
                <button class="flex justify-center items-center p-0.5 group-hover:bg-slate-100 rounded-sm" onclick={toggleItem}>
                    <Icon icon={open ? "gravity-ui:chevron-down" : "gravity-ui:chevron-right"} width="12px"/>
                </button>
            {/if}
            <div class="cursor-pointer pl-1 flex-1 min-w-0 baseis-0">
                {#if isHeader()}
                    <p class={cn(
                        'truncate font-semibold',
                        item.isDeleted ? 'line-through italic' : ''
                    )}>
                        {item.level + ". " + item.headline}
                    </p>
                {:else}
                    <p class={cn(
                        'truncate',
                        item.isDeleted ? 'line-through italic' : ''
                    )}>
                        {item.headline}
                    </p>
                {/if}
            </div>
        </div>
    </div>
{/if}

{#if (item.children) && open || (!item.isDeleted && showDeletions) }
    {#each item.children as child, i (child.id)}
        <IndexTreeItem item={child} level={level+1} onclick={onclick} state={state} isLast={item.children.length-1 === i}/>
    {/each}
{/if}
