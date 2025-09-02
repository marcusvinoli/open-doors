<script lang="ts">
    import Icon from '@iconify/svelte';
    import Check from "lucide-svelte/icons/check";
    import ScrollArea from '../../ui/scroll-area/scroll-area.svelte';
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    
    import { cn } from "$lib/utils.js";
    import { onMount, tick } from "svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from '$lib/stores/Repository.svelte';
    import { getIconFromTreeItemType } from '$lib/utils/tree-item-utils';
    
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    
    import type { TreeItem } from '../../structs/Tree';

    let { 
        selectedItem = $bindable(),
        placeholder = 'Select...',
        items,
    } : {
        selectedItem: TreeItem | null;
        placeholder?: string;
        items: TreeItem[],
    } = $props();

    let open: boolean = $state(false);
    let triggerRef = $state<HTMLButtonElement>(null!);

    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger() {
        open = false;
        tick().then(() => {
            triggerRef.focus();
        });
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
                class="justify-between w-full"
            >
            {selectedItem?.name || placeholder}
            <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        {/snippet}
        </Popover.Trigger>
        <Popover.Content class="p-0 w-full">
            <Command.Root class="w-full">
                <Command.Input placeholder="Search for a repo, project or folder..." />
                <Command.List>
                    <Command.Empty>No Folder, Project or Repo found.</Command.Empty>
                    <ScrollArea class="max-h-[150px] p-1">
                        <Command.Group class="">
                            {#each items as item}
                            <Command.Item
                                class="" 
                                value={item.name} 
                                onSelect={() => {
                                    selectedItem = (selectedItem == item) ? null : item;
                                    closeAndFocusTrigger(); }
                                }>
                                <div class="w-full flex justify-start items-center">
                                    <Check class={cn("mr-1 h-4 w-4", selectedItem?.name !== item.name && "text-transparent" )}/> 
                                    <Icon icon={getIconFromTreeItemType(item)} width="15px"/>
                                    <span class="pl-1">{item.name}</span>
                                    <span class="ml-auto pl-4 text-xs font-light italic text-right text-slate-600 truncate">{item.path?.substring(repository()?.tree.path.length)}</span>
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
