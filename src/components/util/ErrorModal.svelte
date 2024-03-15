<script lang="ts">
    import PopUp from "./PopUp.svelte";
    import {onMount} from "svelte";

    let errorModal: PopUp
    let errors: string[] = []

    window.addEventListener("unhandledrejection", (e) => {
        errors = [...errors, e.reason ? e.reason : e.detail.reason]
        errorModal.show()
    })

    function handleClose() {
        setTimeout(() => {
            errors = errors.filter((_, i) => i !== 0)
            if (errors.length > 0)
                errorModal.show()
        }, 350)
    }

    onMount(() => {
        if (errors.length > 0)
            errorModal.show()
    })
</script>

<PopUp bind:this={errorModal} title="Weave has encountered an error" on:close={handleClose}>
    {#if errors.length > 0}
        <div class="max-w-[30rem] min-h-[5rem] flex py-5 px-5">
            <h1 class="text-lg">{errors[0]}</h1>
        </div>
    {/if}
</PopUp>