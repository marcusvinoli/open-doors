<script lang="ts">
    import Loading from '$lib/components/ui/loading/Loading.svelte';
    import TreeItemsComboBox from '../utils/TreeItemsComboBox.svelte';

    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { isValid } from '$lib/utils/name-validator';
    import { repository } from "$lib/stores/Repository.svelte";
    import { listAllContainers, listAllContainersExceptChildren, listChildren } from '$lib/utils/lists';
    import { createFolder } from '$lib/controllers/Folder';
    import { reloadRepository } from "$lib/controllers/Repository";

    import * as Dialog from "$lib/components/ui/dialog/index.js";
    
    import type { TreeItem } from '$lib/components/structs/Tree';

    const placeholder: string = 'Select a project or folder';

    let { 
        openDialog = $bindable(false), 
        currentParent,
        oncreatefolder,
    } : {
        openDialog?: boolean;
        currentParent: TreeItem | null;
        oncreatefolder?: ((newFolder: TreeItem) => void) | null;
    } = $props();
    
    let repo = $derived(repository());
    let loading: boolean = $state(false);
    let selectedParent: TreeItem | null = $derived(currentParent ?? null);
    let possibleParents = $derived(listAllContainers(repo?.tree));
    let folderName: string = $state('');

    function closeDialog() {
        loading = false;
        openDialog = false;
        folderName = '';
    }

    function handleCreateFolder(event: any) {
        loading = true;

        if(!selectedParent) {
            return;
            // TODO: Insert here an Error Message.
        }

        if (!isValid(folderName)) {
            return;
            // TODO: Insert here an Error Message.
        }

        createFolder(folderName, selectedParent)
            .then((newFolder) => {
                if (oncreatefolder) {
                    oncreatefolder(newFolder as TreeItem);
                }
            })
            .finally(() => {
                closeDialog();
            })
    }

</script>

<Dialog.Root bind:open={openDialog}>
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
                {#if repo}
                    <TreeItemsComboBox items={possibleParents} bind:selectedItem={selectedParent} placeholder={placeholder}/>
                {/if}
                </div>
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Folder Name</Label>
                <Input id="name" placeholder="Folder Name" bind:value={folderName}  class="col-span-3" autocomplete="off"/>
            </div>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" onclick={closeDialog}>Cancel</Button>
            <Button onclick={handleCreateFolder} disabled={(folderName==="")}>Create</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>