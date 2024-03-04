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