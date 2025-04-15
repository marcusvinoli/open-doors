<script lang="ts">
    import type { Object } from "$lib/components/structs/Object";
    import * as ContextMenu from "$lib/components/ui/context-menu";
    import ContextMenuItem from "$lib/components/ui/context-menu/context-menu-item.svelte";
    import { createEventDispatcher } from "svelte";

    export let object: Object;
    export let readOnly: boolean = false;
    const dispatch = createEventDispatcher();

    function itemClick(id: number | string, item: string) {
        dispatch("click", {id, item})
    }
</script>

<ContextMenu.Root>
    <ContextMenu.Trigger>
        <slot />
    </ContextMenu.Trigger>
    <ContextMenu.Content>
        <ContextMenu.Item on:click={() => itemClick(object.id, 'properties')}>Properties</ContextMenu.Item>
    </ContextMenu.Content>
</ContextMenu.Root>
