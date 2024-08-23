<script lang="ts">
    import Icon from "@iconify/svelte";
    import LinkForm from "$lib/components/global/object_editor/LinkForm.svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import AttributeInput from "./AttributeInput.svelte";
    import { user } from "$lib/stores/User";
    import { goto } from "$app/navigation";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { marked } from 'marked'
    import { Checkbox } from "$lib/components/ui/checkbox/index.js";
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js"
    import { encodePath } from "$lib/utils/pathHandler";
    import { createEventDispatcher, onMount } from "svelte";
    import * as Table from "$lib/components/ui/table";
    import type { ObjectView } from "$lib/components/structs/Object";
    import type { Module } from "$lib/components/structs/Module";
    import "./markdown.css";
    import path from "path";
    import { repository } from "$lib/stores/Repository";
    
    export let objectView: ObjectView;
    export let module: Module;
    export let readOnlyMode: boolean = false;
    let allowChanges: boolean;
    
    const dispatch = createEventDispatcher();

    function closeEdit() {
        dispatch('close', {})
    }
    
    function saveDrafObj() {
        if(objectView) {
            objectView.hasChanges = true;
            objectView.object.updatedAt = new Date();
            objectView.object.author = $user.toString();
        }
        dispatch('saveDraft', {objectView: objectView})
    }
    
    function saveObj() {
        if(objectView) {
            objectView.hasChanges = true;
            objectView.object.updatedAt = new Date();
            objectView.object.author = $user.toString();
        }
        dispatch('save', {objectView: objectView})
    }
    
    function deleteObj() {
        if(objectView) {
            objectView.object.author = $user.toString();
        }
        dispatch('delete', {objectView: objectView})
    }

    function handleVisitLink(event: any) {
        let linkPath = path.normalize(event.detail.link.path);
        let repoPath = path.resolve($repository?.tree.path);
        goto("/module/" + encodePath(path.join(repoPath, linkPath)) + "#" + event.detail.link.object);
    }

    $: {
        allowChanges = !(readOnlyMode || (objectView.object.deletedAt ? true : false));
    }
    
    onMount(() => {
        allowChanges = !(readOnlyMode || (objectView.object.deletedAt ? true : false));
    })

</script>

{#if objectView}
<div class="h-full flex flex-col px-3 min-w-[450px] box-border">
    <div class="grow min-h-[50%]">
        <ScrollArea class="h-full">
            {#if objectView.isDraft && !objectView.object.deletedAt}
            <div class="py-2">
                <div class="text-yellow-600 border-yellow-500 border-2 bg-yellow-100 text-center p-2 rounded-md">
                    <div class="italic text-sm flex items-center justify-center gap-2 mb-0">
                        <Icon icon="ci:circle-warning" width="25px"/>
                        This is object is a Draft
                    </div>
                </div>
            </div>
            {/if}
            {#if objectView.object.deletedAt}
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
                    {#if objectView.object.id === 0}
                    <Input id="name" placeholder="Auto Generated" class="col-span-3" disabled/>
                    {:else}
                    <Input id="name" value={module.manifest.prefix+module.manifest.separator+objectView.object.id} class="col-span-3" disabled/>
                    {/if}
                    <Label for="name" class="text-right col-span-1">Level</Label>
                    <Input id="name" bind:value={objectView.object.level} class="col-span-3" disabled={!allowChanges} autocomplete="off"/>
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
                    <Input id="name" bind:value={objectView.object.header}  class="col-span-7" disabled={!allowChanges} autocomplete="off"/>
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
                {#if allowChanges}
                    <div class="grid grid-cols-8 items-center gap-2 px-1">
                        <Label for="name" class="text-right col-span-1">Text</Label>
                        <Textarea id="name" bind:value={objectView.object.content}  class="col-span-7 font-mono" />
                    </div>
                {/if}
                <div class="grid grid-cols-8 items-center gap-2 px-1">
                    <Label for="name" class="text-right col-span-1">Preview</Label>
                    <div class="col-span-7">
                        <ScrollArea class=" col-span-1">
                            <div class="preview rounded-sm">
                                {@html marked((objectView.object.header ? "# " + objectView.object.level + " " + objectView.object.header + "\n" : "") + objectView.object.content)}
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
                    <div class="flex flex-col col-span-3 gap-3 pb-2">
                        <div class="flex items-center col-span-2">
                            <Checkbox id="actCheck" bind:checked={objectView.object.isActive} aria-labelledby="actCheck-label" disabled={!allowChanges}/>
                            <Label
                            id="actCheck-label"
                            for="actCheck"
                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 pl-3">
                            Active
                            </Label>
                        </div>
                        <div class="flex items-center col-span-2">
                            <Checkbox id="reqCheck" bind:checked={objectView.object.isRequirement} aria-labelledby="reqCheck-label" disabled={!allowChanges}/>
                            <Label
                            id="reqCheck-label"
                            for="reqCheck"
                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 pl-3">
                            Requirement
                            </Label>
                        </div>
                        <div class="flex items-center col-span-2">
                            <Checkbox id="reqNorm" bind:checked={objectView.object.isNormative} aria-labelledby="reqNorm-label" disabled={!allowChanges}/>
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
            {#if module.template.fields.length > 0 && objectView.object.customFields}
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
                            {#each module.template.fields as field}
                                <Table.Row>
                                    <Table.Cell class="text-right">
                                        {field.attribute}
                                    </Table.Cell>
                                    <Table.Cell>
                                        <AttributeInput bind:value={objectView.object.customFields[field.key]} field={field} disabled={!allowChanges}/>
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
                        <div class="flex items-center gap-1 ml-1 my-1 font-semibold">
                            <Icon icon="ci:arrow-up-right-lg" width="20px"/>
                            Outbound Links
                        </div>
                        <LinkForm bind:links={objectView.object.outboundLinks} editable={allowChanges} on:visitLink={handleVisitLink}/>
                    </div>
                    {#if objectView?.inboundLinks.length > 0}
                    <Separator/>
                    <div class="gap-2 w-full pt-4">
                        <div class="flex items-center gap-1 ml-1 my-1 font-semibold">
                            <Icon icon="ci:arrow-down-left-lg" width="20px"/>
                            Inbound Links
                        </div>
                        <LinkForm links={objectView.inboundLinks} editable={false} on:visitLink={handleVisitLink}/>
                    </div>
                    {/if}
                </div>
            </div>
            {#if objectView.object.id !== 0 && allowChanges}
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
        {#if allowChanges}
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
        {:else}
            <Button variant="secondary" class="px-5" on:click={closeEdit}>
                <Icon icon="ci:add-minus-square" width="20px"/>
                <p class="pl-2">Close</p>
            </Button>
            <div class="grow"></div>
        {/if}
    </div>
</div>
{/if}