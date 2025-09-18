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
        state,
    } : {
        object: Object;
        children?: import('svelte').Snippet;
        readOnly?: boolean,
        onclick?: (item: string, id: number | string) => void;
        linker?: Linker | null;
        state: Map<string, string>;
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
        <ContextMenu.Separator class="mx-1"/>
        {#if !linker}
            <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'createLink');}}>Link...</ContextMenu.Item>
        {:else}
            <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'stopLinking');}}>Stop linking</ContextMenu.Item>
            <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'stablishLink');}}>Stablish link</ContextMenu.Item>
        {/if}
        <ContextMenu.Separator class="mx-1"/>
        <ContextMenu.Sub>
            <ContextMenu.SubTrigger>
                <p>New Object</p>
            </ContextMenu.SubTrigger>
            <ContextMenu.SubContent> 
                <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'newObjectAfter');}}>Create after</ContextMenu.Item>
                <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'newObjectBelow');}}>Create below</ContextMenu.Item>
            </ContextMenu.SubContent>
        </ContextMenu.Sub>
        {@const movingId = state.get('moving') ? Number.parseInt(state.get('moving')!) : -1}
        {#if movingId === -1}
            <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'startMoving');}}>Start moving</ContextMenu.Item>
        {:else}
            <ContextMenu.Sub>
                <ContextMenu.SubTrigger disabled={object.id === movingId}>Move...</ContextMenu.SubTrigger>
                <ContextMenu.SubContent> 
                    <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'moveAfter');}}>Move after</ContextMenu.Item>
                    <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'moveBelow');}}>Move below</ContextMenu.Item>
                </ContextMenu.SubContent>
            </ContextMenu.Sub>  
            <ContextMenu.Item onclick={(e: any) => {e.preventDefault; itemClick(object.id, 'stopMoving');}}>Stop moving</ContextMenu.Item>
        {/if}
    </ContextMenu.Content>
</ContextMenu.Root>
