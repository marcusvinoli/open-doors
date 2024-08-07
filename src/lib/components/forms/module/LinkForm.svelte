<script lang="ts">
    import Icon from '@iconify/svelte';
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Table from "$lib/components/ui/table";
    import type { Link, ObjectView } from '$lib/components/structs/Object';
    import type { Module } from '$lib/components/structs/Module';
    import type { Object } from '$lib/components/structs/Object';
    import StringDropdown from './StringDropdown.svelte';
    import StringComboBox from './StringComboBox.svelte';
    import { createEventDispatcher, onMount } from 'svelte';
    import { listAllModules } from '$lib/utils/lists';
    import { repository } from '$lib/stores/Repository';
    import type { TreeItem } from '$lib/components/structs/Tree';
    import { readObjects } from '$lib/controllers/Module';

    export let links: Link[] | null = [];
    export let editable: boolean = true;

    export let modPlaceholder = "Select a module...";
    export let objsPlaceholder = "Select a module first...";

    let modules: string[];
    let objects: string[];

    let selectedModule: string = "";
    let selectedObject: string = "";

    let moduleTrees: TreeItem[];
    let moduleObjects: Object[] = [];
    
    let readToLink: boolean = false;

    const dispatch = createEventDispatcher();

    function handleModuleSelection(event: any) {
        if (event.detail.item === modPlaceholder) {
            return;
        }
        readToLink = false;
        objects = [objsPlaceholder];
        selectedModule = event.detail.item;
        let index = moduleTrees.findIndex((mod) => {return (mod.name === selectedModule)});
        if (index < 0) {
            return;
        }
        let modTree = moduleTrees[index]
        readObjects(modTree.path)
            .then((objs) => {
                moduleObjects = objs as Object[];
                listAllObjects(modTree.name, "-")
            })
            .catch((err) => {
                console.log(err);
            })
        dispatch('selectModule', {item: selectedModule})
    }

    function handleObjectSelection(event: any) {
        if (event.detail.item === objsPlaceholder) {
            return;
        }
        readToLink = true;
        let selectedObj = event.detail.item;
        dispatch('selectObject', {item: selectedObj})
    }
    
    function handleAddLink() {
        if (!readToLink) {
            return;
        }

        if (!links) {
            links = [];
        }

        let index = moduleTrees.findIndex((mod) => {return (mod.name === selectedModule)});

        if (index < 0) {
            return;
        }
        
        let newLink: Link = {
            path: moduleTrees[index].path,
            module: selectedModule,
            object: parseInt(selectedObject.split("-").pop()??"0"),
        }

        let findIndex = links?.findIndex((link) => {return (newLink === link)});

        if (findIndex < 0) {
            links = [...links, newLink];
        }

        console.log("Links", links);
        
        readToLink = false;
        selectedModule = "";
        selectedObject = "";
    }
    
    function handleRemoveLink(link: Link) {
        dispatch('removeLink', {link: link})
    }

    function handleVisitLink(link: Link) {
        dispatch('visitLink', {link: link})
    }

    function getAllModules() {
        let allModules: string[] = [];
        moduleTrees = listAllModules($repository?.tree);
        moduleTrees.forEach((mod) => {
            allModules.push(mod.name);
        })
        modules = allModules;
    }

    function listAllObjects(prefix: string, separator: string) {
        let allObjs: string[] = [];
        moduleObjects.forEach((obj) => {
            allObjs.push(`${prefix}${separator}${obj.id}`)
        })
        if (allObjs.length === 0) {
            objsPlaceholder = "No object found on this module."
            objects = [objsPlaceholder];
            return;
        }
        objsPlaceholder = "Select a object..."
        objects = allObjs;
    }

    $: {

    }

    onMount(() => {
        getAllModules();
    })

</script>

<Table.Root class="w-full">
    <Table.Header>
        <Table.Row>
            <Table.Head class="w-[1/3]">
                Module
            </Table.Head>
            <Table.Head class="">
                Object
            </Table.Head>
            <Table.Head class="">
                 
            </Table.Head>
        </Table.Row>
    </Table.Header>
    <Table.Body class="w-full">
        {#if editable}
        <Table.Row>
            <Table.Cell class="px-1">
                <div class="min-w-[80px]">
                    {#if modules}
                    <StringDropdown bind:options={modules} on:select={handleModuleSelection} placeholder={modPlaceholder}/>
                    {/if}
                </div>
            </Table.Cell>
            <Table.Cell class="px-1 w-full">
                <StringDropdown options={objects} bind:selected={selectedObject} on:select ={handleObjectSelection} placeholder={objsPlaceholder}/>
            </Table.Cell>
            <Table.Cell class="pl-1 pr-2 w-[20px]">
                <Button size="sm" variant="ghost" class="hover:text-blue-600" on:click={handleAddLink} disabled={!readToLink}>
                    <Icon icon="gravity-ui:circle-plus" width="20px" />
                </Button>
            </Table.Cell>
        </Table.Row>
        {/if}
        {#if links}
        {#each links as link}
            <Table.Row>
                <Table.Cell class="max-w-[1/3]">
                    <span>{link.module}</span>
                </Table.Cell>
                <Table.Cell class="max-w-12 min-w-12">
                    <div class="flex flex-row items-center">
                        <Button size="sm" variant="ghost" on:click={() => handleVisitLink(link)}>
                            <Icon icon="gravity-ui:circle-chevron-right" rotate={0} width="20px" />
                        </Button>
                        <span>{link.module}-{link.object}</span>
                    </div>
                </Table.Cell>
                <Table.Cell class="pl-1 pr-2">
                    <Button size="sm" variant="ghost" class="hover:text-red-600" on:click={() => handleRemoveLink(link)}>
                        <Icon icon="gravity-ui:circle-minus" width="20px" />
                    </Button>
                </Table.Cell>
            </Table.Row>
        {/each}
        {/if}
    </Table.Body>
</Table.Root>
