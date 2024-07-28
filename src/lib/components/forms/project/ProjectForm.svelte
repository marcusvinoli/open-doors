<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from '../../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from '../utils/ComboboxAllRecipientsOnRepository.svelte';
    import { confirm } from '@tauri-apps/api/dialog';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from "$lib/stores/Repository";
    import { updateFolder } from "$lib/controllers/Folder";
    import { reloadRepository } from "$lib/controllers/Repository";
    import { deleteProject, readProject, updateProject } from "$lib/controllers/Project";
    import { listAllRecipientsExceptChildren, listRelatives } from "$lib/utils/lists";
    import * as path from 'path';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { Project } from "$lib/components/structs/Project";
    import type { TreeItem } from "../../structs/Tree";
    import { createEventDispatcher } from "svelte";
    
    const dispatch = createEventDispatcher();

    export let openDialog: boolean = true;
    export let project: TreeItem;

    let loading: boolean = false;
    let loadingMessage: string = "Reading project information...";

    let parent: TreeItem;
    let possibleParents: TreeItem[];

    let currentParent: TreeItem;

    let currentProject: Project;
    let updatedProject: Project;

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    async function handleProjectUpdate() {
        loadingMessage = "Updating folder...";
        loading = true;
        let updated = false;

        if((parent) && (currentParent) && (project) && (currentProject)){
            if (currentProject.manifest !== updatedProject.manifest) {
                await updateProject(currentProject.tree, updatedProject.manifest)
                currentProject = await readProject(currentProject.tree);
            }

            if (currentParent.path !== parent.path) {
                const projectFolder = path.basename(currentProject.tree.path);
                const newPath = path.join(parent.path, projectFolder);
                await updateFolder(currentProject.tree.path, newPath);
            }
        }

        if(updated) {
            dispatch('updated', {project: currentProject});
            reloadRepository();
        }

        closeDialog();
    }

    async function handleProjectDelete() {
        const confirmed = await confirm('Do you really want to delete this project? All it content will be delete!', 'Deleting project ' + currentProject.manifest.name );
        if (!confirmed) {
            return;
        }
        loading = true;
        deleteProject(currentProject.tree)
            .then(() => {
                reloadRepository();
                dispatch('deleted', {project: project});
            })
            .finally(() => {
                closeDialog();
            })
    }
    
    async function loadData() {
        console.log("Loading data...")
        if ($repository && (project.itemType === "project")) {
            let retProject = await readProject(project);
            currentProject = retProject;
            updatedProject = JSON.parse(JSON.stringify(currentProject));
            possibleParents = listAllRecipientsExceptChildren($repository!.tree, currentProject.tree);
            parent = listRelatives($repository?.tree, $repository?.tree, currentProject?.tree).pop()??$repository?.tree;
            currentParent = parent;
            loading = false;
        }
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
                {#if updatedProject}
                <Dialog.Header>
                    <Dialog.Title>{currentProject.manifest.name}</Dialog.Title>
                    <Dialog.Description>{currentProject.manifest.prefix}</Dialog.Description>
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
                        <Label for="name" class="text-right col-span-1">Project Name</Label>
                        <Input id="name" placeholder="My Awesome Project" bind:value={updatedProject.manifest.name}  class="col-span-3" />
                    </div>
                    <div class="grid grid-cols-4 items-center gap-2">
                        <Label for="prefix" class="text-right col-span-1">Prefix</Label>
                        <Input id="prefix" placeholder="PRJ" bind:value={updatedProject.manifest.prefix} class="col-span-1" disabled/>
                        <Label for="name" class="text-right col-span-1">Separator</Label>
                        <Input id="name" placeholder="-" bind:value={updatedProject.manifest.separator} class="col-span-1"/>
                    </div>
                {/if}
            </div>
            <Dialog.Footer>
                <Button variant="destructive" on:click={handleProjectDelete}>
                    <Icon icon="gravity-ui:trash-bin" width="15px"/>
                    <p class="pl-2">Delete</p>
                </Button>
                <div class="grow"></div>
                <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
                <Button on:click={handleProjectUpdate}>Save Changes</Button>
            </Dialog.Footer>
        {/if}
    </Dialog.Content>
</Dialog.Root>