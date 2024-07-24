<script lang="ts">
    import Icon from "@iconify/svelte";
    import * as Tooltip from "$lib/components/ui/tooltip";
    import Button from "$lib/components/ui/button/button.svelte";
    import type { ButtonInterface, ButtonSimple } from "./ButtonGroup";

    export let button: ButtonInterface;
    let btn: ButtonSimple = button as ButtonSimple;
</script>

<Tooltip.Root openDelay={200}>
    <Tooltip.Trigger>
        <Button variant="ghost" class="cursor-default" on:click={btn.action}>
            <Icon icon={btn.icon} width="20px"/>
        </Button>
    </Tooltip.Trigger>
    <Tooltip.Content>
        <p>{btn.tooltip}</p>
    </Tooltip.Content>
</Tooltip.Root>