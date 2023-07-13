<script lang="ts">
    import TogglePill from "./TogglePill.svelte";
    import {onMount, createEventDispatcher} from "svelte";
    import {writeFile, readTextFile, exists, BaseDirectory} from '@tauri-apps/api/fs'

    const dispatch = createEventDispatcher()

    let theme: string = 'theme-darcula'
    let promptRelaunch: boolean
    let startupRun: boolean
    let autoUpdate: boolean

    // the theme argument will be passed from App.svelte to change theme
    // otherwise when called from Settings.svelte it will pass `let theme: string`
    export async function writeConfigFile(_theme) {
        theme = _theme

        const settings = {
            prompt_relaunch: promptRelaunch,
            startup_run: startupRun,
            auto_update: autoUpdate,
            theme: theme
        }

        await writeFile(
            `.weave/manager.json`,
            JSON.stringify(settings),
            {dir: BaseDirectory.Home}
        )

        dispatch('update', settings)
    }

    onMount(async() => {
        if (!await exists('.weave/manager.json', {dir: BaseDirectory.Home})) {
            await writeConfigFile(theme)
        } else {
            const content = await readTextFile(
                '.weave/manager.json',
                {dir: BaseDirectory.Home}
            )

            const json = JSON.parse(content)

            // Check if any of these keys are missing in the JSON file
            // If they are, add the key with a default value
            const requiredKeys = {
                prompt_relaunch: true,
                startup_run: true,
                auto_update: true,
                theme: 'theme-darcula'
            }
            const missingKeys = Object.keys(requiredKeys).filter(key => !(key in json));
            if (missingKeys.length > 0) {
                missingKeys.forEach(key => {
                    json[key] = requiredKeys[key];
                })

                await writeFile(
                    `.weave/manager.json`,
                    JSON.stringify(json),
                    {dir: BaseDirectory.Home}
                )
            }

            promptRelaunch = json.prompt_relaunch
            startupRun = json.startup_run
            autoUpdate = json.auto_update
            theme = json.theme

            dispatch('update', {
                prompt_relaunch: promptRelaunch,
                startup_run: startupRun,
                auto_update: autoUpdate,
                theme: theme
            })
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
            <TogglePill bind:enabled="{promptRelaunch}" id="prompt-weave" name="Prompt Relaunch" on:toggle={async() => await writeConfigFile(theme)}/>
        </div>
        <div class="setting-toggle">
            <TogglePill bind:enabled="{startupRun}" id="startup-run" name="Run on Startup" on:toggle={async() => await writeConfigFile(theme)}/>
        </div>
        <div class="setting-toggle">
            <TogglePill bind:enabled="{autoUpdate}" id="auto-update" name="Auto Update" on:toggle={async() => await writeConfigFile(theme)}/>
        </div>
    </div>
</div>

<style>
    .setting-toggle {
        @apply w-full h-8 flex flex-row items-center pl-2;
    }
</style>