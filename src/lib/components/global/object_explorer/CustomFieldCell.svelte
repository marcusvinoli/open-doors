<script lang="ts">
    import TableCell from "$lib/components/ui/table/table-cell.svelte";
    import type { Object } from "$lib/components/structs/Object";
    import type { ViewItem } from "./viewStructs";

    export let object: Object;
    export let key: string;
    export let cellClass: string = "";
</script>

<TableCell class={cellClass}>
</TableCell>