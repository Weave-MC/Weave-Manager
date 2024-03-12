<script lang="ts">
    import PopUp from "../util/PopUp.svelte";
    import LoadSpinner from "../util/LoadSpinner.svelte";
    import {relaunch} from "@tauri-apps/api/process";
    import {onMount} from "svelte";
    import {exists} from "@tauri-apps/api/fs";
    import {getWeaveDirectory} from "../../scripts/paths";
    import {installWeave} from "../../scripts/internals";

    let popup: PopUp
    let installing: boolean = false

    onMount(async() => {
        if (!await exists(`${await getWeaveDirectory()}/loader.jar`)) {
            installing = true
            popup.show()
            await installWeave()
            installing = false
        }
    })
</script>

<PopUp bind:this={popup} title="Install Weave" class="w-[20rem] h-[10rem]" on:click-outside={(e) => {e.preventDefault()}}>
    <div class="absolute flex flex-col h-full w-full items-center justify-around">
        {#if installing}
            <div class="w-full h-full flex justify-center items-center">
                <LoadSpinner/>
            </div>
        {:else}
            <h1>Successfully Installed</h1>
            <button class="p-2 rounded bg-overlay" on:click={async() => await relaunch()}>
                <h1>Restart Weave Manager</h1>
            </button>
        {/if}
    </div>
</PopUp>