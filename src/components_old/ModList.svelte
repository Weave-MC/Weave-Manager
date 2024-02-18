<script lang="ts">
    import {watchImmediate} from "tauri-plugin-fs-watch-api"
    import {onMount} from "svelte"
    import {open} from "@tauri-apps/api/shell"
    import {renameFile, BaseDirectory, readDir} from "@tauri-apps/api/fs";
    import {homeDir} from "@tauri-apps/api/path";
    import {invoke} from "@tauri-apps/api/tauri";

    type Mod = {
        path: string;
        name: string;
        author: string;
        version: string;
        link: string;
    }

    let modList: Mod[] = []
    let modInfo: Mod
    let infoModal: HTMLDialogElement

    function showInfoModal(mod) {
        modInfo = mod
        infoModal.showModal()
    }

    async function updateModList() {
        const entries = await readDir('.weave/mods', {dir: BaseDirectory.Home})
        modList = await Promise.all(entries
            .filter(e => e.name.includes('.jar') && e.children == null)
            .map(async (e) => <Mod>{
                path: e.path,
                ...await invoke('read_mod_config', { path: e.path })
            })
        )
    }

    async function disableMod(modPath) {
        await renameFile(modPath, modPath + '.disabled')
    }

    async function enableMod(modPath) {
        await renameFile(modPath, modPath.replace('.disabled', ''))
    }

    onMount(async () => {
        await updateModList()

        await watchImmediate(
            `${await homeDir()}/.weave/mods`,
            async () => await updateModList()
        );
    });
</script>

<div id="mod-list" class="w-full h-full">
    <div id="mod-list-title" class="w-full h-8 flex justify-center items-center border-b-2 border-overlay">
        <h1 class="absolute">Weave Mods</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-regular fa-folder-open" on:click={async () => await open(await homeDir() + '.weave/mods')}></i>
        </div>
    </div>
    <div id="content" class="w-full h-full pb-8 overflow-y-auto">
        <div id="list" class="w-full flex flex-col">
            {#each modList as mod (mod.path)}
                <div class="relative mod-item {mod.path.endsWith('.disabled') ? 'bg-crust' : 'bg-surface'}">
                    <p>{mod.name}</p>
                    <p>{mod.version}</p>
                    <div class="mod-buttons w-full h-full absolute top-0 left-0 px-1 py-1 flex flex-row justify-around items-center bg-overlay opacity-0">
                        <button class="mod-button" on:click={() => showInfoModal(mod)}>Info</button>
                        {#if mod.path.endsWith('.disabled')}
                            <button class="mod-button" on:click={async() => await enableMod(mod.path)}>Enable</button>
                        {:else}
                            <button class="mod-button" on:click={async() => await disableMod(mod.path)}>Disable</button>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    </div>
    <dialog bind:this={infoModal} class="modal w-[26rem] h-[14rem]" on:click={modalClicked}>
        {#if modInfo}
            <div class="w-full h-9 border-b-2 border-overlay flex justify-center items-center">Mod Information</div>
            <div class="w-full h-full flex flex-row flex-wrap items-end justify-between pb-3">
                <div class="modal-mod-info-item">
                    <p class="font-semibold">Name</p>
                    <p class="select-text">{modInfo.name}</p>
                </div>
                <div class="modal-mod-info-item">
                    <p class="font-semibold">Version</p>
                    <p class="select-text">{modInfo.version}</p>
                </div>
                <div class="modal-mod-info-item">
                    <p class="font-semibold">Author</p>
                    <p class="select-text">{modInfo.author}</p>
                </div>
                <div class="modal-mod-info-item">
                    <p class="font-semibold">Status</p>
                    <p class="select-text">{modInfo.path.endsWith('.disabled') ? 'Disabled' : 'Enabled'}</p>
                </div>
                <div class="w-full text-center">
                    <p class="text-sm font-semibold">Website</p>
                    <button class="text-sm" on:click={async() => open(modInfo.link)}>{modInfo.link}</button>
                </div>
            </div>
        {/if}
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

    .mod-item {
        @apply relative w-full h-10 border-b-[0.0625rem] border-overlay flex flex-row justify-between px-4 items-center;
    }

    .mod-buttons {
        transition: opacity 0.1s;
    }

    .mod-item:hover .mod-buttons {
        opacity: 1;
    }

    .mod-button {
        @apply w-[30%] h-full bg-surface rounded-xl flex items-center justify-center text-sm font-semibold;
    }

    .modal-mod-info-item {
        @apply w-1/2 text-center text-sm;
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
