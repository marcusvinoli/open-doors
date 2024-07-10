<script lang="ts">
    import "../app.pcss";
    import { beforeUpdate, setContext } from 'svelte';
    import ToolBar from "$lib/components/global/toolbar/ToolBar.svelte";
    import StatusBar from "$lib/components/global/status_bar/StatusBar.svelte";
    import TabHeader from "$lib/components/global/tabs/TabHeader.svelte";
    import { showToolbar, tabs } from "./store";
    import { page } from "$app/stores";

</script>

<div class="flex flex-col h-full bg-slate-200">
    <div class="">
        {#if $showToolbar}
            <ToolBar />
            <TabHeader tabs={$tabs} active={$page.url.pathname}/>
        {/if}
    </div>
    <div class="grow overflow-auto">
        <slot />
    </div>
    <div class="">
        <StatusBar />
    </div>  
</div>
