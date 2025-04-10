<script lang="ts">
    import type { Attribute, AttributeKind } from "$lib/components/structs/Attributes";
    import type { ModuleManifest } from "$lib/components/structs/ModuleManifest";
    import type { Object } from "$lib/components/structs/Object";
    import type { ViewItem } from "$lib/components/structs/View";
    import { type Template, readOnlyTemplate } from "$lib/components/structs/Template";

    export let object: Object;
    export let moduleManifest: ModuleManifest;
    export let viewItem: ViewItem;
    export let template: Template = readOnlyTemplate;
</script>

{#if object && moduleManifest}
    {#if viewItem.key == 'id'}
        <span>{moduleManifest.prefix}{moduleManifest.separator}{object.id}</span>
    {:else if viewItem.key == 'header' }
        <span>{object.content}</span>
    {:else if viewItem.key == 'content' }
        <span>{object.content}</span>
    {:else if viewItem.key == 'author' }
        <span>{object.author}</span>
    {:else if viewItem.key == 'createdAt' }
        <span>{object.createdAt}</span>
    {:else if viewItem.key == 'updatedAt' }
        <span>{object.updatedAt}</span>
    {:else if viewItem.key == 'deletedAt' }
        <span>{object.deletedAt}</span>
    {:else}
        <span>{object.content}</span>
    {/if}
{/if}
