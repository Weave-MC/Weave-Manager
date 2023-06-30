<div id="theme-selection" class="absolute bg-surface0 top-2 right-2 w-10 h-10 rounded-xl overflow-hidden">
    <div id="selections" class="absolute w-full h-full pt-10 flex flex-col justify-around">
        <div id="mocha-theme" class="theme" on:click={() => selectTheme('mocha')}>
            <h1>Mocha</h1>
        </div>
        <div id="macchiato-theme" class="theme" on:click={() => selectTheme('macchiato')}>
            <h1>Macchiato</h1>
        </div>
        <div id="latte-theme" class="theme" on:click={() => selectTheme('latte')}>
            <h1>Latte</h1>
        </div>
        <div id="frappe-theme" class="theme" on:click={() => selectTheme('frappe')}>
            <h1>Frappe</h1>
        </div>
    </div>
    <div id="title" class="absolute w-48 h-10 pl-3 flex items-center">
        <h1>Theme Selection</h1>
    </div>
    <button id="theme-selector" class="absolute w-10 h-10 right-0 rounded-xl text-2xl aspect-square bg-surface0 text-flamingo" on:click={toggleThemeSelector}>
        <i class="fa-solid fa-palette"></i>
    </button>
</div>

<script lang="ts">
    export let value

    function toggleThemeSelector() {
        const themeSelection = document.getElementById('theme-selection')
        const classList = themeSelection.classList

        classList.toggle('w-0')
        classList.toggle('h-0')
        classList.toggle('w-48')
        classList.toggle('h-56')
    }

    function selectTheme(theme) {
        value = theme
        toggleThemeSelector()
    }
</script>

<style>
    #theme-selection {
        transition: width 0.3s ease, height 0.3s ease;
    }

    h1 {
        @apply font-semibold;
    }

    .theme {
        @apply w-full pl-3 cursor-pointer;
    }
</style>