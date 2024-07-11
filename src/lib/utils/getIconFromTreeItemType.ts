import type { TreeItem } from "$lib/components/structs/Tree";

export function getIconFromTreeItemType(item: TreeItem): string {
    let icon = "gravity-ui:file";
    switch (item.itemType) {
        case "repository": 
        icon = "gravity-ui:database";
        break;
        case "project": 
        icon = "gravity-ui:folder-fill";
        break;
        case "folder": 
        icon = "gravity-ui:folder";
        break;
        case "module": 
        icon = "gravity-ui:layout-header-cells-large-fill";
        break;
    }
    return icon;
}