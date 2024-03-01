import type {MinecraftInfo, MinecraftProcess} from "./types";
import {BaseDirectory, writeFile, exists, createDir} from "@tauri-apps/api/fs"
import {homeDir} from "@tauri-apps/api/path";

export async function showProcessInfo(mcProcess: MinecraftProcess) {

}

/**
 * Creates a Launch Profile using information retrieved from Minecraft processes.
 * @param name The name of the profile
 * @param mcInfo The minecraft information needed to re-launch this process with Weave
 * @return True if successful, False if the file already exists
 */
export async function createLaunchProfile(name: string, mcInfo: MinecraftInfo): Promise<boolean> {
    if (name.length == 0) // empty names are not allowed
        return false

    // sanitize the file name
    const cleanName = name.replace(/[^a-z0-9]/gi, '_').toLowerCase();
    const filePath = `${await getProfileDirectory()}/${cleanName}.launch`

    if (await exists(filePath))
        return false

    await writeFile(filePath, JSON.stringify(mcInfo))
    return true
}

async function getProfileDirectory() {
    const profileDir = `${await getWeaveDirectory()}/profiles`
    if (!await exists(profileDir))
        await createDir(profileDir)

    return profileDir
}

async function getWeaveDirectory() {
    const weaveDir = await homeDir() + "/.weave"
    if (!await exists(weaveDir))
        await createDir(weaveDir)

    return weaveDir
}

export function checkVerticalOverflow(element: HTMLElement): boolean {
    return element.scrollHeight > element.clientHeight
}

/**
 * https://stackoverflow.com/a/42543908
 */
export function getScrollParent(el: HTMLElement, includeHidden: boolean) {
    let style = getComputedStyle(el)
    const excludedStaticParent = style.position === "absolute"
    const overflowRegex = includeHidden ? /(auto|scroll|hidden)/ : /(auto|scroll)/

    if (style.position === "fixed") return document.body
    for (let parent = el; (parent = parent.parentElement);) {
        style = getComputedStyle(parent)
        if (excludedStaticParent && style.position === "static")
            continue

        if (overflowRegex.test(style.overflow + style.overflowX + style.overflowY)) return parent
    }

    return document.body
}