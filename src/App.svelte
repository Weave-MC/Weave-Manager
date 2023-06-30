<script lang="ts">
  import HeaderBar from "./components/HeaderBar.svelte";
  import ThemeSelection from "./components/ThemeSelection.svelte";
  import MinecraftList from "./components/MinecraftList.svelte";

  let selected = 'macchiato'
</script>

<main id="main" class="{selected} w-screen h-screen overflow-clip text-text">
  <HeaderBar bind:value={selected}/>
  <div id="page-content" class="bg-crust relative w-screen h-screen flex flex-row justify-center pb-14 gap-4 p-4">
    <div class="page-panel">
      <MinecraftList/>
    </div>
    <div class="page-panel">
      <MinecraftList/>
    </div>
    <div class="page-panel flex flex-col gap-4">
      <div class="w-full h-1/3 overflow-hidden rounded-xl">
        <MinecraftList/>
      </div>
      <div class="w-full h-2/3 overflow-hidden rounded-xl">
        <MinecraftList/>
      </div>
    </div>
  </div>
</main>

<style>
  .page-panel {
    @apply w-1/3 h-full overflow-hidden rounded-xl;
  }
</style>