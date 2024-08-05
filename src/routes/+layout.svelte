<script lang="ts">
    import "../app.pcss";
    import ToolBar from "$lib/components/global/toolbar/ToolBar.svelte";
    import StatusBar from "$lib/components/global/status_bar/StatusBar.svelte";
    import TabHeader from "$lib/components/global/tabs/TabHeader.svelte";
    import { loadAuthorInformation } from "$lib/controllers/User";
    import { loadRepository } from "$lib/controllers/Repository";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    onMount(() => {
        if (loadRepository()) {
            goto("/home");
        }
    })
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
