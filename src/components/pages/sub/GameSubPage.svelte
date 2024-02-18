<script lang="ts">
    import VerticalScroll from "../../util/VerticalScroll.svelte";
    import {LaunchProfile, type MinecraftProcess} from "../../../scripts/types";

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
</script>

<div class="w-full h-full flex flex-row gap-3">
    <div id="left" class="w-[50%] h-full bg-surface rounded-xl text-center">
        <div id="launch-profiles" class="relative w-full h-full bg-surface rounded-xl text-center overflow-clip p-2">
            <div class="w-full h-8 text-center">
                <h1>Launch Profiles</h1>
            </div>
            <VerticalScroll columns={2} items={Array.from(profileMap.values())} let:prop={profile}>
                <div id="profile-entry" class="relative bg-surface w-full h-full rounded-lg text-lg p-2 grid">
                    <button id="profile-delete" class="icon-button absolute top-2 right-2">
                        <i class="fa-solid fa-trash"></i>
                    </button>

                    <h1 class="font-semibold flex justify-center items-start leading-[100%]">{profile.name}</h1>  <!-- border-b-2 border-overlay -->
                    <h1 class="text-[1rem] flex justify-center items-center">{profile.mcInfo.client} {profile.mcInfo.version}</h1>
                    <div id="profile-buttons" class="text-[1rem] flex justify-center items-end gap-2">
                        <button class="bg-overlay rounded h-8 w-24">
                            <i class="fa-solid fa-play mr-2"></i>
                            Launch
                        </button>
                        <button class="icon-button">
                            <i class="fa-solid fa-gear"></i>
                        </button>
                    </div>
                </div>
            </VerticalScroll>
        </div>
    </div>
    <div id="right" class="w-[50%] h-full overflow-clip">
        <div id="launch-history" class="w-full h-full bg-surface rounded-xl text-center">History</div>
    </div>
</div>

<style>
    #profile-delete {
        display: none;
    }

    #profile-entry:hover #profile-delete {
        display: block;
    }

    #profile-entry {
        justify-content: center;
        grid-template-columns: repeat(1, 1fr);
        grid-template-rows: repeat(3, 1fr);
    }
</style>