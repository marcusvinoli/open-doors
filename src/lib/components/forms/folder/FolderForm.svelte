<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from "$lib/components/ui/loading/Loading.svelte";
    import TreeItemsComboBox from "../utils/TreeItemsComboBox.svelte";
    
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { confirm } from '@tauri-apps/api/dialog';
    import { isValid } from "$lib/utils/name-validator";
    import { repository } from "$lib/stores/Repository.svelte";
    import { deleteFolder, updateFolder } from "$lib/controllers/Folder";
    import { listAllContainersExceptChildren, listRelatives } from "$lib/utils/lists";

    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as path from "path";

    import type { TreeItem } from "$lib/components/structs/Tree";
    import type { Repository } from "$lib/components/structs/Repo";

    const loadingMessage: string = "Loading folder information";
    const deletingFolderMessage: string = "Deleting folder...";
    const updatingFolderMessage: string = "Updating folder...";

    let { 
        openDialog = $bindable(true), 
        folder,
        onfolderupdate,
        onfolderdelete,
    } : {
        openDialog: boolean;
        folder: TreeItem;
        onfolderupdate?: (folder: TreeItem) => void;
        onfolderdelete?: (folder: TreeItem) => void;
    } = $props();
    
    let repo: Repository = $derived(repository()!);
    let loading: boolean = $state(false);
    let infoMessage: string = $state(loadingMessage);

    let possibleParents: TreeItem[] = $derived.by(() => listAllContainersExceptChildren(repo.tree, folder));
    let currentParent: TreeItem = $derived.by(() => listRelatives(repo.tree, repo.tree, folder).pop() ?? repo.tree);
    let newParent: TreeItem = $derived.by(() => listRelatives(repo.tree, repo.tree, folder).pop() ?? repo.tree);
    
    let folderName: string = $derived.by(() => folder.name);
    let isValidName: boolean = $derived(isValid(folderName));

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    async function handleFolderUpdate() {
        //TODO: It is very possible to rename and move on single ´updateFolder´ command.
        loading = true;
        infoMessage = updatingFolderMessage;
        let updated = false;
        
        // Renaming directory
        if (folder.name !== folderName) {
            const parentDir = path.dirname(folder.path);
            const newFolderPath = path.join(parentDir, folderName);
            folder = await updateFolder(folder.path, newFolderPath);
            updated = true;
        }
        
        // Moving directory
        if (currentParent!.path !== newParent!.path) {
            const commomBase = path.dirname(currentParent!.path);
            const relative = path.relative(commomBase, folder.path);
            const newPath = path.join(newParent!.path, relative);
            folder = await updateFolder(folder.path, newPath);
            updated = true;
        }

        if(onfolderupdate && updated) {
            onfolderupdate(folder);
        }

        closeDialog();
    }

    async function handleDeleteFolder() {
        const confirmed = await confirm('Do you really want to delete this folder? All it content will be delete.', 'Deleting folder ' + folder!.name );
        if (!confirmed) {
            return;
        }
        loading = true;
        infoMessage = deletingFolderMessage;

        await deleteFolder(folder!);

        if(onfolderdelete) {
            onfolderdelete(folder);
        }

        closeDialog();            
    }
        
</script>

<Dialog.Root bind:open={openDialog}>
    <Dialog.Content class="sm:max-w-[480px]">
        {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">{infoMessage}</h1>
            </div> 
        {:else}
            <div class="grid gap-4 py-4 min-h-42">
                {#if folder && currentParent}
                <Dialog.Header>
                    <Dialog.Title>{folder.name}</Dialog.Title>
                    <Dialog.Description>{folder.path?.substring(repo?.tree.path?.length)}</Dialog.Description>
                </Dialog.Header>
                    <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="name" class="text-right col-span-1">Parent</Label>
                        <div class="col-span-3">
                            <TreeItemsComboBox items={possibleParents} bind:selectedItem={newParent} />
                        </div>
                    </div>
                    <div class="grid grid-cols-4 items-center gap-2">
                        <Label for="name" class="text-right col-span-1">Folder Name</Label>
                        <Input id="name" placeholder="My Folder Name" bind:value={folderName}  class="col-span-3" autocomplete="off"/>
                    </div>
                {/if}
            </div>
            <Dialog.Footer>
                <Button variant="destructive" onclick={handleDeleteFolder}>
                    <Icon icon="gravity-ui:trash-bin" width="15px"/>
                    <p class="pl-2">Delete</p>
                </Button>
                <div class="grow"></div>
                <Button variant="secondary" onclick={closeDialog}>Cancel</Button>
                <Button onclick={handleFolderUpdate} disabled={!isValidName}>Save Changes</Button>
            </Dialog.Footer>
        {/if}
    </Dialog.Content>
</Dialog.Root>
