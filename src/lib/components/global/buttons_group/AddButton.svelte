<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import Icon from '@iconify/svelte';
    import ButtonsGroup from './ButtonsGroup.svelte';
    import Button from '$lib/components/ui/button/button.svelte';
    import * as Tooltip from "$lib/components/ui/tooltip";

    const dispatch = createEventDispatcher();

    function handleNewProject(event: any) {
        event.stopPropagation();
        dispatch('newProject', event);
    }

    function handleNewFolder(event: any) {
        event.stopPropagation();
        dispatch('newFolder', event);
    }

    function handleNewModule(event: any) {
        event.stopPropagation();
        dispatch('newModule', event);
    }

</script>

<ButtonsGroup>
    <Tooltip.Root openDelay={200}>
        <Tooltip.Trigger>
            <Button variant="ghost" class="cursor-default">
                <Icon icon="gravity-ui:database" width="20px"/>
            </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p>New Repository</p>
        </Tooltip.Content>
    </Tooltip.Root>
    <Tooltip.Root openDelay={200}>
        <Tooltip.Trigger>
            <Button variant="ghost" class="cursor-default">
                <Icon icon="gravity-ui:house" width="20px"/>
            </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p>Home</p>
        </Tooltip.Content>
    </Tooltip.Root>
</ButtonsGroup>
