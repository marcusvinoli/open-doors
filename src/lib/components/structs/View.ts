export interface ViewItem {
    attribute: string,
    key: string,
    show: boolean,
}

export interface View {
    items: ViewItem[],
}

export const readOnlyView: View = {
    items: [
        {
            attribute: 'id',
            key: 'id',
            show: true,
        },
        {
            attribute: 'content',
            key: 'content',
            show: true,
        },
    ]
}