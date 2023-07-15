<script lang="ts">
  import HeaderBar from "./components/HeaderBar.svelte";
  import MinecraftList from "./components/MinecraftList.svelte";
  import Settings from "./components/Settings.svelte";
  import ModList from "./components/ModList.svelte";
  import Analytics from "./components/Analytics.svelte";
  import Console from "./components/ConsoleOutput.svelte";
  import DropZone from "./components/DropZone.svelte";
  import {onDestroy, onMount} from "svelte";
  import {writeFile, writeBinaryFile, createDir, exists, BaseDirectory} from '@tauri-apps/api/fs'
  import {appWindow} from "@tauri-apps/api/window";
  import {Client, getClient, ResponseType} from '@tauri-apps/api/http';
  import {relaunch} from "@tauri-apps/api/process";
  import {invoke} from "@tauri-apps/api/tauri";
  import LoadSpinner from "./components/LoadSpinner.svelte";

  let selected: string = "theme-darcula"
  let mcInstances: number
  const panelColor = 'bg-surface'

  let promptRelaunch: boolean
  let autoUpdate: boolean
  let startupRun: boolean
  let loaderVersion: string

  let installing: boolean = false
  let updating: boolean = false
  let installModal: HTMLDialogElement
  let restartModal: HTMLDialogElement
  let updateModal: HTMLDialogElement
  let updateURL: string
  let updateVersion: string
  let httpClient: Client

  let consoleChild: Console
  let settingsChild: Settings

  // unresolved reference because lang is set to ts...
  // im not good with ts so not sure how to fix this but for now it's just a visual issue
  window.modalClicked = event => {
    const target = event.target as HTMLDialogElement

    const rect = target.getBoundingClientRect()
    const isInDialog = (rect.top <= event.clientY && event.clientY <= rect.top + rect.height && rect.left <= event.clientX && event.clientX <= rect.left + rect.width)
    if (!isInDialog)
      target.close()
  }

  async function selectTheme(event) {
    selected = event.detail.theme
    await settingsChild.writeToConfig('theme', selected);
  }

  function onSettingUpdate(event) {
    promptRelaunch = event.detail.prompt_relaunch
    autoUpdate = event.detail.auto_update
    startupRun = event.detail.startup_run
    selected = event.detail.theme
    loaderVersion = event.detail.loader_version
  }

  function closeWeaveManager() {
    appWindow.close()
  }

  type Asset = {
    name: string
    browser_download_url: string
  }

  type GitHubApiResponse = {
    tag_name: string
    assets: [Asset]
  }

  async function fetchGithubApi() {
    const latest = await httpClient.get('https://api.github.com/repos/Weave-MC/Weave-Loader/releases/latest', {
      headers: {
        'User-Agent': 'weave-manager'
      }
    })

    return latest.data as GitHubApiResponse
  }

  async function downloadWeaveLoader(url: String) {
    const response = await httpClient.get(url, {
      responseType: ResponseType.Binary
    })

    await writeBinaryFile(
            '.weave/loader.jar',
            response.data,
            {dir: BaseDirectory.Home}
    )
  }

  async function updateWeaveLoader() {
    updating = true

    await downloadWeaveLoader(updateURL)

    loaderVersion = updateVersion
    settingsChild.writeToConfig('loader_version', loaderVersion)

    updateModal.close()
    updating = false
  }

  async function installWeave() {
    installing = true
    await createDir('.weave/mods', {dir: BaseDirectory.Home, recursive: true})
    await writeFile(
            '.weave/analytics.json',
            JSON.stringify({
              launch_times: [],
              time_played: 0,
              average_launch_time: 0.0
            }),
            {dir: BaseDirectory.Home})

    const apiResponse = await fetchGithubApi()
    const loaderDownload = apiResponse.assets.filter(asset => asset.name.endsWith('.jar'))[0].browser_download_url
    loaderVersion = apiResponse.tag_name

    settingsChild.writeToConfig('loader_version', loaderVersion)

    await downloadWeaveLoader(loaderDownload)

    installModal.close()
    restartModal.showModal()
  }

  async function checkForLoaderUpdate() {
    const apiResponse = await fetchGithubApi()
    const loaderAsset = apiResponse.assets.filter(asset => asset.name.endsWith('.jar'))[0]
    const sha256Url = apiResponse.assets.filter(asset => asset.name === `${loaderAsset.name}.sha256`)[0].browser_download_url

    updateVersion = apiResponse.tag_name
    updateURL = loaderAsset.browser_download_url

    const sha256Response = await httpClient.get(sha256Url, {
      responseType: ResponseType.Text
    })
    const sha256 = sha256Response.data.split(' ')[0]
    if (!await invoke('check_loader_integrity', {sumToCheck: sha256.toUpperCase()})) {
      updateModal.showModal()
      if (autoUpdate)
        await updateWeaveLoader()
    }
  }

  let updateInterval

  onMount(async() => {
    httpClient = await getClient()

    if (!await(exists('.weave/loader.jar', {dir: BaseDirectory.Home})))
      installModal.showModal()
    else {
      await checkForLoaderUpdate()
    }

    updateInterval = setInterval(async () => {
      await checkForLoaderUpdate()
    }, 30 * 60_000)
  })

  onDestroy(() => {
    clearInterval(updateInterval)
  })

//   width is 50rem
//   height is 35rem
</script>

<main id="main" class="{selected} w-screen h-screen overflow-clip text-text">
  <HeaderBar on:select_theme={async(event) => await selectTheme(event)}/>
  <div id="page-content" class="bg-base relative w-screen h-screen flex items-center flex-col pb-10 gap-4 p-4">
    <div id="top-content" class="flex flex-row gap-4">
      <div class="one-by-two-panel {panelColor}">
        <MinecraftList bind:instances="{mcInstances}" promptRelaunch="{promptRelaunch}" on:switch_console={() => consoleChild.switchConsole()}/>
      </div>
      <div class="one-by-two-panel {panelColor}">
        <ModList/>
      </div>
      <div class="one-by-two-panel {panelColor}">
        <Settings bind:this={settingsChild} on:update={onSettingUpdate}/>
      </div>
    </div>
    <div id="bottom-content" class="flex flex-row gap-4">
      <div class="two-by-one-panel {panelColor}">
        <Console bind:this={consoleChild}/>
      </div>
      <div class="one-by-one-panel {panelColor}">
        <Analytics instances="{mcInstances}"/>
      </div>
    </div>
    <DropZone/>
  </div>
  <dialog bind:this={installModal} id="install-modal" class="modal w-[26rem] h-[12rem] text-text" on:click={modalClicked}>
    <div class="w-full h-9 border-b-2 border-overlay flex justify-center items-center">Weave is not installed on your computer</div>
    <div class="w-full h-full flex flex-col items-center justify-center {installing ? '' : 'gap-2'}">
      {#if installing}
        <h1 class="relative bottom-10">Installing Weave</h1>
        <LoadSpinner/>
      {:else}
        <button class="w-32 h-8 rounded-xl bg-overlay" on:click={async() => await installWeave()}>
          Install Weave
        </button>
        <button class="text-sm" on:click={closeWeaveManager}>
          Close Weave Manager
        </button>
      {/if}
    </div>
  </dialog>
  <dialog bind:this={updateModal} id="update-modal" class="modal w-[26rem] h-[12rem] text-text" on:click={modalClicked}>
    <div class="w-full h-9 border-b-2 border-overlay flex justify-center items-center">Weave is outdated or corrupted on your computer</div>
    <div class="relative w-full h-full pt-10 flex flex-col items-center justify-center {updating ? '' : 'gap-2'}">
      {#if updating}
        <h1 class="relative bottom-10">Updating Weave</h1>
        <LoadSpinner/>
      {:else}
        <h1 class="absolute top-5">New Version: {updateVersion}</h1>
        <button class="w-32 h-8 rounded-xl bg-overlay" on:click={async() => await updateWeaveLoader()}>
          Update Weave
        </button>
        <button class="text-sm" on:click={updateModal.close}>
          Skip Update
        </button>
      {/if}
    </div>
  </dialog>
  <dialog bind:this={restartModal} id="restart-modal" class="modal w-[26rem] h-[10rem] text-text">
    <div class="w-full h-9 border-b-2 border-overlay flex justify-center items-center">Weave Installed</div>
    <div class="w-full h-full flex flex-col items-center justify-center">
      <button class="w-52 h-8 rounded-xl bg-overlay" on:click={async() => await relaunch()}>
        Restart Weave Manager
      </button>
    </div>
  </dialog>
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

  .modal {
    @apply px-4 py-1 fixed top-0 bottom-0 flex flex-col bg-surface rounded-xl items-center;
    scale: 0;
    opacity: 0;
    box-shadow: 0 0 3rem 1px rgba(0, 0, 0, 0.4);
    transition: all 350ms ease-in-out;
    pointer-events: all;
  }
  .modal::backdrop {
    background-color: rgba(0, 0, 0, 0.2);
  }
  .modal:focus {
    outline: none;
  }
  button {
    outline: none;
  }
  .modal[open] {
    scale: 1;
    opacity: 1;
  }

  #page-content>* {
    flex-flow: row wrap
  }
  :global(*) {
    user-select: none;
  }
</style>