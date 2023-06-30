<script lang="ts">
  import HeaderBar from "./components/HeaderBar.svelte";
  import ThemeSelection from "./components/ThemeSelection.svelte";
  import { variants } from '@catppuccin/palette'

  let selected = 'macchiato'
</script>

<main id="main" class="{selected} w-screen h-screen overflow-clip text-text">
  <HeaderBar/>
  <div id="page-content" class="bg-crust relative w-screen h-screen flex flex-col items-center justify-around p-10 mb-10">
    <ThemeSelection bind:value="{selected}"/>
  </div>
</main>