<script lang="ts">
    import Icon from "@iconify/svelte";

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

{#snippet contextMenuItem(object: Object, event: string, label: string, disabled?: boolean, icon?: string)}
    <ContextMenu.Item
        disabled={disabled}
        onclick={
        (e: any) => {
            e.preventDefault;
            itemClick(object.id, event);
        }}>
        {#if icon}
            <Icon {icon} />
        {/if}
        <p class="leading-5">{label}</p>
    </ContextMenu.Item>
{/snippet}

<ContextMenu.Root>
    <ContextMenu.Trigger class="p-0 h-full">
        {@render children?.()}
    </ContextMenu.Trigger>
    <ContextMenu.Content class="min-w-50">
        {@render contextMenuItem(object, 'properties', 'Properties')}
        <ContextMenu.Separator class="mx-1"/>
        {#if !linker}
            {@render contextMenuItem(object, 'createLink', 'Start linking...')}
        {:else}
            {@render contextMenuItem(object, 'stopLinking', 'Stop linking')}
            {@render contextMenuItem(object, 'establishLink', 'Establish link')}
        {/if}
        {#if !readOnly}
            <ContextMenu.Separator class="mx-1"/>
            {#if object.metadata?.status === 'draft'}
                {@render contextMenuItem(object, 'commitDraftObject', 'Commit')}
            {/if}
                {@render contextMenuItem(object, 'commitAllDrafts', 'Commit all')}
            {#if object.metadata?.status === 'deleted'}
                {@render contextMenuItem(object, 'restoreObject', 'Restore object')}
            {/if}
        {/if}
        <ContextMenu.Separator class="mx-1"/>
        <ContextMenu.Sub>
            <ContextMenu.SubTrigger disabled={readOnly}>
                <p>New Object</p>
            </ContextMenu.SubTrigger>
            <ContextMenu.SubContent class="min-w-50">
                {@render contextMenuItem(object, 'newObjectAfter', 'Create after', false, 'gravity-ui:arrow-down-from-line')}
                {@render contextMenuItem(object, 'newObjectBelow', 'Create below', false, 'gravity-ui:arrow-uturn-ccw-right')}
            </ContextMenu.SubContent>
        </ContextMenu.Sub>
        {@const movingId = state.get('moving') ? Number.parseInt(state.get('moving')!) : -1}
        {#if (movingId === -1) }
            {@render contextMenuItem(object, 'startMoving', 'Start moving...')}
        {:else}
            <ContextMenu.Sub>
                <ContextMenu.SubTrigger disabled={(object.id === movingId) || readOnly}>Move...</ContextMenu.SubTrigger>
                <ContextMenu.SubContent>
                    {@render contextMenuItem(object, 'moveAfter', 'Move after', false, 'gravity-ui:arrow-down-from-line')}
                    {@render contextMenuItem(object, 'moveBelow', 'Move below', false, 'gravity-ui:arrow-uturn-ccw-right')}
                </ContextMenu.SubContent>
            </ContextMenu.Sub>
            {@render contextMenuItem(object, 'stopMoving', 'Stop moving')}
        {/if}
    </ContextMenu.Content>
</ContextMenu.Root>
