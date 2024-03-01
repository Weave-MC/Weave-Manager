<script lang="ts">
    import {type MinecraftInfo, type MinecraftProcess} from "../../../../scripts/types";
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import {createLaunchProfile, showProcessInfo} from "../../../../scripts/shared";
    import ButtonBar from "../../../util/ButtonBar.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {createEventDispatcher, onDestroy, onMount} from "svelte";
    import PopUp from "../../../util/PopUp.svelte";
    import CreateLaunchProfilePopUp from "../../../popups/CreateLaunchProfilePopUp.svelte";

    let popup: CreateLaunchProfilePopUp
    let processMap = new Map<number, MinecraftProcess>()
    export let instances: number = 0
    const dispatch = createEventDispatcher()

    async function getMinecraftProcesses() {
        try {
            const processList = await invoke<MinecraftProcess[]>("fetch_minecraft_processes")

            const newMap = new Map()
            for (const process of processList) {
                let version = process.info.version.trim()
                if (version.includes("-"))
                    process.info.version = version.substring(0, version.indexOf("-"))

                newMap.set(process.pid, process)
            }

            processMap = newMap
            instances = processMap.size
        } catch (err) {
            console.error(err)
        }
    }

    async function killProcess(pid: number) {
        try {
            await invoke("kill_pid", {pid: pid})
        } catch (err) {
            console.error("Error killing process", err)
        }
    }

    async function swapConsole(process: MinecraftProcess) {
        try {
            dispatch("switch_console", process)
            await invoke("switch_console_output", {pid: process.pid})
        } catch (err) {
            console.error("Error swapping console", err)
        }
    }

    let processInterval: NodeJS.Timer
    onMount(async () => {
        await getMinecraftProcesses()
        processInterval = setInterval(async () => {
            await getMinecraftProcesses()
        }, 1000)
    })

    onDestroy(() => {
        clearInterval(processInterval)
    })
</script>

<div id="minecraft-processes" class="relative w-full h-full bg-surface rounded-xl p-2">
    <div class="relative w-full text-center">
        <h1>Minecraft Processes</h1>
    </div>
    <VerticalScroll columns={1} items={[...processMap.values()]} let:prop={mcProcess}>
        <button class="w-full h-[3rem] rounded-lg flex gap-5 items-center justify-between p-2 bg-surface">
            <div class="h-full w-full flex flex-row justify-between items-center">
                <h1 class="w-[33%] text-start">{mcProcess.pid}</h1>
                <h1 class="w-[33%] text-center">{mcProcess.info.client}</h1>
                <h1 class="w-[33%] text-end">{mcProcess.info.version}</h1>
            </div>
            <ButtonBar class="gap-2" buttons={[
                {label: "Kill Process", action: () => killProcess(mcProcess.pid), icon: "fa-solid fa-skull"},
                {label: "Create Launch Profile", action: () => popup.startCreateLaunchProfile(mcProcess.info), icon: "fa-solid fa-plus"},
                {label: "Process Info", action: () => showProcessInfo(mcProcess), icon: "fa-solid fa-info"}
            ]}/>
        </button>
    </VerticalScroll>
    <CreateLaunchProfilePopUp bind:this={popup}/>
</div>