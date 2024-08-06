<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ChevronDown from "lucide-svelte/icons/chevron-down";
    import { onMount, tick } from "svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { cn } from "$lib/utils.js"; 

    export let options: string[] = [];
    export let selected: string[] = [];
    export let text: string = "...";
    let placeholder: string = text;

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

    function changeSelection(item: string) {
        let index = selected.findIndex((opt) => {return (opt === item)})
        if (index < 0) {
            selected.push(item);
        } else {
            selected.splice(index, 1);
        }
        generatePlaceholder();
    }

    function generatePlaceholder() {
        if (selected.length > 0) {
            placeholder = selected.join(", ");
        } else {
            placeholder = text;
        }
    }

</script>

<Popover.Root bind:open={openCombobox} let:ids>
    <Popover.Trigger asChild let:builder>
      <Button builders={[builder]} variant="outline" role="combobox" aria-expanded={openCombobox} class="justify-between w-full">
        {placeholder}
        <ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
      </Button>
    </Popover.Trigger>
    <Popover.Content class="p-0">
      <Command.Root>
          <Command.Group class="">
            {#each options as option}
              <Command.Item value={option} onSelect={() => { changeSelection(option); closeAndFocusTrigger(ids.trigger); }}>
                <Check class={cn("mr-2 h-4 w-4", !(selected.includes(option)) && "text-transparent" )}/> 
                <span class="pl-1">{option}</span>
              </Command.Item>
            {/each}
          </Command.Group>
      </Command.Root>
    </Popover.Content>
  </Popover.Root>
