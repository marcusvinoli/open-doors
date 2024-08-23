<script lang="ts">
    import Loading from '../../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from '../utils/ComboboxAllRecipientsOnRepository.svelte';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from "$lib/stores/Repository";
    import { createModule } from '$lib/controllers/Module';
    import { reloadRepository } from "$lib/controllers/Repository";
    import { createProject, readProject } from '$lib/controllers/Project';
    import { createEventDispatcher, onMount } from 'svelte';
    import { listAllRecipientItemsFromRepository } from '$lib/utils/listAllRecipientsFromRepository';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { TreeItem } from '../../structs/Tree';
    import type { ModuleManifest } from "$lib/components/structs/Module";

    export let openDialog: boolean = false;
    export let selectedParent: TreeItem;

    let loading: boolean = false;

    let moduleManifest: ModuleManifest = {
        title:"",
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
        createModule(moduleManifest, selectedParent)
            .then((mod) => {
                console.log(mod);
                dispatch('create', {manifest: moduleManifest});
            })
            .finally(() => {
                closeDialog();
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
                <Input id="name" placeholder="Module" bind:value={moduleManifest.title}  class="col-span-3" />
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
                {#if (moduleManifest.title !== "") && (moduleManifest.prefix !== "") && (parent)}
                Your module will be displayed as <strong>{moduleManifest?.prefix} {moduleManifest?.separator} {moduleManifest?.title}</strong>.
                {/if}
            </Dialog.Description>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleCreateModule} disabled={(moduleManifest.prefix===""||moduleManifest.title==="")}>Create</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>