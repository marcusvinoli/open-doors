<script lang="ts">
    import Loading from '../../ui/loading/Loading.svelte';
    import TreeItemsComboBox from '../utils/TreeItemsComboBox.svelte';

    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { isValid } from '$lib/utils/name-validator';
    import { repository } from '$lib/stores/Repository.svelte';
    import { createProject } from '$lib/controllers/Project';
    import { reloadRepository } from "$lib/controllers/Repository";
    import { listAllContainers } from '$lib/utils/lists';

    import * as Dialog from "$lib/components/ui/dialog/index.js";
    
    import type { TreeItem } from '../../structs/Tree';
    import type { Project, ProjectManifest } from "$lib/components/structs/Project";
    
    const placeholder = 'Select a repository, project or folder...';

    let { 
        openDialog = $bindable(false),
        currentParent,
        oncreateproject,
    } : {
        openDialog?: boolean;
        currentParent?: TreeItem | null;
        oncreateproject?: ((mod: Project) => void) | null;
    } = $props();

    let repo = $derived(repository());
    let loading: boolean = $state(false);
    let selectedParent: TreeItem | null = $derived(currentParent ?? null);
    let possibleParents = $derived(listAllContainers(repo?.tree, true));

    let projectManifest: ProjectManifest = $state({
        name:"",
        separator: "-",
        prefix:"",
    });

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    function handleCreateProject() {
        loading = true;
        
        if(!selectedParent) {
            return;
            // TODO: Insert here an Error Message.
        }

        if (!isValid(projectManifest.prefix)) {
            return;
            // TODO: Insert here an Error Message.
        }

        createProject(projectManifest, selectedParent)
            .then((prj) => {
                if (oncreateproject) {
                    oncreateproject(prj as Project);
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
            <Dialog.Title>Create a New Project</Dialog.Title>
            <Dialog.Description>
                Let's create a new project!
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4 py-4 min-h-42">
            {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">Creating a new Project...</h1>
            </div> 
            {:else}
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Create Here</Label>
                <div class="col-span-3">
                    <TreeItemsComboBox items={possibleParents} bind:selectedItem={selectedParent} placeholder={placeholder}/>
                </div>
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Project Name</Label>
                <Input id="name" placeholder="My Awesome Project" bind:value={projectManifest.name}  class="col-span-3" autocomplete="off"/>
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="prefix" class="text-right col-span-1">Prefix</Label>
                <Input id="prefix" placeholder="PRJ" bind:value={projectManifest.prefix} class="col-span-1" autocomplete="off"/>
                <Label for="name" class="text-right col-span-1">Separator</Label>
                <Input id="name" placeholder="-" bind:value={projectManifest.separator} class="col-span-1" autocomplete="off"/>
            </div>
            <Dialog.Description>
                {#if (projectManifest.name !== "") && (projectManifest.prefix !== "")}
                Your project will be displayed as <strong>{projectManifest?.prefix} {projectManifest?.separator} {projectManifest?.name}</strong>.
                {/if}
            </Dialog.Description>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" onclick={closeDialog}>Cancel</Button>
            <Button onclick={handleCreateProject} disabled={((projectManifest.name==="")||(projectManifest.prefix===""))}>Create</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>