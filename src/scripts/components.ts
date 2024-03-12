import type {
    Agent,
    LaunchProfile,
    MinecraftInfo,
    MinecraftProcess, Mod,
    ModProfile,
    ProcessHistory,
    Profile, Settings
} from "./types";
import {
    writeFile,
    exists,
    readTextFile,
    writeTextFile,
    readDir,
    renameFile,
    removeFile
} from "@tauri-apps/api/fs"
import {launchProfiles, modList, modProfiles, processMap} from "./stores";
import {invoke} from "@tauri-apps/api/tauri";
import {get} from "svelte/store";
import {
    getAgentsDirectory,
    getHistoryLogsDirectory,
    getModsDirectory,
    getProfileDirectory, getWeaveDirectory,
    sanitizeFileName
} from "./paths";

export async function showProcessInfo(mcProcess: MinecraftProcess) {

}

/**
 * Creates a Mod Profile using the currently enabled mods.
 * @param name The name of the profile
 * @param mods The currently enabled mods
 * @return True if successful, False if the file already exists
 */
export async function createModProfile(name: string, mods: Mod[]): Promise<boolean> {
    if (name.length == 0)
        return false

    // sanitize the file name
    const cleanName = sanitizeFileName(name)
    const filePath = `${await getProfileDirectory()}/${cleanName}.mprof`

    if (await exists(filePath))
        return false

    const profile = <ModProfile> {
        name: name,
        mods: mods
    }

    await writeFile(filePath, JSON.stringify(profile))

    modProfiles.update((profiles) => {
        profiles.set(cleanName, profile)
        return profiles
    })

    return true
}

/**
 * Creates a Launch Profile using information retrieved from Minecraft processes.
 * @param name The name of the profile
 * @param mcInfo The minecraft information needed to re-launch this process with Weave
 * @param modProfile The mod profile that will be used when the Launch Profile is launched.
 * @return True if successful, False if the file already exists
 */
export async function createLaunchProfile(name: string, mcInfo: MinecraftInfo, modProfile: ModProfile): Promise<boolean> {
    if (name.length == 0) // empty names are not allowed
        return false

    // sanitize the file name
    const cleanName = sanitizeFileName(name)
    const filePath = `${await getProfileDirectory()}/${cleanName}.lprof`

    if (await exists(filePath))
        return false

    const profile = <LaunchProfile> {
        name: name,
        mc_info: mcInfo,
        mod_profile: modProfile
    }

    await writeFile(filePath, JSON.stringify(profile))

    launchProfiles.update((profiles) => {
        profiles.set(cleanName, profile)
        return profiles
    })

    return true
}

export async function readProcessHistory(): Promise<ProcessHistory> {
    const historyFile = `${await getHistoryLogsDirectory()}/history.log`

    let fileContent: ProcessHistory
    if (await exists(historyFile)) {
        fileContent = <ProcessHistory> JSON.parse(await readTextFile(historyFile))
    } else {
        fileContent = <ProcessHistory> {
            history: []
        }

        await writeTextFile(historyFile, JSON.stringify(fileContent))
    }

    return fileContent
}

export async function readMods(): Promise<Mod[]> {
    const entries = await readDir(await getModsDirectory())
    return await Promise.all(entries
        .filter(e => e.name.includes(".jar") && e.children == null)
        .map(async (e) =>  {
            return <Mod> {
                file_path: e.path.replace(".disabled", ""),
                file_name: e.name.replace(".disabled", ""),
                disabled: e.name.endsWith(".disabled"),
                mod_info: await invoke("read_mod_config", {path: e.path})
            }
        })
    )
}

export async function readAgents(): Promise<Agent[]> {
    const entries = await readDir(await getAgentsDirectory())
    return entries
        .filter(e => e.name.includes(".jar") && e.children == null)
        .map((e) => <Agent> {
            file_name: e.name.replace(".disabled", ""),
            file_path: e.path.replace(".disabled", ""),
            disabled: e.name.endsWith(".disabled")
        })
}

export async function readProfiles<T extends Profile>(extension: string): Promise<Map<string, T>> {
    const entries = await readDir(await getProfileDirectory())
    const map = new Map()
    for (const entry of entries) {
        if (entry.name.endsWith(extension) && entry.children == null) {
            map.set(entry.name.replace(extension, ""), <T> JSON.parse(await readTextFile(entry.path)))
        }
    }

    return map
}

export async function readSettings(): Promise<Settings> {
    const settingsFile = `${await getWeaveDirectory()}/manager.settings`
    let fileContent: Settings
    if (await exists(settingsFile)) {
        fileContent = <Settings> JSON.parse(await readTextFile(settingsFile))
    } else {
        fileContent = <Settings> {
            auto_update: true,
            ignore_updates: false,
            startup_run: true,
            compact_buttons: false,
            theme: "DARCULA",
            loader_version: undefined
        }

        await writeTextFile(settingsFile, JSON.stringify(fileContent))
    }

    return fileContent
}

export async function saveLaunchProfile(profile: LaunchProfile) {
    const cleanName = sanitizeFileName(profile.name)
    const filePath = `${await getProfileDirectory()}/${cleanName}.lprof`

    await writeFile(filePath, JSON.stringify(profile))

    if (profile.mod_profile)
        await saveModProfile(profile.mod_profile)

    launchProfiles.update((profiles) => {
        profiles.set(cleanName, profile)
        return profiles
    })
}

export async function deleteLaunchProfile(profile: LaunchProfile) {
    const cleanName = sanitizeFileName(profile.name)
    const filePath = `${await getProfileDirectory()}/${cleanName}.lprof`
    await removeFile(filePath)

    launchProfiles.update((profiles) => {
        if (profiles.has(cleanName))
            profiles.delete(cleanName)

        return profiles
    })
}

export async function saveModProfile(profile: ModProfile) {
    profile.mods = (await readMods()).filter((mod) => !mod.disabled)

    const cleanName = sanitizeFileName(profile.name)
    const filePath = `${await getProfileDirectory()}/${cleanName}.mprof`

    await writeFile(filePath, JSON.stringify(profile))

    modProfiles.update((profiles) => {
        profiles.set(cleanName, profile)
        return profiles
    })
}

export async function loadModProfile(profile: ModProfile) {
    const allMods = get<Mod[]>(modList)
    allMods.forEach((mod) => {
        if (profile.mods.find((profileMod) => profileMod.file_name === mod.file_name) && mod.disabled)
            toggleMod(mod)
        else if (!mod.disabled) {
            toggleMod(mod)
        }
    })
}

export async function toggleMod(mod: Mod) {
    mod.disabled = !mod.disabled

    if (mod.disabled)
        await renameFile(mod.file_path, `${mod.file_path}.disabled`)
    else
        await renameFile(`${mod.file_path}.disabled`, mod.file_path)

    const list = get<Mod[]>(modList)
    list[list.indexOf(mod)] = mod
    modList.set(list)
}

export function checkVerticalOverflow(element: HTMLElement): boolean {
    return element.scrollHeight > element.clientHeight
}

export function clickOutside(node) {
    const handleClick = event => {
        if (node && !node.contains(event.target) && !event.defaultPrevented) {
            node.dispatchEvent(
                new CustomEvent('click_outside', node)
            )
        }
    }

    document.addEventListener('click', handleClick, true);

    return {
        destroy() {
            document.removeEventListener('click', handleClick, true);
        }
    }
}