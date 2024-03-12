import {createDir, exists} from "@tauri-apps/api/fs";
import {homeDir} from "@tauri-apps/api/path";

export function sanitizeFileName(name: string) {
    return name.replace(/[^a-z0-9]/gi, '_').toLowerCase();
}

export async function getAgentsDirectory() {
    const agentsDir = `${await getWeaveDirectory()}/agents`
    if (!await exists(agentsDir))
        await createDir(agentsDir)

    return agentsDir
}
export async function getModsDirectory() {
    const modsDir = `${await getWeaveDirectory()}/mods`
    if (!await exists(modsDir))
        await createDir(modsDir)

    return modsDir
}
export async function getHistoryLogsDirectory() {
    const historyLogsDir = `${await getLogsDirectory()}/history`
    if (!await exists(historyLogsDir))
        await createDir(historyLogsDir)

    return historyLogsDir
}
export async function getClientLogsDirectory() {
    const clientLogsDir = `${await getLogsDirectory()}/client`
    if (!await exists(clientLogsDir))
        await createDir(clientLogsDir)

    return clientLogsDir
}
export async function getLogsDirectory() {
    const logsDir = `${await getWeaveDirectory()}/logs`
    if (!await exists(logsDir))
        await createDir(logsDir)

    return logsDir
}
export async function getProfileDirectory() {
    const profileDir = `${await getWeaveDirectory()}/profiles`
    if (!await exists(profileDir))
        await createDir(profileDir)

    return profileDir
}

export async function getWeaveDirectory() {
    const weaveDir = `${await homeDir()}/.weave`
    if (!await exists(weaveDir))
        await createDir(weaveDir)

    return weaveDir
}