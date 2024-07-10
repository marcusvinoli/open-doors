<script lang="ts">
    import { onMount } from 'svelte';
    import { showToolbar, newTab } from '../store';
    import type { TabData } from '$lib/components/global/tabs/TabData';
    import * as Resizable from "$lib/components/ui/resizable";

    onMount(() => {
        $showToolbar = true;
        let ProjTab: TabData = {
            path: "/projects",
            icon: "gravity-ui:folder-fill",
            title: "Projects",
        };
        $newTab = ProjTab;
    })
</script>

<div class="bg-slate-50 h-full py-1">
<!--     <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={20} minSize={25}>   
            <span>Hello</span>
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        <Resizable.Pane minSize={5}>
               
        </Resizable.Pane>
    </Resizable.PaneGroup> -->
</div>
