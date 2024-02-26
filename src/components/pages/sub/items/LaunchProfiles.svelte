<script lang="ts">
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import {LaunchProfile} from "../../../../scripts/types";

    let profileMap = new Map<string, LaunchProfile>([
        ["Profile 1", new LaunchProfile("Profile 1", {client: "Lunar", version: "1.8.9", cmd: "dummy", cwd: "dummy"})],
        ["Profile 2", new LaunchProfile("Profile 2", {client: "Lunar", version: "1.8.9", cmd: "dummy", cwd: "dummy"})],
        ["Profile 3", new LaunchProfile("Profile 3", {client: "Lunar", version: "1.8.9", cmd: "dummy", cwd: "dummy"})],
        ["Profile 4", new LaunchProfile("Profile 4", {client: "Lunar", version: "1.8.9", cmd: "dummy", cwd: "dummy"})],
        ["Profile 5", new LaunchProfile("Profile 5", {client: "Lunar", version: "1.8.9", cmd: "dummy", cwd: "dummy"})],
        ["Profile 6", new LaunchProfile("Profile 6", {client: "Lunar", version: "1.8.9", cmd: "dummy", cwd: "dummy"})],
        ["Profile 7", new LaunchProfile("Profile 7", {client: "Lunar", version: "1.8.9", cmd: "dummy", cwd: "dummy"})],
        ["Profile 8", new LaunchProfile("Profile 8", {client: "Lunar", version: "1.8.9", cmd: "dummy", cwd: "dummy"})],
        ["Profile 9", new LaunchProfile("Profile 9", {client: "Lunar", version: "1.8.9", cmd: "dummy", cwd: "dummy"})],
    ])
    
    function deleteProfile(profile: LaunchProfile) {
        
    }
    
    function profileSettings(profile: LaunchProfile) {
        
    }
    
    function launchProfile(profile: LaunchProfile) {
        
    }
</script>

<div id="launch-profiles" class="relative w-full h-full bg-surface rounded-xl p-2">
    <div class="relative w-full text-center">
        <h1>Launch Profiles</h1>
    </div>
    <VerticalScroll columns={2} items={Array.from(profileMap.values())} let:prop={profile}>
        <div id="profile-entry" class="relative bg-surface w-full h-full rounded-lg text-lg p-2 grid">
            <button id="profile-delete" class="absolute top-1 right-3" on:click={() => deleteProfile(profile)}>
                <i class="fa-solid fa-trash"></i>
            </button>

            <h1 class="font-semibold flex justify-center items-start text-ellipsis">{profile.name}</h1>  <!-- border-b-2 border-overlay -->
            <h1 class="text-[1rem] flex justify-center items-center text-ellipsis">{profile.mcInfo.client} {profile.mcInfo.version}</h1>
            <div id="profile-buttons" class="text-[1rem] flex justify-center items-end">
                <button class="bg-overlay rounded-l-lg h-8 w-24" on:click={() => launchProfile(profile)}>
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