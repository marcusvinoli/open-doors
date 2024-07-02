<script lang="ts">
    import type { Baseline } from "./Baseline";
    import Icon from "@iconify/svelte";
    import * as Table from "$lib/components/ui/table";
    import Button from '$lib/components/ui/button/button.svelte';
    export let baselines: Baseline[];
    export let projectName: string;
</script>

<div>
    {#if baselines.length > 0}
    <Table.Root>
        <Table.Header>
            <Table.Row>
                <Table.Head class="w-[80px]">Baseline</Table.Head>
                <Table.Head class="w-[50px] text-center">Status</Table.Head>
                <Table.Head>Comments</Table.Head>
                <Table.Head>Date</Table.Head>
                <Table.Head class="w-[50px] text-center">Open</Table.Head>
            </Table.Row>
        </Table.Header>
        <Table.Body>
            {#each baselines as baseline}
            <Table.Row class="h-[10px]">
              <Table.Cell class="font-medium">{baseline.tag}</Table.Cell>
              <Table.Cell class="font-medium">
                {#if baseline.commitDate}
                    <div class="text-green-500 flex justify-center items-center">
                        <Icon icon="gravity-ui:circle-fill" width="15px"/>
                    </div>
                {:else}
                    <div class="text-yellow-500 flex justify-center items-center">
                        <Icon icon="gravity-ui:circle-fill" width="15px"/>
                    </div>
                    {/if}
                </Table.Cell>
                <Table.Cell>{baseline.message}</Table.Cell>
                <Table.Cell>{baseline.commitDate?.toLocaleDateString() || ""}</Table.Cell>
                <Table.Cell class="w-[80px]">
                    <Button variant="ghost">
                    <div class="flex justify-center items-center">
                        <Icon icon="gravity-ui:arrow-up-right-from-square" width="20px"/>
                    </div>
                </Button>
              </Table.Cell>
            </Table.Row>
            {/each}
          </Table.Body>
    </Table.Root>
    {:else}
    <p>This project has no Baseline yet. Consider creating one.</p>
    {/if}
</div>
