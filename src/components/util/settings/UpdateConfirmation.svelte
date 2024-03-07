<script lang="ts">
    import {createEventDispatcher} from "svelte";

    let visible: boolean = false

    export function display() {
        visible = true
    }

    function onConfirm() {
        dispatch("confirm")
        visible = false
    }

    function onReset() {
        dispatch("reset")
        visible = false
    }

    const dispatch = createEventDispatcher()
</script>

<div id="update-confirmation" class="absolute left-14 right-14 h-14 bg-surface flex flex-row justify-between items-center px-4 rounded-lg {visible ? 'bottom-4' : 'bottom-[-4rem]'}">
    <h1>You have unsaved changes</h1>
    <div class="flex flex-row items-center justify-center gap-2">
        <button class="w-20" on:click={onReset}>
            Reset
        </button>
        <button class="w-20 h-8 bg-overlay rounded" on:click={onConfirm}>
            Confirm
        </button>
    </div>
</div>

<style>
    #update-confirmation {
        transition: bottom 150ms ease-in-out;
    }
</style>