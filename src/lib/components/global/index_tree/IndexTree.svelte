<script lang="ts">
    import IndexTreeItem from "./IndexTreeItem.svelte";

    import { onDestroy, onMount, tick } from "svelte";

    import type { IndexItem } from "$lib/components/structs/IndexItem";

    let {
        id = "",
        trees = $bindable(),
        state = $bindable(),
        showDeletions,
        onclick,
        onscroll,
    } : {
        id?: string,
        trees: IndexItem[],
        state: Map<number, boolean>,
        showDeletions?: boolean,
        onclick?: (id: string | number) => void,
        onscroll?: (e: any) => void,
    } = $props();

    onMount(() => {
        if (!onscroll) {
            return;
        }
        tick().then(() => {
            document.getElementById(id)?.addEventListener("scroll", onscroll);
        });
    })

    onDestroy(() => {
        if (!onscroll) {
            return;
        }
        document.getElementById(id)?.removeEventListener("scroll", onscroll);
    })

</script>

<div class="relative h-full right-0.5 overflow-auto">
    <div id={id} class="absolute top-0 bottom-0 right-0 min-w-30 w-full overflow-x-auto">
        {#each trees as tree (tree.id)}
        <IndexTreeItem item={tree} level={0} onclick={onclick} state={state} {showDeletions}/>
        {/each}
    </div>
</div>
