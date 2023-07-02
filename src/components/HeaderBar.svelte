<div data-tauri-drag-region id="titlebar" class="relative text-xl w-full bg-surface h-10 flex items-center pl-3 gap-3">
    <div id="header-buttons" class="absolute right-0 h-10 flex justify-end gap-2 p-2">
        <ThemeSelection bind:value={value}/>
        <button class="button hover:bg-overlay" on:click={minimize}>
            <i class="fa-solid fa-minus"></i>
        </button>
        <button class="button hover:bg-disabled" on:click={hide}>
            <i class="fa-solid fa-xmark"></i>
        </button>
    </div>
    <img src="{weaveIcon}" id="weave-icon" class="w-7 h-7" alt="Weave Icon">
    <h1>Weave Manager</h1>
</div>

<script lang="ts">
    export let value
    import { appWindow } from '@tauri-apps/api/window'
    import weaveIcon from '../weave-icon.png'
    import ThemeSelection from "./ThemeSelection.svelte";

    function minimize() {
        appWindow.minimize()
    }

    function hide() {
        appWindow.hide()
    }
</script>

<style>
    #titlebar {
        z-index: 1;
    }

    .button {
        @apply h-7 aspect-square text-lg transition-colors duration-300 rounded;
        -webkit-app-region: no-drag;
    }
</style>