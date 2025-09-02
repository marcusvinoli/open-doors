<script lang="ts">
    import Icon from "@iconify/svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import AttributeKindDropDown from "./AttributeKindDropDown.svelte"

    import { Button } from "$lib/components/ui/button/index.js";
    import { isValidAttributeName } from "$lib/utils/name-validator";

    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Table from "$lib/components/ui/table";

    import type { Module } from "$lib/components/structs/Module";
    import type { Attribute, AttributeKind } from "$lib/components/structs/Attributes";
    import type { Template } from "$lib/components/structs/Template";

    let { 
        openDialog = $bindable(false), 
        module,
        ontemplateupdate,
        readOnly = false,
    } : {
        openDialog?: boolean;
        module: Module;
        readOnly?: boolean;
        ontemplateupdate?: (template: Template) => void;
    } = $props();

    let tempTemplate: Template = $state({... module.template} as Template);
    let newAttributeName: string = $state("");
    let newAttributeDescription: string = $state("");
    let newAttributeKind: string | null = $state(null);
    let newAllowedValues: string | null = $state(null);
    let disableValueList: boolean = $derived(!(newAttributeKind === 'singleOption' || newAttributeKind === 'multipleOptions'));
    let disableAddButton: boolean = $derived(!((isValidAttributeName(newAttributeName) && (newAttributeKind)) && (isValidAttributeName(newAllowedValues)&&(!disableValueList) || disableValueList)));

    function closeDialog() {
        clearFields();
        openDialog = false;
    }

    function clearFields() {
        newAttributeName = "";
        newAttributeDescription = "";
        newAttributeKind = null;
        newAllowedValues = null;
    }

    function generateKey(input: string): string {
        const sanitized = input.toLowerCase().replace(/[^a-z0-9]/g, '');
        const truncated = sanitized.length > 30 ? sanitized.substring(0, 30) : sanitized;
        const randomized = truncated + String(Math.floor(Math.random()*100).toString()).padStart(3, '0');
        return randomized;
    }

    function generateDataKind(attributeKind: string | null, allowedValues?: string | null): AttributeKind {
        switch (attributeKind) {
            case "singleOption":
                return { singleOption: allowedValues!.split(",").map(s => s.trim()) };
            case "multipleOptions": 
                return { multipleOptions: allowedValues!.split(",").map(s => s.trim()) };
            case "boolean":
                return 'boolean';
            case "general":
                return 'general';
            default: // For string, integer, real, date, time, dateTime, 
                return 'string';
        }
    }

    function addAttribute() {
        if (newAttributeName && !newAttributeKind) {
            return;
        }

        let newAttr: Attribute = {
            isMandatory: false, // TODO: A validation of this field should be included on software.
            name: newAttributeName,
            description: newAttributeDescription,
            kind: generateDataKind(newAttributeKind, newAllowedValues),
            key: generateKey(newAttributeName),
        };

        tempTemplate.fields = [...tempTemplate.fields, newAttr];
        console.log($state.snapshot(tempTemplate));
        clearFields();
    }

    function removeAttribute(key: string) {
        let index = tempTemplate.fields.findIndex(attr => (attr.key === key));
        if (index < 0) {
            return;
        }
        tempTemplate.fields.splice(index, 1);
    }

    function getAttributeKind(attributeKind: AttributeKind) {
        switch (attributeKind) {
            case "string":
                return "String";
            case "general": // TODO: Include number, date, time, dateTime, etc...
                return "Markdown";
            case "boolean":
                return "True/False";
            default: // TODO: Include number, date, time, dateTime, etc...
                let dataKind = Object.keys(attributeKind)[0];
                if (dataKind === "singleOption") {
                    return "Single Option";
                } else if (dataKind === "multipleOption") {
                    return "Multiple Option";
                } else {
                    return "Text";
                }
        }
    }

    function getDataValues(attributeKind: AttributeKind) {
        switch (attributeKind) {
            case "string":
                return "Text";
            case "general": // TODO: Include number, date, time, dateTime, etc...
                return "Formatted text";
            case "boolean":
                return "True/False";
            default: // TODO: Include number, date, time, dateTime, etc...
                let dataKind = Object.keys(attributeKind)[0];
                let dataValues = Object.values(attributeKind)[0] as string[];
                if (dataKind === "singleOption" || dataKind === "multipleOptions") {
                    return "Options: " + dataValues.join(", ");
                } else {
                    return "Other";
                }
        }
    }

    function handleSaveTemplate() {
        if (ontemplateupdate) {
            ontemplateupdate(tempTemplate);
        }
        closeDialog();
    }

</script>

<Dialog.Root bind:open={openDialog}>
    <Dialog.Content class="flex flex-col min-w-[80%] max-h-[90%] min-h-[80%]">
        <Dialog.Header class="pt-2 pb-1">
            <Dialog.Title>Custom Attributes of Module {module.manifest.prefix}</Dialog.Title>
            <Dialog.Description>{module.manifest.title} module</Dialog.Description>
        </Dialog.Header>
        <Table.Root class="">
            <Table.Header class="">
                <Table.Row class="border-b-[1px]">
                    <Table.Head class="sticky top-0 bg-slate-50 shadow-sm">Attribute</Table.Head>
                    <Table.Head class="sticky top-0 bg-slate-50 shadow-sm">Description</Table.Head>
                    <Table.Head class="sticky top-0 bg-slate-50 shadow-sm w-[200px]">Data Type</Table.Head>
                    <Table.Head class="sticky top-0 bg-slate-50 shadow-sm">Values</Table.Head>
                    <Table.Head class="sticky top-0 bg-slate-50 shadow-sm w-[30px]"></Table.Head>
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {#each tempTemplate.fields as attribute}
                <Table.Row>
                    <Table.Cell>
                        {attribute.name}
                    </Table.Cell>
                    <Table.Cell class="whitespace-normal">
                        {attribute.description}
                    </Table.Cell>
                    <Table.Cell class="max-w-[100px]">
                        { getAttributeKind(attribute.kind) }
                    </Table.Cell>
                    <Table.Cell>
                        { getDataValues(attribute.kind) }
                    </Table.Cell>
                    <Table.Cell class="max-w-[30px] pl-1 pr-2">
                        <Button variant="ghost" class="hover:text-red-600" onclick={() => removeAttribute(attribute.key)}>
                            <Icon icon="gravity-ui:circle-minus" width="20px" />
                        </Button>
                    </Table.Cell>
                </Table.Row>
                {/each}
                {#if !readOnly}
                    <Table.Row class="">
                        <Table.Cell class="pl-2 pr-1">
                            <Input bind:value={newAttributeName} placeholder="Name..." class="px-2 py-1 w-full" autocomplete="off"/>
                        </Table.Cell>
                        <Table.Cell class="pl-2 pr-1">
                            <Input bind:value={newAttributeDescription} placeholder="Description..." class="px-2 py-1 w-full" autocomplete="off"/>
                        </Table.Cell>
                        <Table.Cell class="px-1">
                            <AttributeKindDropDown bind:attributeKind={newAttributeKind} />
                        </Table.Cell>
                        <Table.Cell class="px-1">
                            <Input bind:value={newAllowedValues} placeholder="Comma, Separeted, Values" class="px-2 py-1 w-full" disabled={disableValueList}/>
                        </Table.Cell>
                        <Table.Cell class="w-[30px] pl-1 pr-2">
                            <Button variant="ghost" class="hover:text-blue-600" onclick={addAttribute} disabled={disableAddButton}>
                                <Icon icon="gravity-ui:circle-plus" width="20px" />
                            </Button>
                        </Table.Cell>
                    </Table.Row>
                {/if}
            </Table.Body>
        </Table.Root>
        <div class="grow"></div>
        <Dialog.Footer>
            <Button variant="secondary" onclick={closeDialog}>Cancel</Button>
            <Button onclick={handleSaveTemplate}>Save Changes</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>

