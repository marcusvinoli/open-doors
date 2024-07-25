export interface IndexTreeItem {
    index: string,
    header: string,
    children: [IndexTreeItem],
}

export interface IndexTree {
    items: [IndexTreeItem]
}