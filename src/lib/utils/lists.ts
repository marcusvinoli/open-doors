import type { TreeItem, TreeItemType } from "$lib/components/structs/Tree";

function isTypeMatch(item: TreeItem, type?: TreeItemType | TreeItemType[]): boolean {
    if (!type) {
        return true;
    }
    if (Array.isArray(type)) {
        return type.includes(item.itemType);
    }
    return (item.itemType === type);
}

export function listChildren(parent: TreeItem, inclusiveParent?: boolean, typeFilter?: TreeItemType | TreeItemType[]) {
    let list: TreeItem[] = inclusiveParent ? [parent] : [];
    parent.children.forEach((child) => {
        if (isTypeMatch(child, typeFilter)) {
            list.push(child);
            Array.prototype.push.apply(list, listChildren(child,false,typeFilter));
        }
    })
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

export function listAllContainers(parent: TreeItem, inclusiveParent: boolean = false): TreeItem[] {
    let list: TreeItem[] = inclusiveParent ? [parent] : [];
    parent.children.forEach((child: TreeItem) => {
        list.push(child)
        Array.prototype.push.apply(list, listChildren(child, false, ['folder', 'project']))
    });
    return list;
}

export function listAllContainersExceptChildren(parent: TreeItem, children: TreeItem) {
    let allChilds = listAllContainers(children, true);
    let allRecips = listAllContainers(parent, true);
    return subtractArraysByProperty(allRecips, allChilds, 'name');
}

export function listAllProjects(parent: TreeItem): TreeItem[] {
    return listChildren(parent, true, 'project')
}

export function listAllFolders(parent: TreeItem): TreeItem[] {
    return listChildren(parent, true, 'folder')
}

export function listAllModules(parent: TreeItem): TreeItem[] {
    return listChildren(parent, true, 'module')
}
