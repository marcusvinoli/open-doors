import type { IndexItem } from "$lib/components/structs/IndexItem";
import type { Object } from "$lib/components/structs/Object";

const ELLIPSIS_TEXT_LENGTH = 30;

function ellipsisText(text: string, length?: number) : string {
    if (length) {
        return text.slice(0, length).replaceAll(/[*_]/g, "") + "...";
    }
    return text.replaceAll(/[*_]/g, "").toString();
}

function parseLevel(level: string): (number | string)[] {
    return level.split(/[\.\-]/).map(part => isNaN(Number(part)) ? part : Number(part));
}

export function buildTreeIndex(objects: Object[]) : IndexItem[] {
    let root: IndexItem[] = [];

    function toIndexItem(object: Object) : IndexItem {
        return {
            id: object.id,
            level: object.metadata?.level ?? '',
            headline: object.header || object.content || "Object " + object.id.toString(),
            isDeleted: (object.metadata?.status === 'deleted'),
            children: []
        }
    }

    function searchForParents(objects: Object[]) : IndexItem[] {
        let result: IndexItem[] = [];
        let parents: Object[] = objects.filter(object => object.indexParentId === 0);
        for (const parent of parents) {
            result.push(toIndexItem(parent));
        }
        return result;
    }

    function seachForChildren(objects: Object[], index: IndexItem) {
        let filtredObjects: Object[] = objects.filter(obj => obj.indexParentId === index.id);
        for (const object of filtredObjects) {
            let child = toIndexItem(object);
            seachForChildren(objects, child);
            index.children.push(child);
        }
    }
    
    root = searchForParents(objects);

    root.forEach(parent => {
        seachForChildren(objects, parent)
    })
    
    return root;
}
