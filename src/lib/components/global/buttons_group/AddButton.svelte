<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import Icon from '@iconify/svelte';
    import ButtonsGroup from './ButtonsGroup.svelte';
    import Button from '$lib/components/ui/button/button.svelte';
    import * as Tooltip from "$lib/components/ui/tooltip";
    import CreateProjectForms from '$lib/components/forms/CreateProjectForms.svelte';

    const dispatch = createEventDispatcher();
    let newProjectDialog: boolean = false;
    let newFolderDialog: boolean = false;
    let newModuleDialog: boolean = false;

    function handleNewProject(event: any) {
        event.stopPropagation();
        newProjectDialog = true;
        dispatch('newProject', event);
    }

    function handleNewFolder(event: any) {
        event.stopPropagation();
        newFolderDialog = true;
        dispatch('newFolder', event);
    }

    function handleNewModule(event: any) {
        event.stopPropagation();
        newModuleDialog = true;
        dispatch('newModule', event);
    }

</script>

<ButtonsGroup>
    <Tooltip.Root openDelay={200}>
        <Tooltip.Trigger>
            <Button variant="ghost" class="cursor-default" on:click={handleNewProject}>
                <Icon icon="gravity-ui:folder-open-fill" width="20px"/>
            </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p>New Project</p>
        </Tooltip.Content>
    </Tooltip.Root>
    <Tooltip.Root openDelay={200}>
        <Tooltip.Trigger>
            <Button variant="ghost" class="cursor-default" on:click={handleNewFolder}>
                <Icon icon="gravity-ui:folder-open" width="20px"/>
            </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p>New Folder</p>
        </Tooltip.Content>
    </Tooltip.Root>
    <Tooltip.Root openDelay={200}>
        <Tooltip.Trigger>
            <Button variant="ghost" class="cursor-default" on:click={handleNewModule}>
                <Icon icon="gravity-ui:layout-header-cells-large-fill" width="20px"/>
            </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p>New Module</p>
        </Tooltip.Content>
    </Tooltip.Root>
</ButtonsGroup>
<CreateProjectForms openDialog={newProjectDialog}/>