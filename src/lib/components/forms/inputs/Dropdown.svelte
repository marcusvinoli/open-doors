<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";

    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
    
    let {
        items = [],
        selected = $bindable(null),
        placeholder = "Enter a term for search...",
        emptyMessage = "No results for this search."
    } : {
        items: string[];
        selected: string | null;
        placeholder?: string;
        emptyMessage?: string;
    } = $props();
    
    let open: boolean = $state(false);
    let triggerRef = $state<HTMLButtonElement>(null!);

    let value = $derived(selected);
    
    function closeAndFocusTrigger() {
        open = false;
        tick().then(() => {
            triggerRef.focus();
        });
    }

    function handleClick(item: string) {
        if (value === item) {
            value = null;
        } else {
            value = selected = item;
        }
    }

</script>

<div class="w-full">
    <Popover.Root bind:open>
        <Popover.Trigger bind:ref={triggerRef}>
            {#snippet  child({props})}
                <Button
                variant="outline"
                role="combobox"
                {...props}
                aria-expanded={open}
                class="justify-between w-full font-regular"
                >
                <p class="w-full text-left">{value}</p>
                <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
                </Button>
            {/snippet}
        </Popover.Trigger>
        <Popover.Content class="p-0 w-full">
            <Command.Root class="w-full">
                <Command.Input placeholder={placeholder} />
                <Command.List>
                    <Command.Empty>{emptyMessage}</Command.Empty>
                    <ScrollArea class="max-h-[150px] p-1">
                        <Command.Group>
                            {#each items as item}
                                <Command.Item
                                value={item}
                                onSelect={() => {
                                    handleClick(item);
                                    closeAndFocusTrigger();
                                }}
                                >
                                <div class="w-full flex justify-start items-center">
                                    <Check class={cn("mr-1", value !== item && "text-transparent")}/> 
                                    <span class="pl-1">{item}</span>
                                </div>
                                </Command.Item>
                            {/each}
                        </Command.Group>
                    </ScrollArea>
                </Command.List>
            </Command.Root>
        </Popover.Content>
    </Popover.Root>
</div>
