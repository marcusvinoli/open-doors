<script lang="ts">
    import type { ObjectView } from "$lib/components/structs/Object";
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import type { IndexTree } from "./IndexTree";
    import IndexTreeItem from "./IndexTreeItem.svelte";
    
    export let items: ObjectView[] = [];
    
    let trees: IndexTree[] = [];

    function parseLevel(level: string): (number | string)[] {
        return level.split(/[\.\-]/).map(part => isNaN(Number(part)) ? part : Number(part));
    }

    function ellipsisText(text: string, length: number = 30) {
        return text.slice(0, length).replaceAll(/[*_]/g, "") + "...";
    }

    function buildTreeIndex(objects: ObjectView[]): IndexTree[] {
        const root: IndexTree[] = [];
        const map = new Map<string, IndexTree>();

        for (const obj of objects) {
            let levels = obj.object.level.split(/[\.\-]/).map(part => isNaN(Number(part)) ? part : Number(part));

            while (levels.length > 1 && levels[levels.length - 1] === 0) {
                levels.pop();
            }

            let currentLevel = root;

            for (let i = 0; i < levels.length; i++) {
                const level = levels.slice(0, i + 1).join('.');

                let node = map.get(level);

                if (!node) {
                    node = {
                        level: level,
                        header: i === levels.length - 1 ? obj.object.header : ellipsisText(obj.object.content),
                        content: obj.object.header === "" ? ellipsisText(obj.object.content) : "",
                        path: '',
                        children: [],
                        opened: true,
                    };
                    
                    map.set(level, node);

                    if (i === 0) {
                        currentLevel.push(node);
                    } else {
                        const parentLevel = levels.slice(0, i).join('.');
                        const parent = map.get(parentLevel);
                        if (parent) {
                            parent.children.push(node);
                        }
                    }
                }

                currentLevel = node.children;
            }
        }

    return root;
}

    $: {
        trees = buildTreeIndex(items);
        console.log("tree", trees);
    }

</script>

<div class="h-full w-full flex flex-col">
    <ScrollArea>
        {#each trees as tree}
            <IndexTreeItem item={tree} level={0} on:click/>
        {/each}
    </ScrollArea>
</div>