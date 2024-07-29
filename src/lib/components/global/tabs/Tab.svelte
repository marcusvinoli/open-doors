<script lang="ts">
    import Icon from "@iconify/svelte";
    import { Badge } from "$lib/components/ui/badge";
    import { createEventDispatcher } from 'svelte';

    export let icon: string;
    export let title: string;
    export let path: string;
    export let badge: string | null = null;
    export let active: boolean = false;
   
    const dispatch = createEventDispatcher();
    
    function handleClose(event: any) {
        event.stopPropagation();
        dispatch('close', {path: path});
    }

    function handleOpen(event: any) {
        event.stopPropagation();
        dispatch('open', {path: path});
    }

    function getTitle(path: string): { base: string, lastComponent: string } {
        const components = path.split('/').filter(component => component.length > 0);
    
        if (components.length === 0) {
            return { base: '', lastComponent: '' };
        }
    
        const lastComponent = components.pop()!;
        const base = components.join('/');
    
        return { base, lastComponent };
    }

    let pathBase: string;
    let name: string;

    $: {
        let fullTitle = getTitle(title);
        pathBase = (fullTitle.base === "" ? "" : "/" + fullTitle.base);
        name = fullTitle.lastComponent;
    }

</script>

<button class="flex items-center rounded-t-lg p-1 px-2 mr-1 text-sm {active? 'bg-slate-50 max-w-[350px]' : 'bg-slate-300 max-w-[280px]'} hover:bg-slate-100" on:click={handleOpen}>
    <div class="flex items-center p-1 truncate min-w-[90px] h-full">
        <span>
            <Icon class="" icon={icon} width="18px"/>
        </span>
        <div class="pl-2 truncate">
            {name}
            <span class="truncate text-slate-800 text-[0.85em] font-light leading-0">{pathBase}</span>
        </div>
    </div>
    <div class="flex items-center h-full px-1">
        {#if badge}
            <span class="pr-1">
                <Badge class="">{badge}</Badge>
            </span>
        {/if}
        <button class="hover:bg-slate-300 rounded-full z-5 p-1" on:click={handleClose}>
            <Icon icon="gravity-ui:circle-xmark" width="15px"/>
        </button>
    </div>
</button>