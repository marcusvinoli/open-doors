<script lang="ts">
    import { onMount } from 'svelte';
    import Icon from '@iconify/svelte';
    import Tree from "$lib/components/global/treeview/TreeView.svelte";
    import * as Resizable from "$lib/components/ui/resizable";
    import type { TabData } from "$lib/components/global/tabs/TabData";
    import type { Repository } from '$lib/components/structs/Repo';
    import { newTab, repository, showToolbar } from '../store';
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import CreateProjectForms from "$lib/components/forms/CreateProjectForms.svelte"

    let openRepo: Repository;
    let newProjectDialog = false;

    function openNewProjectDialog() {
        newProjectDialog = true;
    }

    onMount(() => {
        $showToolbar = true;
        openRepo = JSON.parse(localStorage.getItem('repository') as string) as Repository;
        let homeTab: TabData = {
            path: "/home",
            icon: "gravity-ui:house",
            title: "Home",
        };
        $newTab = homeTab;
    })

</script>

<div class="bg-slate-50 h-full py-1">
    <CreateProjectForms bind:openDialog={newProjectDialog} />
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={20} minSize={5}>
            <Tree treeItems={openRepo?.structure}/>
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        <Resizable.Pane minSize={5}>
            {#if openRepo?.structure.length > 0}
                <div></div>
            {:else}
                <div class="w-full flex flex-col text-center items-center text-slate-500 py-10">
                    <div class="my-5">
                        <Icon icon="gravity-ui:folder-exclamation" width="50px"/>
                    </div>
                    <h1 class="font-semibold ">It seems that there is no project on this Repository...</h1>
                    <h2 class="font-regular pb-3">Let's create the first one!</h2>
                    <Button on:click={openNewProjectDialog}>
                        <Icon icon="gravity-ui:folder-plus" width="20px"/>
                        <p class="pl-2">New Project</p>
                    </Button>
                </div>
            {/if}
        </Resizable.Pane>
    </Resizable.PaneGroup> 
</div>