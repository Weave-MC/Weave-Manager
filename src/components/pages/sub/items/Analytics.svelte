<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import type {Analytics} from "../../../../scripts/types";
    import {onDestroy, onMount} from "svelte";
    import {processMap} from "../../../../scripts/stores";

    let memoryUsage: string = "N/A"
    let avgLaunchTime: string = "N/A"
    let timePlayed: string = "None"

    function formatTimePlayed(timePlayed: number): string {
        const totalSeconds = Math.floor(timePlayed / 1000)

        const hours = Math.floor(totalSeconds / 3600);
        const minutes = Math.floor((totalSeconds % 3600) / 60);
        const seconds = totalSeconds % 60;

        return `${hours}h ${minutes}m ${seconds}s`;
    }

    async function getMemoryUsage() {
        try {
            const [used, total] = await invoke<number[]>("get_memory_usage")
            memoryUsage = (used / 1_000_000).toFixed(1)
        } catch (error) {
            console.error("Error retrieving memory usage:", error)
            memoryUsage = "N/A"
        }
    }

    async function getAnalytics() {
        try {
            const analytics = await invoke<Analytics>("get_analytics")

            if (analytics.average_launch_time == -1)
                avgLaunchTime = "N/A"
            else
                avgLaunchTime = `${analytics.average_launch_time.toFixed(1)}s`

            timePlayed = formatTimePlayed(analytics.time_played)
        } catch (error) {
            console.error("Error retrieving analytics", error)
        }
    }

    let interval: NodeJS.Timer
    onMount(async () => {
        await getMemoryUsage()
        await getAnalytics()
        interval = setInterval(async () => {
            await getMemoryUsage()
            await getAnalytics()
        }, 2000)
    })

    onDestroy(() => {
        clearInterval(interval)
    })
</script>
<div id="analytics" class="relative w-full h-[40%] bg-surface rounded-xl text-center p-2 flex flex-col gap-2">
    <div class="w-full text-center">
        <h1>Analytics</h1>
    </div>
    <div id="analytics-container" class="bg-crust h-full w-full rounded-lg pl-2 pr-4 pt-2 pb-4">
        <div id="ram-usage" class="analytic">
            <h1 class="font-semibold">Ram Usage</h1>
            <h1>{memoryUsage === "N/A" ? memoryUsage : `${memoryUsage}mb`}</h1>
        </div>
        <div id="process-count" class="analytic">
            <h1 class="font-semibold">MC Process Count</h1>
            <h1>{$processMap.size} {$processMap.size > 1 ? "Processes" : "Process"} </h1>
        </div>
        <div id="time-played" class="analytic">
            <h1 class="font-semibold">Time Played</h1>
            <h1>{timePlayed}</h1>
        </div>
        <div id="avg-launch" class="analytic">
            <h1 class="font-semibold">Avg. Launch</h1>
            <h1>{avgLaunchTime}</h1>
        </div>
    </div>
</div>

<style>
    #analytics-container {
        @apply gap-2;
        display: grid;
        grid-template-columns: repeat(2, 50%);
        grid-template-rows: repeat(2, 50%);
    }
    .analytic {
        @apply bg-overlay w-full h-full rounded-lg flex flex-col justify-around;
    }
</style>