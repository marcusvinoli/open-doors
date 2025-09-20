<script lang="ts">
    import Icon from "@iconify/svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import DynamicInput from "$lib/components/forms/inputs/DynamicInput.svelte";

    import { app } from "$lib/stores/AppState.svelte";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { marked } from 'marked'
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    import { newObject } from "$lib/utils/object-utils";
    import { ObjectStatus } from "$lib/components/structs/ObjectStatus";
    import { parseTemplate, readOnlyAttributes } from "$lib/utils/attribute-utils";
    
    import * as Tab from "$lib/components/ui/tabs";
    import * as Table from "$lib/components/ui/table";
    import * as Dialog from "$lib/components/ui/dialog";
    import * as Tooltip from "$lib/components/ui/tooltip";
    import * as Accordion from "$lib/components/ui/accordion";
    
    import type { Link } from "$lib/components/structs/Link";
    import type { Module } from "$lib/components/structs/Module";
    import type { Object } from "$lib/components/structs/Object";
    
    import "$lib/assets/preview.css";
    
    let {
        module, 
        object, 
        onsave,
        onclose,
        ondelete,
        onrestore,
        onlinkvisit,
        onunlink,
        onlink,
        onsavedraft,
        openDialog = $bindable(false),
        readOnly = $bindable(false),
    } : {
        module: Module;
        object: Object | null;
        onsave?: (object: Object) => void,
        onclose?: (object: Object) => void,
        ondelete?: (object: Object) => void,
        onrestore?: (object: Object) => void,
        onlinkvisit?: (link: Link) => void,
        onlink?: (object: Object, modulePath: string, link: Link) => void,
        onunlink?: (object: Object, modulePath: string, link: Link) => void,
        onsavedraft?: (object: Object) => void,
        openDialog: boolean,
        readOnly?: boolean;
    } = $props();
    
    let allowChanges: boolean = $derived(!(readOnly || (object?.deletedAt ? true : false)));
    let obj: Object = $derived.by(() => {
            if (object) {
                let draftObj = {...object} as Object;
                let attrs = {...object.attributes};
                parseTemplate(module.template, attrs);
                draftObj.attributes = attrs;
                return {...draftObj};
            }
            return newObject(module.template);
        });
    let objContent: string = $derived(obj.content);
    let objHeader: string = $derived(obj.header);

    function prepareObject() {
        obj.header = objHeader;
        obj.content = objContent;
        obj.updatedAt = new Date();
        obj.author = app.user!.toString();
    }

    function closeDialog() {
        objContent = '';
        objHeader = '';
        obj = newObject(module.template);
        openDialog = false;
    }

    function handleClose() {
        closeDialog();
        if (!onclose) {
            return;
        }
        onclose(object!);
    }
    
    function handleSaveDrafObject() {
        if (!onsavedraft) {
            return;
        }
        prepareObject();
        onsavedraft(obj!);
    }
    
    function handleSaveObject() {
        if (!onsave) {
            return;
        }
        prepareObject();
        onsave(obj!);
    }
    
    function handleDeleteObject() {
        if (!ondelete) {
            return;
        }
        ondelete(obj!);
    }

	function handleRestoreObject() {
		if (!onrestore) {
            return;
        }
        onrestore(obj!);
	}

    function handleLinkVisit(link: Link) {
        if (!onlinkvisit) {
            return;
        }
        onlinkvisit(link);
    }
    
    function handleUnlink(object: Object, modulePath: string, link: Link) {
        if (!onunlink) {
            return;
        }
        onunlink(object, modulePath, link);
    }

</script>

<Dialog.Root bind:open={openDialog}>
    <Dialog.Content class="min-w-[80%] max-h-[80%] min-h-[80%] flex flex-col gap-2" interactOutsideBehavior="close" onEscapeKeydown={closeDialog}>
        <Dialog.Header class="block">
            {#if obj.id === 0}
            <Dialog.Title>New Object</Dialog.Title>
            <Dialog.Description>New object properties</Dialog.Description>
            {:else}
            <Dialog.Title>
                <div class="flex items-center pb-1">
                    {#if readOnly}
                    <Icon icon="ph:pencil-simple-slash" width="25px"  class="mr-1"/>
                    {/if}
                    {module.manifest.prefix}{module.manifest.separator}{obj.id}
                </div>
            </Dialog.Title>
            <Dialog.Description>Object properties</Dialog.Description>
            {/if}
        </Dialog.Header>
        <Tab.Root value="content" class="flex-1 min-h-0">
            <Tab.List class="w-full top-0 sticky">
                <Tab.Trigger value="content">Content</Tab.Trigger>
                <Tab.Trigger value="attributes">Attributes</Tab.Trigger>
                <Tab.Trigger value="links">Links</Tab.Trigger>
                <Tab.Trigger value="history">History</Tab.Trigger>
            </Tab.List>
            <Tab.Content value="content" class="flex-1 flex flex-col min-h-0">
                <h2 class="font-bold my-1">Object Main Content</h2>
                {#if (obj.metadata?.status === ObjectStatus.draft) && !(obj.deletedAt)}
                    <div class="text-yellow-600 border-yellow-500 border-2 bg-yellow-100 text-center p-1 rounded-md italic text-sm flex items-center justify-center gap-2 my-1">
                        <Icon icon="ci:circle-warning" width="25px"/>
                        This is a Draft object.
                    </div>
                {/if}
                {#if obj.deletedAt}
                    <div class="text-red-600 border-red-500 border-2 bg-red-100 text-center p-1 rounded-md italic text-sm flex items-center justify-center gap-2 my-1">
                        <Icon icon="ci:trash-full" width="25px"/>
                        This object has been deleted.
                    </div>
                {/if}
                <div class="flex flex-col gap-2 p-2 flex-1 min-h-0 overflow-auto">
                    <div class="grid grid-cols-8">
                        <Label for="name" class="text-right col-span-1">Header</Label>
                        <Input id="name" bind:value={objHeader} class="col-span-7" readonly={!allowChanges} autocomplete="off"/>
                    </div>
                    <div class="grid flex-2 grid-cols-8">
                        <Label for="name" class="text-right col-span-1">Content</Label>
                        <Textarea id="name" class="text_area h-full col-span-7 border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground shadow-xs flex w-full min-w-0 rounded-md border px-3 outline-none transition-[color,box-shadow] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm resize-none" inputmode="text" bind:value={objContent} readonly={!allowChanges}/>
                    </div>
                    <div class="flex-2 min-h-15 grid grid-cols-8">
                        <Label for="name" class="text-right col-span-1">Preview</Label>
                        <div class="preview col-span-7 h-full overflow-auto bg-slate-100" id="preview-window">
                            {@html marked((objHeader ? "# " + objHeader + "\n" : "") + objContent)}
                        </div>
                    </div>
                </div>
            </Tab.Content>
            <Tab.Content value="attributes" class="flex-1 flex flex-col min-h-0"> 
                <h2 class="font-bold my-1">Object Attributes</h2>
                <div class="flex flex-col gap-2 flex-1 min-h-0 overflow-auto">
                    <Table.Root>
                        <Table.Header>
                            <Table.Row class="relative">
                                <Table.Head class="sticky top-0 bg-slate-50 shadow-sm w-[30%]">Attribute</Table.Head>
                                <Table.Head class="sticky top-0 bg-slate-50 shadow-sm">Value</Table.Head>
                            </Table.Row>
                        </Table.Header>
                        <Table.Body>
                            {#each readOnlyAttributes.filter(ro => (ro.key !== 'header' && ro.key !== 'content')) as roAttribute (roAttribute.key)}
                                <Table.Row>
                                    <Table.Cell class="text-right">
                                        {roAttribute.name}
                                    </Table.Cell>
                                    <Table.Cell class="flex justify-between items-center">
                                        <div class="px-1">
                                            {obj[roAttribute.key as keyof typeof object] ?? "-"}
                                        </div>
                                        <Tooltip.Provider> 
                                            <Tooltip.Root>
                                                <Tooltip.Trigger>
                                                    <div class="text-slate-300 mx-1 px-2">
                                                        <Icon icon="ph:pencil-simple-slash" width="19px"/>
                                                    </div>
                                                </Tooltip.Trigger>
                                                <Tooltip.Content>
                                                    <p>Read-only attribute</p>
                                                </Tooltip.Content>
                                            </Tooltip.Root>
                                        </Tooltip.Provider>
                                    </Table.Cell>
                                </Table.Row>
                            {/each}
                                <Table.Row>
                                    <Table.Cell class="text-right">
                                        Hierarchical Level
                                    </Table.Cell>
                                    <Table.Cell class="flex justify-between items-center">
                                        <div class="px-1">
                                            {obj.metadata?.level ?? '-'}
                                        </div>
                                        <Tooltip.Provider> 
                                            <Tooltip.Root>
                                                <Tooltip.Trigger>
                                                    <div class="text-slate-300 mx-1 px-2">
                                                        <Icon icon="ph:pencil-simple-slash" width="19px"/>
                                                    </div>
                                                </Tooltip.Trigger>
                                                <Tooltip.Content>
                                                    <p>Read-only attribute</p>
                                                </Tooltip.Content>
                                            </Tooltip.Root>
                                        </Tooltip.Provider>
                                    </Table.Cell>
                                </Table.Row>
                            {#each module.template.fields as attribute (attribute.key)}
                                <Table.Row>
                                    <Table.Cell class="text-right">
                                        {attribute.name}
                                    </Table.Cell>
                                    <Table.Cell>
                                        <DynamicInput bind:object={obj!} {attribute} {readOnly}/>
                                    </Table.Cell>
                                </Table.Row>
                            {/each}
                        </Table.Body>
                    </Table.Root>
                </div>
            </Tab.Content>
            <Tab.Content value="links" class="flex-1 flex flex-col min-h-0"> 
                <h2 class="font-bold my-1">Links</h2>
                <div class="flex flex-col gap-2 flex-1 min-h-0 pr-3 overflow-auto">
                    <Accordion.Root type="multiple" class="w-full sm:max-w-full">
                        <Accordion.Item value="item-1">
                            <Accordion.Trigger>
                                <div class="flex items-center gap-1 font-semibold">
                                    <Icon icon="mi:arrow-right" width="20px" class="-rotate-45"/>
                                    Outbound Links
                                </div>
                            </Accordion.Trigger>
                            <Accordion.Content class="flex flex-col text-balance">
                                {#key obj.metadata?.outboundLinks}
                                    {#if obj.metadata?.outboundLinks??[].length < 0}
                                        <Table.Root> 
                                            <Table.Row> 
                                                <Table.Head class="sticky top-0">Module Location</Table.Head>
                                                <Table.Head class="sticky top-0">Object ID</Table.Head>
                                                <Table.Head class="sticky top-0"></Table.Head>
                                            </Table.Row>
                                            <Table.Body> 
                                                {#each obj.metadata?.outboundLinks as link}
                                                    <Table.Row>
                                                        <Table.Cell>
                                                            <div class="flex flex-row items-center word-wrap">
                                                                <Icon icon="mi:arrow-right" width="16px" class="text-red-700 -rotate-45"/>
                                                                <p class="px-1">{link.path}</p>
                                                            </div>
                                                        </Table.Cell>
                                                        <Table.Cell>{link.object}</Table.Cell>
                                                        <Table.Cell>
                                                            <div class="flex flex-row justify-end">
                                                                <Tooltip.Provider> 
                                                                    <Tooltip.Root>
                                                                        <Tooltip.Trigger>
                                                                            <Button variant="ghost" onclick={() => handleLinkVisit(link)}>
                                                                                <Icon icon="gravity-ui:arrow-up-right-from-square" width="20px"/>
                                                                            </Button>
                                                                        </Tooltip.Trigger>
                                                                        <Tooltip.Content>
                                                                            <p>Visit</p>
                                                                        </Tooltip.Content>
                                                                    </Tooltip.Root>
                                                                </Tooltip.Provider>
                                                                <Tooltip.Provider> 
                                                                    <Tooltip.Root>
                                                                        <Tooltip.Trigger>
                                                                            <Button variant="ghost" class="hover:text-red-700 hover:bg-red-100" onclick={() => handleUnlink(obj, module.path, link)}>
                                                                                <Icon icon="gravity-ui:link-slash" width="20px"/>
                                                                            </Button>
                                                                        </Tooltip.Trigger>
                                                                        <Tooltip.Content>
                                                                            <p>Unlink</p>
                                                                        </Tooltip.Content>
                                                                    </Tooltip.Root>
                                                                </Tooltip.Provider>
                                                            </div>
                                                        </Table.Cell>
                                                    </Table.Row>
                                                {/each}
                                            </Table.Body>
                                        </Table.Root>
                                    {:else}
                                        <div class="grow flex flex-row justify-center items-center w-full h-full text-slate-500">
                                            <Icon icon="gravity-ui:circle-link" width="20px"/>
                                            <div class="flex items-center mx-2 my-2">
                                                No Outbound links
                                            </div>
                                        </div>
                                    {/if}
                                {/key}
                            </Accordion.Content>
                        </Accordion.Item>
                        <Accordion.Item value="item-2">
                            <Accordion.Trigger>
                                <div class="flex items-center gap-1 ml-1 my-1 font-semibold">
                                    <Icon icon="mi:arrow-left" width="20px" class="-rotate-45"/>
                                    Inbound Links
                                </div>
                            </Accordion.Trigger>
                            <Accordion.Content class="flex flex-col text-balance">
                                {#key obj.metadata?.inboundLinks}
                                    {#if obj.metadata?.inboundLinks??[].length < 0}
                                        <Table.Root> 
                                            <Table.Row> 
                                                <Table.Head class="sticky top-0">Module Location</Table.Head>
                                                <Table.Head class="sticky top-0">Object ID</Table.Head>
                                                <Table.Head class="sticky top-0"></Table.Head>
                                            </Table.Row>
                                            <Table.Body> 
                                                {#each obj.metadata?.inboundLinks as link}
                                                    <Table.Row>
                                                        <Table.Cell>
                                                            <div class="flex flex-row items-center">
                                                                <Icon icon="mi:arrow-left" width="16px" class="text-amber-500 -rotate-45"/>
                                                                <p class="px-1">{link.path}</p>
                                                            </div>
                                                        </Table.Cell>
                                                        <Table.Cell>{link.object}</Table.Cell>
                                                        <Table.Cell>
                                                            <div class="flex flex-row justify-end">
                                                                <Tooltip.Provider> 
                                                                    <Tooltip.Root>
                                                                        <Tooltip.Trigger>
                                                                            <Button variant="ghost" onclick={() => handleLinkVisit(link)}>
                                                                                <Icon icon="gravity-ui:arrow-up-right-from-square" width="20px"/>
                                                                            </Button>
                                                                        </Tooltip.Trigger>
                                                                        <Tooltip.Content>
                                                                            <p>Visit</p>
                                                                        </Tooltip.Content>
                                                                    </Tooltip.Root>
                                                                </Tooltip.Provider>
                                                            </div>
                                                        </Table.Cell>
                                                    </Table.Row>
                                                {/each}
                                            </Table.Body>
                                        </Table.Root>
                                    {:else}
                                        <div class="grow flex flex-row justify-center items-center w-full h-full text-slate-500">
                                            <Icon icon="gravity-ui:circle-link" width="20px"/>
                                            <div class="flex items-center mx-2 my-2">
                                                No Inbound links
                                            </div>
                                        </div>
                                    {/if}
                                {/key}
                            </Accordion.Content>
                        </Accordion.Item>
                    </Accordion.Root>
                </div>
                <!-- <div class="flex flex-col flex-1 min-h-0 overflow-auto">
                    <div class="gap-2 w-full py-1">
                        <LinkForm bind:links={object.metadata.outboundLinks} editable={allowChanges} onvisitLink={handleVisitLink}/>
                    </div>
                    <Separator/>
                </div> -->
            </Tab.Content>
            <Tab.Content value="history" class="grow flex flex-col "> 
                <div class="grow flex flex-col justify-center items-center w-full h-full text-slate-500">
                    <Icon icon="gravity-ui:clock-arrow-rotate-left" width="40px"/>
                    <div class="flex items-center gap-1 ml-1 my-2">
                        This feature is not implemented yet
                    </div>
                </div>
            </Tab.Content>
        </Tab.Root>
        <Dialog.Footer>
            <div class="flex flex-col w-full">
                <Separator class="mb-2"/>
                <div class="flex flex-row w-full">
                    {#if allowChanges}
                        <div class="flex flex-row gap-3">
                            <Button variant="secondary" class="px-5" onclick={handleClose}>
                                <Icon icon="ci:add-minus-square" width="20px"/>
                                <p class="pl-2">Cancel</p>
                            </Button>
                            {#if !obj.deletedAt && allowChanges && obj.id !== 0}
                                <Button variant="destructive" class="px-5" onclick={handleDeleteObject}>
                                    <Icon icon="ci:close-square" width="20px"/>
                                    <p class="pl-2">Delete Object</p>
                                </Button>
                            {/if}
                        </div>
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
                        {#if obj.deletedAt && !readOnly}
                            <Button variant="secondary" class="px-5" onclick={handleRestoreObject}>
                                <Icon icon="ci:arrow-reload-02" width="20px"/>
                                <p class="pl-2">Restore Object</p>
                            </Button>
                        {/if}
                    {/if}
                </div>
            </div>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
