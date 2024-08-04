export interface ViewItem {
    attribute: string,
    key: string,
    show: boolean,
}

export interface View {
    items: ViewItem[],
}
