<script lang="ts">
    import TableCell from "$lib/components/ui/table/table-cell.svelte";
    import type { Object } from "$lib/components/structs/Object";
    import type { ViewItem } from "./viewStructs";
    import type { Template } from "$lib/components/structs/Template";

    export let key: string;
    export let object: Object;
    export let cellClass: string = "";
</script>

<TableCell class={cellClass}>
    {#if object.customFields}
        {object.customFields[key]}
    {/if}
</TableCell>