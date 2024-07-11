<script lang="ts">
  import { onMount, tick } from "svelte";
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { cn } from "$lib/utils.js";
    import type { TreeItem } from '../structs/Tree';
    import { getIconFromTreeItemType } from '$lib/utils/getIconFromTreeItemType';
    import Icon from '@iconify/svelte';

    export let baseRecipient: TreeItem;
    export let recipients: TreeItem[];
    
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
</script>

{#if baseRecipient}
  <Popover.Root bind:open={openCombobox} let:ids>
    <Popover.Trigger asChild let:builder>
      <Button builders={[builder]} variant="outline" role="combobox" aria-expanded={openCombobox} class="justify-between w-full">
        {baseRecipient.name}
        <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
      </Button>
    </Popover.Trigger>
    <Popover.Content class="p-0">
      <Command.Root>
        <Command.Input placeholder="Search a repo, project or folder..." />
        <Command.Empty>No Folder, Project or Repo found.</Command.Empty>
        <Command.Group>
          {#each recipients as recip}
            <Command.Item value={recip.name} onSelect={(currentValue) => { 
              baseRecipient.name = currentValue; 
              baseRecipient.path = recip.path; 
              baseRecipient.itemType = recip.itemType;
              baseRecipient.children = [] 
              closeAndFocusTrigger(ids.trigger); }
              }>
              <Check class={cn("mr-2 h-4 w-4", baseRecipient.name !== recip.name && "text-transparent" )}/> 
              <Icon icon={getIconFromTreeItemType(recip)} />
              <span class="pl-2">{recip.name}</span>
              <!-- <span class="pl-2 text-xs text-slate-600">{recip.path}</span> -->
            </Command.Item>
          {/each}
        </Command.Group>
      </Command.Root>
    </Popover.Content>
  </Popover.Root>
{/if}
