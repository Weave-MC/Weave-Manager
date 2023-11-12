<script lang="ts">
    import {clickOutside} from '../click-outside'
    import {createEventDispatcher} from "svelte"

    const dispatch = createEventDispatcher()
    let open: boolean = false

    function toggleThemeSelector() {
        open = !open

        const themeSelection = document.getElementById('theme-selection')
        const classList = themeSelection.classList

        classList.toggle('w-7')
        classList.toggle('h-7')
        classList.toggle('w-48')
        classList.toggle('h-72')
        classList.toggle('theme-shadow')
    }

    function clickedOutside() {
        if (open)
            toggleThemeSelector()
    }

    function selectTheme(theme) {
        dispatch('select_theme', {theme: theme})
    }
</script>

<div id="theme-selection" class="relative w-7 h-7 bg-crust overflow-hidden rounded-lg" use:clickOutside on:click_outside={clickedOutside}>
    <div id="selections" class="absolute w-48 h-72 pt-8 flex flex-col justify-around">
        <div id="darcula-theme" class="theme" on:click={() => selectTheme('theme-darcula')}>
            <h1>Darcula</h1>
        </div>
        <div id="light-theme" class="theme" on:click={() => selectTheme('theme-light')}>
            <h1>Light</h1>
        </div>
        <div id="moonlight-theme" class="theme" on:click={() => selectTheme('theme-moonlight')}>
            <h1>Moonlight</h1>
        </div>
        <div id="purple-rain-theme" class="theme" on:click={() => selectTheme('theme-purple-rain')}>
            <h1>Purple Rain</h1>
        </div>
        <div id="gruvbox-theme" class="theme" on:click={() => selectTheme('theme-gruvbox')}>
            <h1>Gruvbox</h1>
        </div>
        <div id="cat-macchiato" class="theme" on:click={() => selectTheme('theme-cat-macchiato')}>
            <h1>Cat Macchiato</h1>
        </div>
        <div id="cat-mocha" class="theme" on:click={() => selectTheme('theme-cat-mocha')}>
            <h1>Cat Mocha</h1>
        </div>
        <div id="cat-frappe" class="theme" on:click={() => selectTheme('theme-cat-frappe')}>
            <h1>Cat Frappé</h1>
        </div>
        <div id="deep-ocean-theme" class="theme" on:click={() => selectTheme('theme-deep-ocean')}>
            <h1>Deep Ocean</h1>
        </div>
        <div id="amoled-theme" class="theme" on:click={() => selectTheme('theme-amoled')}>
            <h1>Amoled</h1>
        </div>
        <div id="halloween-theme" class="theme" on:click={() => selectTheme('theme-halloween')}>
            <h1>Halloween</h1>
        </div>
    </div>
    <div id="title" class="absolute w-48 h-8 pl-3 flex items-center border-b-2 b border-overlay">
        <h1>Theme Selection</h1>
    </div>
    <button id="theme-selector" class="absolute w-7 h-7 right-0 rounded-bl text-xl bg-crust aspect-square text-accent" on:click={toggleThemeSelector}>
        <i class="fa-solid fa-palette"></i>
    </button>
</div>

<style>
    #theme-selection {
        transition: all 0.3s ease;
    }

    :global(.theme-shadow) {
        box-shadow: 5px 6px 5px 0 rgba(0, 0, 0, 0.2);
    }

    h1 {
        @apply font-semibold text-sm;
    }

    .theme {
        @apply w-full pl-3 cursor-pointer;
    }
</style>
