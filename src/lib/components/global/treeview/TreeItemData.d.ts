export type ItemType = "folder" | "project" | "repo" | "module" | "other";

export interface TreeItemAttributes {
    selected: Boolean,
    opened: Boolean,
}

export interface TreeItemData {
    type: ItemType;
    name: String;
    path: String;
    children: TreeItem[];
    attributes: TreeItemAttributes | undefined;
}

export function intoTreeItem(data: any): TreeItemData {
    let children: TreeItemData[]; 
    data.children.forEach(
        (child) => {
            children.push(intoTreeItem(child))
        });
    return TreeItemData = {
        type: data.itemType ?? "other",
        name: data.name ?? "",
        path: data.path ?? "",
        children: children,
        attributes: {
            selected: false,
            open: false,
        }
    };
}
