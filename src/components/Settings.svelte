<script lang="ts">
    import TogglePill from "./TogglePill.svelte";
    import {onMount, createEventDispatcher} from "svelte";
    import {writeFile, readTextFile, exists, BaseDirectory} from '@tauri-apps/api/fs'
    import {enable, isEnabled, disable} from "tauri-plugin-autostart-api";

    const dispatch = createEventDispatcher()

    type Config = {
        prompt_relaunch: boolean
        startup_run: boolean
        auto_update: boolean
        theme: string
        loader_version: string
    }

    const defaultKeys: Config = {
        prompt_relaunch: true,
        startup_run: true,
        auto_update: true,
        theme: 'theme-darcula',
        loader_version: 'none'
    }
    let settings: Config = defaultKeys

    export async function writeSetting(key, value) {
        settings[key] = value

        await writeFile(
            `.weave/manager.json`,
            JSON.stringify(settings),
            {dir: BaseDirectory.Home}
        )

        dispatch('update', settings)
    }

    async function writeSettings() {
        await writeFile(
            `.weave/manager.json`,
            JSON.stringify(settings),
            {dir: BaseDirectory.Home}
        )

        dispatch('update', settings)
    }

    async function toggleRunOnStartup() {
       await writeSettings()

        const enabled = await isEnabled()
        if (settings.startup_run !== enabled) {
            if (settings.startup_run)
                await enable();
            else
                await disable();
        }
    }

    async function readConfig() {
        const content = await readTextFile(
            '.weave/manager.json',
            {dir: BaseDirectory.Home}
        )
        const json = JSON.parse(content)

        const missingKeys = Object.keys(defaultKeys).filter((key => !(key in json)))
        if (missingKeys.length > 0) {
            missingKeys.forEach((key) => {
                json[key] = defaultKeys[key]
            })

            await writeFile(
                `.weave/manager.json`,
                JSON.stringify(json),
                {dir: BaseDirectory.Home}
            )
        }

        // make sure that json has the proper keys, if not use default value
        settings = {
            prompt_relaunch: json.prompt_relaunch ?? defaultKeys.prompt_relaunch,
            startup_run: json.startup_run ?? defaultKeys.startup_run,
            auto_update: json.auto_update ?? defaultKeys.auto_update,
            theme: json.theme ?? defaultKeys.theme,
            loader_version: json.loader_version ?? defaultKeys.loader_version,
        }
    }

    onMount(async() => {
        if (!await exists('.weave/manager.json', {dir: BaseDirectory.Home})) {
            // Writes the settings file, and enables run on startup
            await toggleRunOnStartup()
        } else {
            await readConfig()

            dispatch('update', settings)
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
            <TogglePill bind:enabled="{settings.prompt_relaunch}" id="prompt-weave" name="Prompt Relaunch" on:toggle={async() => await writeSettings()}/>
        </div>
        <div class="setting-toggle">
            <TogglePill bind:enabled="{settings.startup_run}" id="startup-run" name="Run on Startup" on:toggle={async() => await toggleRunOnStartup()}/>
        </div>
        <div class="setting-toggle">
            <TogglePill bind:enabled="{settings.auto_update}" id="auto-update" name="Auto Update" on:toggle={async() => await writeSettings()}/>
        </div>
    </div>
</div>

<style>
    .setting-toggle {
        @apply w-full h-8 flex flex-row items-center pl-2;
    }
</style>