<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import { ChevronsUpDown } from "lucide-svelte";
    
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";  
    
    let {
        items = [],
        selection = $bindable(null),
        multiple = false,
        readOnly = false,
    } : {
        items: string[];
        selection: string | string[] | null;
        multiple?: boolean;
        readOnly?: boolean;
    } = $props();
    
    let open: boolean = $state(false);
    let triggerRef = $state<HTMLButtonElement>(null!);

    let value = $derived(selection);
    
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
        <DropdownMenu.RadioGroup bind:value={value}>
            {#each items as item}
                <DropdownMenu.RadioItem 
                    value={item}
                >
                    {item}
                </DropdownMenu.RadioItem>
            {/each}
        </DropdownMenu.RadioGroup>
        {:else}
        <DropdownMenu.Group>
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
                class="justify-between w-full font-regular"
                disabled={readOnly}
                >
                <p class="w-full text-left">{(typeof value === 'string' && value) ? value : Array.isArray(value) ? value.join(', ') : ''}</p>
                <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content class="w-full">
            {@render content()}
        </DropdownMenu.Content>
    </DropdownMenu.Root>
</div>
