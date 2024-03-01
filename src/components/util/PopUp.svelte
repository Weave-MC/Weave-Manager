<script lang="ts">
    import {createEventDispatcher} from "svelte";

    let popup: HTMLDialogElement
    const dispatch = createEventDispatcher()

    export let title: string
    export function show() {
        popup.showModal()
    }
    export function close() {
        popup.close()
    }
    function handleClick(event: MouseEvent) {
        const target = event.target as HTMLDialogElement

        const rect = target.getBoundingClientRect()
        const isInDialog = (rect.top <= event.clientY && event.clientY <= rect.top + rect.height && rect.left <= event.clientX && event.clientX <= rect.left + rect.width)
        if (!isInDialog) {
            dispatch("click-outside")
            target.close()
        }
    }
    function handleKeyboard(event: KeyboardEvent) {
        if (event.key === "Enter") {
            popup.close()
        }
    }
</script>

<dialog bind:this={popup} class="fixed top-0 bottom-0 left-0 right-0 {$$props.class} bg-overlay rounded-xl select-none" on:mousedown={handleClick} on:keydown={handleKeyboard}>
    <div id="content" class="w-full h-full flex flex-col justify-between items-center text-text px-2 pb-2 pt-1 gap-1">
        <h1 class="text-lg">{title}</h1>
        <div id="content" class="relative h-full w-full bg-base rounded-xl">
            <slot/>
        </div>
    </div>
</dialog>

<style>
    dialog {
        scale: 0;
        opacity: 0;
        padding: 0;
        box-shadow: 0 0 3rem 1px rgba(0, 0, 0, 0.4);
        transition: all 250ms ease-in-out;
    }
    dialog[open] {
        scale: 1;
        opacity: 1;
    }
    dialog::backdrop {
        background-color: rgba(0, 0, 0, 0.2);
    }
    dialog:focus {
        outline: none;
    }
</style>