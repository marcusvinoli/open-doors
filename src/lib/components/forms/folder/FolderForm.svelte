<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from '../../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from '../utils/ComboboxAllRecipientsOnRepository.svelte';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { confirm } from '@tauri-apps/api/dialog';
    import { repository } from "$lib/stores/Repository";
    import { reloadRepository } from "$lib/controllers/Repository";
    import { createEventDispatcher } from "svelte";
    import { deleteFolder, readFolder, updateFolder } from "$lib/controllers/Folder";
    import { listAllRecipientsExceptChildren, listRelatives } from "$lib/utils/lists";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as path from 'path';
    import type { TreeItem } from "$lib/components/structs/Tree";

    const dispatch = createEventDispatcher();

    export let openDialog: boolean = true;
    export let folder: TreeItem;
    
    let loading: boolean = false;
    let loadingMessage: string = "Reading folder information...";
    
    let parent: TreeItem;
    let possibleParents: TreeItem[];
    
    let currentParent: TreeItem;
    let currentFolder: TreeItem;

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    async function handleFolderUpdate() {
        loadingMessage = "Updating folder...";
        loading = true;
        let updated = false;
        if((parent) && (currentParent) && (folder) && (currentFolder)){
            if (currentFolder.name !== folder.name) {
                const parentDir = path.dirname(currentFolder.path);
                const newFolderPath = path.join(parentDir, folder.name);
                folder = await updateFolder(currentFolder.path, newFolderPath);
                updated = true;
            }
            
            if (currentParent.path !== parent.path) {
                const commomBase = path.dirname(folder.path);
                const relative = path.relative(commomBase, folder.path);
                const newPath = path.join(parent.path, relative);
                folder = await updateFolder(folder.path, newPath);
                updated = true;
            }
        }

        if(updated) {
            dispatch('updated', {folder: folder})
            reloadRepository();
        }

        closeDialog();
    }

    async function handleDeleteFolder() {
        const confirmed = await confirm('Do you really want to delete this folder? All it content will be delete.', 'Deleting folder ' + folder.name );
        if (!confirmed) {
            return;
        }
        loadingMessage = "Updating folder...";
        loading = true;
        deleteFolder(folder)
            .then(() => {
                dispatch('deleted', {})
                reloadRepository();
            })
            .finally(() => {
                closeDialog();
            })
    }
    
    function loadData() {
        readFolder(folder)
            .then((ret) => {
                    let folder = ret as TreeItem;
                    possibleParents = listAllRecipientsExceptChildren($repository?.tree, folder);
                    parent = listRelatives($repository?.tree, $repository?.tree, folder).pop()??$repository?.tree;
                    currentFolder = folder;
                    currentParent = parent;
                })
                .catch((err) => {
                    console.log(err)
                })
                .finally(() => {
                    loading = false;
                })
    }

    $: if (openDialog) {
        loadData();
    }
    
</script>

<Dialog.Root bind:open={openDialog} closeOnEscape closeOnOutsideClick>
    <Dialog.Content class="sm:max-w-[480px]">
        {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">{loadingMessage}</h1>
            </div> 
        {:else}
            <div class="grid gap-4 py-4 min-h-42">
                {#if currentFolder && parent}
                <Dialog.Header>
                    <Dialog.Title>{currentFolder.name}</Dialog.Title>
                    <Dialog.Description>{currentFolder.path?.substring($repository?.tree.path?.length)}</Dialog.Description>
                </Dialog.Header>
                    <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="name" class="text-right col-span-1">Parent</Label>
                        <div class="col-span-3">
                        {#if $repository}
                            <ComboboxAllRecipientsOnRepository recipients={possibleParents} bind:selectedItem={parent} />
                        {/if}
                        </div>
                    </div>
                    <div class="grid grid-cols-4 items-center gap-2">
                        <Label for="name" class="text-right col-span-1">Folder Name</Label>
                        <Input id="name" placeholder="My Folder Name" bind:value={folder.name}  class="col-span-3" />
                    </div>
                {/if}
            </div>
            <Dialog.Footer>
                <Button variant="destructive" on:click={handleDeleteFolder}>
                    <Icon icon="gravity-ui:trash-bin" width="15px"/>
                    <p class="pl-2">Delete</p>
                </Button>
                <div class="grow"></div>
                <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
                <Button on:click={handleFolderUpdate}>Save Changes</Button>
            </Dialog.Footer>
        {/if}
    </Dialog.Content>
</Dialog.Root>