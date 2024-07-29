<script lang="ts">
    import type { CustomFields, Object } from "$lib/components/structs/Object";
    import { author } from "$lib/stores/Author";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import * as Tooltip from "$lib/components/ui/tooltip";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import * as Alert from "$lib/components/ui/alert";
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    import Icon from "@iconify/svelte";
    
    export let object: Object;
    export let customFiels: CustomFields | null = null;

    if(!object) {
        object = {
            id: BigInt(0),
            number: "0",
            header: "",
            content: "",
            author: $author,
            isActive: true,
            isNormative: false,
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

<div class="w-full h-full flex flex-col px-4 min-w-[400px]">
    {#if object.isDraft}
    <div class="py-2">
        <Alert.Root variant="destructive" class="text-center h-[30px]">
            <Alert.Title class="italic text-sm leading-[0]">This is a Draft Object</Alert.Title>
        </Alert.Root>
    </div>
    {/if}
    <div class="grid gap-4 my-1">
        <div class="grid grid-cols-8 items-center gap-2">
            <Label for="name" class="text-right col-span-1">ID</Label>
            <Input id="name" bind:value={object.id} class="col-span-2" disabled/>
            <Label for="name" class="text-right col-span-1">Level</Label>
            <Input id="name" bind:value={object.number} class="col-span-2" />
            <Button variant="secondary" class="cursor-default col-span-1">
                <Icon icon="gravity-ui:bars-descending-align-left-arrow-down" width="15px"/>
            </Button>
            <Button variant="secondary" class="cursor-default col-span-1">
                <Icon icon="gravity-ui:bars-descending-align-left-arrow-up" width="15px"/>
            </Button>
        </div>
        <div class="grid grid-cols-8 items-center gap-2">
            <Label for="name" class="text-right col-span-1">Header</Label>
            <Input id="name" bind:value={object.header}  class="col-span-5" />
            <Button variant="secondary" class="cursor-default col-span-1">
                <Icon icon="gravity-ui:text-indent" width="15px"/>
            </Button>
            <Button variant="secondary" class="cursor-default col-span-1">
                <Icon icon="gravity-ui:text-outdent" width="15px"/>
            </Button>
        </div>
        <Separator/>
        <div class="grid grid-cols-8 items-center gap-2">
            <div class="col-span-1"></div>
            <div class="flex flex-row gap-2 justify-between items-center col-span-7">
                <div>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:bold" width="15px"/>
                    </Button>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:italic" width="15px"/>
                    </Button>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:underline" width="15px"/>
                    </Button>
                </div>
                <Separator orientation="vertical" class="h-[25px]"/>
                <div>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:list-unordered" width="15px"/>
                    </Button>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:list-ordered" width="15px"/>
                    </Button>
                </div>
                <Separator orientation="vertical" class="h-[25px]"/>
                <div>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:heading" width="15px"/>
                    </Button>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:table" width="15px"/>
                    </Button>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:code" width="15px"/>
                    </Button>
                    <DropdownMenu.Root closeOnItemClick closeOnOutsideClick>
                        <DropdownMenu.Trigger>
                            <Tooltip.Root openDelay={200}>
                                <Tooltip.Trigger class=" w-full">
                                    <div>
                                        <Button variant="secondary" size="sm" class="cursor-default w-full">
                                            <Icon icon="ci:heading" width="15px"/>
                                        </Button>
                                    </div>
                                </Tooltip.Trigger>
                                <Tooltip.Content>
                                    Heading level
                                </Tooltip.Content>
                            </Tooltip.Root>
                        </DropdownMenu.Trigger>
                        <DropdownMenu.Content>
                            <Button variant="ghost" class="cursor-default">
                                <Icon icon="ci:heading" width="15px"/>
                            </Button>
                            <Button variant="ghost" class="cursor-default">
                                <Icon icon="ci:heading-h1" width="15px"/>
                            </Button>
                            <Button variant="ghost" class="cursor-default">
                                <Icon icon="ci:heading-h2" width="15px"/>
                            </Button>
                            <Button variant="ghost" class="cursor-default">
                                <Icon icon="ci:heading-h3" width="15px"/>
                            </Button>
                            <Button variant="ghost" class="cursor-default">
                                <Icon icon="ci:heading-h4" width="15px"/>
                            </Button>
                            <Button variant="ghost" class="cursor-default">
                                <Icon icon="ci:heading-h5" width="15px"/>
                            </Button>
                            <Button variant="ghost" class="cursor-default">
                                <Icon icon="ci:heading-h6" width="15px"/>
                            </Button>
                        </DropdownMenu.Content>
                    </DropdownMenu.Root>
                </div>
                <Separator orientation="vertical" class="h-[25px]"/>
                <div>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:link-horizontal" width="15px"/>
                    </Button>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:paperclip-attechment-tilt" width="15px"/>
                    </Button>
                </div>
                <Separator orientation="vertical" class="h-[25px]"/>
                <div>
                    <Button variant="secondary" size="sm" class="cursor-default">
                        <Icon icon="ci:arrow-down-up" width="15px"/>
                    </Button>
                </div>
            </div>
        </div>
        <div class="grid grid-cols-8 items-center gap-2 min-h[100px]">
            <Label for="name" class="text-right col-span-1">Content</Label>
            <Textarea id="name" bind:value={object.content}  class="col-span-7" />
        </div>
        <Separator/>
    </div>
    <div class="grow"></div>
    <div class="flex flex-row py-5">
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