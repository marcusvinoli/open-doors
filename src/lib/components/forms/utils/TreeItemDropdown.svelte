<script lang="ts">
    import Icon from '@iconify/svelte';
    import Check from "lucide-svelte/icons/check";
    import ScrollArea from '../../ui/scroll-area/scroll-area.svelte';
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";

    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import { onMount } from 'svelte';
    import { getIconFromTreeItemType } from '$lib/utils/tree-item-utils';
    
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    
    import type { TreeItem } from '../../structs/Tree';

    interface Props {
        recipients?: TreeItem[];
        selected?: TreeItem | null;
        searchMessage?: string;
        noMatchMessage?: string;
    }

    let {
        recipients = [],
        selected = $bindable(null),
        searchMessage = "Enter a term for search",
        noMatchMessage = "No results for this search."
    }: Props = $props();
    
    let openCombobox: boolean = $state(false);
    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger(triggerId: string) {
        openCombobox = false;
        tick().then(() => {
            document.getElementById(triggerId)?.focus();
        });
    }

    function handleClick(item: TreeItem) {
        selected = item;
    }

    onMount(() => {
        if(!selected && recipients.length > 1) {
            selected = recipients[0];
        }
    })

</script>

{#if recipients.length > 0}
  <Popover.Root bind:open={openCombobox} >
    {#snippet children({ ids })}
        <Popover.Trigger asChild >
        {#snippet children({ builder })}
            <Button builders={[builder]} variant="outline" role="combobox" aria-expanded={openCombobox} class="justify-between w-full">
            {selected?.name}
            <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
                    {/snippet}
        </Popover.Trigger>
        <Popover.Content class="p-0 w-full">
            <Command.Root>
            <Command.Input placeholder={searchMessage} />
            <Command.Empty>{noMatchMessage}</Command.Empty>
            <ScrollArea class="h-[150px] p-1">
                <Command.Group class="">
                {#each recipients as recip}
                        <Command.Item value={recip.name} onSelect={() => {
                            handleClick(recip);
                            closeAndFocusTrigger(ids.trigger); }
                        }>
                            <Check class={cn("mr-2 h-4 w-4", selected?.name !== recip.name && "text-transparent" )}/> 
                            <Icon icon={getIconFromTreeItemType(recip)} width="15px"/>
                            <span class="pl-2">{recip.name}</span>
                            <span class="pl-2 text-xs font-light text-slate-600 truncate">{recip.path}</span>
                        </Command.Item>
                {/each}
                </Command.Group>
            </ScrollArea>
            </Command.Root>
        </Popover.Content>
        {/snippet}
    </Popover.Root>
{/if}
