<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import {checkForLoaderUpdate, downloadWeaveLoader} from "../../scripts/internals";
    import PopUp from "../util/PopUp.svelte";
    import {settings} from "../../scripts/stores";
    import type {LoaderUpdateResponse} from "../../scripts/types";
    import {exists} from "@tauri-apps/api/fs";
    import {getWeaveDirectory} from "../../scripts/paths";
    import LoadSpinner from "../util/LoadSpinner.svelte";

    let popup: PopUp
    let updating: boolean = false
    let autoUpdate: boolean = false

    let updateResponse: LoaderUpdateResponse = undefined

    async function updateWeaveLoader() {
        updating = true

        await downloadWeaveLoader(updateResponse.download_url, updateResponse.version)

        popup.close()

        updating = false
        autoUpdate = false
    }

    async function checkForUpdate() {
        if ($settings.ignore_updates)
            return

        updateResponse = await checkForLoaderUpdate()

        if (updateResponse.update) {
            popup.show()

            if ($settings.auto_update) {
                autoUpdate = true
                await updateWeaveLoader()
            }
        }
    }

    let scheduleTask: NodeJS.Timer
    onMount(async() => {
        if (await exists(`${await getWeaveDirectory()}/loader.jar`))
            await checkForUpdate()

        scheduleTask = setInterval(async() => {
            await checkForUpdate()
        }, 30_000 * 60)
    })

    onDestroy(() => {
        clearInterval(scheduleTask)
    })
</script>

<PopUp bind:this={popup} title="Update Available" class="w-[20rem] h-[10rem]" on:click-outside={(e) => e.preventDefault()}>
    {#if updateResponse !== undefined}
        <div class="flex flex-col w-full h-full items-center justify-around p-2">
            {#if !autoUpdate && !updating}
                <h1 class="text-lg">Weave-Loader {updateResponse.version}</h1>
                <button class="p-2 bg-overlay rounded-lg" on:click={async() => await updateWeaveLoader()}>
                    <h1>Update</h1>
                </button>
            {:else if updating}
                <h1>Updating Weave-Loader to {updateResponse.version}</h1>
                <div class="w-full h-full flex justify-center items-center">
                    <LoadSpinner/>
                </div>
            {/if}
        </div>
    {/if}
</PopUp>