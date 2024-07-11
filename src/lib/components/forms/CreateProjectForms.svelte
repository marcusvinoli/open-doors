<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    import { createEventDispatcher, onMount } from 'svelte';
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import type { ProjectManifest } from "$lib/components/structs/Project";
    import Loading from '../ui/loading/Loading.svelte';
    import type { Repository } from '../structs/Repo';
    import type { TreeItem } from '../structs/Tree';
    import { listAllRecipientItemsFromRepository } from '$lib/utils/listAllRecipientsFromRepository';
    import ComboboxAllRecipientsOnRepository from './ComboboxAllRecipientsOnRepository.svelte';

    export let openDialog: boolean = false;

    let loading: boolean = false;
    let repo: Repository;
    let recipients: TreeItem[];
    let baseRecipient: TreeItem;

    let prj: ProjectManifest = {
        name:"",
        separator: "-",
        prefix:"",
    };

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    const dispatch = createEventDispatcher();

    function handleCreate(event: any) {
        loading = true;
        event.stopPropagation();
        dispatch('create', {prj});
    }


    async function createNewProject() {
        invoke('create_project', {project: prj, path: repo.path})
            .then((project) => {
                project
            })
            .catch((err) => {
                console.log(err);
            })
    }

    onMount(() => {
        repo = JSON.parse(localStorage.getItem('repository') as string) as Repository
        recipients = listAllRecipientItemsFromRepository(repo);
        baseRecipient = recipients[0];
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
                    <ComboboxAllRecipientsOnRepository recipients={recipients} baseRecipient={baseRecipient} />
                </div>
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Project Name</Label>
                <Input id="name" placeholder="My Awesome Project" bind:value={prj.name}  class="col-span-3" />
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="prefix" class="text-right col-span-1">Prefix</Label>
                <Input id="prefix" placeholder="PRJ" bind:value={prj.prefix} class="col-span-1" />
                <Label for="name" class="text-right col-span-1">Separator</Label>
                <Input id="name" placeholder="-" bind:value={prj.separator} class="col-span-1"/>
            </div>
            <Dialog.Description>
                {#if prj.name !== ""}
                Your project will be displayed as <strong>{prj?.prefix} {prj?.separator} {prj?.name}</strong>.
                {/if}
            </Dialog.Description>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleCreate}>Create</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>