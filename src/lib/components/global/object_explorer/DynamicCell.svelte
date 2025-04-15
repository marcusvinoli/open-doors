<script lang="ts">
    import type { Attribute, AttributeKind } from "$lib/components/structs/Attributes";
    import type { ModuleManifest } from "$lib/components/structs/ModuleManifest";
    import type { Object } from "$lib/components/structs/Object";
    import type { ViewItem } from "$lib/components/structs/View";
    
    export let object: Object;
    export let viewItem: ViewItem;
    export let moduleManifest: ModuleManifest;
</script>

{#if object && moduleManifest}
    {#if viewItem.key == 'id'}
        <span>{moduleManifest.prefix}{moduleManifest.separator}{object.id}</span>
    {:else if viewItem.key == 'header' }
        <span>{object.header}</span>
    {:else}
        <span>{object.content}</span>
    {/if}
{/if}
