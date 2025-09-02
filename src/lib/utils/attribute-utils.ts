import type { Template } from "$lib/components/structs/Template";
import type { Attribute, AttributeHashMap } from "$lib/components/structs/Attributes";

export function parseTemplate(template: Template, attributes: AttributeHashMap) {
    template.fields.forEach(attribute => {
        if (!attributes[attribute.key]) {
            attributes[attribute.key] = ''; // TODO: Implement a default value for each type.
        }
    })
}

export const readOnlyAttributes : Attribute[] = [
    { key: 'id', name: 'ID', kind: 'general', description: 'Unique Identifier', isMandatory: true },
    { key: 'content', name: 'Object content', kind: 'string', description: 'Object contet', isMandatory: true },
    { key: 'createdAt', name: 'Created at', kind: 'dateTime', description: 'Creation date', isMandatory: true },
    { key: 'updatedAt', name: 'Updated at', kind: 'dateTime', description: 'Last update date', isMandatory: true },
    { key: 'deletedAt', name: 'Deleted at', kind: 'dateTime', description: 'Deletion date', isMandatory: false },
    { key: 'author', name: 'Author', kind: 'user', description: 'Author', isMandatory: true },
    { key: 'indexParentId', name: 'Parent ID', kind: 'integer', description: 'Upper Level', isMandatory: false },
    { key: 'indexLevel', name: 'Index Level', kind: 'string', description: 'Hierarchy identifier', isMandatory: false },
    { key: 'header', name: 'Header', kind: 'string', description: 'Content header', isMandatory: false },
];
