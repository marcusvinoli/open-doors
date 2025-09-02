export interface ViewItem {
    key: string,
    show: boolean,
    attribute: string,
}

export interface View {
    name: string,
    description: String,
    items: ViewItem[],
}

export const defaultView: View = {
    name: 'Default view',
    description: 'Default view of basic objects attributes',
    items: [
        {
            key: 'id',
            show: true,
            attribute: 'ID',
        },
        {
            key: 'content',
            show: true,
            attribute: 'Content',
        },
        {
            key: 'author',
            show: false,
            attribute: 'Author',
        },
        {
            key: 'createdAt',
            show: false,
            attribute: 'Created At',
        },
        {
            key: 'updatedAt',
            show: false,
            attribute: 'Updated At',
        },
    ]
}
