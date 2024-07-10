export type TreeItemType = "repository" | "folder" | "object" | "module" | null;

export interface TreeItem {
    itemType: TreeItemType;
    name: string;
    path: string | null;
    children: TreeItem[];
}
