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
            attribute: 'ID',
            key: 'id',
            show: true,
        },
        {
            attribute: 'Content',
            key: 'content',
            show: true,
        },
    ]
}