<script lang="ts">
    import type { Object } from "$lib/components/structs/Object";
    import * as ContextMenu from "$lib/components/ui/context-menu";
    import ContextMenuContent from "$lib/components/ui/context-menu/context-menu-content.svelte";
    import ContextMenuItem from "$lib/components/ui/context-menu/context-menu-item.svelte";
    import { createEventDispatcher } from "svelte";

    export let object: Object;
    const dispatch = createEventDispatcher();

    function itemClick(id: number | string, item: string) {
        console.log(object.id)
        dispatch("click", {id, item})
    }
</script>

<ContextMenu.Root>
    <ContextMenu.Trigger>
        <slot />
    </ContextMenu.Trigger>
    <ContextMenu.Content>
        <ContextMenuContent on:click={() => itemClick(object.id, 'Hello!')}>{object.id}</ContextMenuContent>
    </ContextMenu.Content>
</ContextMenu.Root>
