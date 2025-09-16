export interface IndexItem {
    id: number,
    level: string,
    headline: string,
    isDeleted: boolean,
    children: IndexItem[],
}
