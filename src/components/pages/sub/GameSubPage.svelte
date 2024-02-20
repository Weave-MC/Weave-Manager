<script lang="ts">
    import LaunchHistory from "./items/LaunchHistory.svelte";
    import LaunchProfiles from "./items/LaunchProfiles.svelte";
</script>

<div class="w-full h-full flex flex-row gap-3">
    <div id="left" class="w-[60%] h-full bg-surface rounded-xl text-center">
        <LaunchProfiles/>
    </div>
    <div id="right" class="w-[40%] h-full overflow-clip">
        <LaunchHistory/>
    </div>
</div>