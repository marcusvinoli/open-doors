<script lang="ts">
    import * as ContextMenu from "$lib/components/ui/context-menu";
    import { createEventDispatcher } from "svelte";

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

    </ContextMenu.Content>
</ContextMenu.Root>
