<script lang="ts">
    import PopUp from "../util/PopUp.svelte";
    import type {MinecraftInfo} from "../../scripts/types.js";
    import {createLaunchProfile} from "../../scripts/shared.js";

    let popup: PopUp
    let dialogInput: HTMLInputElement
    let launchProfileInfo: MinecraftInfo // the information of the last process to undergo a launch profile creation

    let failed: boolean = false

    export function startCreateLaunchProfile(mcInfo: MinecraftInfo) {
        launchProfileInfo = mcInfo
        popup.show()
    }
    async function finalizeCreateLaunchProfile() {
        const result = await createLaunchProfile(dialogInput.value, launchProfileInfo)
        if (!result) {
            // file already exists
            failed = true
        } else {
            closePopup()
        }
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
        <input bind:this={dialogInput} type="text" class="bg-overlay border-none rounded-lg h-10 w-full outline-none">
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