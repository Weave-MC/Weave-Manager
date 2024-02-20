<script lang="ts">
    import ConsoleOutput from "./items/ConsoleOutput.svelte";
    import ProcessList from "./items/ProcessList.svelte";
    import Analytics from "./items/Analytics.svelte";

</script>

<div class="w-full h-full flex flex-row gap-3">
    <div id="left" class="w-[50%] h-full flex flex-col gap-3">
        <ProcessList/>
        <Analytics/>
    </div>
    <div id="right" class="w-[50%] h-full overflow-clip">
        <ConsoleOutput/>
    </div>
</div>