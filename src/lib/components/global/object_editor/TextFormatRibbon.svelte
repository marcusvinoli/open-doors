<script lang="ts">
    import * as Tooltip from "$lib/components/ui/tooltip";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import Icon from "@iconify/svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
</script>

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