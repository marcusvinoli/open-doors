<script lang="ts">
    import type { ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType } from "$lib/components/global/toolbar/Toolbar";
    import PanelView from '$lib/components/global/panelview/PanelView.svelte';
    import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { activeTab, addTab } from "$lib/stores/Tabs";
    import { get } from "svelte/store";
    import { page } from "$app/stores";
    import { repository } from "$lib/stores/Repository";
    import * as Resizable from "$lib/components/ui/resizable";
    import ObjectEditor from "$lib/components/global/object_editor/ObjectEditor.svelte";
    import type { Object } from "$lib/components/structs/Object";

    let selectedObject: Object;

    function loadHomeToolbar() {
        clearToolbar();
    
        let homeButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Home",
            icon: "gravity-ui:house",
            action: () => {
                goto("/home")
            },
        }
        
        let newButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New...",
            icon: "gravity-ui:circle-plus",
            action: () => {},
        }
    
        let newBaselineButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Baseline",
            icon: "gravity-ui:tag",
            action: () => {},
        }
    
        let newObjectButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Object",
            icon: "gravity-ui:square-chart-bar",
            action: () => {},
        }
    
        let newModuleButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Module",
            icon: "gravity-ui:layout-header-cells-large-fill",
            action: () => {},
        }
    
        let creationGroup: ToolbarDropdownType = {
            button: newButton,
            items: [
                {
                    items: [
                        newObjectButton,
                    ],
                    type: "buttonsGroup",
                },
                {
                    items: [
                        newBaselineButton,
                    ],
                    type: "buttonsGroup",
                }
            ],
            type: "dropdown",
        }
    
        let navigationGroup: ToolbarGroupType = {
            items: [homeButton],
            type: "buttonsGroup"
        }
    
        let newGroup: ToolbarGroupType = {
            items: [creationGroup],
            type: "buttonsGroup"
        }
    
        addToolbarItem(navigationGroup);
        addToolbarItem(newGroup);
    }
    
    onMount(() => {
        const params = get(page).params;
        const url: string = $page.url.pathname;
        const name: string = params.mod.substring($repository?.tree.path.length);
        const version: string = params.version;
        addTab(name, "gravity-ui:layout-header-cells-large-fill", url, version);
        loadHomeToolbar();
    })
</script>

<div class="bg-slate-50 h-full py-1">
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={20} minSize={5} collapsible>
            <div>Items Tree</div>
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        <Resizable.Pane>
            <div>Test</div>
        </Resizable.Pane>
        <Resizable.Handle withHandle/>
        <Resizable.Pane class="w-full">
            <ObjectEditor bind:object={selectedObject} />
        </Resizable.Pane>
    </Resizable.PaneGroup>

</div>
