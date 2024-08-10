 export type ToolbarItemType = "button" | "buttonsGroup" | "dropdown" | "void" | "toggle";

export interface ToolbarItemInterface {
    type: ToolbarItemType = "void"
}

export interface ToolbarButtonType extends ToolbarItemInterface {
    type: ToolbarItemType = "button",
    icon: string,
    tooltip: string,
    action: () => void,
}

export interface ToolbarGroupType extends ToolbarItemInterface {
    type: ToolbarItemType = "buttonsGroup",
    items: ToolbarItemInterface[],
}

export interface ToolbarDropdownType extends ToolbarItemInterface {
    type: ToolbarItemType = "dropdown",
    button: ToolbarButtonType,
    items: ToolbarGroupType[],
}

export interface ToolbarToggleType extends ToolbarItemInterface {
    type: ToolbarItemType = "toggle",
    buttonOn: ToolbarButtonType,
    buttonOff: ToolbarButtonType,
    status: boolean,
}

export interface Toolbar {
    items: ToolbarGroupType [];
}