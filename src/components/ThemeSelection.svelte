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

    // looks scuffed ik but better than hardcoding things
    let themeList = [
        {
            id: "theme-darcula",
            jsId: "darcula-theme",
            name: "Darcula"
        },
        {
            id: "theme-light",
            jsId: "light-theme",
            name: "Light"
        },
        {
            id: "theme-moonlight",
            jsId: "moonlight-theme",
            name: "Moonlight"
        },
        {
            id: "theme-purple-rain",
            jsId: "purple-rain-theme",
            nam: "Purple Rain"
        },
        {
            id: "theme-gruvbox",
            jsId: "gruvbox-theme",
            name: "Gruvbox"
        },
        {
            id: "theme-cat-macchiato",
            jsId: "cat-macchiato",
            name: "Cat Macchiato"
        },
        {
            id: "theme-cat-mocha",
            jsId: "cat-mocha",
            name: "Cat Mocha"
        },
        {
            id: "theme-cat-frappe",
            jsId: "cat-frappe",
            name: "Cat Frappe"
        },
        {
            id: "theme-deep-ocean",
            jsId: "deep-ocean-theme",
            name: "Deep Ocean"
        },
        {
            id: "theme-amoled",
            jsId: "amoled-theme",
            name: "Amoled"
        }
    ]

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
        {#each themeList as obj}
            <div id={obj.jsId} class="theme" on:click={() => selectTheme(obj.id)}>
                <h1>{obj.name}</h1>
            </div>
        {/each}
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
