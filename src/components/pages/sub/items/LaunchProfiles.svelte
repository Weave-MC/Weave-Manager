<script lang="ts">
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import type {LaunchProfile} from "../../../../scripts/types";
    import {launchProfiles} from "../../../../scripts/stores";
    import {invoke} from "@tauri-apps/api/tauri";
    import {deleteLaunchProfile, loadModProfile} from "../../../../scripts/components";
    
    function profileSettings(profile: LaunchProfile) {
        
    }

    let cooldown: number
    
    async function launchProfile(profile: LaunchProfile) {
        const now = Date.now()
        if (now - cooldown < 2500) {
            // 2.5 second cooldown between launches
            window.dispatchEvent(
                new CustomEvent("unhandledrejection", {
                    detail: {
                        promise: undefined,
                        reason: new Error(`Launch cooldown active. ${(2.5 - (now - cooldown) / 1000).toFixed(2)}s`)
                    }
                })
            )
            return
        }

        cooldown = Date.now()

        if (profile.mod_profile)
            await loadModProfile(profile.mod_profile)

        await invoke("launch", {profile: profile})
    }
</script>

<div id="launch-profiles" class="relative w-full h-full bg-surface rounded-xl p-2">
    <div class="relative w-full text-center">
        <h1>Launch Profiles</h1>
    </div>
    <VerticalScroll columns={2} items={[...$launchProfiles.values()]} let:prop={profile}>
        <div id="profile-entry" class="relative bg-surface w-full h-full rounded-lg text-lg p-2 grid">
            <button id="profile-delete" class="absolute top-1 right-3" on:click={async () => await deleteLaunchProfile(profile)}>
                <i class="fa-solid fa-trash"></i>
            </button>

            <h1 class="font-semibold flex justify-center items-start text-ellipsis">{profile.name}</h1>  <!-- border-b-2 border-overlay -->
            <h1 class="text-[1rem] flex justify-center items-center text-ellipsis">{profile.mc_info.client} {profile.mc_info.version}</h1>
            <div id="profile-buttons" class="text-[1rem] flex justify-center items-end">
                <button class="bg-overlay rounded-l-lg h-8 w-24" on:click={async () => await launchProfile(profile)}>
                    <i class="fa-solid fa-play mr-2"></i>
                    Launch
                </button>
                <button class="bg-overlay h-8 px-2 rounded-r-lg border-l-2 border-surface" on:click={() => profileSettings(profile)}>
                    <i class="fa-solid fa-gear"></i>
                </button>
            </div>
        </div>
    </VerticalScroll>
</div>

<style>
    #profile-delete {
        opacity: 0;
        transition: opacity 150ms;
    }

    #profile-entry:hover #profile-delete {
        opacity: 100%;
    }

    #profile-entry {
        justify-content: center;
        grid-template-columns: repeat(1, 1fr);
        grid-template-rows: repeat(3, 1fr);
    }
</style>