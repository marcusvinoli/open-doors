export type ToolbarItemType = "button" | "buttonsGroup" | "dropdown" | "void";

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

export interface Toolbar {
    items: ToolbarGroupType [];
}