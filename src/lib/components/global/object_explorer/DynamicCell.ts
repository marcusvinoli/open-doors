import type { View } from "$lib/components/structs/View";
import type { Template } from "$lib/components/structs/Template";
import type { Attribute } from "$lib/components/structs/Attributes";

import { readOnlyAttributes } from "$lib/utils/attribute-utils";

function insertReadOnlyAttribute(map: Map<string, Attribute>) {
    readOnlyAttributes.forEach( roAttribute => {
        map.set(roAttribute.key, roAttribute);
    })
}

export function mapAttributesIntoView(view: View, template: Template) : Attribute[] {
    let attributeMap = new Map(template.fields.map(attr => [attr.key, attr]));
    insertReadOnlyAttribute(attributeMap);
    let currentView = {...view};
    let ret = currentView.items
        .filter(item => item.show) // Only attributes marked as `show` are mapped.
        .map(item =>
            attributeMap.get(item.key)
        )
        .filter((attr): attr is Attribute => attr !== undefined);
    return ret;
}
