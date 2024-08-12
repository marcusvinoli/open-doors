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

<div class="w-full flex flex-col px-1">
    <div class="w-full flex flex-col">
        {#if editable}
        <div class="w-full flex flex-col py-3 border-b-[1px]">
            <div class="w-full flex flex-row text-sm">
                <div class="w-[200px] px-2">
                    Module
                </div>
                <div class="px-2">
                    Object
                </div>
                <div class="w-[50px]">
                
                </div>
            </div>
            <div class="w-full flex py-1 items-center">
                <div class="px-1 w-[200px]">
                    {#if modules}
                    <StringDropdown bind:options={modules} on:select={handleModuleSelection} placeholder={modPlaceholder}/>
                    {/if}
                </div>
                <div class="px-1 grow">
                    <StringDropdown options={objects} bind:selected={selectedObject} on:select ={handleObjectSelection} placeholder={objsPlaceholder}/>
                </div>
                <div class="w-[50px]">
                    <Button size="sm" variant="ghost" class="hover:text-blue-600" on:click={handleAddLink} disabled={!readToLink}>
                        <Icon icon="gravity-ui:circle-plus" width="20px" />
                    </Button>
                </div>
            </div>
        </div>
        {/if}
        {#if links}
            {#each links as link, index}
            <div class="w-full flex flex-row text-sm items-center py-1 {((links.length - index) > 1 ? "border-b-[1px] border-b-slate-200" : "")}">
                <div class="flex flex-row items-center px-1">
                    <Button size="sm" variant="ghost" on:click={() => {handleVisitLink(link)}}>
                        <Icon icon="gravity-ui:circle-chevron-right" rotate={0} width="20px" />
                    </Button>
                </div>
                <div class="px-2 grow">
                    <span>{link.module} / {link.module}-{link.object}</span>
                </div>
                {#if editable}
                <div class="w-[50px]">
                    <Button size="sm" variant="ghost" class="hover:text-red-600" on:click={() => {handleRemoveLink(link)}}>
                        <Icon icon="gravity-ui:circle-minus" width="20px" />
                    </Button>
                </div>
                {/if}
            </div>
            {/each}
        {/if}
    </div>
</div>
