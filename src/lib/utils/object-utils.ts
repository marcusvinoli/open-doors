import type { AttributeHashMap } from "$lib/components/structs/Attributes";
import type { Template } from "$lib/components/structs/Template";
import type { Object } from "$lib/components/structs/Object";

import { user } from "$lib/stores/User.svelte";
import { ObjectStatus } from "$lib/components/structs/ObjectStatus";
import { parseTemplate } from "./attribute-utils";

export function newObject(template?: Template, parentLevel?: number, indexLevel?: string) : Object {
    let blankAttributes: AttributeHashMap = {};
    if (template) {
        parseTemplate(template, blankAttributes);
    }
    const newObject: Object = {
        id: 0,
        indexParentId: parentLevel ?? 0,
        indexLevel: indexLevel ?? '1',
        header: '',
        content: '',
        author: user() ?? '', 
        createdAt: new Date(),
        updatedAt: new Date(),
        deletedAt:  null,
        attributes: blankAttributes,
        metadata: {
            level: "",
            status: ObjectStatus.draft,
            outboundLinks: null,
            inboundLinks: null,
        },
    };
    return newObject;
} 

export function computeIndexLevelSibilings(objects: Object[], targetId: number) : { parentId: number; indexLevel: string } {
    const target = objects.find(o => o.id === targetId);
    if (!target) throw new Error("Target not found");

    const siblings = objects
        .filter(o => o.indexParentId === target.indexParentId)
        .sort((a, b) => parseFloat(a.indexLevel) - parseFloat(b.indexLevel));

    const targetIndex = siblings.findIndex(o => o.id === targetId);
    const nextSibling = siblings[targetIndex + 1];

    let newIndex: number;
    if (nextSibling) {
        newIndex = (parseFloat(target.indexLevel) + parseFloat(nextSibling.indexLevel)) / 2;
    } else {
        newIndex = parseFloat(target.indexLevel) + 1;
    }
    const MAX_DECIMAL_NUMBER = 5;

    return {
        parentId: target.indexParentId,
        indexLevel: newIndex.toFixed(MAX_DECIMAL_NUMBER).replace(/0+$/, "").replace(/\.$/, "")
    };
}

export function computeIndexLevelChild(objects: Object[], targetId: number) : { parentId: number; indexLevel: string } {
    const target = objects.find(o => o.id === targetId);
    if (!target) throw new Error("Target not found");

    const children = objects
        .filter(o => o.indexParentId === targetId)
        .sort((a, b) => parseFloat(a.indexLevel) - parseFloat(b.indexLevel));

    let newIndex: number;
    if (children.length > 0) {
        const lastChild = children[children.length - 1];
        newIndex = parseFloat(lastChild.indexLevel) + 1;
    } else {
        newIndex = 1;
    }

    return {
        parentId: target.id,
        indexLevel: newIndex.toString()
    };
}
