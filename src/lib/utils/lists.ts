import type { Repository } from "$lib/components/structs/Repo";
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

export function listAllRecipients(repo: Repository): TreeItem[] {
    let list: TreeItem[] = [];
    list.push({
        name: repo.manifest.name,
        itemType: "repository",
        path: repo.path,
        children: []
    });
    repo.structure.children.forEach((child: TreeItem) => {
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

export function listAllProjects(parent: TreeItem): TreeItem[] {
    return listChildren(parent, "project")
}

export function listAllFolders(parent: TreeItem): TreeItem[] {
    return listChildren(parent, "folder")
}
