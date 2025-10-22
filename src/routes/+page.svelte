<script lang="ts">   
    import Icon from '@iconify/svelte';
    import Button from "$lib/components/ui/button/button.svelte";
    import OpenDoorsLogo from "$lib/components/global/logo/OpenDoorsLogo.svelte";
    import OpenRepositoryForm from "$lib/components/forms/repository/OpenRepositoryForm.svelte";
    import CloneRepositoryForms from "$lib/components/forms/repository/CloneRepositoryForms.svelte";
    import CreateRepositoryForms from "$lib/components/forms/repository/CreateRepositoryForms.svelte";

    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { clearTabs } from '$lib/stores/Tabs.svelte';
    import { clearToolbar } from '$lib/stores/Toolbar.svelte';
    import { clearAuthorInformation } from '$lib/controllers/User';
    import { clearRepositoryInformation, cloneRepository, createRepository, loadRepository, openRepository } from '$lib/controllers/Repository';
    
    let openRepositoryFlag: boolean = $state(false);
    let cloneRepositoryFlag: boolean = $state(false);
    let createRepositoryFlag: boolean = $state(false);

    function openRepositoryDialog() {
        openRepositoryFlag = true;
    }
    
    function cloneRepositoryDialog() {
        cloneRepositoryFlag = true;
    }
    
    function createRepositoryDialog() {
        createRepositoryFlag = true;
    }

    function redirectHome() {
        clearTabs();
        goto("/home")
    }

    async function handleOpenRepository(event: any) {
        let path = event.detail.path as string;
        openRepository(path)
            .then(() => {
                openRepositoryFlag = false;
                redirectHome();
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
        clearTabs();
        clearToolbar();
        clearAuthorInformation();
        clearRepositoryInformation();
    })

</script>

<div class="h-full flex flex-col items-center justify-center text-slate-500 pb-20">
    <OpenRepositoryForm bind:openDialog={openRepositoryFlag} on:open={handleOpenRepository}/>
    <CloneRepositoryForms bind:openDialog={cloneRepositoryFlag} on:clone={handleCloneRepository} />
    <CreateRepositoryForms bind:openDialog={createRepositoryFlag} on:create={handleCreateRepository}/>
    <div class="flex flex-col items-center justify-center text-center">
        <OpenDoorsLogo class="p-1"/>
        <p class="font-bold">WELCOME TO OPEN-DOORS!</p>
        <p class="items-center">A simple requirement management tool for your projects.</p>
    </div>
    <div class="flex w-[50%] justify-center text-center gap-4 py-4">
        <Button onclick={openRepositoryDialog} class="px-5 bg-slate-500 hover:bg-slate-600">
            <Icon class="mr-1" icon="bi:database-add" width="25px"/> Open
        </Button>
        <Button variant="secondary" onclick={cloneRepositoryDialog} class="px-5 text-slate-500 hover:bg-slate-300 hover:text-slate-600">
            <Icon class="mr-1" icon="bi:database-down" width="25px"/> Clone
        </Button>
        <Button variant="secondary" onclick={createRepositoryDialog} class="px-5 text-slate-500 hover:bg-slate-300 hover:text-slate-600">
            <Icon class="mr-1" icon="bi:database-gear" width="25px"/> Create
        </Button>
    </div>
</div>
