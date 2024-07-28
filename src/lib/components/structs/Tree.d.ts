export type TreeItemType = "repository" | "folder" | "project" | "object" | "module" | null;

export interface TreeItem {
    itemType: TreeItemType;
    name: string;
    path: string;
    children: TreeItem[];
}
