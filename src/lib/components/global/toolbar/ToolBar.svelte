<script lang="ts">
    import { Separator } from "$lib/components/ui/separator";
    import NavigationButtons from '../buttons_group/NavigationButtons.svelte';
    import ViewModeSwitch from "../buttons_group/ViewModeSwitch.svelte";
</script>

<div class="h-12 p-1 bg-slate-100 flex align-center text-neutral-700">
    <NavigationButtons />
    <Separator orientation="vertical" class="mx-1"/>
</div>