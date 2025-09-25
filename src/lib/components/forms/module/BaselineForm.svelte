<script lang="ts">
    import { user } from "$lib/stores/User.svelte";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as RadioGroup from "$lib/components/ui/radio-group";
    
    import type { Module } from "$lib/components/structs/Module";
    import type { Baseline } from "$lib/components/structs/Baseline";
    import type { BaselineStatus } from "$lib/components/structs/BaselineStatus";

    let { 
        openDialog = $bindable(false), 
        module,
        onbaselinecreation,
    } : {
        openDialog: boolean;
        module: Module;
        onbaselinecreation?: (baseline: Baseline) => void,
    } = $props();

    let baselines: Baseline[] = $derived([...module.baselines].reverse());
    let currentBaseline: Baseline = $derived.by(() => {
        if (module.baselines?.length ?? 0 > 0) {
            const bl: Baseline = {...baselines[0]};
            bl.hash = null;
            bl.status = 'workInProgress';
            return bl;
        }
        const bl : Baseline = {
            version: '0.0.0',
            description: '',
            status: 'workInProgress',
            createdAt: new Date(),
            createdBy: user()?.toString(),
            deletedAt: null,
            deletedBy: null,
            hash: null,
        };
        return bl;
    });
    
    let newBaselineDescription: string = $state("");
    let newBaselineSelectedType: string = $state("major");
    let errorMessage: string | null = $state(null);

    function closeDialog() {
        openDialog = false;
    }

    function newMinor(version: string) {
        let values = [...version.split(".")];
        values[1] = (parseInt(values[1]) + 1).toString();
        values[2] = "0";
        return values.join(".");
    }

    function newMajor(version: string) {
        let values = [...version.split(".")];
        values[0] = (parseInt(values[0]) + 1).toString();
        values[1] = "0";
        values[2] = "0";
        return values.join(".");
    }

    function newFix(version: string) {
        let values = [...version.split(".")];
        values[2] = (parseInt(values[2]) + 1).toString();
        return values.join(".");
    }

    function isUnique(version: string) {
        let result = true;
        baselines.forEach(bl => {
            if (bl.version === version) {
                result = false;
            }
        })
        return result;
    }

    function generateVersion() : string {
        if (newBaselineSelectedType === "major") {
            return newMajor(currentBaseline.version);
        } else if (newBaselineSelectedType === "minor") {
            return newMinor(currentBaseline.version);
        } else {
            return newFix(currentBaseline.version);
        }
    }

    function handleCreateBaseline() {
        const newBaseline: Baseline = {
            version: generateVersion(),
            description: newBaselineDescription,
            status: 'workInProgress',
            createdAt: new Date(),
            createdBy: user()?.toString(),
            deletedAt: null,
            deletedBy: null,
            hash: null,
        }
        if (onbaselinecreation) {
            onbaselinecreation(newBaseline);
        }
        closeDialog()
    }
    
</script>

<Dialog.Root bind:open={openDialog}>
    <Dialog.Content class="sm:max-w-[520px]">
        <div class="grid gap-4 py-4 min-h-42">
        {#if currentBaseline}
            <Dialog.Header>
                <Dialog.Title>
                    New Baseline for {module.manifest.prefix}
                </Dialog.Title>
                <Dialog.Description>
                    <p>{currentBaseline.version === "0.0.0" ? "Initial Release" : "Current at " + currentBaseline.version}</p>
                    <p>{currentBaseline.description}</p>
                </Dialog.Description>
            </Dialog.Header>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="name" class="text-right col-span-1">Version</Label>
                <RadioGroup.Root bind:value={newBaselineSelectedType} class="col-span-3 grid grid-cols-4">
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
                        <Label for="fix">{newFix(currentBaseline.version)}</Label>
                    </div>
                </RadioGroup.Root>
            </div>
            <div class="grid grid-cols-4 items-center gap-2">
                <Label for="desc" class="text-right col-span-1">Description</Label>
                <Input multiple id="desc" bind:value={newBaselineDescription}  class="col-span-3" autocomplete="off"/>
            </div>
        {/if}
        </div>
        {#if errorMessage}
            <div class="rounded bg-red-200">
                {errorMessage}
            </div>
        {/if}
        <Dialog.Footer>
            <div class="grow"></div>
            <Button variant="secondary" onclick={closeDialog}>Cancel</Button>
            <Button onclick={handleCreateBaseline}>Create Baseline</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>