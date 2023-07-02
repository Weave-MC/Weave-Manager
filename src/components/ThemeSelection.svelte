<div id="theme-selection" class="relative w-7 h-7 bg-crust overflow-hidden">
    <div id="selections" class="absolute w-48 h-56 pt-7 flex flex-col justify-around">
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
        <div id="purple-cat-macchiato" class="theme" on:click={() => selectTheme('theme-cat-macchiato')}>
            <h1>Cat Macchiato</h1>
        </div>
        <div id="purple-cat-mocha" class="theme" on:click={() => selectTheme('theme-cat-mocha')}>
            <h1>Cat Mocha</h1>
        </div>
        <div id="purple-cat-frappe" class="theme" on:click={() => selectTheme('theme-cat-frappe')}>
            <h1>Cat Frappé</h1>
        </div>
    </div>
    <div id="title" class="absolute w-48 h-7 pl-3 flex items-center">
        <h1>Theme Selection</h1>
    </div>
    <button id="theme-selector" class="absolute w-7 h-7 right-0 rounded-bl text-xl bg-crust aspect-square text-accent" on:click={toggleThemeSelector}>
        <i class="fa-solid fa-palette"></i>
    </button>
</div>

<script lang="ts">
    export let value

    function toggleThemeSelector() {
        const themeSelection = document.getElementById('theme-selection')
        const classList = themeSelection.classList

        classList.toggle('w-7')
        classList.toggle('h-7')
        classList.toggle('w-48')
        classList.toggle('h-56')
    }

    function selectTheme(theme) {
        value = theme
    }
</script>

<style>
    #theme-selection {
        transition: width 0.3s ease, height 0.3s ease;
    }

    h1 {
        @apply font-semibold text-sm;
    }

    .theme {
        @apply w-full pl-3 cursor-pointer;
    }
</style>