<script lang="ts">
    import type { CustomFields, Object } from "$lib/components/structs/Object";
    import { author } from "$lib/stores/Author";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import { Checkbox } from "$lib/components/ui/checkbox/index.js";
    import * as Accordion from "$lib/components/ui/accordion";
    import { marked } from 'marked'

    import * as Alert from "$lib/components/ui/alert";
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    import Icon from "@iconify/svelte";
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";

    import "./markdown.css";
    
    export let object: Object;
    export let customFiels: CustomFields | null = null;

    if(!object) {
        object = {
            id: BigInt(0),
            number: "0",
            header: "",
            content: "",
            author: $author!,
            isActive: false,
            isRequirement: false,
            createdAt: new Date(),
            updatedAt: new Date(),
            deletedAt: null,
            customFiels: customFiels,
            links: {
                input: [],
                output: []
            },
            isDraft: true,
        }
    }
</script>

<div class="w-full h-full flex flex-col px-3">
    {#if object.isDraft}
    <div class="py-2">
        <Alert.Root variant="destructive" class="text-center p-3">
            <Alert.Title class="italic text-sm flex items-center justify-center mb-0">
                <Icon icon="ci:circle-warning" width="20px"/>
                This is a Draft Object
            </Alert.Title>
        </Alert.Root>
    </div>
    {/if}
    <div class="grid gap-2 my-1">
        <h2 class="font-bold mb-1">Object Heading</h2>
        <div class="grid grid-cols-8 items-center gap-2">
            <Label for="name" class="text-right col-span-1">ID</Label>
            {#if object.id === BigInt(0)}
                <Input id="name" placeholder="Auto Generated" class="col-span-3" disabled/>
            {:else}
                <Input id="name" bind:value={object.id} class="col-span-3" disabled/>
            {/if}
            <Label for="name" class="text-right col-span-1">Level</Label>
            <Input id="name" bind:value={object.number} class="col-span-3" />
            <!-- <Button variant="secondary" class="cursor-default col-span-1">
                <Icon icon="gravity-ui:bars-descending-align-left-arrow-down" width="15px"/>
            </Button>
            <Button variant="secondary" class="cursor-default col-span-1">
                <Icon icon="gravity-ui:bars-descending-align-left-arrow-up" width="15px"/>
            </Button> -->
        </div>
        <div class="grid grid-cols-8 items-center gap-2">
            <Label for="name" class="text-right col-span-1">Header</Label>
            <Input id="name" bind:value={object.header}  class="col-span-7" />
            <!-- <Button variant="secondary" class="cursor-default col-span-1">
                <Icon icon="gravity-ui:text-indent" width="15px"/>
            </Button>
            <Button variant="secondary" class="cursor-default col-span-1">
                <Icon icon="gravity-ui:text-outdent" width="15px"/>
            </Button> -->
        </div>
        <Separator/>
    </div>
    <div class="grid gap-2 my-1">
        <h2 class="font-bold my-1">Object Main Content</h2>
        <div class="grid grid-cols-8 items-center gap-2 min-h[100px]">
            <Label for="name" class="text-right col-span-1">Text</Label>
            <Textarea id="name" bind:value={object.content}  class="col-span-7 font-mono" />
        </div>
        <Separator/>
    </div>
    <div class="grid gap-2 my-1">
        <h2 class="font-bold my-1">Object Cassification</h2>
        <div class="grid grid-cols-8 items-center gap-2 min-h[100px]">
            <div class="col-span-1"></div>
            <div class="flex items-center col-span-3">
                <Checkbox id="reqCheck" bind:checked={object.isRequirement} aria-labelledby="reqCheck-label" />
                <Label
                id="reqCheck-label"
                for="reqCheck"
                class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 pl-3">
                Requirement
                </Label>
            </div>
            <div class="flex items-center col-span-3">
                <Checkbox id="actCheck" bind:checked={object.isActive} aria-labelledby="actCheck-label" />
                <Label
                id="actCheck-label"
                for="actCheck"
                class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 pl-3">
                Active
                </Label>
            </div>
        </div>
        <Separator/>
    </div>
    <div class="grid gap-2 my-1">
        <h2 class="font-bold my-1">Links</h2>
        <div class="gap-2 col-span-8 w-full">
        <Accordion.Root>
            <Accordion.Item value="item-1" class="">
                <Accordion.Trigger>
                    <div class="flex items-center gap-2 ml-2">
                        <Icon icon="ci:arrow-down-left-lg" width="20px"/>
                        Inbound Links
                    </div>
                </Accordion.Trigger>
              <Accordion.Content class="">
                    This feature is not supported yet.
              </Accordion.Content>
            </Accordion.Item>
            <Accordion.Item value="item-2">
                <Accordion.Trigger>
                    <div class="flex items-center gap-2 ml-2">
                        <Icon icon="ci:arrow-up-right-lg" width="20px"/>
                        Outbound Links
                    </div>
                </Accordion.Trigger>
                <Accordion.Content>
                    This feature is not supported yet.
                </Accordion.Content>
            </Accordion.Item>
        </Accordion.Root>
        </div>
    </div>
    <div class="gap-2 my-1 h-full">
        <h2 class="font-bold">Preview</h2>
        {#if object}
        <ScrollArea>
            <div class="preview min-h-[100px]">
                {@html marked(object.content)}
            </div>
        </ScrollArea>
        {/if}
    </div>
    <div class="grow flex flex-row py-1">
        <Button variant="destructive" class="px-5">
            <Icon icon="ci:add-minus-square" width="20px"/>
            <p class="pl-2">Cancel</p>
        </Button>
        <div class="grow"></div>
        <div class="flex flex-row gap-3">
            <Button variant="secondary" class="px-5">
                <Icon icon="ci:add-to-queue" width="20px"/>
                <p class="pl-2">Save and Commit</p>
            </Button>
            <Button class="px-5">
                <Icon icon="ci:add-plus-square" width="20px"/>
                <p class="pl-2">Save as Draft</p>
            </Button>
        </div>
    </div>
</div>
