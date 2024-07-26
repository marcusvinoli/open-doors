<script lang="ts">
    import Loading from '../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from './ComboboxAllRecipientsOnRepository.svelte';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from "../../../routes/store";
    import { createProject, readProject } from '$lib/controllers/Project';
    import { reloadRepository } from "$lib/controllers/Repository";
    import { createEventDispatcher, onMount } from 'svelte';
    import { listAllRecipientItemsFromRepository } from '$lib/utils/listAllRecipientsFromRepository';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { TreeItem } from '../structs/Tree';
    import { createModule } from '$lib/controllers/Module';
    import type { ModuleManifest } from "$lib/components/structs/Module";
    import type { Project } from "$lib/components/structs/Project";

    export let openDialog: boolean = false;

    let loading: boolean = false;
    export let selectedParent: TreeItem;
    let parentProject: Project;
    let moduleManifest: ModuleManifest = {
        name:"",
        prefix:"",
        separator: "-",
        description: "",
    };

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    const dispatch = createEventDispatcher();

    function handleCreateModule() {
        loading = true;
        createModule(moduleManifest, selectedParent!).then(() => {
            reloadRepository();
            closeDialog();
        })
        dispatch('create', {manifest: moduleManifest, parent: selectedParent});
    }

    $: {
        readProject($repository!, selectedParent)
            .then((prj) => {
                parentProject = prj as Project;
            })
            .catch((err) => {
                console.log(err);
            })
            .finally(() => {
                loading = false;
            })
    }

</script>

<Dialog.Root bind:open={openDialog} closeOnEscape closeOnOutsideClick>
    <Dialog.Content class="sm:max-w-[520px]">
        <Dialog.Header>
            <Dialog.Title>New Module</Dialog.Title>
            <Dialog.Description>
                Let's create a new Objects Module!
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4 py-4 min-h-42">
            {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">Creating a new Module...</h1>
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
                <Label for="name" class="text-right col-span-1">Module name</Label>
                <Input id="name" placeholder="My Awesome Project" bind:value={moduleManifest.name}  class="col-span-3" />
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="desc" class="text-right col-span-1">Module description</Label>
                <Input multiple id="desc" placeholder="Module Description" bind:value={moduleManifest.description}  class="col-span-3" />
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="prefix" class="text-right col-span-1">Prefix</Label>
                <Input id="prefix" placeholder="PRJ" bind:value={moduleManifest.prefix} class="col-span-1" />
                <Label for="name" class="text-right col-span-1">Separator</Label>
                <Input id="name" placeholder="-" bind:value={moduleManifest.separator} class="col-span-1"/>
            </div>
            <Dialog.Description>
                {#if (moduleManifest.name !== "") && (moduleManifest.prefix !== "") && (parentProject)}
                Your module will be displayed as <strong>{parentProject.manifest.prefix}{parentProject.manifest.separator}{moduleManifest?.prefix} {moduleManifest?.separator} {moduleManifest?.name}</strong>.
                {/if}
            </Dialog.Description>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleCreateModule}>Create</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>