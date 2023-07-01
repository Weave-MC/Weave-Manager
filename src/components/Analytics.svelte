<script>
    import { onMount, onDestroy } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import Statistic from "./Statistic.svelte";

    let memoryUsage = 0;

    async function getMemoryUsage() {
        try {
            memoryUsage = await invoke('get_memory_usage');
        } catch (error) {
            console.error('Error retrieving memory usage:', error);
            memoryUsage = 0;
        }
    }

    let memoryInterval;

    onMount(() => {
        getMemoryUsage();

        memoryInterval = setInterval(() => {
            getMemoryUsage();
        }, 2000);
    });

    onDestroy(() => {
        clearInterval(memoryInterval);
    });
</script>

<div id="analytics" class="w-full h-full">
    <div id="analytics-title" class="w-full h-8 flex justify-center items-center border-b-2 border-surface1">
        <h1 class="absolute">Analytics</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-chart-simple"></i>
        </div>
    </div>
    <div id="content" class="w-full h-full pb-8">
        <div id="statistics" class="w-full h-full flex flex-row flex-wrap items-center justify-center">
            <Statistic name="Hours Played" value="10"/>
            <Statistic name="Avg. Launch" value="32.3s"/>
            <Statistic name="Instances" value="1"/>
            <Statistic name="Mem Usage" value="{memoryUsage}%"/>
        </div>
    </div>
</div>

<style>
    .statistic {

    }
</style>