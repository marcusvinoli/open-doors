<script lang="ts">
    import Icon from "@iconify/svelte";
    import * as Table from "$lib/components/ui/table";
    import type { ObjectView, Object } from "$lib/components/structs/Object";
    import { createEventDispatcher } from "svelte";
    import { Button } from "$lib/components/ui/button";
    import { marked } from "marked";
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import "./markdown.css";

    export let objects: ObjectView[] = [];
    export let prefix: String = "";
    export let separator: String = "";
    export let editMode: boolean = true;

    let objs: ObjectView[] = [];

    const dispatch = createEventDispatcher();

    function onEditClick(obj: ObjectView) {
        dispatch("select", {object: obj})
    }

    function handleRowClick(obj: ObjectView) {
        dispatch('click', {object: obj})
    }

    function generateHashString(level: string): string {
        const delimiters = /[\.\-a-zA-Z]/;
        const parts = level.split(delimiters);
        const numericParts = parts.filter(part => part.match(/^\d+$/));
        const hashCount = Math.min(numericParts.length, 6);
        return '#'.repeat(hashCount) + " ";
    }

    function compareLevels(a: string, b: string): number {
        const parseLevel = (level: string) => level.split(/[\.\-]/).map(part => isNaN(Number(part)) ? part : Number(part));
        
        const aParts = parseLevel(a);
        const bParts = parseLevel(b);
        
        const len = Math.max(aParts.length, bParts.length);
        for (let i = 0; i < len; i++) {
            if (aParts[i] === undefined) return -1;
            if (bParts[i] === undefined) return 1;
            
            if (typeof aParts[i] === 'number' && typeof bParts[i] === 'number') {
                if (aParts[i] !== bParts[i]) return (aParts[i] as number) - (bParts[i] as number);
            } else if (typeof aParts[i] === 'string' && typeof bParts[i] === 'string') {
                let aP = aParts[i] as string;
                let bP = bParts[i] as string;
                if (aParts[i] !== bParts[i]) return aP.localeCompare(bP);
            } else {
                return typeof aParts[i] === 'number' ? -1 : 1;
            }
        }
        return 0;
    }

    function sortItems(items: ObjectView[]): ObjectView[] {
        return items.sort((a, b) => compareLevels(a.object.level, b.object.level));
    }

    $: {       
        objs = sortItems(objects);
    }

    let tableHeaderClass: string = ""
    let tableCellClass: string = "text-sm "
</script>

<div class="h-full w-full flex flex-col">
    {#if objects.length > 0}
    <Table.Root class="w-full">
        <ScrollArea orientation="both" class="w-full">
        <Table.Header class="sticky top-0">
            <Table.Row>
                <Table.Head class="">
                    <div class={tableHeaderClass}>
                        #
                    </div>
                </Table.Head>
                <Table.Head class="">
                    <div class={tableHeaderClass}>
                        ID
                    </div>
                </Table.Head>
                <Table.Head class="">
                    <div class={tableHeaderClass}>    
                        Object Text
                    </div>
                </Table.Head>
                <Table.Head>
                    <div class={tableHeaderClass}>    
                        Active?
                    </div>
                </Table.Head>
                <Table.Head>
                    <div class={tableHeaderClass}>    
                        Normative?
                    </div>
                </Table.Head>
                <Table.Head>
                    <div class={tableHeaderClass}>    
                        Requirement?
                    </div>
                </Table.Head>
                <Table.Head class="">
                    <div class={tableHeaderClass}>    
                        Author
                    </div>
                </Table.Head>
                {#if editMode}
                <Table.Head class="min-w-[50px]">
                </Table.Head>
                {/if}
            </Table.Row>
        </Table.Header>
        <Table.Body>
            {#each objs as ov, index}
                <Table.Row class="" on:click={() => {handleRowClick(ov)}}>
                    <Table.Cell class={tableCellClass}>{index}</Table.Cell>
                    <Table.Cell class={tableCellClass}>{prefix}{separator}{ov.object.id}</Table.Cell>
                    <Table.Cell class={tableCellClass + (ov.isDraft ? " border-l-2 border-l-yellow-500" : "")}>
                        <div class={"markdown min-w-[420px]"}>
                            {#if ov.object.header !== ""}
                            {@html marked(generateHashString(ov.object.level) + ov.object.level + " " + ov.object.header)}
                            {/if}
                            {@html marked(ov.object.content)}
                        </div>
                    </Table.Cell>
                    <Table.Cell class="font-medium w-[20px]">
                        {#if ov.object.isActive}
                        <div class="text-green-500 flex justify-center items-center">
                          <Icon icon="gravity-ui:check" width="15px"/>
                        </div>
                        {:else}
                        <div class="text-red-500 flex justify-center items-center">
                            <Icon icon="gravity-ui:xmark" width="15px"/>
                        </div>
                        {/if}
                    </Table.Cell>
                    <Table.Cell class="font-medium w-[20px]">
                        {#if ov.object.isNormative}
                        <div class="text-green-500 flex justify-center items-center">
                          <Icon icon="gravity-ui:check" width="15px"/>
                        </div>
                        {:else}
                        <div class="text-red-500 flex justify-center items-center">
                            <Icon icon="gravity-ui:xmark" width="15px"/>
                        </div>
                        {/if}
                    </Table.Cell>
                    <Table.Cell class="font-medium w-[20px]">
                        {#if ov.object.isRequirement}
                        <div class="text-green-500 flex justify-center items-center">
                          <Icon icon="gravity-ui:check" width="15px"/>
                        </div>
                        {:else}
                        <div class="text-red-500 flex justify-center items-center">
                            <Icon icon="gravity-ui:xmark" width="15px"/>
                        </div>
                        {/if}
                    </Table.Cell>
                    <Table.Cell class={tableCellClass}>
                        <div class="text-sm italic text-gray-800">
                            {ov.object.author}
                        </div>
                    </Table.Cell>
                    {#if editMode}
                    <Table.Head class="w-[50px] text-center">
                        <Table.Cell class="w-[80px]">
                            <Button variant="ghost" on:click={() => onEditClick(ov)}>
                                <div class="flex justify-center items-center">
                                    <Icon icon="gravity-ui:pencil-to-square" width="20px"/>
                                </div>
                            </Button>
                        </Table.Cell>
                    </Table.Head>
                    {/if}
                </Table.Row>
                {/each}
            </Table.Body>
            </ScrollArea>
        </Table.Root>
    {:else}
    <div class="w-full h-full grow flex flex-col items-center justify-center text-slate-400 pb-[100px] rounded-lg">
        <Icon icon="gravity-ui:layout-header-cells-large-fill" width="50px"/>
        <h1 class="text-xl font-semibold my-1">EMPTY MODULE</h1>
    </div>
    {/if}
</div>
