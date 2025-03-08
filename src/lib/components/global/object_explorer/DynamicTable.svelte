<script lang="ts">
    import type { ObjectView } from "$lib/components/structs/Object";
    import type { View } from "$lib/components/structs/View";
    import * as Table from "$lib/components/ui/table";
    import HeaderContext from "./HeaderContext.svelte";
    
    export let view: View;
    export let readOnly: boolean = true;
    export let objectsView: ObjectView[] = [];
    
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
    {#if !readOnly}
    <Table.Head class="min-w-[50px] sticky top-0 bg-slate-50 shadow-sm"/>
    {/if}
    <Table.Body>
        
    </Table.Body>
</Table.Root>
