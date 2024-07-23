<script lang="ts">
    import Icon from '@iconify/svelte';
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import { getIconFromTreeItemType } from '$lib/utils/getIconFromTreeItemType';
    import { onMount } from 'svelte';
    import ScrollArea from '../ui/scroll-area/scroll-area.svelte';
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import type { TreeItem } from '../structs/Tree';

    export let recipients: TreeItem[] = [];
    export let selected: TreeItem | null = null;
    export let searchMessage: string = "Enter a term for search";
    export let noMatchMessage: string = "No results for this search."
    
    let openCombobox: boolean = false;
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

{#if recipients.length > 0 }
  <Popover.Root bind:open={openCombobox} let:ids>
    <Popover.Trigger asChild let:builder>
      <Button builders={[builder]} variant="outline" role="combobox" aria-expanded={openCombobox} class="justify-between w-full">
        {selected?.name}
        <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
      </Button>
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
  </Popover.Root>
{/if}
