<script lang="ts">
    import Icon from '@iconify/svelte';
    import { user } from "$lib/stores/User";
    import { repository } from '$lib/stores/Repository';
    import * as Tooltip from "$lib/components/ui/tooltip";
    import { getFirstAndLastName } from '$lib/controllers/User';
</script>


{#if $repository}
<div class="h-6 text-sm text-neutral-100 bg-slate-500 z-50 flex flex-col">
    <div class="h-full w-full flex z-10">
        <div class="w-[40px] h-full bg-slate-300 text-slate-500 flex flex-col justify-center items-center ">
            <Icon icon="ci:data" width="20px" />
        </div>
        <div class="text-slate-100 flex flex-row items-center text-xs font-light">
            {#if $user}
            <Icon icon="ci:user-02" width="15px" class="mx-2"/>
            <span class="mr-1">{getFirstAndLastName($user.name)}</span>
            <span>{" <"+$user.email+">"}</span>
            {/if}
        </div>
    </div>
</div>
{/if}
