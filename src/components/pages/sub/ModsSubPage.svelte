<script>
    import AgentList from "./items/AgentList.svelte";
    import ModList from "./items/ModList.svelte";
    import ModProfiles from "./items/ModProfiles.svelte";
</script>

<div class="w-full h-full flex flex-row gap-3">
    <div id="left" class="w-[50%] h-full">
        <ModList/>
    </div>
    <div id="right" class="w-[50%] h-full flex flex-col gap-3">
        <div id="mod-profiles" class="w-full h-[50%]">
            <ModProfiles/>
        </div>
        <div id="agents" class="w-full h-[50%]">
            <AgentList/>
        </div>
    </div>
</div>
