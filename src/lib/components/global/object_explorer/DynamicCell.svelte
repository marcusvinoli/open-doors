<script lang="ts">
    import Icon from "@iconify/svelte";
    
    import { cn } from "$lib/utils";
    import { marked } from "marked";
    
    import type { Module } from "$lib/components/structs/Module";
    import type { Object } from "$lib/components/structs/Object";
    import type { Template } from "$lib/components/structs/Template";
    import type { ViewItem } from "$lib/components/structs/View";
    import type { ModuleManifest } from "$lib/components/structs/ModuleManifest";
    import type { Attribute, AttributeKind } from "$lib/components/structs/Attributes";

    import { ObjectStatus } from "$lib/components/structs/ObjectStatus";
    
    let { 
        object,
        attribute,
        module,
        showLinks = true,
    } : {
        object: Object;
        attribute: Attribute;
        module: Module;
        showLinks: boolean;
    } = $props();

    type ObjectKey = keyof typeof object;
    type AttributeKey = keyof typeof object.attributes;

    let value = object[attribute.key as ObjectKey]?.toString() ?? object.attributes?.[attribute.key as AttributeKey]?.toString();
    
</script>

{#if attribute.key === 'id'}
    <div class="p-2">
        <p>{module.manifest.prefix}{module.manifest.separator}{value}</p>
    </div>
{:else if attribute.key === 'content'}
    <div class={cn(
            "p-2 border-l-3",
            "flex",
            (object.metadata?.status === "draft") ? "border-l-slate-500" : 
            (object.metadata?.status === "baselined") ? "border-l-sky-700" : 
            (object.metadata?.status === "deleted") ? "border-l-red-800" :
            "border-l-amber-400")}
        >
        <div class="grow flex row">
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
{:else}
    {#if attribute.kind === 'general'}
    <div class="p-2">
        {@html marked(value ?? "")}
    </div>
    {:else if attribute.kind === 'boolean'}
    <div class="p-2">
        <div class="{value ? 'text-green-500' : 'text-red-500'} flex justify-center items-center">
            <Icon icon={value ? 'gravity-ui:check' : 'gravity-ui:xmark'} width="15px"/>
        </div>
    </div>
    {:else} <!-- // string, real, date, time, dateTime, enumeration, optional, user, any ... -->
    <div class="p-2">
        <span>
            {value ?? ""}
        </span>
    </div>
    {/if}
{/if}
