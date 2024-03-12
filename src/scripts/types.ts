export interface LaunchProfile extends Profile {
    name: string
    mc_info: MinecraftInfo
    mod_profile: ModProfile
}

export interface ModProfile extends Profile {
    name: string
    mods: Mod[]
}

export interface Profile {}

export interface Mod {
    mod_info: ModInfo
    file_path: string
    file_name: string
    disabled: boolean
}

export interface ModInfo {
    name: string
    version: string
    description: string
    authors: string[]
}

export interface Agent {
    file_path: string
    file_name: string
    disabled: boolean
}

export interface MinecraftProcess {
    pid: number
    start_time: number
    info: MinecraftInfo
    weave_attached: boolean
}

export interface ProcessHistory {
    history: MinecraftProcess[]
}

export interface MinecraftInfo {
    client: string
    version: string
    cmd: string
    cwd: string
}

export interface OptionButton {
    label: string
    icon: string
    action: () => void
}

export interface ConsolePayload {
    line: string
}

export interface WeaveProcess {
    log_file: string
    client: string
    pid: number
    output: string[]
}

export interface Analytics {
    launch_times: number[]
    time_played: number
    average_launch_time: number
}

export interface Settings {
    auto_update: boolean
    ignore_updates: boolean
    startup_run: boolean
    compact_buttons: boolean
    theme: string
    loader_version: string
}

export enum Themes {
    DARCULA = "Darcula",
    LIGHT = "Light",
    MOONLIGHT = "Moonlight",
    PURPLE_RAIN = "Purple Rain",
    GRUVBOX = "Gruvbox",
    CAT_MACCHIATO = "Cat Macchiato",
    CAT_MOCHA = "Cat Mocha",
    CAT_FRAPPE = "Cat Frappe"
}

export interface SelectionOption {
    name: string,
    value: string
}

export interface LoaderUpdateResponse {
    update: boolean
    download_url: string
    version: string
}

export type GitHubApiResponse = {
    tag_name: string
    assets: [Asset]
}

export type Asset = {
    name: string
    browser_download_url: string
}