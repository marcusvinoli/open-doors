<script lang="ts">
    import type { ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType } from "$lib/components/global/toolbar/Toolbar";
    import { goBack, goHome } from "$lib/stores/PanelView";
    import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar";
    import { onMount } from "svelte";
    import { newTab } from "../../store";
    import type { TabData } from "$lib/components/global/tabs/TabData";
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { get } from "svelte/store";
    import { addTab } from "$lib/stores/Tabs";

    let path: string = "";
    
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
    
        let backButton: ToolbarButtonType = {
            type: "button",
            tooltip: "Back",
            icon: "gravity-ui:arrow-left",
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
    
        let newProjectButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Project",
            icon: "gravity-ui:folder-open-fill",
            action: () => {},
        }
    
        let newFolderButton: ToolbarButtonType = {
            type: "button",
            tooltip: "New Folder",
            icon: "gravity-ui:folder-open",
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
                        newProjectButton, 
                        newFolderButton, 
                        newModuleButton
                    ],
                    type: "buttonsGroup",
                }
            ],
            type: "dropdown",
        }
    
        let navigationGroup: ToolbarGroupType = {
            items: [homeButton, backButton],
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
        console.log("Mouting MOD 2 page");
        const params = get(page).params;
        console.log("Params", params);
        addTab("path", "gravity-ui:layout-header-cells-large-fill", "path");
        loadHomeToolbar();
    })
</script>

<div>
    This is module "MOD" with the most recente Baseline
</div>
