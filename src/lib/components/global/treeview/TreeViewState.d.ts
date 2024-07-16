import type { TreeItemType } from "$lib/components/structs/Tree";

export interface TreeItemState {
    itemType: TreeItemType;
    name: string;
    path: string | null;
    opened: boolean;
    children: TreeItemState[];
}
