<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import type { TabData } from "./TabData"
    import Tab from "./Tab.svelte";
    
    export let tabs: TabData[];
    export let active: number | null = null;
    
    const dispatch = createEventDispatcher();

    function handleCloseTab(event: any) {
        dispatch('close', event.detail);
    }
    
    function handleOpenTab(event: any) {
        dispatch('open', event.detail);
    }

</script>

<div class="flex px-2 pt-2 h-[44px]">
    {#each tabs as tab}
        <Tab icon={tab.icon} title={tab.title} id={tab.id} on:close={handleCloseTab} on:open={handleOpenTab} active={active === tab.id}/>
    {/each}
</div>