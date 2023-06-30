<script>
    import TogglePill from "./TogglePill.svelte";

    function something(event) {
        console.log(event.detail.active)
    }
</script>

<div id="settings-content" class="w-full h-full flex flex-col">
    <div id="settings-title" class="w-full h-8 flex justify-center items-center border-b-2 border-surface1">
        <h1>Settings</h1>
        <i class="absolute right-6 fa-solid fa-gear"></i>
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