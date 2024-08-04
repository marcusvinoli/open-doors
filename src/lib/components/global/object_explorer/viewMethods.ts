import type { View } from "./viewStructs"

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
