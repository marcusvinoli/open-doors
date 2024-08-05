<script lang="ts">
    import TableCell from "$lib/components/ui/table/table-cell.svelte";
    import type { Object } from "$lib/components/structs/Object";

    export let key: string;
    export let object: Object;
    export let cellClass: string = "";
</script>

<TableCell class={cellClass}>
    {#if object.customFields}
        {(object.customFields[key]) ? object.customFields[key] : ""}
    {/if}
</TableCell>