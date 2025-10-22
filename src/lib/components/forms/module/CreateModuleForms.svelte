<script lang="ts">
    import Loading from '../../ui/loading/Loading.svelte';
    import TreeItemsComboBox from '../utils/TreeItemsComboBox.svelte';

    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { isValid } from '$lib/utils/name-validator';
    import { repository } from "$lib/stores/Repository.svelte";
    import { createModule } from '$lib/controllers/Module';
    import { listAllContainers } from '$lib/utils/lists';

    import * as Dialog from "$lib/components/ui/dialog/index.js";
    
    import type { Module } from '$lib/components/structs/Module';
    import type { TreeItem } from '$lib/components/structs/Tree';
    import type { ModuleManifest } from "$lib/components/structs/ModuleManifest";

    const placeholder: string = 'Select a repository, project or folder...';
    
    let { 
        openDialog = $bindable(false),
        currentParent,
        oncreatedmodule,
    } : {
        openDialog?: boolean;
        currentParent?: TreeItem | null;
        oncreatedmodule?: ((mod: Module) => void) | null;
    } = $props();

    let repo = $derived(repository());
    let loading: boolean = $state(false);
    let selectedParent: TreeItem | null = $derived(currentParent ?? null);
    let possibleParents: TreeItem[] = $derived.by(() => {
            if (!repo) {
                return []
            } 
            return listAllContainers(repo.tree)
        });

    let moduleManifest: ModuleManifest = $state({
        title:"",
        prefix:"",
        separator: "-",
        description: "",
    });

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    function handleCreateModule() {
        loading = true;

		if(!selectedParent) {
			return;
			// TODO: Insert here an Error Message.
		}

		if (!isValid(moduleManifest.prefix)) {
			return;
			// TODO: Insert here an Error Message.
		}

        loading = true;
        createModule(moduleManifest, selectedParent)
            .then((mod) => {
                let module = mod as Module;
                if (oncreatedmodule) {
                    oncreatedmodule(module);
                }
            })
            .finally(() => {
                closeDialog();
            })
    }

</script>

<Dialog.Root bind:open={openDialog}>
    <Dialog.Content class="sm:max-w-[550px]">
        <Dialog.Header>
            <Dialog.Title>New Module</Dialog.Title>
            <Dialog.Description>
                Let's create a new Objects Module!
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4 py-4 min-h-42">
            {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">Creating a new Module...</h1>
            </div> 
            {:else}
            <div class="grid grid-cols-4 items-center  gap-2">
                <Label for="location" class="text-left col-span-1 ">Location</Label>
                <div class="col-span-3" id="location">
                    <TreeItemsComboBox bind:selectedItem={selectedParent} items={possibleParents} placeholder={placeholder}/>
                </div>
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-left col-span-1">Module name</Label>
                <Input id="name" placeholder="Module" bind:value={moduleManifest.title}  class="col-span-3" />
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="desc" class="text-left col-span-1">Module description</Label>
                <Input multiple id="desc" placeholder="Module Description" bind:value={moduleManifest.description}  class="col-span-3" />
            </div>
            <div class="grid grid-cols-4 items-center gap-2 text-left">
                <Label for="prefix" class="text-left col-span-1">Prefix</Label>
                <Input id="prefix" placeholder="PRJ" bind:value={moduleManifest.prefix} class="col-span-1" />
                <Label for="separator" class="text-left col-span-1">Separator</Label>
                <Input id="separator" placeholder="-" bind:value={moduleManifest.separator} class="col-span-1"/>
            </div>
            <Dialog.Description>
                {#if (moduleManifest.title !== "") && (moduleManifest.prefix !== "") && (parent)}
                Your module will be displayed as <strong>{moduleManifest?.prefix} {moduleManifest?.separator} {moduleManifest?.title}</strong>.
                {/if}
            </Dialog.Description>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" onclick={closeDialog}>Cancel</Button>
            <Button onclick={handleCreateModule} disabled={(moduleManifest.prefix===""||moduleManifest.title==="")}>Create</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
