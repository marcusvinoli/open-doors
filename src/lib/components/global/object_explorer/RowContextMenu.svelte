<script lang="ts">
    import type { Object } from "$lib/components/structs/Object";
    import type { Linker } from "$lib/components/structs/States";

    import * as ContextMenu from "$lib/components/ui/context-menu";

    let { 
        object,  
        children,
        readOnly,
        onclick,
        linker,
    } : {
        object: Object;
        children?: import('svelte').Snippet;
        readOnly?: boolean,
        onclick?: (item: string, id: number | string) => void;
        linker?: Linker | null;
    } = $props();

    function itemClick(id: number | string, item: string) {
        if (!onclick) {
            return;
        }
        onclick(item, id);
    }

</script>

<ContextMenu.Root>
    <ContextMenu.Trigger class="p-0 h-full">
        {@render children?.()}
    </ContextMenu.Trigger>
    <ContextMenu.Content>
        <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'properties');}}>Properties</ContextMenu.Item>
        <ContextMenu.Separator/>
        {#if !linker}
        <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'createLink');}}>Create link</ContextMenu.Item>
        {:else}
        <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'stopLinking');}}>Stop linking</ContextMenu.Item>
        <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'stablishLink');}}>Stablish link</ContextMenu.Item>
        {/if}
        <ContextMenu.Separator/>
        <ContextMenu.Sub>
            <ContextMenu.SubTrigger>Create an Object...</ContextMenu.SubTrigger>
            <ContextMenu.SubContent> 
                <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'newObjectAfter');}}>on same level</ContextMenu.Item>
                <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'newObjectBelow');}}>as sub-level</ContextMenu.Item>
            </ContextMenu.SubContent>
        </ContextMenu.Sub>
    </ContextMenu.Content>
</ContextMenu.Root>
