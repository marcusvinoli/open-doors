<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import Icon from "@iconify/svelte";
    
    export let icon: string | null;
    export let title: string;
    export let id: number;
    export let active: boolean = false;
    
    const dispatch = createEventDispatcher();
    
    function handleClose(event: any) {
        event.stopPropagation();
        dispatch('close', {id: id});
    }

    function handleOpen(event: any) {
        event.stopPropagation();
        dispatch('open', {id: id});
    }
</script>

<button class="flex items-baseline rounded-t-lg p-1 mr-1 text-sm {active? 'bg-slate-50 max-w-[300px]' : 'bg-slate-300 max-w-[180px]'} hover:bg-slate-100" on:click={handleOpen}>
    <div class="flex items-center p-1 truncate min-w-[100px]">
        <span>
            {#if icon}
                <Icon class="" icon={icon} width="18px"/>
            {/if}
        </span>
        <span class="pl-2 truncate">{title}</span>
    </div>
    <div>
        <button class="p-1 hover:bg-slate-300 rounded-full z-5" on:click={handleClose}>
            <Icon icon="gravity-ui:circle-xmark" width="15px"/>
        </button>
    </div>
</button>