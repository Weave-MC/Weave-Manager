<script lang="ts">
    import type {MinecraftInfo, MinecraftProcess} from "../../../scripts/types";

    // <pid, MinecraftProcess>
    let processMap = new Map<number, MinecraftProcess>([
        [5728, {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}}],
        [93415, {pid: 93415, info: {client: 'Forge', cwd: 'dummy', cmd: 'dummy', version: '1.7.10'}}],
        [689, {pid: 689, info: {client: 'Feather', cwd: 'dummy', cmd: 'dummy', version: '1.12.2'}}],
        [52746, {pid: 52746, info: {client: 'Vanilla', cwd: 'dummy', cmd: 'dummy', version: '1.20.1'}}],
        [8179, {pid: 8179, info: {client: 'Badlion', cwd: 'dummy', cmd: 'dummy', version: '1.16.5'}}],
        [64302, {pid: 64302, info: {client: 'Other', cwd: 'dummy', cmd: 'dummy', version: '1.11'}}]
    ])

    import {LaunchProfile} from "../../../scripts/types.js";
    import VerticalScroll from "../../util/VerticalScroll.svelte";

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

    async function killProcess(pid: number) {

    }

    async function createLaunchProfile(mcProcess: MinecraftProcess) {

    }

    async function showProcessInfo(mcProcess: MinecraftProcess) {

    }
</script>

<div class="w-full h-full flex flex-row gap-3">
    <div id="left" class="w-[50%] h-full flex flex-col gap-3">
        <div id="minecraft-processes" class="relative w-full h-[66%] bg-surface rounded-xl overflow-clip p-2">
            <div class="w-full h-8 text-center">
                <h1>Minecraft Processes</h1>
            </div>
            <VerticalScroll columns={1} items={[...processMap.values()]} let:prop={mcProcess}>
                <div class="bg-surface w-full h-[3.25rem] rounded-lg text-lg text-center flex justify-between overflow-clip py-2">
                    <div class="h-full w-full flex flex-row justify-between items-center px-2">
                        <h1 class="w-[33%] text-start">{mcProcess.pid}</h1>
                        <h1 class="w-[33%] text-center">{mcProcess.info.client}</h1>
                        <h1 class="w-[33%] text-end">{mcProcess.info.version}</h1>
                    </div>
                    <div class="h-full flex flex-row justify-between items-center  px-2 gap-2"> <!-- border-l-2 border-overlay -->
                        <button class="icon-button" on:click={async() => killProcess(mcProcess.pid)}>
                            <i class="fa-solid fa-skull"></i>
                        </button>
                        <button class="icon-button" on:click={async() => createLaunchProfile(mcProcess)}>
                            <i class="fa-solid fa-plus"></i>
                        </button>
                        <button class="icon-button" on:click={async() => showProcessInfo(mcProcess)}>
                            <i class="fa-solid fa-info"></i>
                        </button>
                    </div>
                </div>
            </VerticalScroll>
        </div>
        <div id="analytics" class="w-full h-[33%] bg-surface rounded-xl text-center">
            <h1>Analytics</h1>
        </div>
    </div>
    <div id="right" class="w-[50%] h-full overflow-clip">
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
</div>

<style>
    .icon-button {
        @apply w-8 h-8 bg-overlay rounded;
        font-size: 1rem;
    }

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

<!-- profile-name profile-info profile-launch profile-settings  -->