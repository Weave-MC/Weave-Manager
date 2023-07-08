<script lang="ts">
    import { onMount, onDestroy } from 'svelte'
    import { invoke } from '@tauri-apps/api/tauri'
    import Statistic from "./Statistic.svelte"

    export let instances: number
    let memoryUsage: string
    let averageLaunchTime: string

    async function getMemoryUsage() {
        try {
            const [used, total] = await invoke<string[]>('get_memory_usage')
            memoryUsage = (used / total * 100).toFixed(2)
        } catch (error) {
            console.error('Error retrieving memory usage:', error)
            memoryUsage = 'N/A'
        }
    }

    async function getAvgLaunchTime() {
        try {
            const avg_launch_time: number = await invoke('get_avg_launch_time')

            averageLaunchTime = `${avg_launch_time.toFixed(1)}s`;

            if (avg_launch_time == -1)
                averageLaunchTime = 'N/A'
        } catch (error) {
            console.error('Error retrieving average launch time:', error)
            averageLaunchTime = 'N/A'
        }
    }

    let analyticInterval

    onMount(() => {
        getMemoryUsage()
        getAvgLaunchTime()

        analyticInterval = setInterval(() => {
            getMemoryUsage()
            getAvgLaunchTime()
        }, 2000)
    });

    onDestroy(() => {
        clearInterval(analyticInterval)
    });
</script>

<div id="analytics" class="w-full h-full">
    <div id="analytics-title" class="w-full h-8 flex justify-center items-center border-b-2 border-overlay">
        <h1 class="absolute">Analytics</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-chart-simple"></i>
        </div>
    </div>
    <div id="content" class="w-full h-full pb-8">
        <div id="statistics" class="w-full h-full flex flex-row flex-wrap items-center justify-center">
            <Statistic name="Hours Played" value="10"/>
            <Statistic name="Avg. Launch" value="{averageLaunchTime}"/>
            <Statistic name="Instances" value="{instances}"/>
            <Statistic name="Mem Usage" value="{memoryUsage}%"/>
        </div>
    </div>
</div>

<style>
    .statistic {

    }
</style>