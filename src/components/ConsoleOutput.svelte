<script lang="ts">
    import {listen} from '@tauri-apps/api/event'
    import {onMount} from "svelte";

    // const unlisten = await listen('console_output', (event) => {
    //     console.log(event)
    // })

    let output: string[] = []
    let currentPid: number = 0

    onMount(async () => {
        await listen('console_output', (event) => {
            const {line, pid} = event.payload

            if (currentPid != pid) {
                currentPid = pid
                output = []
            }


            output = [...output, line]
        })
    })
</script>

<div id="console-output" class="w-full h-full">
    <div id="console-output-title" class="w-full h-8 flex justify-center items-center border-b-2 border-overlay">
        <h1 class="absolute">Console Output</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-terminal"></i>
        </div>
    </div>
    <div id="output-content" class="w-full h-full pb-8 pr-1">
        <div id="output" class="w-full h-full flex overflow-y-scroll flex flex-col pl-0.5 text-xs break-words gap-1">
            {#each [...output.values()] as line}
                <p>{line}</p>
            {/each}
        </div>
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