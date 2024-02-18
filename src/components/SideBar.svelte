<script lang="ts">
    import HomePage from "./pages/HomePage.svelte";
    import SettingsPage from "./pages/SettingsPage.svelte";
    import {shell} from "@tauri-apps/api";

    export let selectedPage: any = HomePage
</script>

<div id="side-bar" class="fixed text-xl h-full bg-base w-14 flex flex-col items-center pt-20">
    <div id="navigation" class="flex flex-col gap-10">
        <button on:click={() => selectedPage = HomePage}>
            <i class="fa-solid fa-house"></i>
        </button>
        <button on:click={() => selectedPage = SettingsPage}>
            <i class="fa-solid fa-gear"></i>
        </button>
        <button on:click={() => shell.open("https://weavemc.net")}>
            <i class="fa-solid fa-earth-americas"></i>
        </button>
    </div>
</div>

<style>
    button {
        @apply text-[1.75rem];
    }
</style>