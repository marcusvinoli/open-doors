<script lang="ts">
    import Icon from "@iconify/svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import AttributeForm from "./AttributeForm.svelte";

    import { Button } from "$lib/components/ui/button/index.js";
    import { readOnlyAttributes } from "$lib/utils/attribute-utils";
    import { getAttributeValueKind, getDataValues } from "./AttributeKindDropDown";

    import * as Tooltip from "$lib/components/ui/tooltip";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Table from "$lib/components/ui/table";

    import type { Module } from "$lib/components/structs/Module";
    import type { Template } from "$lib/components/structs/Template";
    import type { Attribute, AttributeKind } from "$lib/components/structs/Attributes";

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

    let template: Template = $derived($state.snapshot(module.template) as Template);
    let fields: Attribute[] = $derived([...template.fields]);
    let openEditDialog: boolean = $state(false);

    let tempAttribute: Attribute = $state({
        isMandatory: false,
        kind: 'string',
        name: '',
        description: '',
        key: '',
    });
    
    function closeDialog() {
        closeEditDialog();
        openDialog = false;
    }
    
    function closeEditDialog() {
        clearFields();
        openEditDialog = false;
    }

    function clearFields() {
        tempAttribute = {
            isMandatory: false,
            kind: 'string',
            name: '',
            description: '',
            key: '',
        };
    }

    function addAttribute(attribute: Attribute) {
        let index = fields.findIndex(attr => (attr.key === attribute.key));
        if (index < 0) {
            fields.push(attribute);
        } else {
            fields[index] = {...attribute};
        }
        fields = [...fields];
        closeEditDialog();
    }

    function removeAttribute(key: string) {
        let index = fields.findIndex(attr => (attr.key === key));
        if (index < 0) {
            return;
        }
        fields.splice(index, 1);
        fields = [...fields];
        clearFields();
    }

    function editAttribute(attribute: Attribute) {
        tempAttribute = attribute
        openEditDialog = true;
    }

    function handleSaveTemplate() {
        if (ontemplateupdate) {
            template.fields = [...fields];
            ontemplateupdate(template);
        }
        closeDialog();
    }

</script>

{#snippet attributesValues(attrKind: AttributeKind)}
    {@const val = getDataValues(attrKind)}
    <div>
        {#if typeof val === 'string'}
            <p>{val}</p>
        {:else}
            {#each val as s}
                <p>{s}</p>
            {/each}
        {/if}
    </div>
{/snippet}


<Dialog.Root bind:open={openDialog}>
    <Dialog.Content class="flex flex-col min-w-[80%] max-h-[90%] min-h-[80%]">
        <AttributeForm 
            bind:open={openEditDialog}
            attribute={tempAttribute}
            ondelete={removeAttribute}
            onclose={closeEditDialog}
            onsave={addAttribute}
            />
        <Dialog.Header class="pt-1">
            <Dialog.Title>Attributes of Module {module.manifest.prefix}</Dialog.Title>
            <Dialog.Description>
                {#if readOnly}
                    <div class="flex gap-2">
                        <Icon icon="ph:pencil-simple-slash" width="20px" />
                        Read-Only Mode. No changes can be made.
                    </div>
                {:else}
                    <p>{module.manifest.description}</p>
                {/if}
            </Dialog.Description>
        </Dialog.Header>
        <Tooltip.Provider ignoreNonKeyboardFocus>
            <Table.Root>
                <Table.Header>
                    <Table.Row class="border-b-[1px]">
                        <Table.Head class="sticky top-0 bg-slate-50 shadow-sm z-1">Attribute</Table.Head>
                        <Table.Head class="sticky top-0 bg-slate-50 shadow-sm z-1">Description</Table.Head>
                        <Table.Head class="sticky top-0 bg-slate-50 shadow-sm z-1">Data Type</Table.Head>
                        <Table.Head class="sticky top-0 bg-slate-50 shadow-sm z-1">Values</Table.Head>
                        <Table.Head class="sticky top-0 bg-slate-50 shadow-sm z-1 w-[30px]"></Table.Head>
                    </Table.Row>
                </Table.Header>
                <Table.Body>
                    {#each readOnlyAttributes.filter(ro => (ro.key !== 'header' && ro.key !== 'content')) as roAttribute (roAttribute.key)}
                        <Table.Row>
                            <Table.Cell>
                                <p>{roAttribute.name}</p>
                            </Table.Cell>
                            <Table.Cell class="whitespace-normal">
                                <p>{roAttribute.description}</p>
                            </Table.Cell>
                            <Table.Cell class="max-w-[100px]">
                                <p>{getAttributeValueKind(roAttribute.kind)}</p>
                            </Table.Cell>
                            <Table.Cell>
                                <p>Auto-Generated</p>
                            </Table.Cell>
                            <Table.Cell>
                                <Tooltip.Root>
                                    <Tooltip.Trigger>
                                        <Button variant="ghost" size="icon" disabled>
                                            <Icon icon="ph:pencil-simple-slash" width="20px" />
                                        </Button>
                                    </Tooltip.Trigger>
                                    <Tooltip.Content>
                                        <p>Read-only attribute</p>
                                    </Tooltip.Content>
                                </Tooltip.Root>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                    {#each fields as attribute (attribute.key)}
                        <Table.Row>
                            <Table.Cell>
                                <p>{attribute.name}</p>
                            </Table.Cell>
                            <Table.Cell class="whitespace-normal">
                                <p>{attribute.description}</p>
                            </Table.Cell>
                            <Table.Cell class="max-w-[120px]">
                                <p>{getAttributeValueKind(attribute.kind)}</p>
                            </Table.Cell>
                            <Table.Cell>
                                {@render attributesValues(attribute.kind)}
                            </Table.Cell>
                            <Table.Cell>
                                <Tooltip.Root>
                                    <Tooltip.Trigger>
                                        <Button variant="ghost" onclick={() => editAttribute(attribute)}>
                                            <Icon icon="ph:pencil-simple" width="20px" />
                                        </Button>
                                    </Tooltip.Trigger>
                                    <Tooltip.Content>
                                        <p>Edit</p>
                                    </Tooltip.Content>
                                </Tooltip.Root>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        </Tooltip.Provider>
        <div>
            <Button 
                variant="secondary" 
                onclick={() => editAttribute({
                    isMandatory: false,
                    kind: 'string',
                    name: '',
                    description: '',
                    key: '',
                })}>
                <Icon icon="gravity-ui:square-plus" width="20px" />
                Add Custom Attribute
            </Button>
        </div>
        <Separator/>
        <Dialog.Footer>
            <Button variant="secondary" onclick={closeDialog}>
                <Icon icon="gravity-ui:xmark" width="20px" />
                Cancel
            </Button>
            <Button onclick={handleSaveTemplate}>
                <Icon icon="gravity-ui:floppy-disk" width="20px" />
                Save Changes
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
