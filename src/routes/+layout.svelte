<script lang="ts">
    import "../app.pcss";
    import ToolBar from "$lib/components/global/toolbar/ToolBar.svelte";
    import StatusBar from "$lib/components/global/status_bar/StatusBar.svelte";
    import TabHeader from "$lib/components/global/tabs/TabHeader.svelte";
    import { page } from "$app/stores";
    import { activeTab } from "$lib/stores/Tabs";
</script>

<div class="flex flex-col h-full bg-slate-200">
    <div class="">
        <ToolBar/>
        <TabHeader/>
    </div>
    <div class="grow overflow-auto">
        <slot />
    </div>
    <div class="">
        <StatusBar />
    </div>  
</div>
