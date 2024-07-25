<script lang="ts">
    import Icon from "@iconify/svelte";
    import Loading from '../ui/loading/Loading.svelte';
    import ComboboxAllRecipientsOnRepository from './ComboboxAllRecipientsOnRepository.svelte';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from "../../../routes/store";
    import { afterUpdate, beforeUpdate, createEventDispatcher, onMount } from 'svelte';
    import { listAllRecipients, listRelatives } from "$lib/utils/lists";
    import { readProject } from "$lib/controllers/Project";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { Project } from "$lib/components/structs/Project";
    import type { TreeItem } from "../structs/Tree";

    export let openDialog: boolean = true;
    export let project: TreeItem;

    let loading: boolean = false;
    let loadingMessage: "Reading project information...";

    let parent: TreeItem;
    let possibleParents: TreeItem[];
    let prj: Project;

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    function handleUpdateProject() {

    }
    
    function loadData() {
        readProject($repository!, project)
            .then((retProject) => {
                    prj = retProject as Project;
                    possibleParents = listAllRecipients($repository!);
                    parent = listRelatives($repository?.structure, $repository?.structure, prj?.tree).pop()??$repository?.structure;
                    console.log("Parent: ", parent);
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
                {#if prj}
                <Dialog.Header>
                    <Dialog.Title>{prj.manifest.name}</Dialog.Title>
                    <Dialog.Description>{prj.manifest.prefix}</Dialog.Description>
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
                        <Label for="name" class="text-right col-span-1">Project Name</Label>
                        <Input id="name" placeholder="My Awesome Project" bind:value={prj.manifest.name}  class="col-span-3" />
                    </div>
                    <div class="grid grid-cols-4 items-center gap-2">
                        <Label for="prefix" class="text-right col-span-1">Prefix</Label>
                        <Input id="prefix" placeholder="PRJ" bind:value={prj.manifest.prefix} class="col-span-1" disabled/>
                        <Label for="name" class="text-right col-span-1">Separator</Label>
                        <Input id="name" placeholder="-" bind:value={prj.manifest.separator} class="col-span-1"/>
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