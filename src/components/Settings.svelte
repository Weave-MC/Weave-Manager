<script>
    import TogglePill from "./TogglePill.svelte";

    function something(event) {
        console.log(event.detail.active)
    }
</script>

<div id="settings-content" class="w-full h-full">
    <div id="settings-title" class="w-full h-8 flex justify-center items-center border-b-2 border-overlay">
        <h1 class="absolute">Settings</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-solid fa-gear"></i>
        </div>
    </div>
    <div id="settings" class="flex flex-col w-full h-full pt-2 pb-8 gap-2">
        <div class="setting-toggle">
            <TogglePill id="prompt-weave" name="Prompt Relaunch" on:toggle={something}/>
        </div>
        <div class="setting-toggle">
            <TogglePill id="startup-run" name="Run on Startup"/>
        </div>
        <div class="setting-toggle">
            <TogglePill id="auto-update" name="Auto Update"/>
        </div>
    </div>
</div>

<style>
    .setting-toggle {
        @apply w-full h-8 flex flex-row items-center pl-2;
    }
</style>