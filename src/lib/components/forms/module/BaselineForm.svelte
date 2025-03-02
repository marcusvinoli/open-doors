<script lang="ts">
    import Loading from '../../ui/loading/Loading.svelte';
    import { Input } from "$lib/components/ui/input/index.js";
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { repository } from "$lib/stores/Repository";
    import { createBaseline, readModuleFromPath } from '$lib/controllers/Module';
    import { createEventDispatcher } from 'svelte';
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { TreeItem } from '../../structs/Tree';
    import type { Baseline, Module } from "$lib/components/structs/Module";

    const dispatch = createEventDispatcher();

    export let openDialog: boolean = false;
    export let modulePath: string;

    let loading: boolean = false;
    let loadingMessage: string = "Reading module information...";
    
    let newBaseline: Baseline;
    let currentBaseline: Baseline | null;
    let baselines: Baseline[];
    let currentModule: Module;
    let selectedType: string = "major";
    let fixString: string = "";
    let errorMessage: string | null = null;

    function closeDialog() {
        loading = false;
        openDialog = false;
    }

    async function loadData() {
        if ($repository) {
            currentModule = (await readModuleFromPath(modulePath)) as unknown as Module;
            baselines = JSON.parse(JSON.stringify(currentModule.baselines));
            currentBaseline = baselines[0];
            newBaseline = JSON.parse(JSON.stringify(currentBaseline)) ;
            newBaseline.version = newMajor(newBaseline.version);
            loading = false;
        }
    }

    function newMinor(version: string) {
        let values = version.split(".");
        values[1] = (parseInt(values[1]) + 1).toString();
        values[2] = "0";
        return values.join(".");
    }

    function newMajor(version: string) {
        let values = version.split(".");
        values[0] = (parseInt(values[0]) + 1).toString();
        values[1] = "0";
        values[2] = "0";
        return values.join(".");
    }

    function newFix(version: string) {
        let values = version.split(".");
        values[2] = "";
        return values.join(".");
    }

    function isUnique(version: string) {
        currentModule.baselines.forEach(bl => {
            if (bl.version === version) {
                return false;
            }
        })
        return true;
    }

    async function handleCreateBaseline() {
        if (newBaseline && currentBaseline) {
            if (selectedType == "major") {
                newBaseline.version = newMajor(currentBaseline.version);
            } else if (selectedType == "minor") {
                newBaseline.version = newMinor(currentBaseline.version);
            } else {
                newBaseline.version = newFix(currentBaseline.version) + fixString;
            }
            loading = true;
            console.log(newBaseline)
            createBaseline(currentModule.path, newBaseline)
                .then(() => {
                    loading = false;
                    openDialog = false;
                })
                .catch(e => {
                    errorMessage = e;
                })
            console.log(newBaseline);
        }
    }

    $: if (openDialog) {
        loadData();
        console.log("Loading...");
    }
    
</script>

<Dialog.Root bind:open={openDialog} closeOnEscape closeOnOutsideClick>
    <Dialog.Content class="sm:max-w-[520px]">
        {#if loading}
            <div class="flex flex-col items-center">
                <Loading />
                <h1 class="leading-1 pt-1 my-2">{loadingMessage}</h1>
            </div> 
        {:else}
            <div class="grid gap-4 py-4 min-h-42">
            {#if currentBaseline}
                <Dialog.Header>
                    <Dialog.Title>
                        New Baseline for {currentModule.manifest.prefix}
                    </Dialog.Title>
                    <Dialog.Description>
                        {currentModule.manifest.description}
                    </Dialog.Description>
                </Dialog.Header>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="name" class="text-right col-span-1">Version</Label>
                    <RadioGroup.Root bind:value={selectedType} class="col-span-3 grid grid-cols-4">
                        <div class="flex items-center space-x-2 col-span-1">
                          <RadioGroup.Item value="major" id="major-opt" />
                          <Label for="major-opt">{newMajor(currentBaseline.version)}</Label>
                        </div>
                        <div class="flex items-center space-x-2 col-span-1">
                          <RadioGroup.Item value="minor" id="minor-opt" />
                          <Label for="minor-opt">{newMinor(currentBaseline.version)}</Label>
                        </div>
                        <div class="flex items-center space-x-2 col-span-2">
                          <RadioGroup.Item value="fix" id="fix-opt" />
                          <Label for="fix-opt" class="flex items-center">
                            {newFix(currentBaseline.version)}
                            <Input multiple id="desc" bind:value={fixString} placeholder="bugfix" class="py-1"/>
                          </Label>
                        </div>
                    </RadioGroup.Root>
                </div>
                <div class="grid grid-cols-4 items-center gap-2">
                    <Label for="desc" class="text-right col-span-1">Description</Label>
                    <Input multiple id="desc" bind:value={newBaseline.description}  class="col-span-3" />
                </div>
            {/if}
            </div>
        {/if}
        {#if errorMessage}
            <div class="rounded bg-red-200">
                {errorMessage}
            </div>
        {/if}
        <Dialog.Footer>
            <div class="grow"></div>
            <Button variant="secondary" on:click={closeDialog}>Cancel</Button>
            <Button on:click={handleCreateBaseline}>Create Baseline</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>