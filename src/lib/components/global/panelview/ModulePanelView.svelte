<script lang="ts">
    import Icon from "@iconify/svelte";
    import { getBaselineStatus, getBaselineStatusIcon, getBaselineStatusIconColor } from "$lib/utils/BaselineInformations";
    import type { Baseline, Module, ModuleManifest } from "$lib/components/structs/Module";
    import type { TreeItem } from "$lib/components/structs/Tree";
    import { readModule } from "$lib/controllers/Module";
    import * as Table from "$lib/components/ui/table/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip";
    import { Button } from "$lib/components/ui/button";
    import { goto } from "$app/navigation";
    import { encodePath } from "$lib/utils/pathHandler";

    export let moduleTree: TreeItem;

    let module: Module;

    function loadData() {
        readModule(moduleTree)
            .then((mod) => {
                module = mod;
            })
    }

    $: {
        if(moduleTree) {
            loadData();
        }
    }

</script>

{#if module}
<Table.Root>
    <Table.Header>
        <Table.Head class="w-[80px]">Status</Table.Head>
        <Table.Head class="w-[100px]">Version</Table.Head>
        <Table.Head>Description</Table.Head>
        <Table.Head></Table.Head>
    </Table.Header>
    <Table.Body>
        {#each module.baselines.reverse() as baseline, index}
        <Table.Row class="h-10px">
            <Table.Cell class="text-center">
                <Tooltip.Root openDelay={100}>
                    <Tooltip.Trigger>
                        <Icon icon={getBaselineStatusIcon(getBaselineStatus(baseline))} width="15px" color={getBaselineStatusIconColor(getBaselineStatus(baseline))}/>
                    </Tooltip.Trigger>
                    <Tooltip.Content>
                        <p>{getBaselineStatus(baseline)}</p>
                    </Tooltip.Content>
                </Tooltip.Root>
            </Table.Cell>
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
                            <Button variant="ghost" on:click={() => {goto("/module/" + encodePath(module.path) + "/baseline/" + baseline.version )}}>
                                <Icon icon="gravity-ui:arrow-up-right-from-square" width="20px" color=""/>
                            </Button>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>Open in new tab</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                    <Tooltip.Root openDelay={300}>
                        <Tooltip.Trigger>
                            <Button variant="ghost">
                                <Icon icon="gravity-ui:pencil-to-square" width="20px" color=""/>
                            </Button>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>Edit details...</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                </div>
            </Table.Cell>
        </Table.Row>
        {/each}
    </Table.Body>
</Table.Root>
{/if}