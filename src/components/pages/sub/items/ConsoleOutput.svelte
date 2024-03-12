<script lang="ts">
    import {shell} from "@tauri-apps/api";
    import LoadSpinner from "../../../util/LoadSpinner.svelte";
    import type {MinecraftProcess} from "../../../../scripts/types";
    import {selectedWeaveProcess, weaveProcessMap} from "../../../../scripts/stores";
    import {readTextFile} from "@tauri-apps/api/fs";

    async function openLogFile() {
        if ($selectedWeaveProcess)
            await shell.open($selectedWeaveProcess.log_file)
    }
    export async function switchConsole(process: MinecraftProcess) {
        const weaveProcess = $weaveProcessMap.get(process.pid)

        if (weaveProcess) {
            weaveProcess.output = (await readTextFile(weaveProcess.log_file)).split("\n")
            $selectedWeaveProcess = weaveProcess
        }
    }
</script>

<div id="console" class="relative w-full h-full bg-surface rounded-xl text-center overflow-clip p-2">
    <div class="relative w-full text-center">
        <button id="console-popout" class="absolute right-1.5 cursor-pointer" on:click={async () => openLogFile()}>
            <i class="fa-solid fa-arrow-up-right-from-square"></i>
        </button>
        <h1>Console Output</h1>
    </div>

    <div id="console-container" class="absolute bg-crust rounded-lg top-10 bottom-2 left-2 right-2 break-words py-2 pl-2 pr-1">
        <div id="target-process-container" class="w-full h-6 flex items-center pr-1 gap-2">
            <div class="px-2 h-full bg-overlay rounded">
                <h1>Client: {$selectedWeaveProcess.client}</h1>
            </div>
            <div class="px-2 h-full bg-overlay rounded">
                <h1>PID: {$selectedWeaveProcess.pid}</h1>
            </div>
        </div>
        <div id="console-content" class="absolute top-9 bottom-2 left-2 right-1 overflow-y-scroll gap-2 pr-1 text-sm text-start">
            {#if $selectedWeaveProcess.output.length === 0 && $selectedWeaveProcess.pid !== -1}
                <div class="w-full h-full flex justify-center items-center">
                    <LoadSpinner/>
                </div>
            {:else}
                {#each [...$selectedWeaveProcess.output.values()] as line}
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