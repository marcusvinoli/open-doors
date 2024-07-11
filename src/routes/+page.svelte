<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    
    import Icon from '@iconify/svelte';
    import Button from "$lib/components/ui/button/button.svelte";
    import OpenDoorsLogo from "$lib/assets/open-doors-logo-111725.svg";
    import OpenRepositoryForm from "$lib/components/forms/OpenRepositoryForm.svelte";
    import CreateRepositoryForms from "$lib/components/forms/CreateRepositoryForms.svelte";
    import CloneRepositoryForms from "$lib/components/forms/CloneRepositoryForms.svelte";
    import type { Repository } from "$lib/components/structs/Repo";
    import { goto } from "$app/navigation";
    import { repository, showToolbar } from "./store";
    import { onMount } from "svelte";
    
    let openRepositoryFlag: boolean = false;
    let createRepositoryFlag: boolean = false;
    let cloneRepositoryFlag: boolean = false;

    let repo: Repository = {
        path: "",
        structure: null,
        manifest: {
            name: "",
            remote: null   
        }
    }

    function openRepositoryDialog() {
        openRepositoryFlag = true;
    }

    function createRepositoryDialog() {
        createRepositoryFlag = true;
    }

    function cloneRepositoryDialog() {
        cloneRepositoryFlag = true;
    }

    function saveRepoInformation(repos: any) {
        localStorage.setItem('repository', JSON.stringify(repos));
    }

    function redirectHome() {
        goto("/home")
    }

    async function openRepository(event: any) {
        invoke('read_repo', {path: event.detail.path})
            .then((repos) => {
                openRepositoryFlag = false;
                saveRepoInformation(repos);
                redirectHome();
            })
            .catch((err) => {
                console.log(err);
                openRepositoryFlag = true;
            })
    }

    async function createRepository(event: any) {
        console.log(event.detail.data);
        invoke('create_repo', {repo: event.detail.data})
            .then((repos) => {
                createRepositoryFlag = false;
                saveRepoInformation(repos);
                redirectHome()
            })
            .catch((err) => {
                console.log(err);
                createRepositoryFlag = true;
            })
    }

    async function cloneRepository(event: any) {
        console.log(event.detail);
        invoke('clone_repo', {repo: event.detail.data})
            .then((repos) => {
                createRepositoryFlag = false;
                saveRepoInformation(repos);
                redirectHome()
            })
            .catch((err) => {
                console.log(err);
                createRepositoryFlag = true;
            })
    }

    onMount(() => {
        $showToolbar = false;
    })

</script>

<div class="flex flex-col items-center text-zinc-60 py-20">
    <OpenRepositoryForm bind:openDialog={openRepositoryFlag} path={repo.path} on:open={openRepository}/>
    <CreateRepositoryForms bind:openDialog={createRepositoryFlag} path={repo.path} data={repo.manifest} on:create={createRepository}/>
    <CloneRepositoryForms bind:openDialog={cloneRepositoryFlag} path={repo.path} data={repo.manifest} on:clone={cloneRepository} />
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
