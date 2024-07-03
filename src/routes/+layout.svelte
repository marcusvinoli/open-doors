<script lang="ts">
    import "../app.pcss";
    import { setContext } from 'svelte';
    import ToolBar from "$lib/components/global/toolbar/ToolBar.svelte";
    import StatusBar from "$lib/components/global/status_bar/StatusBar.svelte";
    import TabHeader from "$lib/components/global/tabs/TabHeader.svelte";
    
    import type {TabData} from "$lib/components/global/tabs/TabData";
    import Tab from "$lib/components/global/tabs/Tab.svelte";

    // Toolbar Module
    let showToolbar: boolean = false;
    
    // Tabs Module
    let tabs: Array<TabData> = [];
    
    let activeTab: number | null = 1;

    function closeTabEvent(event: any) {
        const tabId = event.detail.id;
        unloadTab(tabId);
    }
    
    function activateTabEvent(event: any) {
        activeTab = event.detail.id;
    }

    function unhideToolbar() {
        showToolbar = true;
    }

    function hideToolbar() {
        showToolbar = false;
    }

    function activateTab(tab: TabData) {
        activeTab = tab.id;
    }

    function loadTab(targetTab: TabData) {
        tabs.push(targetTab);
        let newTabs = tabs;
        tabs = newTabs;
    }

    function unloadTab(targetTab: TabData) {
        var newTabs: TabData[] = tabs;

        const indexToRemove = newTabs.findIndex((tab) => {
            return(tab.id === targetTab.id);
        });

        if(indexToRemove < 0) {
            return;
        }

        if(newTabs[indexToRemove].id === activeTab) {
            if (indexToRemove === 0) {
                activeTab = newTabs[indexToRemove+1] ? newTabs[indexToRemove+1].id : null;
            } else {
            }
            activeTab = (newTabs[indexToRemove-1]) ? newTabs[indexToRemove-1].id : null;
        }

        newTabs.splice(indexToRemove, 1);
        
        tabs = newTabs;
    }

    setContext('layout', {
        unhideToolbar,
        hideToolbar,
        loadTab,
        unloadTab,
    })

</script>

<div class="flex flex-col h-full bg-slate-200">
    <div class="">
        {#if showToolbar}
            <ToolBar />
            <TabHeader tabs={tabs} on:close={closeTabEvent} on:open={activateTabEvent} active={activeTab}/>
        {/if}
    </div>
    <div class="grow overflow-auto">
        <slot />
    </div>
    <div class="">
        <StatusBar />
    </div> 
</div>
