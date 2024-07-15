<script lang="ts">
    import Loading from '../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from './ComboboxAllRecipientsOnRepository.svelte';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { listAllRecipientItemsFromRepository } from '$lib/utils/listAllRecipientsFromRepository';
    import { createEventDispatcher, onMount } from 'svelte';
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import { loadRepoInformation, reloadRepository } from "$lib/controllers/Repository";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { goto } from "$app/navigation";
    import type { ProjectManifest, Project } from "$lib/components/structs/Project";
    import type { Repository } from '../structs/Repo';
    import type { TreeItem } from '../structs/Tree';
    import { counter, repository } from "../../../routes/store";
    import { createProject } from '$lib/controllers/Project';

    export let openDialog: boolean = false;

    let repo: Repository | null;
    let loading: boolean = false;
    let recipients: TreeItem[];
    let treeParent: TreeItem;
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
        createProject(projectManifest, treeParent).then(() => {
            reloadRepository();
            closeDialog();
        })
        event.stopPropagation();
        dispatch('create', {manifest: projectManifest, parent: treeParent});
    }

    onMount(() => {
        repo = $repository;
        if (repo) {
            recipients = listAllRecipientItemsFromRepository(repo);
            treeParent = recipients[0];
        }
    })
</script>

<Dialog.Root bind:open={openDialog} closeOnEscape={true}>
    <Dialog.Content class="sm:max-w-[480px]">
        <Dialog.Header>
            <Dialog.Title>Create a New Project</Dialog.Title>
            <Dialog.Description>
                Let's create a new project.
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
                    <ComboboxAllRecipientsOnRepository recipients={recipients} baseRecipient={treeParent} />
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