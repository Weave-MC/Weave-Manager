<script lang="ts">
    import { onMount, onDestroy } from 'svelte'
    import { invoke } from '@tauri-apps/api/tauri'
    import Statistic from "./Statistic.svelte"

    let memoryUsage: string
    let averageLaunchTime: string

    async function getMemoryUsage() {
        try {
            memoryUsage = await invoke('get_memory_usage')
        } catch (error) {
            console.error('Error retrieving memory usage:', error)
            memoryUsage = "N/A"
        }
    }

    async function getAvgLaunchTime() {
        try {
            averageLaunchTime = await invoke('get_avg_launch_time')
        } catch (error) {
            console.error('Error retrieving average launch time:', error)
            averageLaunchTime = "N/A"
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
    <div id="analytics-title" class="w-full h-8 flex justify-center items-center border-b-2 border-surface1">
        <h1 class="absolute">Analytics</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-chart-simple"></i>
        </div>
    </div>
    <div id="content" class="w-full h-full pb-8">
        <div id="statistics" class="w-full h-full flex flex-row flex-wrap items-center justify-center">
            <Statistic name="Hours Played" value="10"/>
            <Statistic name="Avg. Launch" value="{averageLaunchTime}"/>
            <Statistic name="Instances" value="1"/>
            <Statistic name="Mem Usage" value="{memoryUsage}%"/>
        </div>
    </div>
</div>

<style>
    .statistic {

    }
</style>