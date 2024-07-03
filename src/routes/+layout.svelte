<script lang="ts">
    import "../app.pcss";
    import ToolBar from "$lib/components/global/toolbar/ToolBar.svelte";
    import StatusBar from "$lib/components/global/status_bar/StatusBar.svelte";
    import TabHeader from "$lib/components/global/tabs/TabHeader.svelte";
    
    import type {TabData} from "$lib/components/global/tabs/TabData";

    // Toolbar Module
    let showToolbar: boolean = false;
    
    // Tabs Module
    let tabs: Array<TabData> = [];
    
    let activeTab: number | null = 1;

    function closeTab(event: any) {
        const tabId = event.detail.id;
        var newTabs: TabData[] = tabs;
        console.log("Closing tab "+tabId);
        console.log("Active tab "+activeTab);

        const indexToRemove = newTabs.findIndex((tab) => {
            return(tab.id === tabId);
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
    
    function openTab(event: any) {
        activeTab = event.detail.id;
    }

</script>

<div class="flex flex-col h-full bg-slate-200">
    <div class="">
        {#if showToolbar}
            <ToolBar />
            <TabHeader tabs={tabs} on:close={closeTab} on:open={openTab} active={activeTab}/>
        {/if}
    </div>
    <div class="grow my-1 overflow-auto">
        <slot />
    </div>
    <div class="">
        <StatusBar />
    </div> 
</div>
