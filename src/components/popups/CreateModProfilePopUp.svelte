<script lang="ts">
    import PopUp from "../util/PopUp.svelte";
    import type {Mod} from "../../scripts/types.js";
    import {createModProfile} from "../../scripts/components.js";

    let popup: PopUp
    let dialogInput: HTMLInputElement

    let failed: boolean = false

    let mods: Mod[]

    export function startCreateModProfile(enabledMods: Mod[]) {
        mods = enabledMods
        popup.show()
    }
    async function finalizeCreateModProfile() {
        const result = await createModProfile(dialogInput.value, mods)

        failed = !result
        if (!failed) {
            closePopup()
            dialogInput.value = ""
        }
    }

    function closePopup() {
        mods = undefined
        failed = false
        popup.close()
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === "Enter")
            finalizeCreateModProfile()
    }
</script>

<PopUp bind:this={popup} title="Create Mod Profile" class="w-[20rem]" on:keydown={handleKeyDown}>
    <div class="w-full h-full flex flex-col justify-center gap-4 p-4 items-center">
        <div class="{failed ? 'visible' : 'hidden'} w-full text-disabled text-center">
            Failed to create Mod Profile
        </div>
        <div class="flex flex-col justify-center items-center gap-2 w-full">
            <h1>Profile Name</h1>
            <input bind:this={dialogInput} type="text" class="bg-overlay border-none rounded-lg h-10 w-full outline-none">
        </div>
        <div id="action-buttons" class="flex flex-row justify-around w-full">
            <button class="w-20 h-10 bg-overlay rounded text-text" on:click={finalizeCreateModProfile}>
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