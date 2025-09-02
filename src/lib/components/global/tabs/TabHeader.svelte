<script lang="ts">

    import Tab from "./Tab.svelte";
    import { activeTab, tabs, closeTab, openTab } from '$lib/stores/Tabs.svelte';

    
    function handleCloseTab(path: string) {
        closeTab(path);
    }
    
    function handleOpenTab(path: string) {
        openTab(path);
    }
    

</script>

<div class="flex px-2 pt-2 h-[44px] select-none cursor-default">
    {#each tabs() as tab}
        <Tab icon={tab.icon} title={tab.title} path={tab.path} badge={tab.badge} onclose={handleCloseTab} onopen={handleOpenTab} active={(tab.path === activeTab())}/>
    {/each}
</div>
