<script lang="ts">
    import Icon from '@iconify/svelte';
    import Switch from '$lib/components/ui/switch/switch.svelte';
    import Label from '$lib/components/ui/label/label.svelte';
    let isReadOnly = "true";
</script>

<div class="flex items-center">
    <Label for="view_mode" />
    <Switch id="view_mode" value={isReadOnly}/>
</div>
