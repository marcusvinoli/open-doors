export type ButtonType = "simple" | "dropdown" | "group" | "void";

export interface ButtonInterface {
    type: ButtonType = "void"
}

export interface ButtonsGroup extends ButtonInterface {
    type: ButtonType = "group",
    items: ButtonInterface[],
}

export interface ButtonSimple extends ButtonInterface {
    type: ButtonType = "simple",
    icon: string,
    tooltip: string,
    action: () => void,
}

export interface ButtonDropdown extends ButtonInterface {
    type: ButtonType = "dropdown",
    button: ButtonSimple,
    items: ButtonsGroup[],
}