<script lang="ts">
    import type {MinecraftProcess} from "../../../../scripts/types";
    import VerticalScroll from "../../../util/VerticalScroll.svelte";

    // <pid, MinecraftProcess>
    let processMap = new Map<number, MinecraftProcess>([
        [5728, {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}}],
        [93415, {pid: 93415, info: {client: 'Forge', cwd: 'dummy', cmd: 'dummy', version: '1.7.10'}}],
        [689, {pid: 689, info: {client: 'Feather', cwd: 'dummy', cmd: 'dummy', version: '1.12.2'}}],
        [52746, {pid: 52746, info: {client: 'Vanilla', cwd: 'dummy', cmd: 'dummy', version: '1.20.1'}}],
        [8179, {pid: 8179, info: {client: 'Badlion', cwd: 'dummy', cmd: 'dummy', version: '1.16.5'}}],
        [64302, {pid: 64302, info: {client: 'Other', cwd: 'dummy', cmd: 'dummy', version: '1.11'}}]
    ])

    async function killProcess(pid: number) {

    }

    async function showProcessInfo(mcProcess: MinecraftProcess) {

    }

    async function createLaunchProfile(mcProcess: MinecraftProcess) {

    }
</script>

<div id="minecraft-processes" class="relative w-full h-[60%] bg-surface rounded-xl overflow-clip p-2">
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
            <div class="h-full flex flex-row justify-between items-center px-2 gap-2"> <!-- border-l-2 border-overlay -->
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

<style>
    .icon-button {
        @apply w-8 h-8 bg-overlay rounded;
        font-size: 1rem;
    }
</style>
