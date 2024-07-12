<script lang="ts">
    import Icon from '@iconify/svelte';
    import Loading from '../ui/loading/Loading.svelte';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { open } from '@tauri-apps/api/dialog';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import { createEventDispatcher } from 'svelte';
    import type { RepositoryManifest } from "$lib/components/structs/Repo";
    
    export let openDialog: boolean = false;
    export let loading: boolean = false;
    let remote: string = "";
    let path: string | string[] = "";

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    async function selectFolder() {
        const folder = await open({
            directory: true,
            multiple: false
        });
        if (folder) {
            path = folder as string;
        }
    }

    const dispatch = createEventDispatcher();

    function handleClone(event: any) {
        loading = true;
        event.stopPropagation();
        dispatch('clone', {remote: remote, path: path});
    }
</script>

<Dialog.Root bind:open={openDialog} closeOnEscape={true}>
    <Dialog.Content class="sm:max-w-[450px]">
        <Dialog.Header>
            <Dialog.Title>Clone a Repository</Dialog.Title>
            <Dialog.Description>
                Clone a remote repository.
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4 py-4 min-h-42">
            {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">Cloning repository...</h1>
            </div>
            {:else}
            <div class="grid grid-cols-5 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Remote URL</Label>
                <Input id="name" bind:value={remote} class="col-span-4" />
            </div>
            <div class="grid grid-cols-5 items-center gap-4">
                <Label for="name" class="text-right col-span-1">Location</Label>
                <div class="flex flex-row gap-3 col-span-4">
                    <Input id="name" bind:value={path} class="" />
                    <Button variant="secondary" class="" on:click={selectFolder}>
                        <Icon icon="gravity-ui:folder-magnifier" width="25px"/>
                    </Button>
                </div>
            </div>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleClone}>Clone</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>