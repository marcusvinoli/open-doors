<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from '../../ui/loading/Loading.svelte';
    import TreeItemsComboBox from "../utils/TreeItemsComboBox.svelte";

    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { confirm } from '@tauri-apps/api/dialog';
    import { isValid } from "$lib/utils/name-validator";
    import { repository } from "$lib/stores/Repository.svelte";
    import { updateFolder } from "$lib/controllers/Folder";
    import { listRelatives } from '$lib/utils/lists';
    import { listAllContainers } from '$lib/utils/lists';
    import { deleteModule, readModule, updateModuleManifest } from '$lib/controllers/Module';

    import * as path from 'path';
    import * as Dialog from "$lib/components/ui/dialog/index.js";

    import type { Module } from "$lib/components/structs/Module";
    import type { TreeItem } from "$lib/components/structs/Tree";
    import type { Repository } from "$lib/components/structs/Repo";

    const loadingMessage: string = "Loading module information";
    const deletingModuleMessage: string = "Deleting module...";
    const updatingModuleMessage: string = "Updating module...";

    let { 
        openDialog = $bindable(false), 
        moduleItem,
        onmoduleupdate,
        onmoduledelete, 
    } : {
        openDialog?: boolean;
        moduleItem: TreeItem;
        onmoduleupdate?: (mod: TreeItem) => void;
        onmoduledelete?: (mod: TreeItem) => void;
    } =  $props();

    let repo: Repository = $derived(repository()!);
    let loading: boolean = $state(false);
    let infoMessage: string = $state(loadingMessage);

    let possibleParents: TreeItem[] = $derived.by(() => listAllContainers(repo.tree, true));
    let currentParent: TreeItem = $derived.by(() => listRelatives(repo.tree, repo.tree, moduleItem).pop() ?? repo.tree);
    let newParent: TreeItem = $derived.by(() => listRelatives(repo.tree, repo.tree, moduleItem).pop() ?? repo.tree);
    
    let currentModule: Module | null = $state(null);
    let updatedModule: Module | null = $state(null);
    let isValidName: boolean = $derived.by(() => {
        if (!updatedModule) {
            return false;
        }
        return isValid(updatedModule?.manifest.title);
    });
    
    function closeDialog() {
        loading = false;
        openDialog = false;
        currentModule = null;
        updatedModule = null;
    }

    async function handleModuleUpdate() {
        infoMessage = updatingModuleMessage;
        let retModule: TreeItem = {...moduleItem};
        loading = true;

        let updated = false;

        if (!updatedModule) {
            return;
        }

        if (currentModule !== updatedModule) {
            currentModule = await updateModuleManifest(moduleItem, updatedModule.manifest);
            updated = true;
        }

        if (currentParent.path !== newParent.path) {
            const moduleFolder = path.basename(moduleItem.path);
            const newPath = path.join(newParent.path, moduleFolder);
            retModule = await updateFolder(moduleItem.path, newPath);
            updated = true;
        }

        if (updated && onmoduleupdate) {
            onmoduleupdate(retModule);
        }

        closeDialog();
    }

    async function handleModuleDelete() {
        const confirmed = await confirm('Do you really want to delete this Module? All its content will be delete!', 'Deleting module ' + currentModule?.manifest.title );
        
        if (!confirmed) {
            return;
        }

        loading = true;
        infoMessage = deletingModuleMessage;

        deleteModule(moduleItem)
            .then(() => {
                
            })
            .finally(() => {
                closeDialog();
            })
    }
    
    $effect(() => {
        if (moduleItem.itemType !== 'module') {
            return;
        }
        if (!openDialog) {
            return;
        }
        loading = true;
        infoMessage = loadingMessage;
        readModule(moduleItem)
            .then((mod) => {
                currentModule = mod as Module;
                updatedModule = structuredClone(mod as Module);
            })
            .finally(() => {
                loading = false;
            })
    });
    
</script>

<Dialog.Root bind:open={openDialog}>
    <Dialog.Content class="sm:max-w-[520px]">
        {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">{infoMessage}</h1>
            </div> 
        {:else}
            <div class="grid gap-4 py-4 min-h-42">
            {#if currentModule && updatedModule}
                <Dialog.Header>
                    <Dialog.Title>{currentModule.manifest.title}</Dialog.Title>
                    <Dialog.Description>
                        {currentModule.manifest.description}
                    </Dialog.Description>
                </Dialog.Header>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="name" class="text-right col-span-1">Parent</Label>
                    <div class="col-span-3">
                        <TreeItemsComboBox items={possibleParents} bind:selectedItem={newParent} />
                    </div>
                </div>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="name" class="text-right col-span-1">Module name</Label>
                    <Input id="name" placeholder="Module" bind:value={updatedModule.manifest.title}  class="col-span-3" />
                </div>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="desc" class="text-right col-span-1">Module description</Label>
                    <Input multiple id="desc" placeholder="Module Description" bind:value={updatedModule.manifest.description}  class="col-span-3" autocomplete="off"/>
                </div>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="prefix" class="text-right col-span-1">Prefix</Label>
                    <Input id="prefix" placeholder="PRJ" bind:value={updatedModule.manifest.prefix} class="col-span-1" disabled/>
                    <Label for="name" class="text-right col-span-1">Separator</Label>
                    <Input id="name" placeholder="-" bind:value={updatedModule.manifest.separator} class="col-span-1" autocomplete="off"/>
                </div>
            {/if}
            </div>
        {/if}
        <Dialog.Footer>
            <Button variant="destructive" onclick={handleModuleDelete}>
                <Icon icon="gravity-ui:trash-bin" width="15px"/>
                <p class="pl-2">Delete</p>
            </Button>
            <div class="grow"></div>
            <Button variant="secondary" onclick={closeDialog}>Cancel</Button>
            <Button onclick={handleModuleUpdate} disabled={!isValidName}>Save Changes</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>