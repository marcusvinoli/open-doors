<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from "$lib/components/ui/loading/Loading.svelte";
    
    import { Button } from "$lib/components/ui/button";
    import { readModule } from "$lib/controllers/Module";
    
    import type { TreeItem } from "$lib/components/structs/Tree";
    import type { Module } from "$lib/components/structs/Module";
    
    import * as Tooltip from "$lib/components/ui/tooltip";
    import * as Table from "$lib/components/ui/table/index.js";
    
    let { 
        moduleTree,
        onopenbaseline,
    } : {
        moduleTree: TreeItem;
        onopenbaseline?: (bl: string) => void;
    } = $props();

    let module: Module | null = $state(null);

    function openBaseline(baseline: string) {
        if (onopenbaseline) {
            onopenbaseline(baseline);
        }
    }

    async function load() {
        return readModule(moduleTree)
            .then(mod => {
                module = mod as Module;
            })
            .catch(err => {
                console.error(err);
            })
    }

    let result = load();
    
</script>

{#await result}
    <div class="flex flex-col justify-center items-center w-full h-full text-slate-500">
        <Loading/>
        <h1 class="font-semibold">LOADING MODULE...</h1>
    </div>
{:then}
    {#if module!.baselines.length > 0}
        <Table.Root>
            <Table.Header>
                <Table.Head class="w-[100px]">Version</Table.Head>
                <Table.Head>Description</Table.Head>
                <Table.Head></Table.Head>
            </Table.Header>
            <Table.Body>
                {#each [...module!.baselines].reverse() as baseline}
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
                            <Tooltip.Provider>
                                <Tooltip.Root delayDuration={300}>
                                    <Tooltip.Trigger>
                                        <Button variant="ghost" onclick={() => {openBaseline(baseline.version)}}>
                                            <Icon icon="gravity-ui:arrow-up-right-from-square" width="20px" color=""/>
                                        </Button>
                                    </Tooltip.Trigger>
                                    <Tooltip.Content>
                                        <p>Open baselined module</p>
                                    </Tooltip.Content>
                                </Tooltip.Root>
                            </Tooltip.Provider>
                            <!--
                            TODO: Do the baselines will support updates post-frozing? 
                            <Tooltip.Provider> 
                                <Tooltip.Root>
                                    <Tooltip.Trigger>
                                    <Button variant="ghost" onclick={() => {editBaseline(baseline.version)}}>
                                        <Icon icon="gravity-ui:pencil-to-square" width="20px" color=""/>
                                    </Button>
                                    </Tooltip.Trigger>
                                    <Tooltip.Content>
                                        <p>Edit baseline details</p>
                                    </Tooltip.Content>
                                </Tooltip.Root>
                            </Tooltip.Provider> 
                            -->
                        </div>
                    </Table.Cell>
                </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    {:else}
    <div class="w-full h-full grow flex flex-col items-center justify-center text-slate-400 pb-[100px] rounded-lg">
        <Icon icon="gravity-ui:tag" width="50px"/>
        <h1 class="text-xl font-semibold my-1">NO BASELINES YET</h1>
    </div>
    {/if}
{:catch e}
    <div class="flex flex-col justify-center items-center w-full h-full text-slate-500">
        <Icon icon="mdi:dinosaur-pixel" width="50px"/>
        <h1 class="text-xl font-semibold my-1">OOPS! SOMETHING WENT WRONG...</h1>
        <div class="bg-red-100 border-red-900 rounded-sm text-red-800 mt-2 max-w-[80%] font-mono text-sm px-2 py-1 overflow-auto max-h-50">
            <div class="border-b-2 border-b-red-200">
                <p class="bold">Error Details:</p> 
            </div>
            <p>{e}</p>
        </div>
    </div>
{/await}



