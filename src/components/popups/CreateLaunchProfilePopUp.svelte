<script lang="ts">
    import PopUp from "../util/PopUp.svelte";
    import type {MinecraftInfo, SelectionOption} from "../../scripts/types.js";
    import {createLaunchProfile} from "../../scripts/components.js";
    import Selection from "../util/Selection.svelte";
    import {modProfiles} from "../../scripts/stores";

    let popup: PopUp
    let dialogInput: HTMLInputElement
    let launchProfileInfo: MinecraftInfo // the information of the last process to undergo a launch profile creation
    let selected: SelectionOption = {
        name: "None",
        value: "None"
    }

    let failed: boolean = false

    export function startCreateLaunchProfile(mcInfo: MinecraftInfo) {
        launchProfileInfo = mcInfo
        popup.show()
    }
    async function finalizeCreateLaunchProfile() {
        let modProfile = null
        if (selected.value != "None")
            modProfile = Array.from($modProfiles.values()).find((profile) => profile.name == selected.value)

        const result = await createLaunchProfile(dialogInput.value, launchProfileInfo, modProfile)

        failed = !result
        if (!failed)
            closePopup()
    }

    function getModProfileOptions(): SelectionOption[] {
        return [{name: "None", value: "None"}, ...Array.from($modProfiles.values()).map((mod) => <SelectionOption> {
            name: mod.name,
            value: mod.name
        })]
    }

    function closePopup() {
        launchProfileInfo = undefined
        failed = false
        popup.close()
    }
</script>

<PopUp bind:this={popup} title="Create Launch Profile" class="w-[20rem]" on:click-outside={() => {launchProfileInfo = undefined; failed = false;}}>
    <div class="w-full h-full flex flex-col justify-center gap-4 p-4 items-center">
        <div class="{failed ? 'visible' : 'hidden'} w-full text-disabled text-center">
            Failed to create Launch Profile
        </div>
        <Selection bind:selected={selected} title="Mod Profile" options={getModProfileOptions()} class="bg-overlay rounded-lg w-full py-2"/>
        <div class="flex flex-col justify-center items-center gap-2 w-full">
            <h1>Profile Name</h1>
            <input bind:this={dialogInput} type="text" class="bg-overlay border-none rounded-lg h-10 w-full outline-none">
        </div>
        <div id="action-buttons" class="flex flex-row justify-around w-full">
            <button class="w-20 h-10 bg-overlay rounded text-text" on:click={finalizeCreateLaunchProfile}>
                Create
            </button>
            <button class="w-20 h-10 bg-surface rounded" on:click={closePopup}>
                Cancel
            </button>
        </div>
    </div>
</PopUp>


<style>
    input:focus {
        box-shadow: none;
    }
</style>