import type { AttributeKind } from "$lib/components/structs/Attributes";

export function getAttributeValueKind(attributeKind: AttributeKind) {
    switch (attributeKind) {
        case "string":
            return "String";
        case "general": // TODO: Include number, date, time, dateTime, etc...
            return "Markdown";
        case "boolean":
            return "True/False";
        default: // TODO: Include number, date, time, dateTime, etc...
            let dataKind = Object.keys(attributeKind)[0];
            if (dataKind === "singleOption") {
                return "Single Option";
            } else if (dataKind === "multipleOptions") {
                return "Multiple Options";
            } else {
                return "String";
            }
    }
}

export function getDataValues(attributeKind: AttributeKind) : string | string[] {
    switch (attributeKind) {
        case "string":
            return "Text";
        case "general": 
            return "Formatted text";
        case "boolean":
            return "True/False";
        default: 
            // TODO: Include number, date, time, dateTime, etc...
            if (typeof attributeKind === 'object') {
                if ('singleOption' in attributeKind || 'multipleOptions' in attributeKind) {
                    return Object.values(attributeKind)[0] as string[];
                }
                return '';
            } else {
                return '';
            }
    }
}
