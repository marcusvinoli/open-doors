import type { TreeItem, TreeItemType } from "$lib/components/structs/Tree";

function listSubItems(item: TreeItem): TreeItem[] {
    let list: TreeItem[] = [];
    item.children.forEach((subitem) => {
        if(subitem.itemType === "project" || subitem.itemType === "folder" ) {
            list.push(subitem);
            Array.prototype.push.apply(list, listSubItems(subitem))
        }
    });
    return list;
}

function isTypeMatch(item: TreeItem, type: TreeItemType | TreeItemType[]): boolean {
    if (Array.isArray(type)) {
        return type.includes(item.itemType);
    }
    return (item.itemType === type);
}

function searchOnSubItems(item: TreeItem, type: TreeItemType | TreeItemType[]): TreeItem[] {
    let list: TreeItem[] = [];
    item.children.forEach((subitem) => {
        if(isTypeMatch(item, type)) {
            list.push(subitem);
            Array.prototype.push.apply(list, searchOnSubItems(subitem, type));
        }
    });
    return list;
}

function subtractArraysByProperty<T, K extends keyof T>(array1: T[], array2: T[], key: K): T[] {
    const set = new Set(array2.map(item => item[key]));
    return array1.filter(item => !set.has(item[key]));
}

export function listRelatives(tree: TreeItem, parent: TreeItem, child: TreeItem): TreeItem[] {
    let relatives: TreeItem[] = [];
    let pathFound = false;

    const visit = (node: TreeItem, currentPath: TreeItem[]) => {
        if (!pathFound && node.name === parent.name && node.path === parent.path) {
            pathFound = true;
        }

        if (pathFound) {
            currentPath.push(node);
            if (node.name === child.name && node.path === child.path) {
                relatives = currentPath.slice(0, -1);
                pathFound = false;
            }
        }

        for (let childNode of node.children) {
            visit(childNode, [...currentPath]);
        }
    };

    visit(tree, []);

    return relatives;
}

export function listAllRecipients(parent: TreeItem, inclusive: boolean = false): TreeItem[] {
    let list: TreeItem[] = [];
    if(inclusive) {
        list.push(parent);
    }
    parent.children.forEach((child: TreeItem) => {
        list.push(child)
        Array.prototype.push.apply(list, listSubItems(child))
    });
    return list;
}

export function listChildren(parent: TreeItem, type: TreeItemType | TreeItemType[]): TreeItem[] {
    let list: TreeItem[] = [];
    parent.children.forEach((child: TreeItem) => {
        list.push(child)
        Array.prototype.push.apply(list, searchOnSubItems(child, type));
    })
    return list;
}

export function listAllRecipientsExceptChildren(parent: TreeItem, children: TreeItem) {
    let allChilds = listAllRecipients(children, true);
    let allRecips = listAllRecipients(parent, true);
    return subtractArraysByProperty(allRecips, allChilds, 'name');
}

export function listAllProjects(parent: TreeItem): TreeItem[] {
    return listChildren(parent, "project")
}

export function listAllFolders(parent: TreeItem): TreeItem[] {
    return listChildren(parent, "folder")
}

export function listAllModules(parent: TreeItem): TreeItem[] {
    let list: TreeItem[] = [];
    parent.children.forEach((subitem) => {
        if(subitem.itemType === "project" || subitem.itemType === "folder" ) {
            Array.prototype.push.apply(list, listAllModules(subitem))
        } else if(subitem.itemType === "module") {
            list.push(subitem);
        }
    });
    return list;
}
