<script lang="ts">
    import Icon from "@iconify/svelte";
    
    import { cn } from "$lib/utils";
    import { marked } from "marked";
    
    import type { Module } from "$lib/components/structs/Module";
    import type { Object } from "$lib/components/structs/Object";
    import type { Attribute } from "$lib/components/structs/Attributes";

    let { 
        object,
        attribute,
        module,
        showLinks = true,
    } : {
        object: Object;
        attribute: Attribute;
        module: Module;
        showLinks?: boolean;
    } = $props();

    type ObjectKey = keyof typeof object;
    type AttributeKey = keyof typeof object.attributes;

    let value = object[attribute.key as ObjectKey]?.toString() ?? object.attributes?.[attribute.key as AttributeKey]?.toString();
    
</script>

{#snippet verticalCenter(content: any)}
    <div class="p-2 h-full flex items-center">
        {@render content()}
    </div>
{/snippet}

{#snippet id()}
    <p>{module.manifest.prefix}{module.manifest.separator}{value}</p>
{/snippet}

{#snippet content()}
    <div class={cn(
            "px-2 border-l-3",
            "flex h-full",
            (object.metadata?.status === "draft") ? "border-l-slate-500" : 
            (object.metadata?.status === "baselined") ? "border-l-sky-700" : 
            (object.metadata?.status === "deleted") ? "border-l-red-800" :
            "border-l-amber-400")}
        >
        <div class="grow flex row h-full justify-center items-center">
            {#if object.header.trim() !== ""}
                {#if object.indexParentId === 0}
                    <h1 class="font-bold text-[1.5rem]">{object.metadata?.level+'. '}{object.header}</h1>
                {:else}
                    <h2 class="font-bold text-[1.1rem]">{object.metadata?.level+'. '}{object.header}</h2>
                {/if}
            {/if}
            {#if object.content.trim() !== ""}
                {@html marked(value ?? "")}
            {/if}
            <div class="flex flex-col justify-between items-end p1-2  grow">
                {#if showLinks}
                    {#if object.metadata?.outboundLinks}
                        <Icon icon="mi:arrow-right" width="15px" class="text-red-700 -rotate-45"/>
                    {/if}
                    {#if object.metadata?.inboundLinks}
                        <Icon icon="mi:arrow-left" width="15px" class="text-amber-500 -rotate-45"/>
                    {/if}
                {/if}
            </div>
        </div>
    </div>
{/snippet}

{#snippet markdown()}
    <div>
        {@html marked(value ?? "")}
    </div>
{/snippet}

{#snippet boolean()}
    <div class="{value === 'True' ? 'text-green-500' : 'text-red-500'} flex justify-center items-center">
        <Icon icon={value === 'True' ? 'gravity-ui:check' : 'gravity-ui:xmark'} width="15px"/>
    </div>
{/snippet}

{#snippet singleOption()}
    <p>{object.attributes![attribute.key] ?? ""}</p>
{/snippet}

{#snippet multipleOptions()}
    {@const values = object.attributes![attribute.key]?.split(',').map(v => v.trim()) ?? []}
    <div>
        {#each values as item}
            <p>{item}</p>
        {:else}
            <p></p>
        {/each}
    </div>
{/snippet}

{#snippet others()}
    <p>{value}</p>
{/snippet}

<div class="h-full">
    {#if attribute.key === 'id'}
        {@render verticalCenter(id)}
    {:else if attribute.key === 'content'}
        {@render content()}
    {:else}
        {#if attribute.kind === 'general'}
            {@render verticalCenter(markdown)}
        {:else if attribute.kind === 'boolean'}
            {@render verticalCenter(boolean)}
        {:else if typeof attribute.kind === 'object'}
            {#if 'singleOption' in attribute.kind}
                {@render verticalCenter(singleOption)}
            {:else if 'multipleOptions' in attribute.kind}
                {@render verticalCenter(multipleOptions)}
            {/if}
        {:else}
            <!-- // Render of other types, such as string, real, date, time, dateTime, enumeration, optional, user, any ... -->
            {@render verticalCenter(others)}
        {/if}
    {/if}
</div>
