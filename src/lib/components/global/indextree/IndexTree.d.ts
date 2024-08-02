export interface IndexTree {
    level: string,
    header: string,
    content: string,
    opened: boolean,
    path: string
    children: IndexTree[],
}
