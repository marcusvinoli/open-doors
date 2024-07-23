<script lang="ts">
    import Loading from '../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from './ComboboxAllRecipientsOnRepository.svelte';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from "../../../routes/store";
    import { createProject } from '$lib/controllers/Project';
    import { reloadRepository } from "$lib/controllers/Repository";
    import { createEventDispatcher } from 'svelte';
    import { listAllRecipientItemsFromRepository } from '$lib/utils/listAllRecipientsFromRepository';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { TreeItem } from '../structs/Tree';
    import type { ProjectManifest, Project } from "$lib/components/structs/Project";

    export let openDialog: boolean = false;

    let loading: boolean = false;
    let selectedParent: TreeItem;
    let projectManifest: ProjectManifest = {
        name:"",
        separator: "-",
        prefix:"",
    };

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    const dispatch = createEventDispatcher();

    function handleCreateProject(event: any) {
        loading = true;
        createProject(projectManifest, selectedParent).then(() => {
            reloadRepository();
            closeDialog();
        })
        dispatch('create', {manifest: projectManifest, parent: selectedParent});
    }
</script>

<Dialog.Root bind:open={openDialog} closeOnEscape closeOnOutsideClick>
    <Dialog.Content class="sm:max-w-[480px]">
        <Dialog.Header>
            <Dialog.Title>Create a New Project</Dialog.Title>
            <Dialog.Description>
                Let's create a new project!
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4 py-4 min-h-42">
            {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">Creating a new Project...</h1>
            </div> 
            {:else}
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Create Here</Label>
                <div class="col-span-3">
                {#if $repository}
                <ComboboxAllRecipientsOnRepository recipients={listAllRecipientItemsFromRepository($repository)} bind:selectedItem={selectedParent} />
                {/if}
                </div>
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Project Name</Label>
                <Input id="name" placeholder="My Awesome Project" bind:value={projectManifest.name}  class="col-span-3" />
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="prefix" class="text-right col-span-1">Prefix</Label>
                <Input id="prefix" placeholder="PRJ" bind:value={projectManifest.prefix} class="col-span-1" />
                <Label for="name" class="text-right col-span-1">Separator</Label>
                <Input id="name" placeholder="-" bind:value={projectManifest.separator} class="col-span-1"/>
            </div>
            <Dialog.Description>
                {#if (projectManifest.name !== "") && (projectManifest.prefix !== "")}
                Your project will be displayed as <strong>{projectManifest?.prefix} {projectManifest?.separator} {projectManifest?.name}</strong>.
                {/if}
            </Dialog.Description>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleCreateProject}>Create</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>