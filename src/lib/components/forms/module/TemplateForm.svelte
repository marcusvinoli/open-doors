<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from '../../ui/loading/Loading.svelte';
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import { createEventDispatcher, onMount } from 'svelte';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Table from "$lib/components/ui/table";
    import type { Module } from "$lib/components/structs/Module";
    import type { Field, DataKind, NullableAny, NullableOption, NullableOptions } from "$lib/components/structs/Template";
    import DataTypeDropdown from "./DataTypeDropdown.svelte";
    import { readModule, readModuleFromPath, saveTemplate } from "$lib/controllers/Module";

    const dispatch = createEventDispatcher();

    export let openDialog: boolean = false;
    export let module: Module;
    let tempModule: Module;

    let loading: boolean = false;
    let loadingMessage: string = "Saving template...";

    let newAttribute: string;
    let newDataTypeSelected: string;
    let newAllowedValues: string;
    
    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    function generateKey(input: string): string {
        const sanitized = input.toLowerCase().replace(/[^a-z0-9]/g, '');
        const truncated = sanitized.length > 30 ? sanitized.substring(0, 30) : sanitized;
        return truncated;
    }

    function generateDataKing(dataType: string, allowedValues: string): DataKind {
        switch (dataType) {
            case "nullableOptions": 
            return { nullableOptions: allowedValues.split(",") };

            case "nullableOptions": 
            return { nullableOptions: allowedValues.split(",") };
            
            default:
            return { any: "" };
        }
    }

    function addAttribute() {
        if (newAttribute === "") {
            return;
        }

        let newField: Field = {
            attribute: newAttribute,
            kind: generateDataKing(newDataTypeSelected, newAllowedValues),
            key: generateKey(newAttribute),
        }

        tempModule.template.fields = [...tempModule.template.fields, newField];

        newAttribute = "";
        newDataTypeSelected = "any";
        newAllowedValues = "";

        console.log(newField);
        console.log("tempModule", tempModule.template.fields);
    }

    function removeAttribute(key: string) {
        let index = tempModule.template.fields.findIndex((fd) => fd.key === key);
        if (index < 0) {
            return;
        }
        tempModule.template.fields.splice(index, 1);
        tempModule.template.fields = tempModule.template.fields;
    }

    async function handleSaveTemplate() {
        await saveTemplate(module.path, tempModule.template);
        readModuleFromPath(module.path)
            .then(mod => {
                module = mod as Module;
            })
            .finally(() => {
                closeDialog();
            })
    }

    $: if (openDialog && module) {
        tempModule = module;
    }
</script>

{#if tempModule}
<Dialog.Root bind:open={openDialog} closeOnEscape closeOnOutsideClick>
    <Dialog.Content class="min-w-[70%] min-h-[50%] flex flex-col">
        {#if loading}
        <div class="flex flex-col items-center">
            <Loading />
            <h1 class="leading-1 pt-1 my-2">{loadingMessage}</h1>
        </div> 
        {:else}
        <Dialog.Header>
            <Dialog.Title>Template for {tempModule.manifest.prefix} Module</Dialog.Title>
            <Dialog.Description>{tempModule.manifest.title} formal module</Dialog.Description>
        </Dialog.Header>
        <!-- <ScrollArea> -->
            <Table.Root class="w-full">
                <Table.Header class="">
                    <Table.Row>
                        <Table.Head class="sticky top-0 bg-slate-50 shadow-sm">Attribute</Table.Head>
                        <Table.Head class="sticky top-0 bg-slate-50 shadow-sm w-[200px]">Data Type</Table.Head>
                        <Table.Head class="sticky top-0 bg-slate-50 shadow-sm">Parameters</Table.Head>
                        <Table.Head class="sticky top-0 bg-slate-50 shadow-sm w-[30px]"></Table.Head>
                    </Table.Row>
                </Table.Header>
                <Table.Body class="">
                    <Table.Row class="">
                        <Table.Cell>
                            <input bind:value={newAttribute} placeholder="New attribute..." class="bg-transparent px-2 py-1 w-full"/>
                        </Table.Cell>    
                        <Table.Cell>
                            <DataTypeDropdown bind:dataType={newDataTypeSelected} />
                        </Table.Cell>
                        <Table.Cell>
                            <input bind:value={newAllowedValues} placeholder="Comma,Separeted,Values" class="bg-transparent px-2 py-1 w-full"/>
                        </Table.Cell>
                        <Table.Cell class="w-12">
                            <Button variant="ghost" class="hover:text-blue-600" on:click={addAttribute}>
                                <Icon icon="gravity-ui:circle-plus" width="20px" />
                            </Button>
                        </Table.Cell>
                    </Table.Row>
                    {#each tempModule.template.fields as field}
                        <Table.Row>
                            <Table.Cell class="max-h-24">
                                <input bind:value={field.attribute} placeholder="Create a new attribute..." class="bg-transparent px-2 py-1 w-full"/>
                            </Table.Cell>
                            <Table.Cell>
                                {Object.keys(field.kind)[0]}
                            </Table.Cell>
                            <Table.Cell>
                                {Object.values(field.kind)}
                            </Table.Cell>
                            <Table.Cell class="w-12">
                                <Button variant="ghost" class="hover:text-red-600" on:click={() => removeAttribute(field.key)}>
                                    <Icon icon="gravity-ui:circle-minus" width="20px" />
                                </Button>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        <!-- </ScrollArea> -->
        {/if}
        <div class="grow"></div>
        <Dialog.Footer>
            <div class="grow"></div>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleSaveTemplate}>Save Changes</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
{/if}