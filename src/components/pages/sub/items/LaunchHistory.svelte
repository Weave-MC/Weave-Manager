<script lang="ts">
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import ButtonBar from "../../../util/ButtonBar.svelte";
    import {showProcessInfo} from "../../../../scripts/components";
    import {relativeTime} from "../../../../scripts/utils";
    import CreateLaunchProfilePopUp from "../../../popups/CreateLaunchProfilePopUp.svelte";
    import {processHistory} from "../../../../scripts/stores";

    let popup: CreateLaunchProfilePopUp
</script>

<div id="launch-history" class="relative w-full h-full bg-surface rounded-xl text-center overflow-clip p-2">
    <div class="relative w-full text-center">
        <h1>History</h1>
    </div>
    <VerticalScroll items={$processHistory.history} let:prop={process}>
        <div class="bg-surface w-full h-[2.5rem] rounded-lg flex justify-between items-center p-2 gap-5">
            <div id="process-info" class="w-full h-full flex gap-2 items-center justify-between">
                <h1>{process.info.client}</h1>
                <h1>{process.info.version}</h1>
                <h1>{relativeTime(process.start_time)}</h1>
            </div>
            <ButtonBar class="gap-2" buttons={[
                {label: "Create Launch Profile", action: () => popup.startCreateLaunchProfile(process.info), icon: "fa-solid fa-plus"},
                {label: "Process Info", action: () => showProcessInfo(process), icon: "fa-solid fa-info"}
            ]}/>
        </div>
    </VerticalScroll>
    <CreateLaunchProfilePopUp bind:this={popup}/>
</div>