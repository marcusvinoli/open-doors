<script lang="ts">
    import Tab from "./Tab.svelte";
    import { page } from "$app/stores";
    import { createEventDispatcher } from 'svelte';
    import { activeTab, tabs, closeTab, openTab } from '$lib/stores/Tabs';

    let currentTab: string;
    
    const dispatch = createEventDispatcher();

    function handleCloseTab(event: any) {
        const tabPath = event.detail.path;
        closeTab(tabPath);
        event.stopPropagation();
        dispatch('close', event.detail);
    }
    
    function handleOpenTab(event: any) {
        openTab(event.detail.path);
        event.stopPropagation();
        dispatch('open', event.detail);
    }
    
    $: {
        currentTab = $page.url.pathname;
        $activeTab = currentTab;
    }

</script>

<div class="flex px-2 pt-2 h-[44px]">
    {#each $tabs as tab}
        <Tab icon={tab.icon} title={tab.title} path={tab.path} badge={tab.badge} on:close={handleCloseTab} on:open={handleOpenTab} active={(tab.path === currentTab)}/>
    {/each}
</div>