<script lang="ts">
    import Icon from "@iconify/svelte";

    import { Badge } from "$lib/components/ui/badge";
    import { onMount } from "svelte";

    let {
        icon,
        title,
        path,
        badge = null,
        active = false,
        onopen = null,
        onclose = null,
    } : {
        icon: string;
        title: string;
        path: string;
        badge?: string | null;
        active?: boolean;
        onopen?: ((path: string) => void) | null,
        onclose?: ((path: string) => void) | null,
    } = $props();

    let fullTitle = $derived(getTitle(title))
    let pathBase = $derived((fullTitle.base) === "" ? "" : "/" + fullTitle.base);
    let name = $derived(fullTitle.lastComponent);
    
    function handleClose(e: any) {
        e.stopPropagation();
        if (onclose) {
            onclose(path);
        }
    }

    function handleOpen(e: any) {
        e.stopPropagation();
        if (onopen) {
            onopen(path);
        }
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

</script>

<div role="button" tabindex="-1" onclick={handleOpen} onkeypress={() => {}} >
    <div class="flex items-center rounded-t-lg p-1 px-2 mr-1 text-sm {active? 'bg-slate-50 max-w-[350px]' : 'bg-slate-300 max-w-[280px]'} hover:bg-slate-100">
        <div class="flex items-center p-1 truncate min-w-[90px] h-full">
            <span>
                <Icon class="" icon={icon} width="18px"/>
            </span>
            <div class="pl-2 truncate">
                {name}
                <span class="truncate text-slate-800 text-[0.85em] font-light leading-0">{pathBase}</span>
            </div>
        </div>
        <div class="flex items-center justify-center h-full px-1">
            {#if badge}
            <Badge class="mx-2 px-2">{badge}</Badge>
            {/if}
            <button class="hover:bg-slate-300 rounded-full z-5 p-1" onclick={handleClose}>
                <Icon icon="gravity-ui:circle-xmark" width="15px"/>
            </button>
        </div>
    </div>
</div>