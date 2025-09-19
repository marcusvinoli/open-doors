<script lang="ts">
    import Icon from "@iconify/svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import Dropdown from "./Dropdown.svelte";
    import Combobox from "./Combobox.svelte";
    
    import { cn } from "$lib/utils";
    import { marked } from "marked";
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    import { readOnlyAttributes } from "$lib/utils/attribute-utils";
    
    import type { Object } from "$lib/components/structs/Object";
    import type { Attribute } from "$lib/components/structs/Attributes";
    
    let { 
        object = $bindable(),
        attribute,
    } : {
        object: Object;
        attribute: Attribute;
    } = $props();

    type ObjectKey = keyof typeof object;
    type AttributeKey = keyof typeof attribute;

    function isCustomAttribute(attribute: Attribute) : boolean {
        return (!readOnlyAttributes.includes(attribute));
    }

    function hasCustomAttribute(attribute: Attribute) : boolean {
        if (!object.attributes) {
            return false;
        }
        if (typeof object.attributes[attribute.key] === 'undefined') {
            return false;
        }
        return true;
    }

    function safeWriteAttribute(attribute: Attribute, val: any) {
        if (isCustomAttribute(attribute) && hasCustomAttribute(attribute)) {
            object.attributes = {... object.attributes};
            if (val) {
                let newVal = '';
                if (Array.isArray(val)) {
                    newVal = val.join(', ');
                } else {
                    newVal = val.toString();
                }
                object.attributes[attribute.key] = newVal;
            } else {
                delete object.attributes[attribute.key];
            }
            return;
        }
        //TODO: Other object native attributes may require a Write function. 
    }

    function safeReadAttributes(attribute: Attribute) {
        if (isCustomAttribute(attribute) && hasCustomAttribute(attribute)) {
            if ((typeof attribute.kind === 'object') && ('multipleOptions' in attribute.kind)) {
                return object.attributes?.[attribute.key as AttributeKey]?.split(',').map(v => v.trim()).filter(v => v.length > 0) ?? [];
            }
            return object.attributes?.[attribute.key as AttributeKey]?.toString();
        }
        //TODO:  Maybe a `null` should be returned;
        return object[attribute.key as ObjectKey]?.toString() ?? '';
    }

    const bindValue = {
        get value() {
            return safeReadAttributes(attribute);
        },
        set value(val: any) {
            safeWriteAttribute(attribute, val);
        }
    }
    
</script>

{#if attribute.key === 'id'}
    <div class="p-2">
        <p>{bindValue.value}</p>
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
                {@html marked(bindValue.value ?? "")}
            {/if}
        </div>
    </div>
{:else}
    {#if attribute.kind === 'general'}
    <div class="p-2">
        <Textarea bind:value={bindValue.value}/>
        {@html marked(bindValue.value)}
    </div>
    {:else if attribute.kind === 'boolean'}
    <div class="p-2">
        <div class="{bindValue.value ? 'text-green-500' : 'text-red-500'} flex justify-center items-center">
            <Icon icon={bindValue.value ? 'gravity-ui:check' : 'gravity-ui:xmark'} width="15px"/>
        </div>
    </div>
    {:else if typeof attribute.kind === 'object'}
        <div class="p-2">
            {#if 'singleOption' in attribute.kind}
                <Dropdown items={attribute.kind.singleOption} bind:selected={bindValue.value} placeholder=""/>
            {:else if 'multipleOptions' in attribute.kind}
                <Combobox items={attribute.kind.multipleOptions} bind:selected={bindValue.value} placeholder=""/>
            {/if}
        </div>
    {:else}
    <!--// TODO: Implements inputs for other AttributeKinds, as for now, all other are being treated as a String-->
    <!-- // string, real, date, time, dateTime, enumeration, optional, user, any ... -->
    <div class="p-2">
        <Input id="name" class="col-span-6" autocomplete="off" bind:value={bindValue.value}/>
    </div>
    {/if}
{/if}
