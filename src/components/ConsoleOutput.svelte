<script lang="ts">
    import {listen} from '@tauri-apps/api/event'
    import {onMount} from "svelte";
    import LoadSpinner from "./LoadSpinner.svelte";
    import {shell} from "@tauri-apps/api";

    let output: string[] = []
    let currentLogFile: string = ""

    let switchingConsole: boolean = false

    export function switchConsole() {
        switchingConsole = true
    }

    async function openLogFile() {
        await shell.open(currentLogFile)
    }

    onMount(async () => {
        await listen('console_output', (event) => {
            const {line, file_path} = event.payload

            if (currentLogFile != file_path) {
                console.log(file_path)
                currentLogFile = file_path
                output = []
                switchingConsole = false
            }

            output = [...output, line]
        })
    })
</script>

<div id="console-output" class="w-full h-full">
    <div id="console-output-title" class="relative w-full h-8 flex justify-center items-center border-b-2 border-overlay">
        <h1 class="absolute">Console Output</h1>
        {#if currentLogFile.length > 0}
            <div class="flex justify-end px-2 cursor-pointer" on:click={async() => await openLogFile()}>
                <i class="fa-solid fa-up-right-from-square"></i>
            </div>
        {/if}
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-terminal"></i>
        </div>
    </div>
    <div id="output-content" class="w-full h-full pb-8 pr-1">
        {#if switchingConsole}
            <div id="console-swap" class="w-full h-full flex justify-center items-center">
                <LoadSpinner/>
            </div>
        {:else}
            <div id="output" class="w-full h-full flex flex-col pl-0.5 text-xs overflow-y-scroll break-words gap-1">
                {#each [...output.values()] as line}
                    <p>{line}</p>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
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