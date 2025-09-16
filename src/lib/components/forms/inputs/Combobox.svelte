<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ScrollArea from '../../ui/scroll-area/scroll-area.svelte';
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    
    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    
    let { 
        items = [],
        selected = $bindable(),
        placeholder = 'Select...',
        emptyMessage = 'No result found.',
    } : {
        items: string[];
        selected: string[];
        placeholder?: string;
        emptyMessage?: string;
    } = $props();

    let open: boolean = $state(false);
    let triggerRef = $state<HTMLButtonElement>(null!);
    
    let values = $derived(selected);
    
    function closeAndFocusTrigger() {
        tick().then(() => {
            triggerRef.focus();
        });
    }

    function handleClick(item: string) {
        if(values.includes(item)) {
            values = values.filter(i => i !== item);
        } else {
            values = [...values,  item]
        }
        selected = values;
    }

</script>

<div class="w-full">
    <Popover.Root bind:open>
        <Popover.Trigger bind:ref={triggerRef} >
        {#snippet child({ props })}
            <Button 
                variant="outline" 
                role="combobox" 
                {...props}
                aria-expanded={open} 
                class="justify-between w-full font-regular"
            >
            <p class="w-full text-left">{values.join(', ') || placeholder}</p>
            <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        {/snippet}
        </Popover.Trigger>
        <Popover.Content class="p-0 w-full">
            <Command.Root class="w-full">
                <Command.Input {placeholder} />
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
                                    }}>
                                    <div class="w-full flex justify-start items-center">
                                        <Check class={cn("mr-1", !values.includes(item) && "text-transparent")}/>
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
