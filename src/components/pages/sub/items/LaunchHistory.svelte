<script lang="ts">
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import type {MinecraftProcess} from "../../../../scripts/types";

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
        <div class="relative bg-surface w-full h-[2.5rem] rounded-lg flex justify-between items-center p-2">
            <div id="process-info" class="w-[65%] h-full flex gap-2 items-center justify-between">
                <h1>{process.info.client}</h1>
                <h1>{process.info.version}</h1>
            </div>
            <div id="process-buttons" class="h-full flex gap-2 items-center">
                <button id="save-profile" class="icon-button">
                    <i class="fa-solid fa-plus"></i>
                </button>
                <button id="process-info" class="icon-button">
                    <i class="fa-solid fa-info"></i>
                </button>
            </div>
        </div>
    </VerticalScroll>
</div>

<style>
    .icon-button {
        @apply w-7 h-7 bg-overlay rounded;
        font-size: 1rem;
    }
</style>