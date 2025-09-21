<script lang="ts">
    import type { Template } from "$lib/components/structs/Template";
    import type { Attribute } from "$lib/components/structs/Attributes";
    import type { View, ViewItem } from "$lib/components/structs/View";
    
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
    
    let {
        view = $bindable(),
        template,
        showLinks = $bindable(true),
        showRowNumbering = $bindable(false),
        children
    } : {
        view: View;
        template?: Template;
        showLinks?: boolean;
        showRowNumbering?: boolean;
        children?: import('svelte').Snippet;
    } = $props();

    function toggleNumbering() {
        showRowNumbering = !showRowNumbering;
    }

    function toggleLinks() {
        showLinks = !showLinks;
    }

    function updateView(field: Attribute) {
        const index = view.items.findIndex(item => item.key === field.key);
        if (index < 0) {
            const newView = {
                key: field.key,
                show: true,
                attribute: field.name,
            };
            view.items = [...view.items, newView];
            return;
        }
        view.items.splice(index, 1);
        view.items = [...view.items];
    }

</script>

<ContextMenu.Root>
    <ContextMenu.Trigger>
        {@render children?.()}
    </ContextMenu.Trigger>
    <ContextMenu.Content>
        <ContextMenu.Sub>
            <ContextMenu.SubTrigger>
                View
            </ContextMenu.SubTrigger>
            <ContextMenu.SubContent>
                {#each view.items as item}
                    <ContextMenu.CheckboxItem bind:checked={item.show}>
                        {item.attribute}
                    </ContextMenu.CheckboxItem>
                {/each}
            </ContextMenu.SubContent>
        </ContextMenu.Sub>
        <ContextMenu.Sub>
            <ContextMenu.SubTrigger disabled={(template ? (template.fields.length > 0 ? false : true) : true)}>
                Add Attribute to View
            </ContextMenu.SubTrigger>
            <ContextMenu.SubContent>
                {#if template}
                {#each template.fields as field}
                    <ContextMenu.CheckboxItem 
                        checked={view.items.findIndex(item => item.attribute === field.name) >= 0}
                        onclick={() => updateView(field)}
                        >
                        {field.name}
                    </ContextMenu.CheckboxItem>
                    {/each}
                {/if}
            </ContextMenu.SubContent>
        </ContextMenu.Sub>
        <ContextMenu.Separator/>
        <ContextMenu.Item onclick={toggleNumbering}>
            {(showRowNumbering)? "Hide" : "Show"} Row Number
        </ContextMenu.Item>
        <ContextMenu.Item onclick={toggleLinks}>
            {(showLinks)? "Hide" : "Show"} Object Links
        </ContextMenu.Item>
    </ContextMenu.Content>
</ContextMenu.Root>
