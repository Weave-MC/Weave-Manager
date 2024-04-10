<script lang="ts">
    import CheckboxSetting from "../util/settings/CheckboxSetting.svelte";
    import {settings} from "../../scripts/stores.js";
    import UpdateConfirmation from "../util/settings/UpdateConfirmation.svelte";
    import {type SelectionOption, type Settings, Themes} from "../../scripts/types";
    import {writeTextFile} from "@tauri-apps/api/fs";
    import {getWeaveDirectory} from "../../scripts/paths";
    import SelectionSetting from "../util/settings/SelectionSetting.svelte";
    import {enable, isEnabled, disable} from "tauri-plugin-autostart-api"

    let updateConfirmation: UpdateConfirmation

    let temporarySettings: Settings = {...$settings}

    const themesArray: SelectionOption[] = Object.keys(Themes).map((key) => <SelectionOption> {
        name: Themes[key],
        value: key
    })
    let selectedTheme: SelectionOption = {
        name: Themes[temporarySettings.theme],
        value: Themes[temporarySettings.theme]
    }

    async function confirmUpdate() {
        if (temporarySettings.startup_run != $settings.startup_run) {
            if (temporarySettings.startup_run)
                await enable()
            else
                await disable()
        }

        temporarySettings.theme = selectedTheme.value
        $settings = {...temporarySettings}

        const settingsFile = `${await getWeaveDirectory()}/manager.settings`
        await writeTextFile(settingsFile, JSON.stringify(temporarySettings))
    }

    function resetUpdate() {
        temporarySettings = {...$settings}
    }
</script>

<div id="settings" class="relative w-full h-full px-24 py-6 flex flex-col overflow-y-scroll gap-6">
    <SelectionSetting
            title="Theme Selection"
            description=""
            options={themesArray}
            bind:value={selectedTheme}
            on:select={() => updateConfirmation.display()}
    />
    <CheckboxSetting
            bind:enabled={temporarySettings.auto_update}
            title="Auto Update"
            description="Automatically updates Weave-Loader and Weave-Manager"
            on:toggle={() => updateConfirmation.display()}
    />
    <CheckboxSetting
        bind:enabled={temporarySettings.ignore_updates}
        title="Ignore Updates"
        description="Prevents update checks for Weave-Loader"
        on:toggle={() => updateConfirmation.display()}
    />
    <CheckboxSetting
            bind:enabled={temporarySettings.startup_run}
            title="Launch on Startup"
            description="If enabled, Weave Manager will launch when you startup your Computer"
            on:toggle={() => updateConfirmation.display()}
    />
    <CheckboxSetting
            bind:enabled={temporarySettings.compact_buttons}
            title="Compact Buttons"
            description="Compact buttons into a single button with dropdown selection"
            on:toggle={() => updateConfirmation.display()}
    />
</div>
<UpdateConfirmation bind:this={updateConfirmation} on:confirm={async() => await confirmUpdate()} on:reset={resetUpdate}/>

<style>
    ::-webkit-scrollbar {
        @apply w-1;
    }
    ::-webkit-scrollbar-track {
        @apply bg-none;
    }
    ::-webkit-scrollbar-thumb {
        @apply bg-overlay rounded-xl relative;
    }
</style>