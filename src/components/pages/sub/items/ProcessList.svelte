<script lang="ts">
    import type {MinecraftProcess} from "../../../../scripts/types";
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import {createLaunchProfile, showProcessInfo} from "../../../../scripts/shared";
    import ButtonBar from "../../../util/ButtonBar.svelte";

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
</script>

<div id="minecraft-processes" class="relative w-full h-full bg-surface rounded-xl p-2">
    <div class="relative w-full text-center">
        <h1>Minecraft Processes</h1>
    </div>
    <VerticalScroll columns={1} items={[...processMap.values()]} let:prop={mcProcess}>
<!--        bg-surface w-full h-[3rem] rounded-lg text-lg text-center flex justify-between overflow-clip py-2-->
        <div class="w-full h-[3rem] rounded-lg flex gap-5 items-center justify-between p-2 bg-surface">
            <div class="h-full w-full flex flex-row justify-between items-center">
                <h1 class="w-[33%] text-start">{mcProcess.pid}</h1>
                <h1 class="w-[33%] text-center">{mcProcess.info.client}</h1>
                <h1 class="w-[33%] text-end">{mcProcess.info.version}</h1>
            </div>
            <ButtonBar class="gap-2" buttons={[
                {label: "Kill Process", action: () => killProcess(mcProcess.pid), icon: "fa-solid fa-skull"},
                {label: "Create Launch Profile", action: () => createLaunchProfile(mcProcess), icon: "fa-solid fa-plus"},
                {label: "Process Info", action: () => showProcessInfo(mcProcess), icon: "fa-solid fa-info"}
            ]}/>
        </div>
    </VerticalScroll>
</div>