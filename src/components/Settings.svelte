<script lang="ts">
    import TogglePill from "./TogglePill.svelte";
    import {onMount} from "svelte";
    import {writeFile, readTextFile, exists, BaseDirectory} from '@tauri-apps/api/fs'

    export let promptRelaunch: boolean
    export let startupRun: boolean
    export let autoUpdate: boolean

    async function writeConfigFile() {
        await writeFile(
            `.weave/manager.json`,
            JSON.stringify({
                prompt_relaunch: promptRelaunch,
                startup_run: startupRun,
                auto_update: autoUpdate
            }),
            {dir: BaseDirectory.Home}
        )
    }

    onMount(async() => {
        if (!await exists('.weave/manager.json', {dir: BaseDirectory.Home})) {
            await writeConfigFile()
        } else {
            const content = await readTextFile(
                '.weave/manager.json',
                {dir: BaseDirectory.Home}
            )

            const json = JSON.parse(content)

            promptRelaunch = json.prompt_relaunch
            startupRun = json.startup_run
            autoUpdate = json.auto_update
        }
    })
</script>

<div id="settings-content" class="w-full h-full">
    <div id="settings-title" class="w-full h-8 flex justify-center items-center border-b-2 border-overlay">
        <h1 class="absolute">Settings</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-gear"></i>
        </div>
    </div>
    <div id="settings" class="flex flex-col w-full h-full pt-2 pb-8 gap-2">
        <div class="setting-toggle">
            <TogglePill bind:enabled="{promptRelaunch}" id="prompt-weave" name="Prompt Relaunch" on:toggle={async() => await writeConfigFile()}/>
        </div>
        <div class="setting-toggle">
            <TogglePill bind:enabled="{startupRun}" id="startup-run" name="Run on Startup" on:toggle={async() => await writeConfigFile()}/>
        </div>
        <div class="setting-toggle">
            <TogglePill bind:enabled="{autoUpdate}" id="auto-update" name="Auto Update" on:toggle={async() => await writeConfigFile()}/>
        </div>
    </div>
</div>

<style>
    .setting-toggle {
        @apply w-full h-8 flex flex-row items-center pl-2;
    }
</style>