<script lang="ts">
    import Icon from "@iconify/svelte";
    // import LinkForm from "$lib/components/global/object_editor/LinkForm.svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    // import AttributeInput from "./AttributeInput.svelte";

    // import { goto } from "$app/navigation";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { marked } from 'marked'
    // import { Checkbox } from "$lib/components/ui/checkbox/index.js";
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js"
    import { encodePath } from "$lib/utils/path-handler";
    import { repository } from "$lib/stores/Repository.svelte";
    import { ObjectStatus } from "$lib/components/structs/ObjectStatus";

    import * as Tab from "$lib/components/ui/tabs";
    import * as Table from "$lib/components/ui/table";
    import * as Dialog from "$lib/components/ui/dialog/index.js";

    import type { Module } from "$lib/components/structs/Module";
    import type { Object } from "$lib/components/structs/Object";
    import type { Repository } from "$lib/components/structs/Repo";

    //import "./markdown.css"; // This should be moved to an appropriate place.
    
    let {
        openDialog = $bindable(false),
        object, 
        module, 
        onclose,
        onsavedraft,
        onsave,
        ondelete,
        onrestore,
        onlinkvisit,
        readOnlyMode = true,
    } : {
        openDialog: boolean,
        object: Object;
        module: Module;
        readOnlyMode?: boolean;
        onclose?: (object: Object) => void,
        onsavedraft?: (object: Object) => void,
        onsave?: (object: Object) => void,
        ondelete?: (object: Object) => void,
        onrestore?: (object: Object) => void,
        onlinkvisit?: (path: string) => void,
    } = $props();
    
    let allowChanges: boolean = $derived(!(readOnlyMode || (object.deletedAt ? true : false)));
    let repo: Repository = $derived(repository()!);
    
    function handleClose() {
        if (!onclose) {
            return;
        }
        onclose(object);
    }
    
    function handleSaveDrafObject() {
        if (!onsavedraft) {
            return;
        }
        onsavedraft(object);
    }
    
    function handleSaveObject() {
        if (!onsave) {
            return;
        }
        onsave(object);
    }
    
    function handleDeleteObject() {
        if (!ondelete) {
            return;
        }
        ondelete(object);
    }

	function handleRestoreObject() {
		if (!onrestore) {
            return;
        }
        onrestore(object);
	}

    function handleLinkVisit(event: any) {
        if (!onlinkvisit) {
            return;
        }
        let linkPath = event.detail.link.path;
        let repoPath = repo.tree.path;
        onlinkvisit("/module/" + encodePath(repoPath + "/" + linkPath) + "#" + event.detail.link.object);
    }

</script>


<Dialog.Root bind:open={openDialog}>
    <Dialog.Header>
        <Dialog.Title>{module.manifest.prefix}{module.manifest.separator}{object.id}</Dialog.Title>
        <Dialog.Description>
        </Dialog.Description>
    </Dialog.Header>
    <Dialog.Content>
        <Tab.Root value="content">
            <Tab.List>
                <Tab.Trigger value="content">Content</Tab.Trigger>
                <Tab.Trigger value="attributes">Attributes</Tab.Trigger>
                <Tab.Trigger value="links">Links</Tab.Trigger>
                <Tab.Trigger value="history">History</Tab.Trigger>
            </Tab.List>
            <Tab.Content value="content">
                {#if (object.metadata.status === ObjectStatus.draft) && !(object.deletedAt)}
                    <div class="py-2">
                        <div class="text-yellow-600 border-yellow-500 border-2 bg-yellow-100 text-center p-2 rounded-md">
                            <div class="italic text-sm flex items-center justify-center gap-2 mb-0">
                                <Icon icon="ci:circle-warning" width="25px"/>
                                This is object is a Draft
                            </div>
                        </div>
                    </div>
                {/if}
                {#if object.deletedAt}
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
                    <h2 class="font-bold my-1">Object Main Content</h2>
                    {#if allowChanges}
                        <div class="grid grid-cols-8 items-center gap-2 px-1">
                            <Label for="name" class="text-right col-span-1">Text</Label>
                            <Textarea id="name" bind:value={object.content}  class="col-span-7 font-mono" />
                        </div>
                    {/if}
                    <div class="grid grid-cols-8 items-center gap-2 px-1">
                        <Label for="name" class="text-right col-span-1">Preview</Label>
                        <div class="col-span-7">
                            <ScrollArea class=" col-span-1">
                                <div class="preview rounded-sm">
                                    {@html marked((object.header ? "# " + object.header + "\n" : "") + object.content)}
                                </div>
                            </ScrollArea>
                        </div>
                    </div>
                    <Separator/>
                </div>
                <div class="grid gap-2 my-1">
                    <h2 class="font-bold mb-1">Object Heading</h2>
                    <div class="grid grid-cols-8 items-center gap-2 px-1">
                        <Label for="name" class="text-right col-span-1">ID</Label>
                        {#if object.id === 0}
                        <Input id="name" placeholder="Auto Generated" class="col-span-3" disabled/>
                        {:else}
                        <Input id="name" value={module.manifest.prefix+module.manifest.separator+object.id} class="col-span-3" disabled/>
                        {/if}
                        <Label for="name" class="text-right col-span-1">Level</Label>
                        <Input id="name" bind:value={object.indexLevel} class="col-span-3" disabled={!allowChanges} autocomplete="off"/>
                        <Button variant="secondary" class="cursor-default col-span-1">
                            <Icon icon="gravity-ui:bars-descending-align-left-arrow-down" width="15px"/>
                        </Button>
                        <Button variant="secondary" class="cursor-default col-span-1">
                            <Icon icon="gravity-ui:bars-descending-align-left-arrow-up" width="15px"/>
                        </Button> 
                    </div>
                    <div class="grid grid-cols-8 items-center gap-2 px-1">
                        <Label for="name" class="text-right col-span-1">Header</Label>
                        <Input id="name" bind:value={object.header}  class="col-span-7" disabled={!allowChanges} autocomplete="off"/>
                        
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
            </Tab.Content>
            <Tab.Content value="attributes"> 
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
                            {#each module.template.fields as attribute}
                                <Table.Row>
                                    <Table.Cell class="text-right">
                                        {attribute.name}
                                    </Table.Cell>
                                    <Table.Cell>
                                        <!-- <AttributeInput bind:value={object.attributes[field.key]} field={field} disabled={!allowChanges}/> -->
                                    </Table.Cell>
                                </Table.Row>
                            {/each}
                        </Table.Body>
                    </Table.Root>
                </div>
                <Separator/>
            </div>
            </Tab.Content>
            <Tab.Content value="links"> 
                <div class="grid my-1">
                <h2 class="font-bold my-1">Links</h2>
                <div class="flex flex-col my-1">
                    <div class="gap-2 w-full py-1">
                        <div class="flex items-center gap-1 ml-1 my-1 font-semibold">
                            <Icon icon="ci:arrow-up-right-lg" width="20px"/>
                            Outbound Links
                        </div>
                        <!-- <LinkForm bind:links={object.metadata.outboundLinks} editable={allowChanges} onvisitLink={handleVisitLink}/> -->
                    </div>
                    {#if object.metadata.inboundLinks?.length > 0}
                    <Separator/>
                    <div class="gap-2 w-full pt-4">
                        <div class="flex items-center gap-1 ml-1 my-1 font-semibold">
                            <Icon icon="ci:arrow-down-left-lg" width="20px"/>
                            Inbound Links
                        </div>
                        <!-- <LinkForm links={object.metadata.inboundLinks} editable={false} onvisitLink={handleVisitLink}/> -->
                    </div>
                    {/if}
                </div>
            </div>
            </Tab.Content>
            <Tab.Content value="history"> 
                <p>This feature is not implemented yet... :(</p>
            </Tab.Content>
        </Tab.Root>
    </Dialog.Content>
    <Dialog.Footer>
        <div class="flex flex-row pb-1 pt-3">
            <div class="grid wrap pag-2 my-3">
                    {#if !object.deletedAt}
                    <Button variant="destructive" class="px-5" onclick={handleDeleteObject}>
                        <Icon icon="ci:close-square" width="20px"/>
                        <p class="pl-2">Delete Object</p>
                    </Button>
                    {/if}
            </div>
            <Separator/>
            {#if allowChanges}
                <Button variant="secondary" class="px-5" onclick={handleClose}>
                    <Icon icon="ci:add-minus-square" width="20px"/>
                    <p class="pl-2">Cancel</p>
                </Button>
                <div class="grow"></div>
                <div class="flex flex-row gap-3">
                    <Button variant="secondary" class="px-5" onclick={handleSaveObject}>
                        <Icon icon="ci:add-to-queue" width="20px"/>
                        <p class="pl-2">Save and Stage</p>
                    </Button>
                    <Button class="px-5" onclick={handleSaveDrafObject}>
                        <Icon icon="ci:add-plus-square" width="20px"/>
                        <p class="pl-2">Save as Draft</p>
                    </Button>
                </div>
            {:else}
                <Button variant="secondary" class="px-5" onclick={handleClose}>
                    <Icon icon="ci:add-minus-square" width="20px"/>
                    <p class="pl-2">Close</p>
                </Button>
                <div class="grow"></div>
                {#if object.deletedAt && !readOnlyMode}
                    <Button variant="secondary" class="px-5" onclick={handleRestoreObject}>
                        <Icon icon="ci:arrow-reload-02" width="20px"/>
                        <p class="pl-2">Restore Object</p>
                    </Button>
                {/if}
            {/if}
        </div>
    </Dialog.Footer>
</Dialog.Root>