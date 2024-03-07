<script lang="ts">
    import HeaderBar from "./components/HeaderBar.svelte";
    import SideBar from "./components/SideBar.svelte";
    import {onDestroy, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import type {ConsolePayload, MinecraftProcess, WeaveProcess} from "./scripts/types";
    import {processHistory, processMap, selectedWeaveProcess, settings, weaveProcessMap} from "./scripts/store";
    import {listen} from "@tauri-apps/api/event";
    import ErrorModal from "./components/util/ErrorModal.svelte";
    import {getHistoryLogsDirectory} from "./scripts/shared";
    import {writeTextFile} from "@tauri-apps/api/fs";

    let selectedPage: any

    export async function updateProcessMap() {
        try {
            const newMap = new Map()

            const fetched = await invoke<MinecraftProcess[]>("fetch_minecraft_processes")
            for (const proc of fetched) {
                if (!$processMap.has(proc.pid)) {
                    if (!proc.weave_attached)
                        await logProcess(proc)
                }
                newMap.set(proc.pid, proc)
            }

            processMap.set(newMap)
        } catch (err) {
            console.error(err)
        }
    }

    export async function logProcess(process: MinecraftProcess) {
        processHistory.update((history) => {
            history.history.push(process)
            return history
        })

        const historyFile = `${await getHistoryLogsDirectory()}/history.log`
        await writeTextFile(historyFile, JSON.stringify($processHistory))
    }

    let scheduleTask: NodeJS.Timer
    onMount(async () => {
        await listen<ConsolePayload>("console_output", (event) => {
            $selectedWeaveProcess.output = [...$selectedWeaveProcess.output, event.payload.line]
        })
        await listen<WeaveProcess>("spawned_weave", (event) => {
            $selectedWeaveProcess = event.payload
            $weaveProcessMap.set(event.payload.pid, event.payload)
        })

        scheduleTask = setInterval(async () => {
            await updateProcessMap()
            console.log($settings.theme.toLowerCase())
        }, 1000)
    })

    onDestroy(() => {
        clearInterval(scheduleTask)
    })
</script>

<main id="main" class="{$settings.theme.toLowerCase()} absolute w-screen h-screen overflow-clip text-text bg-crust select-none">
    <HeaderBar/>
    <SideBar bind:selectedPage/>
    <div id="content" class="fixed top-14 left-14 right-0 bottom-0 flex flex-col p-3 gap-3">
        <svelte:component this={selectedPage}/>
    </div>
    <ErrorModal/>
</main>