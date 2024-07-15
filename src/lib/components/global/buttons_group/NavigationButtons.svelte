<script lang="ts">
    import Icon from '@iconify/svelte';
    import ButtonsGroup from './ButtonsGroup.svelte';
    import Button from '$lib/components/ui/button/button.svelte';
    import * as Tooltip from "$lib/components/ui/tooltip";
    import { goto } from '$app/navigation';
</script>

<ButtonsGroup>
    <Tooltip.Root openDelay={200}>
        <Tooltip.Trigger>
            <Button variant="ghost" class="cursor-default" on:click={()=>{goto("/home")}}>
                <Icon icon="gravity-ui:house" width="20px"/>
            </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p>Home</p>
        </Tooltip.Content>
    </Tooltip.Root>
</ButtonsGroup>
