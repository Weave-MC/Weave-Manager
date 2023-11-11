<script lang="ts">
    import {onDestroy, onMount, createEventDispatcher} from "svelte"
    import {invoke} from "@tauri-apps/api/tauri"
    import { appWindow } from "@tauri-apps/api/window"
    import {shiftDown} from "../key-stores";

    enum MinecraftType {
        LunarClient = "Lunar",
        Forge = "Forge",
        Vanilla = "Vanilla",
        BadlionClient = "Badlion"
    }

    type Process = {
        pid: number;
        cmd: string[];
        cwd: string;
        version: string;
        runtime: number;
        client_type: MinecraftType;
        weave_attached: boolean;
    }

    export let instances = 0
    export let promptRelaunch: boolean

    const dispatch = createEventDispatcher()

    let processInfo: Process = null
    let relaunchInfo: Process = null
    let infoModal: HTMLDialogElement
    let relaunchModal: HTMLDialogElement
    let minecraftMap = new Map<number, Process>()

    async function getMinecraftProcesses() {
        try {
            const rustMcList = await invoke('fetch_minecraft_instances')

            const newMcMap = new Map()
            for (const rustProcess of rustMcList) {
                const clientType = MinecraftType[rustProcess.client_type as keyof typeof MinecraftType]

                let version = rustProcess.version.trim()
                if (version.includes('-'))
                    version = version.substring(0, version.indexOf('-'))

                const minecraft = {
                    pid: rustProcess.pid,
                    cmd: rustProcess.cmd,
                    cwd: rustProcess.cwd,
                    version: version,
                    start_time: rustProcess.start_time,
                    client_type: clientType,
                    weave_attached: rustProcess.weave_attached
                }

                if (!minecraftMap.has(rustProcess.pid) && !rustProcess.weave_attached && promptRelaunch) {
                    await appWindow.setFocus()
                    relaunchInfo = minecraft as Process
                    relaunchModal.showModal()
                }

                newMcMap.set(rustProcess.pid, minecraft)
            }

            minecraftMap = newMcMap
            instances = minecraftMap.size
        } catch (err) {
            console.error("Error retrieving process list", err)
        }
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
            formattedRuntime = `${runtimeSeconds}s`

        return formattedRuntime
    }

    // Kills the process and relaunches it with weave loader attached
    async function relaunchWithWeave(process) {
        if (relaunchModal.open)
            relaunchModal.close()

        try {
            // kill process
            await killProcess(process.pid)

            minecraftMap.delete(process.pid)
            // relaunch process with weave
            process.cmd = process.cmd.filter(arg => !arg.startsWith('-Dlog4j.configurationFile='))
            await invoke('relaunch_with_weave', {cwd: process.cwd, cmdLine: process.cmd})
        } catch (err) {
            console.error("Error relaunching process", err)
        }
    }

    async function killProcess(pid) {
        try {
            await invoke('kill_pid', {pid: pid})
        } catch (err) {
            console.error("Error killing process", err)
        }
    }

    async function showConsole(pid) {
        try {
            dispatch('switch_console')
            await invoke('switch_console_output', {pid: pid})
        } catch (err) {
            console.error("Error switching console output", err)
        }
    }

    function showInfoModal(process) {
        processInfo = process
        infoModal.showModal()
    }

    let processInterval

    onMount(() => {
        getMinecraftProcesses()
        processInterval = setInterval(async () => {
            await getMinecraftProcesses()
        }, 1000)
    });

    onDestroy(() => {
        clearInterval(processInterval)
    });
</script>

<div id="minecraft-list" class="w-full h-full bg-surface">
    <div id="minecraft-list-title" class="w-full h-8 bg-surface flex justify-center items-center border-b-2 border-overlay">
        <h1 class="absolute">Minecraft Processes</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-display"></i>
        </div>
    </div>
    <div id="content" class="w-full h-full pb-8 overflow-y-auto">
        <div id="list" class="w-full flex flex-col">
            {#each [...minecraftMap.values()] as process}
                <div class="relative process-item {process.client_type}">
                    <p class="absolute left-4">{process.client_type}</p>
                    <p>{process.version}</p>
                    <p class="absolute right-4">{calculateRuntime(process.start_time)}</p>
                    <div class="process-buttons w-full h-full absolute top-0 left-0 px-1 py-1 flex flex-row justify-around items-center bg-overlay opacity-0">
                        <button class="process-button" on:click={() => showInfoModal(process)}>Info</button>
                        <button class="process-button" on:click={async() => await killProcess(process.pid)}>Kill</button>
                        {#if process.weave_attached}
                            <button class="process-button" on:click={async() => await showConsole(process.pid)}>Console</button>
                        {:else}
                            <button class="process-button" on:click={async() => await relaunchWithWeave(process)}>Relaunch</button>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    </div>
    <dialog bind:this={infoModal} id="process-info-modal" class="modal w-[30rem] h-[28rem]" on:click={modalClicked}>
        {#if processInfo}
            <div class="w-full h-9 border-b-2 border-overlay flex justify-center items-center">Process Information</div>
            <div class="w-full h-full flex flex-row flex-wrap items-end justify-center pb-3">
                <div class="w-1/3 info-item">
                    <p class="font-semibold">Client</p>
                    <p class="select-text">{processInfo.client_type}</p>
                </div>
                <div class="w-1/3 info-item">
                    <p class="font-semibold">Status</p>
                    {#if processInfo.weave_attached}
                        <p>Weave attached</p>
                    {:else}
                        <p>Weave not attached</p>
                    {/if}
                </div>
                <div class="w-1/3 info-item">
                    <p class="font-semibold">Version</p>
                    <p class="select-text">{processInfo.version}</p>
                </div>
                <div class="w-32 info-item">
                    <p class="font-semibold">PID</p>
                    <p class="select-text">{processInfo.pid}</p>
                </div>
                <div class=" w-32 info-item">
                    <p class="font-semibold">Runtime</p>
                    <p class="select-text">{calculateRuntime(processInfo.start_time)}</p>
                </div>
                <div class="w-full info-item">
                    <p class="font-semibold">Working Directory</p>
                    <p class="select-text">{processInfo.cwd}</p>
                </div>
                <div class="w-full h-48 info-item">
                    <p class="font-semibold mb-1">Command Line</p>
                    <div class="w-full h-44 rounded-xl p-1 bg-base">
                        <div class="w-full h-full overflow-y-scroll break-words select-text p-1">
                            {processInfo.cmd.join(" ")}
                        </div>
                    </div>
                </div>
            </div>

        {/if}
    </dialog>
    <dialog bind:this={relaunchModal} id="relaunch-modal" class="modal w-[25rem] h-[20rem]" on:click={modalClicked}>
        <div class="w-full h-9 border-b-2 border-overlay flex justify-center items-center">Detected Minecraft Instance</div>
        <div class="w-full h-full flex flex-col items-center justify-between pb-3">
            {#if relaunchInfo}
                <div class="w-full h-2/3 flex flex-row flex-wrap justify-between items-center">
                    <div class="w-1/2 info-item">
                        <p class="text-sm font-semibold">Client</p>
                        <p class="text-sm select-text">{relaunchInfo.client_type}</p>
                    </div>
                    <div class="w-1/2 info-item">
                        <p class="text-sm font-semibold">Version</p>
                        <p class="text-sm select-text">{relaunchInfo.version}</p>
                    </div>
                    <div class="w-1/2 info-item">
                        <p class="text-sm font-semibold">PID</p>
                        <p class="text-sm select-text">{relaunchInfo.pid}</p>
                    </div>
                    <div class="w-1/2 info-item">
                        <p class="text-sm font-semibold">Runtime</p>
                        <p class="text-sm select-text">{calculateRuntime(relaunchInfo.start_time)}</p>
                    </div>
                </div>
            {/if}
            <div class="w-full h-1/3 flex flex-col justify-around items-center">
                <button class="w-44 h-8 rounded-xl border-2 border-enabled bg-overlay flex justify-center items-center" on:click={async() => await relaunchWithWeave(relaunchInfo)}>
                    Relaunch with Weave
                </button>
                <button class="w-24 h-8 rounded-xl bg-overlay flex justify-center items-center" on:click={() => relaunchModal.close()}>
                    No Thanks
                </button>
            </div>
        </div>
    </dialog>
</div>

<style>
    button {
        outline: none;
    }
    .modal {
        @apply px-4 py-1 fixed top-0 bottom-0 flex flex-col bg-surface rounded-xl text-text;
        z-index: 1;
        scale: 0;
        opacity: 0;
        box-shadow: 0 0 3rem 1px rgba(0, 0, 0, 0.4);
        transition: all 350ms ease-in-out;
    }
    .modal::backdrop {
        background-color: rgba(0, 0, 0, 0.2);
    }
    .modal:focus {
        outline: none;
    }
    .modal[open] {
        scale: 1;
        opacity: 1;
    }
    .info-item {
        @apply text-center text-sm;
    }
    .process-item {
        @apply relative bg-surface w-full h-10 border-b-[0.0625rem] border-overlay flex flex-row justify-center items-center;
    }
    .lunarclient {

    }
    .vanilla {

    }
    .forge {

    }
    .process-item:hover .process-buttons {
        opacity: 1;
    }
    .process-buttons {
        transition: opacity 0.1s;
    }
    .process-button {
        @apply w-[30%] h-full bg-surface rounded-xl flex items-center justify-center text-sm font-semibold;
    }
    ::-webkit-scrollbar {
        @apply w-1;
    }
    ::-webkit-scrollbar-track {
        @apply bg-none;
    }
    ::-webkit-scrollbar-thumb {
        @apply bg-overlay rounded-xl;
    }
</style>