<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from '../../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from '../utils/ComboboxAllRecipientsOnRepository.svelte';
    import { confirm } from '@tauri-apps/api/dialog';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from "$lib/stores/Repository";
    import { createModule, deleteModule, readModule, updateModuleManifest } from '$lib/controllers/Module';
    import { reloadRepository } from "$lib/controllers/Repository";
    import { createProject, readProject } from '$lib/controllers/Project';
    import { createEventDispatcher, onMount } from 'svelte';
    import { listAllRecipientItemsFromRepository } from '$lib/utils/listAllRecipientsFromRepository';
    import * as path from 'path';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { TreeItem } from '../../structs/Tree';
    import type { ModuleManifest } from "$lib/components/structs/Module";
    import { listAllRecipientsExceptChildren, listRelatives } from '$lib/utils/lists';
    import { updateFolder } from "$lib/controllers/Folder";

    const dispatch = createEventDispatcher();

    export let openDialog: boolean = false;
    export let module: TreeItem;

    let loading: boolean = false;
    let loadingMessage: string = "Reading module information...";

    let parent: TreeItem;
    let currentParent: TreeItem;
    let possibleParents: TreeItem[];
    
    let currentModule: ModuleManifest;
    let updatedModule: ModuleManifest;

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    async function handleModuleUpdate() {
        loadingMessage = "Updating module...";
        loading = true;
        let updated = false;

        if((module) && (currentParent) && (currentModule) && (updatedModule)){
            if (updatedModule.title !== currentModule.title ||
                updatedModule.description !== currentModule.description ||
                updatedModule.separator !== currentModule.separator
            ) {
                await updateModuleManifest(module, updatedModule)
                currentModule = (await readModule(module)).manifest;
                updated = true;
            }
            
            if (currentParent.path !== parent.path) {
                const moduleFolder = path.basename(module.path);
                const newPath = path.join(parent.path, moduleFolder);
                await updateFolder(module.path, newPath);
                updated = true;
            }
        }

        if(updated) {
            dispatch('updated', {module: currentModule});
            reloadRepository();
        }

        closeDialog();
    }

    async function handleModuleDelete() {
        const confirmed = await confirm('Do you really want to delete this Module? All its content will be delete!', 'Deleting module ' + currentModule.title );
        if (!confirmed) {
            return;
        }
        loading = true;
        deleteModule(module)
            .then(() => {
                reloadRepository();
                dispatch('deleted', {module: module});
            })
            .finally(() => {
                closeDialog();
            })
    }
    
    async function loadData() {
        console.log("Loading data...")
        if ($repository && (module.itemType === "module")) {
            let retModule = (await readModule(module)).manifest;
            currentModule = retModule;
            updatedModule = JSON.parse(JSON.stringify(retModule));
            possibleParents = listAllRecipientsExceptChildren($repository!.tree, module);
            parent = listRelatives($repository?.tree, $repository?.tree, module).pop()??$repository?.tree;
            currentParent = parent;
            loading = false;
        }
    }

    $: if (openDialog) {
        loadData();
    }
    
</script>

<Dialog.Root bind:open={openDialog} closeOnEscape closeOnOutsideClick>
    <Dialog.Content class="sm:max-w-[520px]">
        {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">{loadingMessage}</h1>
            </div> 
        {:else}
            <div class="grid gap-4 py-4 min-h-42">
            {#if updatedModule}
                <Dialog.Header>
                    <Dialog.Title>{currentModule.title}</Dialog.Title>
                    <Dialog.Description>
                        {currentModule.description}
                    </Dialog.Description>
                </Dialog.Header>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="name" class="text-right col-span-1">Parent</Label>
                    <div class="col-span-3">
                    {#if $repository}
                        <ComboboxAllRecipientsOnRepository recipients={listAllRecipientItemsFromRepository($repository)} bind:selectedItem={parent} />
                    {/if}
                    </div>
                </div>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="name" class="text-right col-span-1">Module name</Label>
                    <Input id="name" placeholder="Module" bind:value={updatedModule.title}  class="col-span-3" />
                </div>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="desc" class="text-right col-span-1">Module description</Label>
                    <Input multiple id="desc" placeholder="Module Description" bind:value={updatedModule.description}  class="col-span-3" />
                </div>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="prefix" class="text-right col-span-1">Prefix</Label>
                    <Input id="prefix" placeholder="PRJ" bind:value={updatedModule.prefix} class="col-span-1" disabled/>
                    <Label for="name" class="text-right col-span-1">Separator</Label>
                    <Input id="name" placeholder="-" bind:value={updatedModule.separator} class="col-span-1"/>
                </div>
            {/if}
            </div>
        {/if}
        <Dialog.Footer>
            <Button variant="destructive" on:click={handleModuleDelete}>
                <Icon icon="gravity-ui:trash-bin" width="15px"/>
                <p class="pl-2">Delete</p>
            </Button>
            <div class="grow"></div>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleModuleUpdate}>Save Changes</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>