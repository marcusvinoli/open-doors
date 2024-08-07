<script lang="ts">   
    import Icon from '@iconify/svelte';
    import Button from "$lib/components/ui/button/button.svelte";
    import OpenDoorsLogo from "$lib/assets/open-doors-logo-optimized.svg";
    import OpenRepositoryForm from "$lib/components/forms/repository/OpenRepositoryForm.svelte";
    import CreateRepositoryForms from "$lib/components/forms/repository/CreateRepositoryForms.svelte";
    import CloneRepositoryForms from "$lib/components/forms/repository/CloneRepositoryForms.svelte";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { cloneRepository, createRepository, loadRepository, openRepository } from '$lib/controllers/Repository';
    import { clearToolbar } from '$lib/stores/Toolbar';
    import { clearTabs } from '$lib/stores/Tabs';
    
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
        clearTabs();
        goto("/home")
    }

    async function handleOpenRepository(event: any) {
        let path = event.detail.path as string;
        openRepository(path)
            .then(() => {
                redirectHome();
                openRepositoryFlag = false;
            })
            .catch((err) => {
                console.log(err);
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
        let name = event.detail.name as string;
        let remote = event.detail.remote as string | null;
        createRepository(path, name, remote)
            .then(() => {
                redirectHome();
                createRepositoryFlag = false;
            })
    }

    onMount(() => {
        clearToolbar();
        if(loadRepository()) {
            //redirectHome();
        }
    })

</script>

<div class="flex flex-col items-center text-zinc-60 py-10">
    <OpenRepositoryForm bind:openDialog={openRepositoryFlag} on:open={handleOpenRepository}/>
    <CreateRepositoryForms bind:openDialog={createRepositoryFlag} on:create={handleCreateRepository}/>
    <CloneRepositoryForms bind:openDialog={cloneRepositoryFlag} on:clone={handleCloneRepository} />
    <div class="p-5">
        <img class="object-contain p-2 w-[480px]" src={OpenDoorsLogo} alt="OpenDOORs"/>
    </div>
    <div class="text-center leading-10 mb-4">
        <h1 class="text-2xl font-bold p-2">Welcome to OpenDOORS!</h1>
        <p class="text-xl">A simple tool that open doors for yours projects.</p>
        <p class="font-semi italic text-[0.95em]">Let's start opening, creating or cloning a repository.</p>
    </div>
    <div class="flex w-[50%] justify-center text-center gap-4">
        <Button on:click={openRepositoryDialog} class="px-5">
            <Icon class="mr-1" icon="bi:database-add" width="25px"/> Open
        </Button>
        <Button variant="secondary" on:click={cloneRepositoryDialog} class="px-5">
            <Icon class="mr-1" icon="bi:database-down" width="25px"/> Clone
        </Button>
        <Button variant="secondary" on:click={createRepositoryDialog} class="px-5">
            <Icon class="mr-1" icon="bi:database-gear" width="25px"/> Create
        </Button>
    </div>
</div>
