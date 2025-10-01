<script lang="ts">
    import Icon from '@iconify/svelte';
    
    import { app } from '$lib/stores/AppState.svelte';
    import { getFirstAndLastName } from '$lib/controllers/User';
    
    import type { Task } from '$lib/components/structs/Task';
    import type { Linker } from '$lib/components/structs/States';
    import type { Repository } from '$lib/components/structs/Repo';
    import type { User } from '$lib/components/structs/User';
    
    import * as Tooltip from "$lib/components/ui/tooltip";

    let user : User | null = $derived(app.user);
    let repo : Repository | null = $derived(app.repository);
    let linker : Linker | null = $derived(app.linker);

    let globalTasksList: Task[] = $derived([...app.tasks.values()]);
    let moduleTasksList: Task[] = $derived([...app.currentModule?.tasks.values() ?? []]);

</script>

{#snippet statusTask(icon: string, job: string, tooltip: string)}
    <div class="flex items-center justify-center w-[25px] mx-1">
        <Tooltip.Provider>
            <Tooltip.Root> 
                <Tooltip.Trigger>
                    <div>
                        <Icon {icon} width="15px" />
                    </div>
                </Tooltip.Trigger>
                <Tooltip.Content class="">
                    <p class="font-bold pb-1">{job}</p>
                    <p>{tooltip}</p>
                </Tooltip.Content>
            </Tooltip.Root>
        </Tooltip.Provider>
    </div>
{/snippet}

{#snippet taskList(taskList: Task[])}
    <div class="flex flex-row-reverse h-full w-full">
        {#each taskList as task, i (i)}
            {@render statusTask(task.icon, task.job, task.tooltip)}
        {/each}
    </div>
{/snippet}

{#if repo}
<div class="flex h-6 text-sm text-neutral-100 bg-slate-500 select-none cursor-default pr-1">
    <div class="flex flex-col justify-center items-center h-full min-w-[40px] bg-slate-300 text-slate-500">
        <Tooltip.Provider>
            <Tooltip.Root> 
                <Tooltip.Trigger>
                    <Icon icon="ci:data" width="20px"/>
                </Tooltip.Trigger>
                <Tooltip.Content class="">
                    <p class="font-bold">{repo.manifest.name} (API Version {repo.manifest.apiVersion})</p>
                    <p>{repo.tree.path}</p>
                </Tooltip.Content>    
            </Tooltip.Root>
        </Tooltip.Provider>
    </div>
    {#if user}
        <div class="flex grow px-1">
                <div class="flex justify-center items-center text-slate-100 text-xs whitespace-nowrap">
                    <Icon icon="ci:user-02" width="15px" class="mx-1"/>
                    <p class="mr-1">{user.toString()}</p>
                </div>
        </div>
    {/if}
    {@render taskList(globalTasksList)}
    {@render taskList(moduleTasksList)}
    {#if linker}
        {@render statusTask("gravity-ui:link", "Object Linker", `Linking from ${linker.from?.path}-${linker.from?.object}`)}
    {/if}
</div>
{/if}
