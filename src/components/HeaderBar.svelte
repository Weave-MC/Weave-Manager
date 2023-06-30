<div data-tauri-drag-region class="titlebar relative text-xl w-full bg-base h-10 flex items-center pl-3 gap-3">
    <div id="header-buttons" class="absolute right-0 h-10 flex justify-end gap-2 p-2">
        <button class="button hover:bg-overlay2" on:click={minimize}>
            <i class="fa-solid fa-minus"></i>
        </button>
        <button class="button hover:bg-red" on:click={close}>
            <i class="fa-solid fa-xmark"></i>
        </button>
    </div>
    <img src="icons/weave-icon.png" class="w-7 h-7" alt="Weave Icon">
    <h1>Weave Manager</h1>
</div>

<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window'

    function minimize() {
        appWindow.minimize()
    }

    function close() {
        appWindow.minimize()
    }
</script>

<style>
    #header {
        z-index: 1;
    }

    .button {
        @apply h-7 aspect-square text-lg transition-colors duration-300 rounded;
        -webkit-app-region: no-drag;
        text-shadow: -1px -1px 0 #00000030, 1px -1px 0 #00000030, -1px 1px 0 #00000030, 1px 1px 0 #00000030;
    }
</style>