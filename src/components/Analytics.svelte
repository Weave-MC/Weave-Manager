<script lang="ts">
    import { onMount, onDestroy } from 'svelte'
    import { invoke } from '@tauri-apps/api/tauri'
    import Statistic from "./Statistic.svelte"

    export let instances: number
    let memoryUsage: string
    let averageLaunchTime: string
    let timePlayed: string

    type Analytics = {
        launch_times: number[],
        time_played: number,
        average_launch_time: number
    }

    async function getMemoryUsage() {
        try {
            const [used, total] = await invoke<number[]>('get_memory_usage')
            memoryUsage = (used / 1000000).toFixed(1)
        } catch (error) {
            console.error('Error retrieving memory usage:', error)
            memoryUsage = 'N/A'
        }
    }

    async function getAnalytics() {
        try {
            const analytics: Analytics = await invoke('get_analytics')

            averageLaunchTime = `${analytics.average_launch_time.toFixed(1)}s`
            if (analytics.average_launch_time == -1)
                averageLaunchTime = 'N/A'

            timePlayed = formatTimePlayed(analytics.time_played)
        } catch (error) {
            console.error('Error retrieving average launch time:', error)
            averageLaunchTime = 'N/A'
        }
    }

    function formatTimePlayed(timePlayed): string {
        const totalSeconds = Math.floor(timePlayed / 1000)

        const hours = Math.floor(totalSeconds / 3600);
        const minutes = Math.floor((totalSeconds % 3600) / 60);
        const seconds = totalSeconds % 60;

        return `${hours}h ${minutes}m ${seconds}s`;
    }

    let analyticInterval

    onMount(() => {
        getMemoryUsage()
        getAnalytics()

        analyticInterval = setInterval(() => {
            getMemoryUsage()
            getAnalytics()
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
            <Statistic name="Time Played" value="{timePlayed}"/>
            <Statistic name="Avg. Launch" value="{averageLaunchTime}"/>
            <Statistic name="Instances" value="{instances}"/>
            <Statistic name="Mem Usage" value="{memoryUsage}mb"/>
        </div>
    </div>
</div>

<style>
    .statistic {

    }
</style>