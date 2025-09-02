<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from '$lib/components/ui/loading/Loading.svelte';
    import TreeItemsComboBox from '$lib/components/forms/utils/TreeItemsComboBox.svelte';

    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { confirm } from '@tauri-apps/api/dialog';
    import { isValid } from "$lib/utils/name-validator";
    import { repository } from "$lib/stores/Repository.svelte";
    import { updateFolder } from "$lib/controllers/Folder";
    import { deleteProject, readProject, updateProject } from "$lib/controllers/Project";
    import { listAllContainersExceptChildren, listRelatives } from "$lib/utils/lists";
    
    import * as path from 'path';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    
    import type { Project } from "$lib/components/structs/Project";
    import type { TreeItem } from "$lib/components/structs/Tree";
    import type { Repository } from "$lib/components/structs/Repo";

    const loadingMessage: string = "Loading project information";
    const deletingFolderMessage: string = "Deleting project...";
    const updatingFolderMessage: string = "Updating project...";

    let { 
        openDialog = $bindable(true), 
        project,
        onprojectupdate,
        onprojectdelete,
    } : {
        openDialog: boolean;
        project: TreeItem;
        onprojectupdate?: (project: TreeItem) => void;
        onprojectdelete?: (project: TreeItem) => void;
    } = $props();

    let repo: Repository = $derived(repository()!);
    let loading: boolean = $state(false);
    let infoMessage: string = $state(loadingMessage);

    let possibleParents: TreeItem[] = $derived.by(() => listAllContainersExceptChildren(repo.tree, project));
    let currentParent: TreeItem = $derived.by(() => listRelatives(repo.tree, repo.tree, project).pop() ?? repo.tree);
    let newParent: TreeItem = $derived.by(() => listRelatives(repo.tree, repo.tree, project).pop() ?? repo.tree);
    
    let projectName: string = $derived.by(() => project.name);
    let isValidName: boolean = $derived(isValid(projectName));

    let currentProject: Project | null = $state(null);
    let updatedProject: Project | null = $state(null);

    function closeDialog() {
        loading = false;
        openDialog = false;
        currentProject = null;
        updatedProject = null;
    }

    async function handleProjectUpdate() {
        loading = true;
        infoMessage = updatingFolderMessage;
        let updated = false;

        if (currentProject!.manifest !== updatedProject!.manifest) {
            currentProject = await updateProject(currentProject!.tree, updatedProject!.manifest);
            if (currentProject) {
                updated = true;
            }
        }

        if (currentParent.path !== newParent.path) {
            const projectFolder = path.basename(currentProject!.tree.path);
            const newPath = path.join(newParent.path, projectFolder);
            currentProject!.tree = await updateFolder(project.path, newPath);
            updated = true;
        }

        if(updated && onprojectupdate) {
            onprojectupdate(currentProject!.tree);
        }

        closeDialog();
    }

    async function handleProjectDelete() {
        const confirmed = await confirm('Do you really want to delete this project? All it content will be delete!', 'Deleting project ' + currentProject!.manifest.name );
        if (!confirmed) {
            return;
        }
        loading = true;
        infoMessage = deletingFolderMessage;
        deleteProject(currentProject!.tree)
            .then(() => {
                if (onprojectdelete) {
                    onprojectdelete(project)
                }
            })
            .finally(() => {
                loading = true;
                closeDialog();
            });
    }
    
    $effect(() => {
        if (project.itemType !== 'project') {
            return;
        }
        if (!openDialog) {
            return;
        }
        loading = true;
        infoMessage = loadingMessage;
        readProject(project)
            .then((prj) => {
                currentProject = prj as Project;
                updatedProject = structuredClone(prj as Project);
            })
            .finally(() => {
                loading = false;
            });
    })
    
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
                {#if currentProject && updatedProject}
                    <Dialog.Header>
                        <Dialog.Title>{currentProject.manifest.name}</Dialog.Title>
                        <Dialog.Description>{currentProject.manifest.prefix}</Dialog.Description>
                    </Dialog.Header>
                    <div class="grid grid-cols-4 items-center gap-2">
                        <Label for="name" class="text-right col-span-1">Parent</Label>
                        <div class="col-span-3">
                            <TreeItemsComboBox items={possibleParents} bind:selectedItem={newParent} />
                        </div>
                    </div>
                    <div class="grid grid-cols-4 items-center gap-2">
                        <Label for="name" class="text-right col-span-1">Project Name</Label>
                        <Input id="name" placeholder="My Awesome Project" bind:value={updatedProject.manifest.name}  class="col-span-3" autocomplete="off"/>
                    </div>
                    <div class="grid grid-cols-4 items-center gap-2">
                        <Label for="prefix" class="text-right col-span-1">Prefix</Label>
                        <Input id="prefix" placeholder="PRJ" bind:value={updatedProject.manifest.prefix} class="col-span-1" disabled/>
                        <Label for="name" class="text-right col-span-1">Separator</Label>
                        <Input id="name" placeholder="-" bind:value={updatedProject.manifest.separator} class="col-span-1" autocomplete="off"/>
                    </div>
                {/if}
            </div>
            <Dialog.Footer>
                <Button variant="destructive" onclick={handleProjectDelete}>
                    <Icon icon="gravity-ui:trash-bin" width="15px"/>
                    <p class="pl-2">Delete</p>
                </Button>
                <div class="grow"></div>
                <Button variant="secondary" onclick={closeDialog}>Cancel</Button>
                {#if updatedProject}
                    <Button onclick={handleProjectUpdate} disabled={(!isValidName)}>Save Changes</Button>
                {/if}
            </Dialog.Footer>
        {/if}
    </Dialog.Content>
</Dialog.Root>