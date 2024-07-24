import type { TreeItem } from "$lib/components/structs/Tree";

export function getIconFromTreeItemType(item: TreeItem, open = false): string {
    let icon = "gravity-ui:file";
    switch (item.itemType) {
        case "repository": 
        icon = "gravity-ui:database";
        break;
        case "project":
            if(open) {
                icon = "gravity-ui:folder-open-fill";
            } else {
                icon = "gravity-ui:folder-fill";
            }
        break;
        case "folder": 
            if(open) {
                icon = "gravity-ui:folder-open";
            } else {
                icon = "gravity-ui:folder";
            }
        break;
        case "module": 
        icon = "gravity-ui:layout-header-cells-large-fill";
        break;
    }
    return icon;
}
