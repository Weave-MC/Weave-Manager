<script lang="ts">
    import {onMount} from "svelte";
    import {listen} from "@tauri-apps/api/event";
    import {invoke} from "@tauri-apps/api/tauri";
    import {copyFile} from "@tauri-apps/api/fs";
    import {homeDir} from "@tauri-apps/api/path";

    let dragging: boolean = false
    let failedFiles: string[] = []
    let dropzoneModal: HTMLDialogElement

    function closeDropZone() {
        dragging = false
        dropzoneModal.close()
    }

    onMount(async() => {
        await listen('tauri://file-drop', async event => {
            dragging = false
            const files = event.payload as string[]

            for (let i in files) {
                const file = files[i]

                if (!file.endsWith('.jar')) {
                    failedFiles = [...failedFiles, file]
                    continue
                }

                const modConfig = await invoke('read_mod_config', {path: file})

                if (!modConfig) {
                    failedFiles = [...failedFiles, file]
                    continue
                }

                const filename = file.split(/[\\/]/).pop();
                await copyFile(file, `${await homeDir()}/.weave/mods/${filename}`)
            }

            if (failedFiles.length > 0)
                dropzoneModal.showModal()
        })
        await listen('tauri://file-drop-hover', () => {
            dragging = true
            dropzoneModal.close()
            failedFiles = []
        })
        await listen('tauri://file-drop-cancelled', () => {
            dragging = false
        })
    })
</script>

<div id="dropzone-content" class="absolute let-0 top-0 w-full h-full items-center justify-center">
    <div id="dropzone" class="absolute left-0 top-0 w-full h-full flex items-center justify-center flex-col gap-10 text-xl font-semibold pb-32 opacity-0 backdrop-blur-sm {dragging ? 'visible' : ''}">
        <h1>Drop to add Weave Mod(s)</h1>
        <i class="fa-solid fa-arrow-up-from-bracket"></i>
    </div>
    <dialog bind:this={dropzoneModal} id="dropzone-modal" class="w-[26rem] h-[14rem] text-text" on:click={modalClicked}>
        <div class="w-full h-9 border-b-2 border-overlay flex justify-center items-center font-bold">You have dropped files that are not Weave Mods</div>
        {#each [...failedFiles.values()] as file}
            <p>{file}</p>
        {/each}
    </dialog>
</div>



<style>
    #dropzone-content {
        pointer-events: none;
    }
    #dropzone {
        background-color: rgba(0, 0, 0, 0.3);
        transition: 0.3s all;
    }
    #dropzone-modal {
        @apply px-4 py-1 fixed top-0 bottom-0 flex flex-col bg-surface rounded-xl items-center;
        scale: 0;
        opacity: 0;
        box-shadow: 0 0 1rem 5px theme('backgroundColor.disabled');
        transition: all 350ms ease-in-out;
        pointer-events: all;
    }
    #dropzone-modal::backdrop {
        background-color: rgba(0, 0, 0, 0.2);
    }

    #dropzone-modal:focus {
        outline: none;
    }

    #dropzone-modal[open] {
        scale: 1;
        opacity: 1;
    }
    .visible {
        opacity: 1;
    }
    i {
        font-size: 2.5rem;
    }
</style>