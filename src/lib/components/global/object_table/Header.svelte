<script lang="ts">
    import type { ViewItem } from "$lib/components/structs/View";
    interface Props {
        view: ViewItem[];
    }

    let { view }: Props = $props();
</script>

{#if view}
<table class="w-full h-6 flex">
    <thead>
        {#each view as item}
            <th class="p-2 font-medium text-slate-500 text-sm resize-x">
                {item.attribute}
            </th>
        {/each}
    </thead>
</table>
{/if}
