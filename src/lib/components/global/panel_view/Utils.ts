import type { TreeItem } from "$lib/components/structs/Tree"

function updateTreeView(event: any) {
    let currentSelection = event.detail.item as TreeItem;

    if (selectHist.length === 0) {
        selectHist.push(currentSelection);
        selectHist = selectHist;
        return;
    }
    
    if (selectHist.includes(currentSelection) ) {
        while (selectHist.pop() !== currentSelection) {}
        selectHist = selectHist;
        return;
    }
    
    let lastIndex = selectHist.length - 1;
    if (selectHist.at(lastIndex)?.children.includes(currentSelection)) {
        selectHist.push(currentSelection);
        selectHist = selectHist;
        return
    }
}