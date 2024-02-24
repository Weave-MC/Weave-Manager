<script lang="ts">
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import type {MinecraftProcess} from "../../../../scripts/types";
    import ButtonBar from "../../../util/ButtonBar.svelte";
    import {createLaunchProfile, showProcessInfo} from "../../../../scripts/shared";

    const history: MinecraftProcess[] = [
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}},
        {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}}
    ]
</script>

<div id="launch-history" class="relative w-full h-full bg-surface rounded-xl text-center overflow-clip p-2">
    <div class="relative w-full text-center">
        <h1>History</h1>
    </div>
    <VerticalScroll items={history} let:prop={process}>
        <div class="bg-surface w-full h-[2.5rem] rounded-lg flex justify-between items-center p-2 gap-5">
            <div id="process-info" class="w-full h-full flex gap-2 items-center justify-between">
                <h1>{process.info.client}</h1>
                <h1>{process.info.version}</h1>
            </div>
            <ButtonBar class="gap-2" buttons={[
                {label: "Create Launch Profile", action: () => createLaunchProfile(process), icon: "fa-solid fa-plus"},
                {label: "Process Info", action: () => showProcessInfo(process), icon: "fa-solid fa-info"}
            ]}/>
        </div>
    </VerticalScroll>
</div>