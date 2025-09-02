import type { TreeItemType } from "$lib/components/structs/Tree"

export function isValid(name: string, itemType?: TreeItemType) : boolean {
    const validFormat = /^(?!^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(\..*)?$)(?!.*[\\/:*?"<>|])(?!.*[. ]$)[^/]{1,255}$/;
    return validFormat.test(name);
}

export function isValidAttributeName(name: string | null) : boolean {
    if (!name) {
        return false;
    }
    const nonEmptyRegex = /.+/; // Matches any non-empty string
    return nonEmptyRegex.test(name);
}
