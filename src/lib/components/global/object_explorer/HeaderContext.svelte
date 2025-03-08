<script lang="ts">
    import type { View } from "$lib/components/structs/View";
    import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
    
    export let view: View;
    export let showLinks: boolean = true;
    export let showNumbering: boolean = false;

    function toggleNumbering() {
        showNumbering = !showNumbering;
    }

    function toggleLinks() {
        showLinks = !showLinks;
    }
</script>

<ContextMenu.Trigger>
    <ContextMenu.Trigger>
        <slot />
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
        <ContextMenu.Separator/>
        <ContextMenu.Item on:click={toggleNumbering}>
            {(showNumbering)? "Hide" : "Show"} Row Number
        </ContextMenu.Item>
        <ContextMenu.Item on:click={toggleLinks}>
            {(showLinks)? "Hide" : "Show"} Object Links
        </ContextMenu.Item>
    </ContextMenu.Content>
</ContextMenu.Trigger>
