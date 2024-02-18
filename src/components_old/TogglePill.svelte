<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let id;
    export let name;
    const dispatch = createEventDispatcher()
    export let enabled: boolean = true

    function onToggle(event) {
        enabled = event.target.checked
        dispatch('toggle')
    }
</script>

<div class="toggle-pill flex flex-row items-center">
    <div class="checkbox-container mr-2">
        <input type="checkbox" id="{id}" name="check" checked="{enabled ? 'checked' : ''}" on:click={onToggle}>
        <label for="{id}"></label>
    </div>
    <label class="cursor-pointer" for={id}>{name}</label>
</div>

<style>
    input[type="checkbox"] {
        display: none;
    }
    input[type="checkbox"] + label {
        @apply bg-disabled;

        display: block;
        position: relative;
        width: 2rem;
        height: 1.25rem;
        border-radius: 1rem;
        box-shadow: inset 0 0 5px rgba(0, 0, 0, 0.3);
        cursor: pointer;
        -webkit-user-select: none;
        -moz-user-select: none;
        -ms-user-select: none;
        transition: background 0.1s ease-in-out;
    }
    input[type="checkbox"] + label:before {
        @apply bg-text;

        content: "";
        display: block;
        width: 0.85rem;
        height: 0.85rem;
        border-radius: 1rem;
        box-shadow: 2px 0 5px rgba(0, 0, 0, 0.2);
        position: absolute;
        left: 0.2rem;
        top: 0.2rem;
        -webkit-transition: all 0.2s ease-in-out;
        transition: all 0.2s ease-in-out;
    }
    input[type="checkbox"]:checked + label {
        @apply bg-enabled;
    }
    input[type="checkbox"]:checked + label:before {
        box-shadow: -2px 0 5px rgba(0, 0, 0, 0.2);
        left: 0.95rem;
    }
</style>