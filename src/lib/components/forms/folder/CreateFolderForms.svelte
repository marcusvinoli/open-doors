<script lang="ts">
    import Loading from '../../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from '../utils/ComboboxAllRecipientsOnRepository.svelte';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from "$lib/stores/Repository";
    import { createFolder } from '$lib/controllers/Folder';
    import { listChildren } from '$lib/utils/lists';
    import { reloadRepository } from "$lib/controllers/Repository";
    import { createEventDispatcher, onMount } from 'svelte';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { TreeItem } from '../../structs/Tree';

    const dispatch = createEventDispatcher();

    export let openDialog: boolean = false;

    let loading: boolean = false;
    export let selectedParent: TreeItem;
    let folderName: string;

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    function handleCreateFolder(event: any) {
        loading = true;
        createFolder(folderName, selectedParent)
            .then((folder) => {
                reloadRepository();
                closeDialog();
                dispatch('created', {folder: folder}) ;
            })
    }

    onMount(() => {
        if (!selectedParent && $repository) {
            selectedParent = listChildren($repository.tree, ["folder", "project"])[0];
        }
    })
</script>

<Dialog.Root bind:open={openDialog} closeOnEscape closeOnOutsideClick>
    <Dialog.Content class="sm:max-w-[480px]">
        <Dialog.Header>
            <Dialog.Title>Create a New Folder</Dialog.Title>
            <Dialog.Description>
                Let's create a new folder.
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4 py-4 min-h-42">
            {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">Creating a new Folder...</h1>
            </div> 
            {:else}
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Create Here</Label>
                <div class="col-span-3">
                {#if $repository}
                    <ComboboxAllRecipientsOnRepository recipients={listChildren($repository.tree, ["folder", "project"])} bind:selectedItem={selectedParent} />
                {/if}
                </div>
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Folder Name</Label>
                <Input id="name" placeholder="Folder Name" bind:value={folderName}  class="col-span-3" />
            </div>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleCreateFolder}>Create</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>