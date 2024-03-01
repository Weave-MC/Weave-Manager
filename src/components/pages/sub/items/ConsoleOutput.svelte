<script lang="ts">
    import {shell} from "@tauri-apps/api";
    import {onMount} from "svelte";
    import {listen} from "@tauri-apps/api/event";
    import type {ConsolePayload, MinecraftProcess} from "../../../../scripts/types";
    import LoadSpinner from "../../../../components_old/LoadSpinner.svelte";

    let output: string[] = []
    let currentLogFile: string = ""

    let client: string = "None"
    let pid: number = -1

    let switchingConsole: boolean = false

    export function switchConsole(process: MinecraftProcess) {
        pid = process.pid
        client = process.info.client

        switchingConsole = true
    }

    async function openLogFile() {
        await shell.open(currentLogFile)
    }

    onMount(async () => {
        await listen<ConsolePayload>("console_output", (event) => {
            console.log("console_ouptput event")
            const payload = event.payload

            if (currentLogFile != payload.file_path) {
                console.log(payload.file_path)
                currentLogFile = payload.file_path
                output = []
                switchingConsole = false
            }

            // reactive update
            output = [...output, payload.line]
        })
    })
</script>

<div id="console" class="relative w-full h-full bg-surface rounded-xl text-center overflow-clip p-2">
    <div class="relative w-full text-center">
        <div id="console-popout" class="absolute right-1.5 cursor-pointer">
            <i class="fa-solid fa-arrow-up-right-from-square"></i>
        </div>
        <h1>Console Output</h1>
    </div>

    <div id="console-container" class="absolute bg-crust rounded-lg top-10 bottom-2 left-2 right-2 break-words py-2 pl-2 pr-1">
        <div id="target-process-container" class="w-full h-6 flex items-center pr-1 gap-2">
            <div class="px-2 h-full bg-overlay rounded">
                <h1>Client: {client}</h1>
            </div>
            <div class="px-2 h-full bg-overlay rounded">
                <h1>PID: {pid}</h1>
            </div>
        </div>
        <div id="console-content" class="absolute top-9 bottom-2 left-2 right-1 overflow-y-scroll gap-2 pr-1 text-sm text-start">
            {#if switchingConsole}
                <div class="w-full h-full flex justify-center items-center">
                    <LoadSpinner/>
                </div>
            {:else}
                {#each [...output.values()] as line}
                    <p>{line}</p>
                {/each}
            {/if}
        </div>
    </div>
</div>

<style>
    #console-content {
        font-size: 0.8rem;
    }
    ::-webkit-scrollbar {
        @apply w-1;
    }
    ::-webkit-scrollbar-track {
        @apply bg-none;
    }
    ::-webkit-scrollbar-thumb {
        @apply bg-overlay rounded-xl relative;
    }
</style>