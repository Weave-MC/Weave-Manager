<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window'
    import weaveIcon from '../weave-icon.png'

    function minimize() {
        appWindow.minimize()
    }

    function hide() {
        appWindow.hide()
    }
</script>

<div data-tauri-drag-region id="header-bar" class="fixed text-xl w-full bg-base h-14 flex items-center">
    <div id="header-buttons" class="absolute right-0 flex justify-end gap-3 p-3">
        <button class="button bg-surface hover:bg-overlay" on:click={minimize}>
            <i class="fa-solid fa-minus"></i>
        </button>
        <button class="button bg-surface hover:bg-disabled" on:click={hide}>
            <i class="fa-solid fa-xmark"></i>
        </button>
    </div>
    <div id="weave-icon-wrapper" class="w-14 h-14 flex justify-center items-center">
        <img src="{weaveIcon}" id="weave-icon" class="w-11 h-11 m-auto" alt="Weave Icon">
    </div>
    <div id="title" class="relative left-5 flex flex-row gap-1.5 items-baseline font-normal">
        <h1 class="text-2xl">Weave Manager</h1>
        <p class="text-xl text-overlay">v1.0.0</p>
    </div>
</div>

<style>
    #header-bar {
        z-index: 1;
    }

    .button {
        @apply h-8 aspect-square text-xl transition-colors duration-300 rounded;
        -webkit-app-region: no-drag;
    }
</style>