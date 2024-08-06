<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import { cn } from "$lib/utils.js"; 
    import { tick } from "svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { ChevronDown } from "lucide-svelte";

    export let options: string[] = [];
    export let selected: string = options[0] ? options[0] : "";
    let selection: string;

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

    $: {
        selection = selected;
    }
</script>

<Popover.Root bind:open={openCombobox} let:ids>
    <Popover.Trigger asChild let:builder>
      <Button builders={[builder]} variant="outline" role="combobox" aria-expanded={openCombobox} class="justify-between w-full">
        {selection}
      <ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
      </Button>
    </Popover.Trigger>
    <Popover.Content class="p-0 min-w-12">
      <Command.Root>
          <Command.Group class="min-w-12">
            {#each options as option}
              <Command.Item value={option} onSelect={() => { closeAndFocusTrigger(ids.trigger); selected = option;  }}>
                <Check class={cn("mr-2 h-4 w-4", !(option === selected) && "text-transparent" )}/> 
                <span class="pl-1">{option}</span>
              </Command.Item>
            {/each}
          </Command.Group>
      </Command.Root>
    </Popover.Content>
  </Popover.Root>
