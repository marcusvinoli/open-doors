import { show } from "@tauri-apps/api/app";
import type { Template } from "$lib/components/structs/Template";
import type { View, ViewItem } from "../components/structs/View";

export const parseTemplate = (template: Template, show: boolean = false) : ViewItem[] => {
    let temp: ViewItem[] = [];
    template.fields.forEach((field) => {
        let newViewItem: ViewItem = {
            attribute: field.name,
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
                attribute: "Content",
                show: true,
                key: "content"
            }
        ]
    }
}
