<script lang="ts">
    import Icon from '@iconify/svelte';
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Table from "$lib/components/ui/table";
    import type { Link } from '$lib/components/structs/Object';
    import type { Module } from '$lib/components/structs/Module';
    import type { Object } from '$lib/components/structs/Object';
    import StringDropdown from './StringDropdown.svelte';
    import StringComboBox from './StringComboBox.svelte';
    import { createEventDispatcher } from 'svelte';

    export let links: Link[] = [];

    let modules: string[];
    let objects: string[];

    let selectedModule: string;
    let selectedObject: string;

    const dispatch = createEventDispatcher();

    function handleModuleSelection(event: any) {
        let selectedModule = event.detail.item;
        dispatch('selectModule', {item: selectedModule})
    }

    function handleObjectSelection(event: any) {
        let selectedObj = event.detail.item;
        dispatch('selectObject', {item: selectedObj})
    }
    
    function handleAddLink() {
        dispatch('selectObject', {mod: selectedModule, obj: selectedObject})
    }

    function handleRemoveLink(link: Link) {
        
    }

</script>

<Table.Root class="w-full">
    <Table.Body class="">
        <Table.Row>
            <Table.Cell class="px-1">
                <StringDropdown options={modules} on:select={handleModuleSelection}/>
            </Table.Cell>
            <Table.Cell class="px-1">
                <StringDropdown options={objects} on:select ={handleObjectSelection}/>
            </Table.Cell>
            <Table.Cell class="px-1 w-[10px]">
                <Button size="sm" variant="ghost" class="hover:text-blue-600" on:click={handleAddLink}>
                    <Icon icon="gravity-ui:circle-plus" width="20px" />
                </Button>
            </Table.Cell>
        </Table.Row>
        {#each links as link}
            <Table.Row>
                <Table.Cell class="max-h-24">
                    {link.module}
                </Table.Cell>
                <Table.Cell>
                    {link.module}-{link.object}
                </Table.Cell>
                <Table.Cell class="w-12">
                    <Button variant="ghost" class="hover:text-red-600" on:click={() => handleRemoveLink(link)}>
                        <Icon icon="gravity-ui:circle-minus" width="20px" />
                    </Button>
                </Table.Cell>
            </Table.Row>
        {/each}
    </Table.Body>
</Table.Root>
