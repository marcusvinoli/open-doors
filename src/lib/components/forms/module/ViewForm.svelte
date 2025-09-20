<script lang="ts">
    import Icon from "@iconify/svelte";
    import Input from "$lib/components/ui/input/input.svelte";

    import { Label } from "$lib/components/ui/label";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Separator } from "$lib/components/ui/separator";
    import { isValidAttributeName } from "$lib/utils/name-validator";

    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Table from "$lib/components/ui/table";

    import type { Template } from "$lib/components/structs/Template";
    import type { Attribute } from "$lib/components/structs/Attributes";
    import type { View, ViewItem } from "$lib/components/structs/View";
    import { readOnlyAttributes } from "$lib/utils/attribute-utils";
    import Combobox from "../inputs/Combobox.svelte";

    let {
        view,
        template,
        readOnly = false,
        ondelete,
        onclose,
        onsave,
        open = $bindable(false),
    } : {
        view: View;
        template: Template;
        readOnly?: boolean;
        ondelete?: (view: View) => void;
        onclose?: () => void;
        onsave?: (view: View) => void;
        open: boolean;
    } = $props();

    let tempView: View = $derived($state.snapshot(view) as View);

    let tempViewName: string = $derived(tempView.name);
    let tempViewDesc: string = $derived(tempView.description);
    let tempViewItems: ViewItem[] = $derived([...tempView.items]);
    let attributeList: string[] = $derived.by(() => {
        let result: string[] = [];
        readOnlyAttributes.forEach(ro => result.push(ro.name));
        template.fields.forEach(a => result.push(a.name));
        return result;
    });
    let selectedAttribute: string[] = $derived.by(() => {
        let result: string[] = []
        tempViewItems.forEach(i => result.push(i.attribute));
        return result;
    });
    let sortedList: string[] = $derived([...selectedAttribute]);
    
    let disableAddButton: boolean = $derived.by(() => {
        return (!isValidAttributeName(tempViewName) || !(sortedList.length > 0))
    });

    function deleteView() {
        if (ondelete) {
            ondelete(view);
        }
    }

    function saveView() {
        if (onsave) {
            tempView.name = tempViewName.trim();
            tempView.description = tempViewDesc.trim();
            let newItems: ViewItem[] = [];
            sortedList.forEach(listed => {
                let vi = createViewItem(listed);
                if(vi) {
                    newItems.push(vi);
                }
            })
            tempView.items = newItems;
            const newView: View = {...tempView} as View;
            onsave(newView);
        }
    }

    function getAttributeFromReadOnlyAttributes(name: string) : Attribute | null {
        let index: number = readOnlyAttributes.findIndex(roAttr => roAttr.name === name);
        if (index >= 0) {
            return readOnlyAttributes[index];
        }
        return null;
    }

    function getAttributeFromTemplate(name: string) : Attribute | null {
        let index: number = template.fields.findIndex(attr => attr.name === name);
        if (index >= 0) {
            return template.fields[index];
        }
        return null;
    }

    function createViewItem(attributeName: string) : ViewItem | null {
        let attribute: Attribute | null  = getAttributeFromReadOnlyAttributes(attributeName);
        if (!attribute) {
            attribute = getAttributeFromTemplate(attributeName);
        }
        if (!attribute) {
            return null;
        }
        return {
            key: attribute.key.toString(),
            attribute: attribute.name.toString(),
            show: true,
        } as ViewItem;
    }

    function moveViewItemUp(index: number) {
        if (index === 0) {
            return;
        }
        const temp = sortedList[index - 1];
        sortedList[index - 1] = sortedList[index];
        sortedList[index] = temp;
        sortedList = [...sortedList];
    }

    function moveViewItemDown(index: number) {
        if (index === tempViewItems.length - 1) {
            return;
        }
        const temp = sortedList[index + 1];
        sortedList[index + 1] = sortedList[index];
        sortedList[index] = temp;
        sortedList = [...sortedList];
    }

    function handleClose() {
        if (onclose) {
            onclose();
        }
    }

</script>

<Dialog.Root bind:open={open}>
    <Dialog.Content class="min-w-150" onclose={handleClose}>
        <Dialog.Header class="py-1">
            <Dialog.Title>
                {#if readOnly}
                    Details of "{tempView.name}"
                {:else}
                    {(tempView.name.trim() !== '') ? 'Edditing view "' + tempView!.name + '"' : 'New View'}
                {/if}
            </Dialog.Title>
            <Dialog.Description>
                {#if readOnly}
                    <div class="flex gap-2">
                        <Icon icon="ph:pencil-simple-slash" width="20px" />
                        Read-Only
                    </div>
                {/if}
            </Dialog.Description>
        </Dialog.Header>
        <div class="flex flex-col gap-2">
            <div class="grid flex-2 grid-cols-8">
                <Label for="name" class="text-right col-span-2">Name</Label>
                <Input id="name" class="col-span-6" autocomplete="off" bind:value={tempViewName} readonly={readOnly}/>
            </div>
            <div class="grid flex-2 grid-cols-8">
                <Label for="description" class="text-right col-span-2">Description</Label>
                <Input id="description" class="col-span-6" autocomplete="off" bind:value={tempViewDesc} readonly={readOnly}/>
            </div>
            <div class="grid flex-2 grid-cols-8">
                <Label for="attributes" class="text-right col-span-2">Attributes</Label>
                <div class="col-span-6">
                    <Combobox items={attributeList} bind:selected={selectedAttribute} />
                </div>
            </div>
            <div class="grow max-h-40 overflow-auto">
                <Table.Root class="box-border">
                    <Table.Body>
                        {#each sortedList as vi, i}
                            <Table.Row>
                                <Table.Cell class="py-0 pl-2">
                                    <div class="flex items-center justify-between">
                                        <p class="pl-1">{vi}</p>
                                        <div class="text-slate-400">
                                            <Button variant="ghost" size="icon" class="" onclick={() => moveViewItemUp(i)} disabled={(i === 0)}>
                                                <Icon icon="gravity-ui:caret-up" width="20px" />
                                            </Button>
                                            <Button variant="ghost" size="icon" class="" onclick={() => moveViewItemDown(i)} disabled={(i === tempViewItems.length - 1)}>
                                                <Icon icon="gravity-ui:caret-down" width="20px" />
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
                                        At least one Attribute must be selected.
                                    </div>
                                </Table.Cell>
                            </Table.Row>
                        {/each}
                    </Table.Body>
                </Table.Root>
            </div>
        </div>
        <Separator class="my-1" />
        <Dialog.Footer class="px-2">
            {#if tempView.name}
                <Button variant="destructive" onclick={deleteView} disabled={readOnly}>
                    <Icon icon="gravity-ui:trash-bin" width="20px" />
                    Delete Attribute
                </Button>
            {/if}
            <div class="grow"></div>
            <Button variant="secondary" onclick={handleClose}>
                <Icon icon="gravity-ui:xmark" width="20px" />
                Cancel
            </Button>
            <Button disabled={disableAddButton} onclick={saveView}>
                <Icon icon="gravity-ui:floppy-disk" width="20px" />
                Save
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
