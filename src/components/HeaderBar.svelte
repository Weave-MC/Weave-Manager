<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window'

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
        <svg viewBox="0 0 300 300" class="fill-accent w-12 h-12 m-auto">
            <path d="M213.359 57.92c7.916-11.875 25.366-11.875 33.282 0l33.991 50.986a20.002 20.002 0 0 1 0 22.188L206.641 242.08c-7.916 11.875-25.366 11.875-33.282 0L150 207.042l-.007.01-18.972-28.484-.002.002L70 87.042 48.028 120 110 212.958l7.519-11.278 9.682-14.524 18.02 27.055-2.74 4.11-15.84 23.759c-7.916 11.875-25.366 11.875-33.282 0l-73.99-110.986a20 20 0 0 1 0-22.188l33.99-50.986c7.916-11.875 25.366-11.875 33.282 0L150 152.958l.013-.019 18.019 27.054-.004.007L190 212.958 251.972 120 230 87.042l-27.519 41.279-29.688 44.532-18.02-27.055 22.746-34.118 35.84-53.76Z" clip-rule="evenodd"/>
        </svg>
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