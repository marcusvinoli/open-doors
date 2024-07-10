<script lang="ts">
    import { getContext } from 'svelte';
    import { onMount } from 'svelte';

    import TreeItem from '$lib/components/global/treeview/TreeItem.svelte';
    import Tree from "$lib/components/global/treeview/TreeView.svelte";
    /* import intoTreeItem from "$lib/components/global/treeview/TreeItemData"; */
    import type { TreeItemData } from "$lib/components/global/treeview/TreeItemData";
    import * as Resizable from "$lib/components/ui/resizable";
    import type { TabData } from "$lib/components/global/tabs/TabData";
    import type { Repository } from '$lib/components/structs/Repo';
    import { newTab, showToolbar } from '../store';
    import { goto } from '$app/navigation';

    let openRepo: Repository;
    
    onMount(() => {
        $showToolbar = true;
        openRepo = JSON.parse(localStorage.getItem('repository') as string) as Repository;
        let homeTab: TabData = {
            path: "/home",
            icon: "gravity-ui:house",
            title: "Home",
        };
        $newTab = homeTab;
    })

</script>

<div class="bg-slate-50 h-full py-1">
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={20} minSize={25}>
            <button on:click={() => {goto("/projects")}}>Open new Tab...</button>
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        <Resizable.Pane minSize={5}>
        </Resizable.Pane>
<!--     
            <Tree treeItems={openRepo?.structure}/>
            <span>{openRepo?.manifest.name}</span>   
            <span>Hello</span>
            <div>Hello World 2</div>
            <button on:click={()=>{goto("/projects")}}>Open New Tab</button>
            <span>Hello</span>
            <span>{openRepo?.manifest.name}</span>    
        -->
    </Resizable.PaneGroup> 
</div>