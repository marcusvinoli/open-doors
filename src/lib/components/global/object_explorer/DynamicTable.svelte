<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { ModuleManifest } from "$lib/components/structs/ModuleManifest";
    import type { Object } from "$lib/components/structs/Object";
    import type { View } from "$lib/components/structs/View";
    import * as Table from "$lib/components/ui/table";
    import DynamicCell from "./DynamicCell.svelte";
    import HeaderContext from "./HeaderContextMenu.svelte";
    import RowContextMenu from "./RowContextMenu.svelte";
    
    export let moduleManifest: ModuleManifest;
    export let view: View;
    export let readOnly: boolean = true;
    export let objects: Object[] = [];
    const dispatch = createEventDispatcher();

    function contextClick(e: any) {
        dispatch("contextClick", {id: e.detail.id, item: e.detail.item})
    }

</script>

<Table.Root class="w-full relative" id="scroll-table">
    <Table.Header class="w-full min-w-96" id="scroll-table-header">
        <Table.Row>
            {#each view.items as viewAttribute}
                <Table.Head>
                    <HeaderContext bind:view={view}>
                        {viewAttribute.attribute}
                    </HeaderContext>
                </Table.Head>
            {/each}
        </Table.Row>
    </Table.Header>
    <Table.Body>
        {#each objects as object}
        <Table.Row>
            {#each view.items as attribute}
                <Table.Cell>
                    <RowContextMenu object={object} on:click={contextClick} readOnly={readOnly}>
                        <DynamicCell object={object} viewItem={attribute} moduleManifest={moduleManifest}/>
                        <!-- {object[attribute.key]} -->
                    </RowContextMenu>
                </Table.Cell>
            {/each}
        </Table.Row>
        {/each}
    </Table.Body>
</Table.Root>
