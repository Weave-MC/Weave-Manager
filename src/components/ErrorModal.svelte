<script lang="ts">
    import {onDestroy, onMount} from "svelte";

    type Error = {
        title: string
        desc: string
    }

    let errorModal: HTMLDialogElement
    let errors: Error[] = [{title: 'Failed Relaunch', desc: 'Failed to relaunch Lunar Client with Weave'}, {title: '2', desc: '2'}]

    let awaitClose: boolean = false

    async function modalClicked(event) {
        const target = event.target as HTMLDialogElement

        const rect = target.getBoundingClientRect()
        const isInDialog = (rect.top <= event.clientY && event.clientY <= rect.top + rect.height && rect.left <= event.clientX && event.clientX <= rect.left + rect.width)
        if (!isInDialog) {
            closeDialog(target, () => {
                awaitClose = false
                errors = errors.filter((_, i) => i !== 0);
            });
        }
    }

    function closeDialog(dialog, callback) {
        awaitClose = true
        dialog.close()
        setTimeout(callback, 350)
    }

    let errorInterval
    onMount(async() => {
        errorInterval = setInterval(async () => {
            if (!errorModal.open && errors.length > 0 && !awaitClose)
                errorModal.showModal()
        })
    })

    onDestroy(() => {
        clearInterval(errorInterval)
    })
</script>

<dialog bind:this={errorModal} id="error-modal" class="w-[26rem] h-[10rem] text-text" on:click={async(event) => modalClicked(event)}>
    <div class="w-full h-9 border-b-2 border-overlay flex justify-center items-center">{errors[0].title}</div>
    <div class="w-full h-full flex flex-col items-center justify-center">
        {#if errors.length > 0}
            <h1>{errors[0].desc}</h1>
        {/if}
    </div>
</dialog>

<style>
    #error-modal {
        @apply px-4 py-1 fixed top-0 bottom-0 flex flex-col bg-surface rounded-xl items-center;
        scale: 0;
        opacity: 0;
        box-shadow: 0 0 3rem 1px rgba(0, 0, 0, 0.4);
        transition: all 350ms ease-in-out;
        pointer-events: all;
    }
    #error-modal::backdrop {
        background-color: rgba(0, 0, 0, 0.2);
    }
    #error-modal:focus {
        outline: none;
    }
    #error-modal[open] {
        scale: 1;
        opacity: 1;
    }
</style>