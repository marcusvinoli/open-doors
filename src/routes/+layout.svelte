<script lang="ts">
    import "../app.pcss";
    import Toolbar from "$lib/components/global/toolbar/Toolbar.svelte";
    import StatusBar from "$lib/components/global/status_bar/StatusBar.svelte";
    import TabHeader from "$lib/components/global/tabs/TabHeader.svelte";
    import { tabs } from "./store";
    import { page } from "$app/stores";

</script>

<div class="flex flex-col h-full bg-slate-200">
    <div class="">
        <Toolbar/>
        <TabHeader tabs={$tabs} active={$page.url.pathname}/>
    </div>
    <div class="grow overflow-auto">
        <slot />
    </div>
    <div class="">
        <StatusBar />
    </div>  
</div>
