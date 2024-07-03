<script lang="ts">
    import { getContext } from 'svelte';
    import * as Resizable from "$lib/components/ui/resizable";
    import Tree from "$lib/components/global/treeview/TreeView.svelte";
    import type { TabData } from "$lib/components/global/tabs/TabData";

    let repository_name = "Local"

    const { 
        hideToolbar,
        unhideToolbar,
        loadTab,
    } = getContext('layout');

    unhideToolbar();

    let tab: TabData = {
        id: 1,
        icon: "gravity-ui:database",
        title: repository_name,
    }
    
    loadTab(tab);
</script>

<div class="bg-slate-50 h-full py-1">
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={20} minSize={25}>
            <Tree />
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        <Resizable.Pane minSize={5}>
        </Resizable.Pane>
    </Resizable.PaneGroup>
</div>