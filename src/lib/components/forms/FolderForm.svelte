<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from '../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from './ComboboxAllRecipientsOnRepository.svelte';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from "../../../routes/store";
    import { listAllRecipientsExceptChildren, listRelatives } from "$lib/utils/lists";
    import { readProject } from "$lib/controllers/Project";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { Project } from "$lib/components/structs/Project";
    import type { TreeItem } from "../structs/Tree";
    import { path } from "@tauri-apps/api";

    export let openDialog: boolean = true;
    export let folder: TreeItem;

    let loading: boolean = false;
    let loadingMessage: "Reading project information...";

    let parent: TreeItem;
    let possibleParents: TreeItem[];

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    function handleUpdateProject() {

    }
    
    function loadData() {
        readProject($repository!, folder)
            .then((retProject) => {
                    let prj = retProject as Project;
                    possibleParents = listAllRecipientsExceptChildren($repository?.structure, folder);
                    parent = listRelatives($repository?.structure, $repository?.structure, folder).pop()??$repository?.structure;
                    console.log("Parent: ", possibleParents);
                })
                .catch((err) => {
                    console.log(err)
                })
                .finally(() => {
                    loading = false;
                })
    }

    
    $: if (openDialog) {
        console.log("Loading ProjectForm.");
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
                {#if folder}
                <Dialog.Header>
                    <Dialog.Title>{folder.name}</Dialog.Title>
                    <Dialog.Description>{folder.path}</Dialog.Description>
                </Dialog.Header>
                    <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="name" class="text-right col-span-1">Parent</Label>
                        <div class="col-span-3">
                        {#if $repository}
                        <ComboboxAllRecipientsOnRepository recipients={possibleParents} selectedItem={parent} />
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
                <Button variant="destructive" on:click={closeDialog}>
                    <Icon icon="gravity-ui:trash-bin" width="15px"/>
                    <p class="pl-2">Delete</p>
                </Button>
                <div class="grow"></div>
                <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
                <Button on:click={handleUpdateProject}>Save Changes</Button>
            </Dialog.Footer>
        {/if}
    </Dialog.Content>
</Dialog.Root>