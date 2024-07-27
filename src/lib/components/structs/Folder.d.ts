import type { TreeItem } from "./Tree";

export interface Folder extends TreeItem {
    itemType: "folder",
}