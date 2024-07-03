<script lang="ts">
    import OpenDoorsLogo from "$lib/assets/open-doors-logo-111725.svg";
    
    import Icon from '@iconify/svelte';
    import Button from "$lib/components/ui/button/button.svelte";
    import OpenRepositoryForm from "$lib/components/forms/OpenRepositoryForm.svelte";
    import CreateRepositoryForms from "$lib/components/forms/CreateRepositoryForms.svelte";
    import CloneRepositoryForms from "$lib/components/forms/CloneRepositoryForms.svelte";
    import type { OpenRepositoryData } from "$lib/components/forms/OpenRepository";
    import type { CreateRepositoryData } from "$lib/components/forms/CreateRepository";
    import type { CloneRepositoryData } from "$lib/components/forms/CloneRepository";
        
    let openRepositoryFlag: boolean = false;
    let openRepositoryData: OpenRepositoryData = { path: "" };
    
    function openRepositoryDialog() {
        openRepositoryFlag = true;
    }
    function openRepository(event: any) {
        console.log(event.detail);
        openRepositoryFlag = false;
    }

    let createRepositoryFlag: boolean = false;
    let createRepositoryData: CreateRepositoryData = { name: "", path: "", remote: null, error: null };

    function createRepositoryDialog() {
        createRepositoryFlag = true;
    }

    function createRepository(event: any) {
        console.log(event.detail);
        createRepositoryFlag = false;
    }

    let cloneRepositoryFlag: boolean = false;
    let cloneRepositoryData: CloneRepositoryData = { path: "", remote: "", error: null };

    function cloneRepositoryDialog() {
        cloneRepositoryFlag = true;
    }

    function cloneRepository(event: any) {
        console.log(event.detail);
        cloneRepositoryFlag = false;
    }
</script>

<div class="flex flex-col items-center text-zinc-60 py-20">
    <OpenRepositoryForm bind:openDialog={openRepositoryFlag} data={openRepositoryData} on:open={openRepository}/>
    <CreateRepositoryForms bind:openDialog={createRepositoryFlag} data={createRepositoryData} on:create={createRepository}/>
    <CloneRepositoryForms bind:openDialog={cloneRepositoryFlag} data={cloneRepositoryData} on:clone={cloneRepository} />
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
