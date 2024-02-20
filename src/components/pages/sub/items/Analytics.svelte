<div id="analytics" class="relative w-full h-[40%] bg-surface rounded-xl text-center p-2 flex flex-col gap-2">
    <div class="w-full text-center">
        <h1>Analytics</h1>
    </div>
    <div id="analytics-container" class="bg-crust h-full w-full rounded-lg pl-2 pr-4 pt-2 pb-4">
        <div id="ram-usage" class="analytic">
            <h1 class="font-semibold">Ram Usage</h1>
            <h1>0.01%</h1>
        </div>
        <div id="process-count" class="analytic">
            <h1 class="font-semibold">MC Processes</h1>
            <h1>10 Processes</h1>
        </div>
        <div id="time-played" class="analytic">
            <h1 class="font-semibold">Time Played</h1>
            <h1>10 Hours</h1>
        </div>
        <div id="avg-launch" class="analytic">
            <h1 class="font-semibold">Avg. Launch</h1>
            <h1>10 Seconds</h1>
        </div>
    </div>
</div>

<style>
    #analytics-container {
        @apply gap-2;
        display: grid;
        grid-template-columns: repeat(2, 50%);
        grid-template-rows: repeat(2, 50%);
    }
    .analytic {
        @apply bg-overlay w-full h-full rounded-lg flex flex-col justify-around;
    }
</style>