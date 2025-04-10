import type { Attribute } from "./Attributes";

export interface Template {
    fields: Attribute[],
}

export const readOnlyTemplate: Template = {
    fields: [
        {
            isMandatory : false,
            kind : 'general',
            name : 'ID',
            description : 'Object unique identification',
            key : 'id'
        },
        {
            isMandatory : false,
            kind : 'general',
            name : 'header',
            description : 'Object Header',
            key : 'Header'
        },
        {
            isMandatory : false,
            kind : 'general',
            name : 'Content',
            description : 'Object Text',
            key : 'content'
        },
        {
            isMandatory : true,
            kind : 'user',
            description : 'Object Text',
            name : 'Author',
            key : 'author'
        }
    ]
}