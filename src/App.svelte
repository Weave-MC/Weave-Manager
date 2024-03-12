<script lang="ts">
    import HeaderBar from "./components/HeaderBar.svelte";
    import SideBar from "./components/SideBar.svelte";
    import {onDestroy, onMount} from "svelte";
    import type {ConsolePayload, WeaveProcess} from "./scripts/types";
    import {selectedWeaveProcess, settings, weaveProcessMap} from "./scripts/stores";
    import {listen} from "@tauri-apps/api/event";
    import ErrorModal from "./components/util/ErrorModal.svelte";
    import {updateProcessMap} from "./scripts/internals";
    import InstallPopUp from "./components/popups/InstallPopUp.svelte";
    import UpdateLoaderPopUp from "./components/popups/UpdateLoaderPopUp.svelte";

    let selectedPage: any

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
        }, 1000)
    })

    onDestroy(() => {
        clearInterval(scheduleTask)
    })
</script>

<main id="main" class="theme-{$settings.theme.replace('_', '-').toLowerCase()} absolute w-screen h-screen overflow-clip text-text bg-crust select-none">
    <HeaderBar/>
    <SideBar bind:selectedPage/>
    <div id="content" class="fixed top-14 left-14 right-0 bottom-0 flex flex-col p-3 gap-3">
        <svelte:component this={selectedPage}/>
    </div>
    <InstallPopUp/>
    <UpdateLoaderPopUp/>
    <ErrorModal/>
</main>