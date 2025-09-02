<script lang="ts">
    import Icon from '@iconify/svelte';
    
    import { app } from '$lib/stores/AppState.svelte';
    import { getFirstAndLastName } from '$lib/controllers/User';
    import * as Tooltip from "$lib/components/ui/tooltip";
    import { relativePath } from '$lib/utils/path-handler';

    let user = $derived(app.user);
    let repo = $derived(app.repository);
    let tasks = $derived(app.tasks);
    let linker = $derived(app.linker);

    let tasksList = $derived.by(() => {
        return Array.from(tasks.values())
    });

</script>

{#if repo}
<div class="flex h-6 text-sm text-neutral-100 bg-slate-500 select-none cursor-default">
    <div class="flex flex-col justify-center items-center h-full min-w-[40px] bg-slate-300 text-slate-500">
        <Tooltip.Provider>
            <Tooltip.Root> 
                <Tooltip.Trigger>
                    <Icon icon="ci:data" width="20px"/>
                </Tooltip.Trigger>
                <Tooltip.Content class="">
                    <p>{repo.manifest.name}</p>
                    <p>{repo.tree.path}</p>
                </Tooltip.Content>    
            </Tooltip.Root>
        </Tooltip.Provider>
    </div>
    {#if user}
        <div class="flex grow px-1">
                <div class="flex justify-center items-center text-slate-100 text-xs whitespace-nowrap">
                    <Icon icon="ci:user-02" width="15px" class="mx-2"/>
                    <p class="mr-1">{getFirstAndLastName(user?.name??"") + " <"+ user?.email + ">"}</p>
                </div>
        </div>
    {/if}
    <div class="flex flex-col-reverse h-full w-full px-2">
        {#each tasksList as task}
            <Tooltip.Provider>
                <Tooltip.Root> 
                    <Tooltip.Trigger>
                        <Icon icon={task.icon} width="20px" />
                    </Tooltip.Trigger>
                    <Tooltip.Content class="">
                        <p>{task.job}</p>
                        <p>{task.tooltip}</p>
                    </Tooltip.Content>
                </Tooltip.Root>
            </Tooltip.Provider>
        {/each}
    </div>
    {#if linker}
        <div class="h-full flex flex-row-reverse px-2">
            <Tooltip.Provider>
                <Tooltip.Root> 
                    <Tooltip.Trigger>
                        <Icon icon="gravity-ui:link" width="18px"/>
                    </Tooltip.Trigger>
                    <Tooltip.Content class="">
                        <p>Linking from {linker.from?.path}-{linker.from?.object}</p>
                    </Tooltip.Content>
                </Tooltip.Root>
            </Tooltip.Provider>
        </div>
    {/if}
</div>
{/if}
