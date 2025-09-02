<script lang="ts">
    import Icon from '@iconify/svelte';
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";

    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import { onMount } from 'svelte';
    
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";

    const attributeKindList: any[] = [
        {
            name: "String",
            dataType: "string",
            icon: "gravity-ui:text"
        },
        {
            name: "Markdown",
            dataType: "general",
            icon: "gravity-ui:logo-markdown"
        },
        {
            name: "Single Option",
            dataType: "singleOption",
            icon: "gravity-ui:circle-check"
        },
        {
            name: "Multiple Option",
            dataType: "multipleOptions",
            icon: "gravity-ui:square-check"
        },
        {
            name: "Boolean",
            dataType: "boolean",
            icon: "gravity-ui:copy-check-xmark"
        },
    ];
    
    let { 
        attributeKind = $bindable(null)
    } : { 
        attributeKind: string | null; 
    } = $props();

    let selection: string | null = $state(null);
    let openCombobox: boolean = $state(false);
    let triggerRef = $state<HTMLButtonElement>(null!);

    function closeAndFocusTrigger() {
        openCombobox = false;
        tick().then(() => {
        triggerRef.focus();
        });
    }

    function handleClick(item: string | null) {
        attributeKind = null;
        selection = item;
        if (!item) {
            return;
        }
        let seleIndex = attributeKindList.findIndex(attr => {return (attr.name === selection)})
        if (seleIndex < 0) {
            return
        }
        attributeKind = attributeKindList[seleIndex].dataType;
    }

    onMount(() => {
        if(attributeKind) {
            let seleIndex = attributeKindList.findIndex(item => {return (item.dataType === attributeKind)})
            if (seleIndex < 0) { 
                seleIndex = 0;
                
            }
            selection = attributeKindList[seleIndex].name;
        } else {
            selection = null;
        }
    })
    
</script>

<Popover.Root bind:open={openCombobox}>
    <Popover.Trigger bind:ref={triggerRef} class="flex justify-between w-full max-w-96">
        {#snippet child({ props })}
            <Button
                variant="outline"
                {...props}
                role="combobox"
                aria-expanded={openCombobox}
            >
            <p>{ selection || "Attribute Type..."}</p>
            <ChevronsUpDown class="ml-2 size-4 shrink-0 opacity-50" />
            </Button>
        {/snippet}
    </Popover.Trigger>
    <Popover.Content class="p-0 w-full">
        <Command.Root>
            <Command.Group>
                {#each attributeKindList as atKind}
                <Command.Item value={atKind.name} onSelect={() => { handleClick(atKind.name); closeAndFocusTrigger(); }}>
                    <Check class={cn("mx-1 h-4 w-4", selection !== atKind.name && "text-transparent" )}/> 
                    <Icon icon={atKind.icon} width="15px"/>
                    <span class="pl-2">{atKind.name}</span>
                </Command.Item>
                {/each}
            </Command.Group>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
