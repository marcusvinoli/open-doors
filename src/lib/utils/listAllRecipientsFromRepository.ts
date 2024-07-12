import type { Repository } from "$lib/components/structs/Repo";
import type { TreeItem } from "$lib/components/structs/Tree";

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

export function listAllRecipientItemsFromRepository(repo: Repository): TreeItem[] {
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
