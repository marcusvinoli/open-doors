<script lang="ts">
    import type { ViewItem } from "$lib/components/structs/View";
    export let view: ViewItem[];
</script>

{#if view}
<table class="w-full h-6 flex">
    {#each view as item}
        <th class="p-2 font-medium text-slate-500 text-sm resize-x">
            {item.attribute}
        </th>
    {/each}
    </table>
{/if}
