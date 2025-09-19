<script lang="ts">
    import Icon from "@iconify/svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import AttributeKindDropDown from "./AttributeKindDropDown.svelte"

    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Separator } from "$lib/components/ui/separator";
    import { getDataValues } from "./AttributeKindDropDown";
    import { isValidAttributeName } from "$lib/utils/name-validator";

    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Table from "$lib/components/ui/table";

    import type { Attribute, AttributeKind } from "$lib/components/structs/Attributes";

    let {
        attribute,
        ondelete,
        onsave,
        onclose,
        open = $bindable(false),
    } : {
        attribute: Attribute;
        open: boolean;
        ondelete?: (key: string) => void;
        onsave?: (attribute: Attribute) => void;
        onclose?: () => void;
    } = $props();

    let tempAttribute: Attribute = $derived($state.snapshot(attribute) as Attribute);

    let tempAttrName: string = $derived(tempAttribute.name);
    let tempAttrDesc: string = $derived(tempAttribute.description);
    let tempAttrKind: AttributeKind = $derived(tempAttribute.kind);
    let tempAttrValue: string = $state('');
    let tempAttrValues: string[] = $derived(
        (typeof tempAttrKind === 'object') ? 
        getDataValues(tempAttrKind) as string[] : 
        []
    );
    //let tempAttrKind: AttributeKind = $derived(cloneAttributeKind(tempAttribute.kind));
    //let tempAttrValues: string | string [] = $derived(getDataValues(tempAttrKind));
    //let dataValues: string[] = $derived(typeof tempAttrValues === 'string' ? [] : tempAttrValues);

    let disableAddButton: boolean = $derived.by(() => {
        return (!isValidAttributeName(tempAttrName) || 
        (isValidAttributeName(tempAttrName) && requiresValueList && !(tempAttrValues.length > 0)))
    });

    let requiresValueList: boolean = $derived.by(() => {
        if (typeof tempAttrKind === 'object') {
            return ('singleOption' in tempAttrKind || 'multipleOptions' in tempAttrKind)
        }
        return false;
    });

    function handleDeleteAttribute() {
        if (ondelete) {
            ondelete(tempAttribute.key);
        }
    }

    function handleSaveAttribute() {
        if (onsave) {
            if (!tempAttribute.key) {
                tempAttribute.key = generateKey(tempAttrName);
            }
            tempAttribute.name = tempAttrName.trim();
            tempAttribute.description = tempAttrDesc.trim();
            if (typeof tempAttrKind === 'object') {
                if ('singleOption' in tempAttrKind) {
                    tempAttribute.kind = { singleOption: tempAttrValues };
                } else if ('multipleOptions' in tempAttrKind) {
                    tempAttribute.kind = { multipleOptions: tempAttrValues };
                }
            } else {
                tempAttribute.kind = tempAttrKind;
            }
            const newAttr: Attribute = {...tempAttribute} as Attribute;
            onsave(newAttr);
        }
    }

    function generateKey(input: string): string {
        const sanitized = input.toLowerCase().replace(/[^a-z0-9]/g, '');
        const truncated = sanitized.length > 30 ? sanitized.substring(0, 30) : sanitized;
        return truncated;
    }

    function moveValueUp(index: number) {
        if (index === 0) {
            return;
        }
        const temp: string = tempAttrValues[index - 1];
        tempAttrValues[index - 1] = tempAttrValues[index];
        tempAttrValues[index] = temp;
        tempAttrValues = [...tempAttrValues];
    }

    function moveValueDown(index: number) {
        if (index === tempAttrValues.length - 1) {
            return;
        }
        const temp: string = tempAttrValues[index + 1];
        tempAttrValues[index + 1] = tempAttrValues[index];
        tempAttrValues[index] = temp;
        tempAttrValues = [...tempAttrValues];
    }

    function removeValue(index: number) {
        tempAttrValues.splice(index, 1);
        tempAttrValues = [...tempAttrValues];
    }

    function addValue(value: string) {
        tempAttrValues.push(value.trim());
        tempAttrValues = [...tempAttrValues];
        tempAttrValue = '';
    }

    function handleClose() {
        if (onclose) {
            onclose();
        }
    }

</script>

<Dialog.Root bind:open={open}>
    <Dialog.Content class="min-w-150" onclose={handleClose}>
        <Dialog.Header class="pt-1">
            <Dialog.Title>{tempAttribute.key ? 'Edditing "' + tempAttribute!.name + '"' : 'New Attribute'}</Dialog.Title>
            {#if tempAttribute.key !== ''}
            <Dialog.Description><p>Object key: <span class="font-mono">{tempAttribute.key}</span></p></Dialog.Description>
            {:else}
            <Dialog.Description><p>Creating a new custom attribute</p></Dialog.Description>
            {/if}
        </Dialog.Header>
        <div class="flex flex-col gap-2">
            <div class="grid flex-2 grid-cols-8">
                <Label for="name" class="text-right col-span-2">Name</Label>
                <Input id="name" class="col-span-6" autocomplete="off" bind:value={tempAttrName}/>
            </div>
            <div class="grid flex-2 grid-cols-8">
                <Label for="name" class="text-right col-span-2">Description</Label>
                <Input id="name" class="col-span-6" autocomplete="off" bind:value={tempAttrDesc}/>
            </div>
            <div class="grid flex-2 grid-cols-8">
                <Label for="name" class="text-right col-span-2">Type</Label>
                <AttributeKindDropDown class="col-span-6"  bind:attributeKind={tempAttrKind}/>
            </div>
            {#if requiresValueList}
                <div class="grid flex-2 grid-cols-8">
                    <Label for="name" class="text-right col-span-2">Values</Label>
                    <div class="col-span-6 max-h-40 overflow-auto">
                        <Table.Root class="box-border">
                            <Table.Header class="z-10">
                                <Table.Row class="bg-slate-100">
                                    <Table.Head class="sticky top-0 p-0 z-10 shadow-sm">
                                        <div class="flex items-center gap-2 px-2 py-1.5 box-content bg-slate-100">
                                            <Input bind:value={tempAttrValue} autocomplete="off" placeholder="Add new Value..."/>
                                            <div class="text-slate-400">
                                                <Button variant="ghost" size="icon" class="hover:text-blue-400 p-0" onclick={() => addValue(tempAttrValue)}>
                                                    <Icon icon="gravity-ui:square-plus" width="10px" />
                                                </Button>
                                            </div>
                                        </div>
                                    </Table.Head>
                                </Table.Row>
                            </Table.Header>
                            <Table.Body>
                                {#each tempAttrValues as val, i}
                                    <Table.Row>
                                        <Table.Cell class="py-0 pl-2">
                                            <div class="flex items-center justify-between">
                                                <p class="pl-1">{val}</p>
                                                <div class="text-slate-400">
                                                    <Button variant="ghost" size="icon" class="hover:text-slate-600 p-0" onclick={() => moveValueUp(i)} disabled={(i === 0)}>
                                                        <Icon icon="gravity-ui:caret-up" width="20px" />
                                                    </Button>
                                                    <Button variant="ghost" size="icon" class="hover:text-slate-600 p-0" onclick={() => moveValueDown(i)} disabled={(i === tempAttrValues.length - 1)}>
                                                        <Icon icon="gravity-ui:caret-down" width="20px" />
                                                    </Button>
                                                    <Button variant="ghost" size="icon" class="hover:text-red-400 p-0" onclick={() => removeValue(i)}>
                                                        <Icon icon="gravity-ui:square-minus" width="10px" />
                                                    </Button>
                                                </div>
                                            </div>
                                        </Table.Cell>
                                    </Table.Row>
                                {:else}
                                    <Table.Row>
                                        <Table.Cell>
                                            <div class="flex items-center justify-center gap-2 text-amber-600 rounded bg-amber-100 p-2 select-none">
                                                <Icon icon="gravity-ui:triangle-exclamation" width="20px" />
                                                At least one Value must be specified.
                                            </div>
                                        </Table.Cell>
                                    </Table.Row>
                                {/each}
                            </Table.Body>
                        </Table.Root>
                    </div>
                </div>
            {/if}
        </div>
        <Separator class="my-1" />
        <Dialog.Footer class="px-2">
            {#if tempAttribute.key}
                <Button variant="destructive" onclick={handleDeleteAttribute}>
                    <Icon icon="gravity-ui:trash-bin" width="20px" />
                    Delete Attribute
                </Button>
            {/if}
            <div class="grow"></div>
            <Button variant="secondary" onclick={handleClose}>
                <Icon icon="gravity-ui:xmark" width="20px" />
                Cancel
            </Button>
            <Button disabled={disableAddButton} onclick={handleSaveAttribute}>
                <Icon icon="gravity-ui:floppy-disk" width="20px" />
                Save
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
