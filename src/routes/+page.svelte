<script lang="ts">
    //import { open } from "@tauri-apps/cli";

    import * as Dialog from "$lib/components/ui/dialog";

    import OpenDoorsLogo from "$lib/assets/open-doors-logo-71717a.svg";
    import Button from "$lib/components/ui/button/button.svelte";

    let selectedFolder = "";

    /* async function selectFolder() {
        const folder = await open({
            directory: true,
            multiple: false
        });
        if (folder) {
            selectedFolder = folder;
        }
    } */
    
</script>

<div class="flex flex-col items-center justify-center w-full h-full pb-48 text-zinc-600">
    <div class="w-[180px] mb-6">
        <img src={OpenDoorsLogo} alt="OpenDOORs" />
    </div>
    <div class="text-center leading-10 mb-4">
        <h1 class="text-2xl font-bold mb-1">Welcome to OpenDOORs.</h1>
        <p class="text-xl">A simple tool that open doors for yours projects.</p>
        <p class="font-semi italic text-[0.95em]">Let's start opening, creating or cloning a repository.</p>
    </div>
    <div>
        <div class="w-96 flex gap-5 justify-center">
            <Dialog.Root>
                <Dialog.Trigger><Button>Open</Button></Dialog.Trigger>
                <Dialog.Content>
                    <Dialog.Header>
                        <Dialog.Title>Open</Dialog.Title>
                        <Dialog.Description>
                        </Dialog.Description>
                    </Dialog.Header>
                </Dialog.Content>
            </Dialog.Root>
            <Dialog.Root>
                <Dialog.Trigger><Button variant="secondary">Clone</Button></Dialog.Trigger>
                <Dialog.Content>
                    <Dialog.Header>
                        <Dialog.Title>Open</Dialog.Title>
                        <Dialog.Description>
                            <p>Insert the Git URL of the Repository and select a local folder. Then select a name for this repository.</p>
                        </Dialog.Description>
                    </Dialog.Header>
                </Dialog.Content>
            </Dialog.Root>
            <Dialog.Root>
                <Dialog.Trigger><Button variant="secondary">Clone</Button></Dialog.Trigger>
                <Dialog.Content>
                    <Dialog.Header>
                        <Dialog.Title>Open</Dialog.Title>
                        <Dialog.Description>
                        </Dialog.Description>
                    </Dialog.Header>
                </Dialog.Content>
            </Dialog.Root>
                
            
        </div>
    </div>
</div>
