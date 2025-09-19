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

    import type { AttributeKind } from '$lib/components/structs/Attributes';

    const attributeKindList: any[] = [
        {
            name: 'String',
            dataType: 'string',
            icon: 'gravity-ui:text',
        },
        {
            name: 'Markdown',
            dataType: 'general',
            icon: 'gravity-ui:logo-markdown',
        },
        {
            name: 'Boolean',
            dataType: 'boolean',
            icon: 'gravity-ui:copy-check-xmark'
        },
        {
            name: 'Single Option',
            dataType: { singleOption: [] },
            icon: 'gravity-ui:circle-check',
        },
        {
            name: 'Multiple Options',
            dataType: { multipleOptions: [] },
            icon: 'gravity-ui:square-check',
        },
    ];
    
    let { 
        attributeKind = $bindable(),
        class: className = "",
    } : { 
        attributeKind: AttributeKind;
        class?: string;
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
        selection = item;
        if (!item) {
            return;
        }
        let index = attributeKindList.findIndex(attr => {return (attr.name === selection)})
        if (index < 0) {
            return
        }
        attributeKind = attributeKindList[index].dataType;
    }

    onMount(() => {
        let index = attributeKindList.findIndex(item => {
            if (typeof item.dataType === 'string' && item.dataType === attributeKind) {
                return true;
            }
            if (typeof item.dataType === 'object' && typeof  attributeKind === 'object') {
                const keyItem = Object.keys(item.dataType)[0];
                const keyInput = Object.keys( attributeKind)[0];
                return keyItem === keyInput;
            }
            return false;
        });
        if (index < 0) {
            index = 0;
        }
        selection = attributeKindList[index].name;
    })
    
</script>

<Popover.Root bind:open={openCombobox}>
    <Popover.Trigger bind:ref={triggerRef} class={cn("flex justify-between w-full", className)}>
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
