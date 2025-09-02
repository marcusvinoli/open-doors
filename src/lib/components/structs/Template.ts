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
            description : 'Object unique identifier.',
            key : 'id'
        },
        {
            isMandatory : false,
            kind : 'general',
            name : 'header',
            description : 'Object header.',
            key : 'Header'
        },
        {
            isMandatory : false,
            kind : 'general',
            name : 'Content',
            description : 'Object content text.',
            key : 'content'
        },
        {
            isMandatory : true,
            kind : 'user',
            description : 'Last user that modifies the Object Content/Header',
            name : 'Author',
            key : 'author'
        }
    ]
}