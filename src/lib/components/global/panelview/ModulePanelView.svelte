<script lang="ts">
    import Icon from "@iconify/svelte";
    import type { Baseline, Module, ModuleManifest } from "$lib/components/structs/Module";
    import type { TreeItem } from "$lib/components/structs/Tree";
    import { readModule } from "$lib/controllers/Module";
    import * as Table from "$lib/components/ui/table/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip";
    import { Button } from "$lib/components/ui/button";
    import { goto } from "$app/navigation";
    import { encodePath } from "$lib/utils/pathHandler";
    import { createEventDispatcher } from "svelte";

    export let moduleTree: TreeItem;

    let module: Module;
    let dispatch = createEventDispatcher();

    function loadData() {
        readModule(moduleTree)
            .then((mod) => {
                module = mod;
            })
    }

    function openBaseline(baseline: string) {
        dispatch('openBaseline', baseline);
    }

/*     function editBaseline(baseline: string) {
        dispatch('editBaseline', baseline)
    } */

    $: {
        if(moduleTree) {
            loadData();
        }
    }

</script>

{#if module && module.baselines.length > 0}
<Table.Root>
    <Table.Header>
        <Table.Head class="w-[100px]">Version</Table.Head>
        <Table.Head>Description</Table.Head>
        <Table.Head></Table.Head>
    </Table.Header>
    <Table.Body>
        {#each module.baselines.reverse() as baseline, index}
        <Table.Row class="h-10px">
            <Table.Cell>
                <span class="px-2">{baseline.version}</span>
            </Table.Cell>
            <Table.Cell>
                {#if baseline.description === ""}
                <span class="px-2 font-light italic text-slate-500">No description given to this Baseline</span>
                {:else}
                <span class="px-2">{baseline.description}</span>
                {/if}
            </Table.Cell>
            <Table.Cell class="w-[50px]">
                <div class="flex flex-row-reverse gap-2">
                    <Tooltip.Root openDelay={300}>
                        <Tooltip.Trigger>
                            <Button variant="ghost" on:click={() => {openBaseline(baseline.version)}}>
                                <Icon icon="gravity-ui:arrow-up-right-from-square" width="20px" color=""/>
                            </Button>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>Open baselined module</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                    <!-- <Tooltip.Root openDelay={300}>
                        <Tooltip.Trigger>
                            <Button variant="ghost" on:click={() => {editBaseline(baseline.version)}}>
                                <Icon icon="gravity-ui:pencil-to-square" width="20px" color=""/>
                            </Button>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>Edit baseline details</p>
                        </Tooltip.Content>
                    </Tooltip.Root> -->
                </div>
            </Table.Cell>
        </Table.Row>
        {/each}
    </Table.Body>
</Table.Root>
{:else}
<div class="w-full h-full grow flex flex-col items-center justify-center text-slate-400 pb-[100px] rounded-lg">
    <Icon icon="gravity-ui:tag" width="50px"/>
    <h1 class="text-xl font-semibold my-1">NO BASELINE YET</h1>
</div>
{/if}
