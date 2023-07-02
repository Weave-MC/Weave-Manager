<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/tauri";

    let hoverStates = {}

    function handleMouseEnter(pid) {
        hoverStates[pid] = true
    }

    function handleMouseLeave(pid) {
        hoverStates[pid] = false
    }

    enum MinecraftType {
        LunarClient = "Lunar",
        Forge = "Forge",
        Vanilla = "Vanilla",
        BadlionClient = "Badlion"
    }

    type Process = {
        pid: number;
        cmd: string;
        version: string;
        runtime: number;
        client_type: MinecraftType;
    }

    let minecraftList: Process[] = []

    async function getMinecraftProcesses() {
        const rustMcList = await invoke('fetch_minecraft_instances')

        minecraftList = rustMcList.map((rustProcess) => {
            const clientType = MinecraftType[rustProcess.client_type as keyof typeof MinecraftType]

            return {
                pid: rustProcess.pid,
                cmd: rustProcess.cmd,
                version: rustProcess.version,
                runtime: calculateRuntime(rustProcess.start_time),
                client_type: clientType
            }
        })
    }

    function calculateRuntime(startTime: number): string {
        const currentTime = Date.now()
        const runtimeSeconds = Math.floor((currentTime / 1000) - startTime)

        const hours = Math.floor(runtimeSeconds / 3600)
        const minutes = Math.floor((runtimeSeconds % 3600) / 60)

        let formattedRuntime = ''

        if (hours > 0)
            formattedRuntime += `${hours}h `

        if (minutes > 0)
            formattedRuntime += `${minutes}m`

        if (formattedRuntime === '')
            formattedRuntime = '0m'

        return formattedRuntime
    }

    let processInterval

    onMount(() => {
        getMinecraftProcesses()

        processInterval = setInterval(() => {
            getMinecraftProcesses()
        }, 1000)
    });

    onDestroy(() => {
        clearInterval(processInterval)
    });
</script>

<div id="minecraft-list" class="w-full h-full">
    <div id="minecraft-list-title" class="w-full h-8 flex justify-center items-center border-b-2 border-overlay">
        <h1 class="absolute">Minecraft Processes</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-display"></i>
        </div>
    </div>
    <div id="content" class="w-full h-full pb-8">
        <div id="list" class="w-full h-full flex flex-col">
            {#each minecraftList as process (process.id)}
                <div class="process-item {process.client_type}" on:mouseenter={() => handleMouseEnter(process.pid)} on:mouseleave={() => handleMouseLeave(process.pid)}>
                    <p>{process.client_type}</p>
                    <p>{process.version}</p>
                    <p>{process.runtime}</p>
                        <div id="process-buttons" class="w-full h-full absolute top-0 left-0 px-1 py-1 flex flex-row justify-around items-center bg-overlay opacity-0">
                            <button class="process-button">Relaunch</button>
                            <button class="process-button">Kill</button>
                            <button class="process-button">Info</button>
                        </div>
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    .process-item {
        @apply relative w-full h-1/6 border-b-2 border-overlay flex flex-row items-center justify-between px-4;
    }
    .lunarclient {

    }
    .vanilla {

    }
    .forge {

    }
    .process-item:hover #process-buttons {
        opacity: 1;
    }
    #process-buttons {
        transition: opacity 0.2s;
    }
    .process-button {
        @apply w-[30%] h-full bg-surface rounded-xl flex items-center justify-center text-sm font-semibold;
    }
</style>