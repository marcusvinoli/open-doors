<script lang="ts">
    import Icon from "@iconify/svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";

    import { Button } from "$lib/components/ui/button/index.js";

    import * as Tooltip from "$lib/components/ui/tooltip";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Table from "$lib/components/ui/table";

    import type { Module } from "$lib/components/structs/Module";
    import { defaultView, type View } from "$lib/components/structs/View";
    import ViewForm from "./ViewForm.svelte";

    let { 
        openDialog = $bindable(false), 
        module,
        onviewsupdate,
        readOnly = false,
    } : {
        openDialog?: boolean;
        module: Module;
        readOnly?: boolean;
        onviewsupdate?: (views: View[]) => void;
    } = $props();

    let views: View[] = $derived($state.snapshot(module.views) as View[]);
    let openViewFormDialog: boolean = $state(false);
    let readOnlyView: boolean = $state(false);
    let selectedView: View = $state({
        name: '',
        description: '',
        items: [],
    })

    function clearFields() {
        readOnlyView = readOnly;
        selectedView = {
            name: '',
            description: '',
            items: [],
        };
    }

    function openEditDialog(view?: View | null, readOnlyThisView?: boolean) {
        clearFields();
        readOnlyView = readOnlyThisView ?? false;
        if (view) {
            selectedView = view;
        }
        openViewFormDialog = true;
    }

    function handleCloseViewFormDialog() {
        clearFields();
        openViewFormDialog = false;
    }

    function handleSaveView(view: View) {
        if (selectedView.name.trim() !== '') {
            const index = views.findIndex(v => v.name === selectedView.name);
            if (index >= 0) {
                views[index] = view;
            }
            handleCloseViewFormDialog();
            views = [...views];
            return;
        }
        views.push(view);
        views = [...views];
        handleCloseViewFormDialog();
    }

    function handleDeleteView(view: View) {
        if (selectedView.name.trim() !== '') {
            const index = views.findIndex(v => v.name === selectedView.name); 
            if (index >= 0) {
                views.splice(index, 1);
            }
        }
        views = [...views];
        handleCloseViewFormDialog();
    }
    
    function closeDialog() {
        handleCloseViewFormDialog();
        openDialog = false;
    }

    function handleSaveViews() {
        if (onviewsupdate) {
            onviewsupdate($state.snapshot(views));
        }
        closeDialog();
    }

</script>

<Dialog.Root bind:open={openDialog}>
    <Dialog.Content class="flex flex-col min-w-[80%] max-h-[90%] min-h-[80%]">
        <Dialog.Header class="pt-1">
            <Dialog.Title>Views of Module {module.manifest.prefix}</Dialog.Title>
            <Dialog.Description>
                {#if readOnly}
                    <div class="flex gap-2">
                        <Icon icon="ph:pencil-simple-slash" width="20px" />
                        Read-Only Mode. No changes can be made.
                    </div>
                {:else}
                    <p>{module.manifest.description}</p>
                {/if}
            </Dialog.Description>
        </Dialog.Header>
        <div class="min-h-15 grow">
            <Tooltip.Provider ignoreNonKeyboardFocus>
                <Table.Root>
                    <Table.Header>
                        <Table.Row class="border-b-[1px]">
                            <Table.Head class="sticky top-0 bg-slate-50 shadow-sm z-1">View</Table.Head>
                            <Table.Head class="sticky top-0 bg-slate-50 shadow-sm z-1">Description</Table.Head>
                            <Table.Head class="sticky top-0 bg-slate-50 shadow-sm z-1 w-[30px]"></Table.Head>
                        </Table.Row>
                    </Table.Header>
                    <Table.Body>
                        <Table.Row>
                            <Table.Cell>
                                <p>{defaultView.name}</p>
                            </Table.Cell>
                            <Table.Cell>
                                <p>{defaultView.description}</p>
                            </Table.Cell>
                            <Table.Cell>
                            <Tooltip.Root>
                                <Tooltip.Trigger>
                                    <Button variant="ghost" onclick={() => openEditDialog(defaultView, true)}>
                                        <Icon icon="gravity-ui:ellipsis" width="20px" />
                                    </Button>
                                </Tooltip.Trigger>
                                <Tooltip.Content>
                                    <p>Edit</p>
                                </Tooltip.Content>
                            </Tooltip.Root>
                        </Table.Cell>
                        </Table.Row>
                        {#each views as view (view.name)}
                            <Table.Row>
                                <Table.Cell>
                                    <p>{view.name}</p>
                                </Table.Cell>
                                <Table.Cell class="whitespace-normal">
                                    <p>{view.description}</p>
                                </Table.Cell>
                                <Table.Cell>
                                    <Tooltip.Root>
                                        <Tooltip.Trigger>
                                            <Button variant="ghost" onclick={() => openEditDialog(view)}>
                                                <Icon icon="gravity-ui:ellipsis" width="20px" />
                                            </Button>
                                        </Tooltip.Trigger>
                                        <Tooltip.Content>
                                            <p>Edit</p>
                                        </Tooltip.Content>
                                    </Tooltip.Root>
                                </Table.Cell>
                            </Table.Row>
                        {/each}
                    </Table.Body>
                </Table.Root>
            </Tooltip.Provider>
        </div>
        <div>
            <Button variant="secondary" onclick={() => openEditDialog()} >
                <Icon icon="gravity-ui:square-plus" width="20px" />
                Add Custom View
            </Button>
        </div>
        <ViewForm 
            bind:open={openViewFormDialog}
            template={module.template}
            view={selectedView} 
            readOnly={(readOnly || readOnlyView)}
            ondelete={handleDeleteView}
            onclose={handleCloseViewFormDialog}
            onsave={handleSaveView}
            />
        <Separator/>
        <Dialog.Footer>
            <Button variant="secondary" onclick={closeDialog}>
                <Icon icon="gravity-ui:xmark" width="20px" />
                Cancel
            </Button>
            <Button onclick={handleSaveViews}>
                <Icon icon="gravity-ui:floppy-disk" width="20px"/>
                Save Changes
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
