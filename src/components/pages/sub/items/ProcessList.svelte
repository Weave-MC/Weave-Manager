<script lang="ts">
    import {type MinecraftProcess} from "../../../../scripts/types";
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import {showProcessInfo} from "../../../../scripts/components";
    import ButtonBar from "../../../util/ButtonBar.svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {createEventDispatcher} from "svelte";
    import CreateLaunchProfilePopUp from "../../../popups/CreateLaunchProfilePopUp.svelte";
    import {processMap, weaveProcessMap} from "../../../../scripts/stores";

    let popup: CreateLaunchProfilePopUp
    const dispatch = createEventDispatcher()

    async function killProcess(pid: number) {
        try {
            await invoke("kill_pid", {pid: pid})
            if ($weaveProcessMap.has(pid))
                $weaveProcessMap.delete(pid)
        } catch (err) {
            console.error("Error killing process", err)
        }
    }

    async function swapConsole(process: MinecraftProcess) {
        try {
            await invoke("switch_console_output", {pid: process.pid})
            dispatch("switch_console", process)
        } catch (err) {
            console.error("Error swapping console", err)
        }
    }

    function processCompare(a: MinecraftProcess, b: MinecraftProcess) {
        if (a.weave_attached && !b.weave_attached)
            return 1
        if (b.weave_attached && !a.weave_attached)
            return -1
        return 0
    }

    const normalButtons = (process: MinecraftProcess) => [
        {label: "Kill Process", action: () => killProcess(process.pid), icon: "fa-solid fa-skull"},
        {label: "Create Launch Profile", action: () => popup.startCreateLaunchProfile(process.info), icon: "fa-solid fa-plus"},
        {label: "Process Info", action: () => showProcessInfo(process), icon: "fa-solid fa-info"}
    ]

    const weaveProcessButton = (process: MinecraftProcess) => [
        {label: "Kill Process", action: () => killProcess(process.pid), icon: "fa-solid fa-skull"},
        {label: "Show Console", action: () => swapConsole(process), icon: "fa-solid fa-terminal"},
        {label: "Process Info", action: () => showProcessInfo(process), icon: "fa-solid fa-info"}
    ]
</script>

<div id="minecraft-processes" class="relative w-full h-full bg-surface rounded-xl p-2">
    <div class="relative w-full text-center">
        <h1>Minecraft Processes</h1>
    </div>
    <VerticalScroll columns={1} items={[...$processMap.values()].sort(processCompare)} let:prop={process}>
        <div class="w-full h-[3rem] rounded-lg flex gap-5 items-center justify-between p-2 {process.weave_attached ? 'bg-base' : 'bg-surface'}">
            <div class="h-full w-full flex flex-row justify-between items-center">
                <h1 class="w-[33%] text-start">{process.pid}</h1>
                <h1 class="w-[33%] text-center">{process.info.client}</h1>
                <h1 class="w-[33%] text-end">{process.info.version}</h1>
            </div>
            <ButtonBar class="gap-2"  buttons={process.weave_attached ? weaveProcessButton(process) : normalButtons(process)}/>
        </div>
    </VerticalScroll>
    <CreateLaunchProfilePopUp bind:this={popup}/>
</div>