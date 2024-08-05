import type { Template } from "$lib/components/structs/Template"
import { show } from "@tauri-apps/api/app";
import type { View, ViewItem } from "./viewStructs"

export const parseTemplate = (template: Template, show: boolean = false) : ViewItem[] => {
    let temp: ViewItem[] = [];
    template.fields.forEach((field) => {
        let newViewItem: ViewItem = {
            attribute: field.attribute,
            show: show,
            key: field.key
        }
        temp.push(newViewItem);
    })
    return temp;
} 

export const defaultView = () : View => {
    return {
        items: [
            {
                attribute: "ID",
                show: true,
                key: "id"
            },
            {
                attribute: "Object Content",
                show: true,
                key: "content"
            },
            {
                attribute: "Active?",
                show: true,
                key: "isActive"
            },
            {
                attribute: "Normative?",
                show: true,
                key: "isNormative"
            },
            {
                attribute: "Requirement?",
                show: true,
                key: "isRequirement"
            },
            {
                attribute: "Author",
                show: true,
                key: "author"
            },
        ]
    }
}
