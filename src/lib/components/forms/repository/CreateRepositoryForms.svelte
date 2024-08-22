<script lang="ts">
    import Icon from '@iconify/svelte';
    import Loading from '../../ui/loading/Loading.svelte';
    import { open } from '@tauri-apps/api/dialog';
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { createEventDispatcher } from 'svelte';
    
    export let openDialog: boolean = false;
    export let loading: boolean = false;
    let path: string | string[] = "";
    let name: string;
    let remote: string | null = null;

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

    function handleCreate(event: any) {
        loading = true;
        event.stopPropagation();
        dispatch('create', {path: path, name: name, remote: remote})
    }

</script>

<Dialog.Root bind:open={openDialog} closeOnEscape={true}>
    <Dialog.Content class="sm:max-w-[480px]">
        <Dialog.Header>
            <Dialog.Title>Create a New Repository</Dialog.Title>
            <Dialog.Description>
                Let's create a new repository.
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4 py-4 min-h-42">
            {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">Creating a new repository...</h1>
            </div> 
            {:else}
            <div class="grid grid-cols-5 items-center gap-4">
                <Label for="name" class="text-right col-span-1">Location</Label>
                <div class="flex flex-row gap-3 col-span-4">
                    <Input id="name" bind:value={path} class="" autocomplete="off"/>
                    <Button variant="secondary" class="" on:click={selectFolder}>
                        <Icon icon="gravity-ui:folder-magnifier" width="25px"/>
                    </Button>
                </div>
            </div>
            <div class="grid grid-cols-5 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Name</Label>
                <Input id="name" bind:value={name} class="col-span-4" autocomplete="off"/>
            </div>
            <div class="grid grid-cols-5 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Remote URL</Label>
                <Input id="name" bind:value={remote} class="col-span-4" placeholder="*Optional" autocomplete="off"/>
            </div>
            {/if}
        </div>
        <Dialog.Footer>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleCreate} disabled={((path==="")||(name===""))}>Create</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>