export type ToolbarItemType = ToolbarButtonType | ToolbarDropdownType | ToolbarToggleType | ToolbarGroupType;

export type ToolbarButtonType = {
    type: 'button',
    icon?: string | (() => string),
    label?: string | (() => string),
    tooltip?: string | (() => string),
    onclick?: () => void,
    get disabled(): boolean,
}

export type ToolbarDropdownType = {
    type: 'dropdown',
    button: ToolbarButtonType,
    get items(): ToolbarItemType[],
}

export type ToolbarToggleType = {
    type: 'toggle',
    buttonTrue: ToolbarButtonType,
    buttonFalse: ToolbarButtonType,
    onchange: (status: boolean) => void,
    get status(): boolean,
}

export type ToolbarGroupType = {
    type: 'group',
    items: ToolbarItemType[],
}

export type Toolbar = {
    items: ToolbarItemType[];
}
