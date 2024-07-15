<script lang="ts">
    import { Separator } from "$lib/components/ui/separator";
    import NavigationButtons from '../buttons_group/NavigationButtons.svelte';
    import AddButton from "../buttons_group/AddButton.svelte";

    export let navigation: boolean = true;
    export let add: boolean = true;
</script>

<div class="h-12 p-1 bg-slate-100 flex align-center text-neutral-700">
    {#if navigation}
        <NavigationButtons />
        <Separator orientation="vertical" class="mx-1"/>
    {/if}
    {#if add}
        <AddButton />
        <Separator orientation="vertical" class="mx-1"/>
    {/if}
</div>