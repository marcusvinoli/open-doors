<script lang="ts">
    import type { ObjectView } from "$lib/components/structs/Object";
    import { user } from "$lib/stores/User";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import { Checkbox } from "$lib/components/ui/checkbox/index.js";
    import * as Accordion from "$lib/components/ui/accordion";
    import { marked } from 'marked'
    import { beforeUpdate, createEventDispatcher, onMount } from "svelte";
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    import Icon from "@iconify/svelte";
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js"
    import "./markdown.css";
    import type { Template } from "$lib/components/structs/Template";
    import * as Table from "$lib/components/ui/table";
    import LinkForm from "$lib/components/forms/module/LinkForm.svelte";
    
    export let objv: ObjectView | null = createEmptyObject();
    export let template: Template;
    export let prefix: string = "";
    export let separator: string = "";
    
    interface IHash {
        [key: string]: string;
    }

    let isDeletable: boolean = true;

    const dispatch = createEventDispatcher();

    function createEmptyObject(): ObjectView {
        isDeletable = false;
        return {
            object: {
                id: 0,
                header: "",
                content: "",
                author: $user.toString()!,
                isActive: true,
                isNormative: false,
                isRequirement: false,
                createdAt: new Date(),
                updatedAt: new Date(),
                deletedAt: null,
                customFields: createCustomFieldHashFromTemplate(template),
                level: "",
                outboundLinks: [],
            },
            inboundLinks: [],
            isDraft: false,
            hasChanges: false,
        }
    }

    function createCustomFieldHashFromTemplate(template: Template): IHash {
        const obj: IHash = {};
        
        let keys: string[] = [];

        template.fields.forEach((field) => {
            keys.push(field.key);
        })

        keys.forEach(key => {
            obj[key] = "";
        });

        return obj;
    }

    function closeEdit() {
        dispatch('close', {})
    }
    
    function saveDrafObj() {
        if(objv) {
            objv.hasChanges = true;
            objv.object.updatedAt = new Date();
            objv.object.author = $user.toString();
        }
        dispatch('saveDraft', {obj: objv})
    }
    
    function saveObj() {
        if(objv) {
            objv.hasChanges = true;
            objv.object.updatedAt = new Date();
            objv.object.author = $user.toString();
        }
        dispatch('save', {obj: objv})
    }
    
    function deleteObj() {
        if(objv) {
            objv.object.author = $user.toString();
        }
        dispatch('delete', {obj: objv})
    }

    beforeUpdate(() => {
        if (!objv) {
            objv = createEmptyObject();
            isDeletable = false;
        } else {
            isDeletable = false;
        }
        if (!objv?.object.customFields) {
            objv!.object.customFields = createCustomFieldHashFromTemplate(template);
        }
    })

</script>

{#if objv}
<div class="h-full flex flex-col px-3 min-w-[450px] box-border">
    <div class="grow min-h-[50%]">
        <ScrollArea class="h-full">
            {#if objv.isDraft && !objv.object.deletedAt}
            <div class="py-2">
                <div class="text-yellow-600 border-yellow-500 border-2 bg-yellow-100 text-center p-2 rounded-md">
                    <div class="italic text-sm flex items-center justify-center gap-2 mb-0">
                        <Icon icon="ci:circle-warning" width="25px"/>
                        This is object is a Draft
                    </div>
                </div>
            </div>
            {/if}
            {#if objv.object.deletedAt}
            <div class="py-2">
                <div class="text-red-600 border-red-500 border-2 bg-red-100 text-center p-2 rounded-md">
                    <div class="italic text-sm flex items-center justify-center gap-2 mb-0">
                        <Icon icon="ci:trash-full" width="25px"/>
                        This object has been deleted and no longer allows changes.
                    </div>
                </div>
            </div>
            {/if}
            <div class="grid gap-2 my-1">
                <h2 class="font-bold mb-1">Object Heading</h2>
                <div class="grid grid-cols-8 items-center gap-2 px-1">
                    <Label for="name" class="text-right col-span-1">ID</Label>
                    {#if objv.object.id === 0}
                    <Input id="name" placeholder="Auto Generated" class="col-span-3" disabled/>
                    {:else}
                    <Input id="name" value={prefix+separator+objv.object.id} class="col-span-3" disabled/>
                    {/if}
                    <Label for="name" class="text-right col-span-1">Level</Label>
                    <Input id="name" bind:value={objv.object.level} class="col-span-3" />
                    <!-- 
                    <Button variant="secondary" class="cursor-default col-span-1">
                        <Icon icon="gravity-ui:bars-descending-align-left-arrow-down" width="15px"/>
                    </Button>
                    <Button variant="secondary" class="cursor-default col-span-1">
                        <Icon icon="gravity-ui:bars-descending-align-left-arrow-up" width="15px"/>
                    </Button> 
                    -->
                </div>
                <div class="grid grid-cols-8 items-center gap-2 px-1">
                    <Label for="name" class="text-right col-span-1">Header</Label>
                    <Input id="name" bind:value={objv.object.header}  class="col-span-7" />
                    <!-- 
                    <Button variant="secondary" class="cursor-default col-span-1">
                        <Icon icon="gravity-ui:text-indent" width="15px"/>
                    </Button>
                    <Button variant="secondary" class="cursor-default col-span-1">
                        <Icon icon="gravity-ui:text-outdent" width="15px"/>
                    </Button>
                    -->
                </div>
                <Separator/>
            </div>
            <div class="grid gap-2 my-1">
                <h2 class="font-bold my-1">Object Main Content</h2>
                <div class="grid grid-cols-8 items-center gap-2 px-1">
                    <Label for="name" class="text-right col-span-1">Text</Label>
                    <Textarea id="name" bind:value={objv.object.content}  class="col-span-7 font-mono" />
                </div>
                <div class="grid grid-cols-8 items-center gap-2 px-1">
                    <Label for="name" class="text-right col-span-1">Preview</Label>
                    <div class="col-span-7">
                        <ScrollArea class=" col-span-1">
                            <div class="preview rounded-sm">
                                <!-- {@html marked("# " + objv.object.header)} -->
                                {@html marked((objv.object.header ? "# " + objv.object.header + "\n" : "") + objv.object.content)}
                                <!-- {@html marked("# ")} -->
                            </div>
                        </ScrollArea>
                    </div>
                </div>
                <Separator/>
            </div>
            <div class="grid gap-2 my-1">
                <h2 class="font-bold my-1">Object Cassification</h2>
                <div class="grid grid-cols-8 items-center gap-2 min-h[100px]">
                    <div class="col-span-1">
                    </div>
                    <div class="flex flex-col col-span-3 gap-2">
                        <div class="flex items-center col-span-2">
                            <Checkbox id="actCheck" bind:checked={objv.object.isActive} aria-labelledby="actCheck-label" />
                            <Label
                            id="actCheck-label"
                            for="actCheck"
                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 pl-3">
                            Active
                            </Label>
                        </div>
                        <div class="flex items-center col-span-2">
                            <Checkbox id="reqCheck" bind:checked={objv.object.isRequirement} aria-labelledby="reqCheck-label" />
                            <Label
                            id="reqCheck-label"
                            for="reqCheck"
                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 pl-3">
                            Requirement
                            </Label>
                        </div>
                        <div class="flex items-center col-span-2">
                            <Checkbox id="reqNorm" bind:checked={objv.object.isNormative} aria-labelledby="reqNorm-label" />
                            <Label
                            id="reqNorm-label"
                            for="reqNorm"
                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 pl-3">
                            Normative
                            </Label>
                        </div>
                    </div>
                </div>
                <Separator/>
            </div>
            {#if template.fields.length > 0 && objv.object.customFields}
            <div class="grid gap-2 my-1">
                <h2 class="font-bold my-1">Custom Attributes</h2 >
                <div class="">
                    <Table.Root class="w-full">
                        <Table.Header class="">
                            <Table.Row>
                                <Table.Head class="sticky top-0 bg-slate-50 shadow-sm w-[30%]">Attribute</Table.Head>
                                <Table.Head class="sticky top-0 bg-slate-50 shadow-sm">Value</Table.Head>
                            </Table.Row>
                        </Table.Header>
                        <Table.Body class="">
                            {#each template.fields as field}
                                <Table.Row>
                                    <Table.Cell class="text-right">
                                        {field.attribute}
                                    </Table.Cell>
                                    <Table.Cell>
                                        <Input id="name" bind:value={objv.object.customFields[field.key]}/>
                                    </Table.Cell>
                                </Table.Row>
                            {/each}
                        </Table.Body>
                    </Table.Root>
                </div>
                <Separator/>
            </div>
            {/if}
            <div class="grid my-1">
                <h2 class="font-bold my-1">Links</h2>
                <div class="flex flex-col my-1">
                    <div class="gap-2 w-full py-1">
                        <div class="flex items-center gap-1 ml-2">
                            <Icon icon="ci:arrow-up-right-lg" width="20px"/>
                            Outbound Links
                        </div>
                        <LinkForm bind:links={objv.object.outboundLinks} editable={true}/>
                    </div>
                    <Separator/>
                    {#if objv?.inboundLinks.length > 0}
                    <div class="gap-2 w-full pt-4">
                        <div class="flex items-center gap-1 ml-2">
                            <Icon icon="ci:arrow-down-left-lg" width="20px"/>
                            Inbound Links
                        </div>
                        <LinkForm links={objv.inboundLinks} editable={false}/>
                    </div>
                    {/if}
                </div>
            </div>
            {#if isDeletable}
            <Separator/>
            <div class="grid wrap pag-2 mt-3">
                <Button variant="destructive" class="px-5" on:click={deleteObj}>
                    <Icon icon="ci:close-square" width="20px"/>
                    <p class="pl-2">Delete Object</p>
                </Button>
            </div>
            {/if}
        </ScrollArea>
    </div>
    <Separator/>
    <div class="flex flex-row pb-1 pt-3">
        <Button variant="secondary" class="px-5" on:click={closeEdit}>
            <Icon icon="ci:add-minus-square" width="20px"/>
            <p class="pl-2">Cancel</p>
        </Button>
        <div class="grow"></div>
        <div class="flex flex-row gap-3">
            <Button variant="secondary" class="px-5" on:click={saveObj}>
                <Icon icon="ci:add-to-queue" width="20px"/>
                <p class="pl-2">Save and Stage</p>
            </Button>
            <Button class="px-5" on:click={saveDrafObj}>
                <Icon icon="ci:add-plus-square" width="20px"/>
                <p class="pl-2">Save as Draft</p>
            </Button>
        </div>
    </div>
</div>
{/if}