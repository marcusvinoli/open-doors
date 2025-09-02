export interface IndexItem {
    id: number,
    level: string,
    headline: string,
    children: IndexItem[],
}
