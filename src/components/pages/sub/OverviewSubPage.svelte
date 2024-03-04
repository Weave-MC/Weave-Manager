<script lang="ts">
    import ConsoleOutput from "./items/ConsoleOutput.svelte";
    import ProcessList from "./items/ProcessList.svelte";
    import Analytics from "./items/Analytics.svelte";

    let consoleOutput: ConsoleOutput
</script>

<div class="w-full h-full flex flex-row gap-3">
    <div id="left" class="w-[50%] h-full flex flex-col gap-3">
        <ProcessList on:switch_console={(event) => consoleOutput.switchConsole(event.detail)}/>
        <Analytics/>
    </div>
    <div id="right" class="w-[50%] h-full overflow-clip">
        <ConsoleOutput bind:this={consoleOutput}/>
    </div>
</div>