<script lang="ts">
    import { createEventDispatcher, onDestroy, onMount } from 'svelte';
    import type { TabData } from "./TabData"
    import Tab from "./Tab.svelte";
    import { goto } from '$app/navigation';
    import { newTab } from '../../../../routes/store';
    
    export let tabs: TabData[];
    export let active: string | null = null;
    
    const dispatch = createEventDispatcher();

    function handleCloseTab(event: any) {
        const tabPath = event.detail.path;
        unloadTab(tabPath);
        dispatch('close', event.detail);
    }
    
    function handleOpenTab(event: any) {
        loadTab(event.detail);
        dispatch('open', event.detail);
    }
    
    function loadTab(newTab: TabData) {
        let newTabs = tabs;
        if(tabs.findIndex((tab) => {return (tab.path === newTab.path)}) < 0) {
            newTabs.push(newTab);
        }
        tabs = newTabs;
        goto(newTab.path);
    }
    
    function unloadTab(path: string) {
        var newTabs: TabData[] = tabs;

        const indexToRemove = newTabs.findIndex((tab) => {
            return(tab.path === path);
        });

        if(indexToRemove < 0) {
            return;
        }

        if(newTabs[indexToRemove].path === active) {
            if (indexToRemove === 0) {
                active = newTabs[indexToRemove+1] ? newTabs[indexToRemove+1].path : "/";
            } else {
                active = (newTabs[indexToRemove-1]) ? newTabs[indexToRemove-1].path : "/";
            }
        }

        newTabs.splice(indexToRemove, 1);
        
        tabs = newTabs;

        goto(active?? "/")
    }

    onMount(() => {
        let currentTab = $newTab;
        if(currentTab) {loadTab(currentTab)};
        const unsub = newTab.subscribe((value: TabData | null) => {
            if (value) {
                loadTab(value)
                newTab.set(null);
            }
        });
        return () => {
            unsub();
        }
    })

</script>

<div class="flex px-2 pt-2 h-[44px]">
    {#each tabs as tab}
        <Tab icon={tab.icon} title={tab.title} path={tab.path} on:close={handleCloseTab} on:open={handleOpenTab} active={active === tab.path}/>
    {/each}
</div>