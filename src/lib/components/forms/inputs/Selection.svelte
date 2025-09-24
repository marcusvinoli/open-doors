<script lang="ts">
    import { cn } from "$lib/utils";
    import { Button } from "$lib/components/ui/button/index.js";
    import { ChevronsUpDown } from "lucide-svelte";
    
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";  
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
    
    let {
        items = [],
        selection = $bindable(null),
        multiple = false,
        readOnly = false,
        class: customClass = "",
    } : {
        items: string[];
        selection: string | string[] | null;
        multiple?: boolean;
        readOnly?: boolean;
        class?: string;
    } = $props();
    
    let open: boolean = $state(false);
    let value: string | string[] | null = $derived(selection);
    
    function handleClick(item: string) {
        if (typeof value !== 'string' && value) {
            if(value.includes(item)) {
                value = value.filter(i => i !== item);
            } else {
                value = [...value,  item]
            }
            selection = value;
        }
    }

</script>

{#snippet content()}
    {#if !multiple && (typeof value === 'string')}
        <DropdownMenu.RadioGroup bind:value={selection as string}>
            <ScrollArea>
                {#each items as item}
                    <DropdownMenu.RadioItem 
                        value={item}
                        onclick={() => {value = item;}}
                    >
                        {item}
                    </DropdownMenu.RadioItem>
                {/each}
            </ScrollArea>
        </DropdownMenu.RadioGroup>
    {:else}
        <DropdownMenu.Group>
            <ScrollArea>
                {#each items as item}
                    <DropdownMenu.CheckboxItem 
                        checked={value?.includes(item)}  
                        onclick={(e) => {
                            e.preventDefault();
                            e.stopPropagation();
                            handleClick(item)
                        }}
                    >
                        {item}
                    </DropdownMenu.CheckboxItem>
                {/each}
            </ScrollArea>
        </DropdownMenu.Group>
    {/if}
{/snippet}

<div class="w-full">
    <DropdownMenu.Root bind:open>
        <DropdownMenu.Trigger class="w-full" disabled={readOnly}>
            <Button
                variant="outline"
                role="combobox"
                aria-expanded={open}
                class={cn(
                    "justify-between w-full font-regular",
                    customClass
                    )}
                disabled={readOnly}
                >
                <p class="truncate text-left">
                    {(Array.isArray(value)) ? value.join(', ') : selection}
                </p>
                <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content class="w-full max-h-50">
            {@render content()}
        </DropdownMenu.Content>
    </DropdownMenu.Root>
</div>
