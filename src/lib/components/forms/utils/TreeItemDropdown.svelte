<script lang="ts">
    import Icon from '@iconify/svelte';
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";

    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import { onMount } from 'svelte';
    import { getIconFromTreeItemType } from '$lib/utils/tree-item-utils';
    
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    
    import type { TreeItem } from '../../structs/Tree';

    let {
        recipients = [],
        selected = $bindable(null),
        searchMessage = "Enter a term for search...",
        noMatchMessage = "No results for this search."
    } : {
        recipients: TreeItem[];
        selected: TreeItem | null;
        searchMessage?: string;
        noMatchMessage?: string;
    } = $props();
    
    let openCombobox: boolean = $state(false);
    let triggerRef = $state<HTMLButtonElement>(null!);
    
    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger() {
        openCombobox = false;
        tick().then(() => {
            triggerRef.focus();
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

<Popover.Root bind:open={openCombobox}>
    <Popover.Trigger bind:ref={triggerRef}>
        {#snippet  child({props})}
            <Button
            variant="outline"
            class="w-[200px] justify-between"
            {...props}
            role="combobox"
            aria-expanded={openCombobox}
            >
            {selected?.name}
            <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        {/snippet}
    </Popover.Trigger>
    <Popover.Content class="w-[200px] p-0">
        <Command.Root>
            <Command.Input placeholder={searchMessage} />
            <Command.List>
                <Command.Empty>{noMatchMessage}</Command.Empty>
                <Command.Group>
                    {#each recipients as recipient}
                        <Command.Item
                        value={recipient.name}
                        onSelect={() => {
                            handleClick(recipient);
                            closeAndFocusTrigger();
                        }}
                        >
                            <Check 
                                class={cn(
                                    "mr-2 h-4 w-4", 
                                    selected?.name !== recipient.name && "text-transparent"
                                    )}
                                /> 
                            <Icon icon={getIconFromTreeItemType(recipient)} width="15px"/>
                            <span class="pl-2">{recipient.name}</span>
                            <span class="pl-2 text-xs font-light text-slate-600 truncate">{recipient.path}</span>
                        </Command.Item>
                    {/each}
                </Command.Group>
            </Command.List>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
