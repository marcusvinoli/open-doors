<script lang="ts">
    import "../app.css";

    import ToolBar from "$lib/components/global/toolbar/Toolbar.svelte";
    import TabHeader from "$lib/components/global/tabs/TabHeader.svelte";
    import StatusBar from "$lib/components/global/status_bar/StatusBar.svelte";
    
    import { goto } from "$app/navigation";
    import { loadRepository } from "$lib/controllers/Repository";
    import { onMount, type Snippet } from "svelte";
    
    let { children } : { children?: Snippet} = $props();

    onMount(() => {
        if (loadRepository()) {
            goto("/home");
        }
    })
</script>

<div class="flex flex-col h-full bg-slate-200">
    <div>
        <ToolBar />
        <TabHeader />
    </div>
    <div class="grow overflow-auto">
        {@render children?.()}
    </div>
    <div>
        <StatusBar />
    </div>  
</div>
