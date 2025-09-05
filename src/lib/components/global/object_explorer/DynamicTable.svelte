<script lang="ts">
    import Icon from "@iconify/svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import DynamicCell from "./DynamicCell.svelte";
    import HeaderContext from "./HeaderContextMenu.svelte";
    import RowContextMenu from "./RowContextMenu.svelte";

    import { cn } from "$lib/utils";
    import { mapAttributesIntoView } from "./DynamicCell";
    import { onDestroy, onMount, tick } from "svelte";

    import type { View } from "$lib/components/structs/View";
    import type { Linker } from "$lib/components/structs/States";
    import type { Object } from "$lib/components/structs/Object";
    import type { Module } from "$lib/components/structs/Module";
    import type { Attribute } from "$lib/components/structs/Attributes";
    
    import * as Table from "$lib/components/ui/table";

    import "$lib/assets/flashing.css";

    const OBJECT_TABLE_CONTAINER_SUFFIX = '-container';
    
    let {
        id = "",
        module,
        view = $bindable(),
        readOnly = true,
        objects = [],
        selectedObject = $bindable(null),
        oncontextclick,
        onscroll,
        ondblclick,
        onclick,
        scroll,
        showDeletions,
        linker,
    } : {
        id?: string,
        module: Module;
        view: View;
        readOnly?: boolean;
        objects?: Object[];
        selectedObject?: Object | null;
        oncontextclick?: (item: string, id: string | number, arg?: any) => void;
        onscroll?: (e: any) => void;
        ondblclick?: (id: string | number) => void;
        onclick?: (id: string | number) => void;
        scroll?: {x: number, y: number};
        showDeletions?: boolean;
        linker?: Linker | null;
    } = $props();

    let attributeView: Attribute[] = $derived(mapAttributesIntoView({...view}, module.template));

    let _objects: Object[] = $derived.by(() => {
        if (showDeletions) {
            return objects;
        }
        return objects.filter(obj => {
            return (obj.metadata?.status !== 'deleted')
        })
    })

    $effect(() => {
        if (scroll) {
            document.getElementById(id + OBJECT_TABLE_CONTAINER_SUFFIX)?.scrollTo({top: scroll.x, left: scroll.y, behavior: 'instant'});
        }
    })

    function contextClick(item: string, id: string | number) {
        if (oncontextclick) {
            oncontextclick(item, id);
        }
    }

    function onDoubleClick(object: Object) {
        if (ondblclick) {
            ondblclick(object.id);
        }
    }

    function onClick(object: Object) {
        selectedObject = (selectedObject?.id === object.id) ? null : object;
    }

    onMount(() => {
        if (!onscroll) {
            return;
        }
        if (scroll) {
            document.getElementById(id + OBJECT_TABLE_CONTAINER_SUFFIX)?.scrollTo({top: scroll.x, left: scroll.y, behavior: 'instant'});
        }
        tick().then(() => {
            document.getElementById(id + OBJECT_TABLE_CONTAINER_SUFFIX)?.addEventListener("scroll", onscroll);
        });
    })
    
    onDestroy(() => {
        if (!onscroll) {
            return;
        }
        document.getElementById(id + OBJECT_TABLE_CONTAINER_SUFFIX)?.removeEventListener("scroll", onscroll);
    })

</script>

{#if objects.length > 0}
<div class="relative h-full">
    <div class="absolute top-0 bottom-0 w-full overflow-auto">
        <Table.Root id={id} class="min-h-0">
            <Table.Header id={id + "-header"} class="w-full min-w-96 z-20">
                <Table.Row class="bg-slate-50">
                    {#each view.items as viewItem (viewItem.key)}
                        {#if viewItem.show}
                            <Table.Head class="sticky top-0 bg-slate-50 !hover:bg-slate-200 shadow-sm">
                                <HeaderContext bind:view template={module.template}>
                                    {viewItem.attribute}
                                </HeaderContext>
                            </Table.Head>
                        {/if}
                    {/each}
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {#each _objects as object (object.updatedAt)}
                    <Table.Row 
                        id="row-{object.id}" 
                        class={cn(
                            (object.metadata?.status === 'deleted') ? 'bg-red-100 text-rose-800 line-through border-red-50' : '', 
                            (object.id === selectedObject?.id) ? 'border-b-slate-300 border-t-slate-500 bg-slate-200' : '',
                        )}
                        ondblclick={() => onDoubleClick(object)}
                        onclick={() => onClick(object)}
                        >
                        {#each attributeView as attribute (attribute.key)}
                            <Table.Cell class="p-0">
                                <RowContextMenu 
                                    object={object} 
                                    onclick={contextClick} 
                                    readOnly={readOnly} 
                                    linker={linker}
                                >
                                    <DynamicCell 
                                        object={object} 
                                        attribute={attribute} 
                                        module={module}
                                        showLinks={true}
                                    />
                                </RowContextMenu>
                            </Table.Cell>
                        {/each}
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </div>
</div>
{:else}
    <div class="w-full h-full grow flex flex-col items-center justify-center text-slate-400 pb-[80px] rounded-lg">
        <Icon icon="gravity-ui:layout-header" width="50px"/>
        <h1 class="text-xl font-semibold my-1">EMPTY MODULE</h1>
        <Button variant="ghost"  onclick={() => contextClick('newObject', 0)}>
            <Icon icon="gravity-ui:circle-plus" width="50px"/>
            Create a new object
        </Button>
    </div>
{/if}
