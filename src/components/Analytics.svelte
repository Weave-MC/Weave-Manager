<script>
    import LineGraph from "./LineGraph.svelte";
    import Statistic from "./Statistic.svelte";
</script>

<div id="analytics" class="w-full h-full">
    <div id="analytics-title" class="w-full h-8 flex justify-center items-center border-b-2 border-surface1">
        <h1 class="absolute">Analytics</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-chart-simple"></i>
        </div>
    </div>
    <div id="content" class="w-full h-full pb-8">
        <div id="statistics" class="w-full h-full flex flex-row flex-wrap items-center justify-center">
            <Statistic name="Hours Played" value="10"/>
            <Statistic name="Avg. Launch" value="32.3s"/>
            <Statistic name="Instances" value="1"/>
            <Statistic name="Mem Usage" value="5%"/>
        </div>
    </div>
</div>

<style>
    .statistic {

    }
</style>