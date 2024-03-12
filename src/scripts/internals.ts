import type {
    Analytics,
    GitHubApiResponse,
    LoaderUpdateResponse,
    MinecraftProcess,
    ProcessHistory,
    Settings
} from "./types";
import {fetch, ResponseType, Response} from "@tauri-apps/api/http"
import {writeBinaryFile, writeFile, writeTextFile} from "@tauri-apps/api/fs";
import {getHistoryLogsDirectory, getWeaveDirectory} from "./paths";
import {processHistory, processMap, settings} from "./stores";
import {get} from "svelte/store";
import {invoke} from "@tauri-apps/api/tauri";

export async function installWeave() {
    await writeFile(
        `${await getWeaveDirectory()}/analytics.json`,
        JSON.stringify(<Analytics> {
            launch_times: [],
            time_played: 0,
            average_launch_time: 0.0
        })
    )

    const response = await fetchGithubApi()
    const loaderDownload = response.assets.filter(asset => asset.name.endsWith(".jar"))[0].browser_download_url

    await downloadWeaveLoader(loaderDownload, response.tag_name)
}

async function fetchGithubApi() {
    const latest = await fetch("https://api.github.com/repos/Weave-MC/Weave-Loader/releases/latest", {
        method: "GET",
        headers: {
            "User-Agent": "weave-manager"
        }
    })
    return latest.data as GitHubApiResponse
}

export async function downloadWeaveLoader(url: string, version: string) {
    const response = await fetch(url, {
        method: "GET",
        responseType: ResponseType.Binary
    }) as Response<Uint8Array>

    await writeBinaryFile(
        `${await getWeaveDirectory()}/loader.jar`,
        response.data
    )

    const _settings = get<Settings>(settings)
    _settings.loader_version = version
    settings.set(_settings)

    const settingsFile = `${await getWeaveDirectory()}/manager.settings`
    await writeTextFile(settingsFile, JSON.stringify(_settings))
}

export async function checkForLoaderUpdate(): Promise<LoaderUpdateResponse> {
    const response = await fetchGithubApi()

    const loaderAsset = response.assets.filter(asset => asset.name.endsWith(".jar"))[0]
    const sha256Url = response.assets.filter(asset => asset.name === `${loaderAsset.name}.sha256`)[0].browser_download_url

    const sha256Response = await fetch(sha256Url, {
        method: "GET",
        responseType: ResponseType.Text
    }) as Response<string>

    const sha256 = sha256Response.data.split(" ")[0]
    const shouldUpdate = !await invoke("check_loader_integrity", {sumToCheck: sha256.toUpperCase()})

    return <LoaderUpdateResponse> {
        update: shouldUpdate,
        download_url: loaderAsset.browser_download_url,
        version: response.tag_name
    }
}

export async function updateProcessMap() {
    try {
        const pMap = get(processMap)
        const newMap = new Map()

        const fetched = await invoke<MinecraftProcess[]>("fetch_minecraft_processes")
        for (const proc of fetched) {
            if (!pMap.has(proc.pid)) {
                if (!proc.weave_attached)
                    await logProcess(proc)
            }
            newMap.set(proc.pid, proc)
        }

        processMap.set(newMap)
    } catch (err) {
        console.error(err)
    }
}

export async function logProcess(process: MinecraftProcess) {
    processHistory.update((history) => {
        history.history.push(process)
        return history
    })

    const pHistory = get<ProcessHistory>(processHistory)
    const historyFile = `${await getHistoryLogsDirectory()}/history.log`
    await writeTextFile(historyFile, JSON.stringify(pHistory))
}