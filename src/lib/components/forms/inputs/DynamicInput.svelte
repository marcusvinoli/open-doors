<script lang="ts">
    import Icon from "@iconify/svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import Selection from "./Selection.svelte";
    
    import { cn } from "$lib/utils";
    import { marked } from "marked";
    import { Button } from "$lib/components/ui/button";
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    import { readOnlyAttributes } from "$lib/utils/attribute-utils";
    
    import type { Object } from "$lib/components/structs/Object";
    import type { Attribute } from "$lib/components/structs/Attributes";

    import * as RadioGroup from "$lib/components/ui/radio-group/index.js";
    import Label from "$lib/components/ui/label/label.svelte";
    import { useId } from "bits-ui";
    
    let { 
        object = $bindable(),
        attribute,
        readOnly = false,
    } : {
        object: Object;
        attribute: Attribute;
        readOnly?: boolean;
    } = $props();

    type ObjectKey = keyof typeof object;
    type AttributeKey = keyof typeof attribute;

    // TODO: Currently, this derived state is only being used for 'boolean' type. Some experiments might be required in order to expand the use of this for other types.
    let currentValue = $derived(safeReadAttributes(attribute));

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
            if ((typeof attribute.kind === 'object')) {
                if ('multipleOptions' in attribute.kind) {
                    return object.attributes?.[attribute.key as AttributeKey]?.split(',').map(v => v.trim()).filter(v => v.length > 0) ?? [];
                } else if ('singleOption' in attribute.kind) {
                    return object.attributes?.[attribute.key as AttributeKey]?.toString();
                }
            }
            if ((typeof attribute.kind === 'string')) {
                switch (attribute.kind) {
                    case 'boolean': {
                        return object.attributes?.[attribute.key as AttributeKey]?.toString() === 'True' ? 'True' : 'False';
                    }
                    default: 
                    return object.attributes?.[attribute.key as AttributeKey]?.toString() ?? '';
                }
            }
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
<div class="flex gap-2">
    {#if attribute.kind === 'general'}
        <Textarea bind:value={bindValue.value} readonly={readOnly}/>
        <!-- TODO: Add a preview feature for Markdown also for an attribute view. -->
        <!-- {@html marked(currentValue?.toString() ?? "")} -->
    {:else if attribute.kind === 'boolean'}
        {@const fieldid = useId()}
        {@const idTrue = useId()}
        {@const idFalse = useId()}
        <RadioGroup.Root value={currentValue?.toString()} class="flex py-2 gap-4" id={fieldid}>
            <div class="text-green-500 flex justify-center items-center">
                <RadioGroup.Item value="True" id={idTrue}
                    aria-checked=true
                    onclick={(e) => {
                        safeWriteAttribute(attribute, 'True');
                    }
                }/>
                <Label for={idTrue}> 
                    <Icon icon='gravity-ui:check' class="ml-1" width="18px"/>
                </Label>
            </div>
            <div class="text-red-500 flex justify-center items-center">
                <RadioGroup.Item value="False" id={idFalse} 
                    onclick={(e) => {
                        safeWriteAttribute(attribute, 'False');
                    }
                }/>
                <Label for={idFalse} >
                    <Icon icon='gravity-ui:xmark' class="ml-1" width="18px"/>
                </Label>
            </div>
        </RadioGroup.Root>
    {:else if typeof attribute.kind === 'object'}
        {#if 'singleOption' in attribute.kind}
            <Selection items={attribute.kind.singleOption} bind:selection={bindValue.value} {readOnly}/>
        {:else if 'multipleOptions' in attribute.kind}
            <Selection items={attribute.kind.multipleOptions} bind:selection={bindValue.value} {readOnly} multiple/>
        {/if}
    {:else}
        <Input id="name" class="col-span-6" autocomplete="off" bind:value={bindValue.value} readonly={readOnly}/>
    {/if}
    <!-- TODO: Reserved for future implementations. User may want to clear the attributes and for Booleans of Single-Options this is a little trick to do on current implementation.
    <Button variant="ghost" class="text-slate-400" disabled={readOnly} >
        <Icon icon='gravity-ui:eraser' width="15px"/>
    </Button> 
    -->
</div>
{/if}
