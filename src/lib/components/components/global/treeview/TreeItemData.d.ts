export type ItemType = "folder" | "project" | "repo" | "module" | "other";

export interface TreeItemAttributes {
    selected: Boolean,
    opened: Boolean,
}

export interface TreeItemData {
    type: ItemType;
    name: String;
    path: String;
    children?: TreeItem[];
    attributes: TreeItemAttributes | undefined;
}
