<script>
    import { draw } from 'svelte/transition'

    import { extent } from 'd3-array'
    import { scaleLinear } from 'd3-scale'
    import { line, curveBasis } from 'd3-shape'

    export let data = [
        {date: 1, time: 20},
        {date: 2, time: 22},
        {date: 3, time: 18},
        {date: 4, time: 8},
        {date: 5, time: 60},
        {date: 6, time: 19},
        {date: 7, time: 21},
        {date: 8, time: 43},
        {date: 9, time: 24},
        {date: 10, time: 18},
    ];
    export let show = true;
    export let xAxis
    export let yAxis

    const xScale = scaleLinear()
    .domain(extent(data.map(d => d.date)))
    .range([-20, 120])

    const yScale = scaleLinear()
    .domain(extent(data.map(d => d.time)).reverse())
    .range([0, 100])

    const pathLine = line()
    .x(d => xScale(d.date))
    .y(d => yScale(d.time))
    .curve(curveBasis)
</script>

<div id="line-graph" class="w-full h-full">
    <svg class="w-full h-full" viewBox="0 0 100 100">
        {#if (show)}
            <path class="stroke-mauve" transition:draw={{duration: 2000}}
                  d={pathLine(data)} />
        {/if}
    </svg>
</div>


<style>
    path {
        stroke-width: 2;
        fill: none;
        stroke-linecap: round;
    }
</style>