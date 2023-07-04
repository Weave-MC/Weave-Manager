<script lang="ts">
  import HeaderBar from "./components/HeaderBar.svelte";
  import MinecraftList from "./components/MinecraftList.svelte";
  import Settings from "./components/Settings.svelte";
  import ModList from "./components/ModList.svelte";
  import Analytics from "./components/Analytics.svelte";
  import Console from "./components/ConsoleOutput.svelte";

  let selected = 'theme-darcula'
  let mcInstances: number
  const panelColor = 'bg-surface'

  // unresolved reference because lang is set to ts...
  // im not good with ts so not sure how to fix this but for now it's just a visual issue
  window.modalClicked = event => {
    const target = event.target as HTMLDialogElement

    const rect = target.getBoundingClientRect()
    const isInDialog = (rect.top <= event.clientY && event.clientY <= rect.top + rect.height && rect.left <= event.clientX && event.clientX <= rect.left + rect.width)
    if (!isInDialog)
      target.close()
  }

//   width is 50rem
//   height is 35rem
</script>

<main id="main" class="{selected} w-screen h-screen overflow-clip text-text">
  <HeaderBar bind:value={selected}/>
  <div id="page-content" class="bg-base relative w-screen h-screen flex items-center flex-col pb-10 gap-4 p-4">
    <div id="top-content" class="flex flex-row gap-4">
      <div class="one-by-two-panel {panelColor}">
        <MinecraftList bind:instances="{mcInstances}"/>
      </div>
      <div class="one-by-two-panel {panelColor}">
        <ModList/>
      </div>
      <div class="one-by-two-panel {panelColor}">
        <Settings/>
      </div>
    </div>
    <div id="bottom-content" class="flex flex-row gap-4">
      <div class="two-by-one-panel {panelColor}">
        <Console/>
      </div>
      <div class="one-by-one-panel {panelColor}">
        <Analytics instances="{mcInstances}"/>
      </div>
    </div>
  </div>
</main>


<style>
  .one-by-three-panel {
    @apply overflow-hidden rounded-xl;
    width: calc(46rem * 0.3333);
    height: 32rem;
  }
  .one-by-two-panel {
    @apply overflow-hidden rounded-xl;
    width: calc(46rem * 0.3333);
    height: calc(32rem * 0.6666);
  }
  .one-by-one-panel {
    @apply overflow-hidden rounded-xl;
    width: calc(46rem * 0.3333);
    height: calc(32rem * 0.3333);
  }
  .two-by-one-panel {
    @apply overflow-hidden rounded-xl;
    width: calc(47.5rem * 0.6666);
    height: calc(32rem * 0.3333);
  }
  .three-by-one-panel {
    @apply overflow-hidden rounded-xl;
    width: 50rem;
    height: calc(32rem * 0.3333);
  }
  #page-content>* {
    flex-flow: row wrap
  }
  :global(*) {
    user-select: none;
  }
</style>