<script lang="ts">   
    import Icon from '@iconify/svelte';
    import Button from "$lib/components/ui/button/button.svelte";
    import OpenDoorsLogo from "$lib/assets/open-doors-logo-111725.svg";
    import OpenRepositoryForm from "$lib/components/forms/OpenRepositoryForm.svelte";
    import CreateRepositoryForms from "$lib/components/forms/CreateRepositoryForms.svelte";
    import CloneRepositoryForms from "$lib/components/forms/CloneRepositoryForms.svelte";
    import type { RepositoryManifest } from "$lib/components/structs/Repo";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { cloneRepository, createRepository, loadRepoInformation, openRepository } from '$lib/controllers/Repository';
    import { clearToolbar } from '$lib/stores/Toolbar';
    
    let openRepositoryFlag: boolean = false;
    let createRepositoryFlag: boolean = false;
    let cloneRepositoryFlag: boolean = false;

    function openRepositoryDialog() {
        openRepositoryFlag = true;
    }

    function createRepositoryDialog() {
        createRepositoryFlag = true;
    }

    function cloneRepositoryDialog() {
        cloneRepositoryFlag = true;
    }

    function redirectHome() {
        goto("/home")
    }

    async function handleOpenRepository(event: any) {
        let path = event.detail.path as string;
        openRepository(path)
            .then(() => {
                redirectHome();
                openRepositoryFlag = false;
            })
    }

    async function handleCloneRepository(event: any) {
        let path = event.detail.path as string;
        let remote = event.detail.remote as string;
        cloneRepository(remote, path)
            .then(() => {
                redirectHome();
                cloneRepositoryFlag = false;
            })
    }

    async function handleCreateRepository(event: any) {
        let path = event.detail.path as string;
        let manifest = event.detail.manifest as RepositoryManifest;
        createRepository(path, manifest)
            .then(() => {
                redirectHome();
                createRepositoryFlag = false;
            })
    }

    onMount(() => {
        clearToolbar();
        if(loadRepoInformation()) {
            //redirectHome();
        }
    })

</script>

<div class="flex flex-col items-center text-zinc-60 py-10">
    <OpenRepositoryForm bind:openDialog={openRepositoryFlag} on:open={handleOpenRepository}/>
    <CreateRepositoryForms bind:openDialog={createRepositoryFlag} on:create={handleCreateRepository}/>
    <CloneRepositoryForms bind:openDialog={cloneRepositoryFlag} on:clone={handleCloneRepository} />
    <div class="w-32 mb-5">
        <img class="object-contain" src={OpenDoorsLogo} alt="OpenDOORs" />
    </div>
    <div class="text-center leading-10 mb-4">
        <h1 class="text-2xl font-bold">Welcome to OpenDOORs!</h1>
        <p class="text-xl">A simple tool that open doors for yours projects.</p>
        <p class="font-semi italic text-[0.95em]">Let's start opening, creating or cloning a repository.</p>
    </div>
    <div class="flex w-[50%] justify-center text-center gap-4">
        <Button on:click={openRepositoryDialog}>
            <Icon class="mr-1" icon="hugeicons:folder-02" width="25px"/> Open
        </Button>
        <Button variant="secondary" on:click={cloneRepositoryDialog}>
            <Icon class="mr-1" icon="hugeicons:folders" width="25px"/> Clone
        </Button>
        <Button variant="secondary" on:click={createRepositoryDialog}>
            <Icon class="mr-1" icon="hugeicons:folder-add" width="25px"/> Create
        </Button>
    </div>
</div>
