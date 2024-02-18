export class LaunchProfile {
    public name: string
    public mcInfo: MinecraftInfo

    constructor(
        name: string,
        mcInfo: MinecraftInfo
    ) {
        this.name = name
        this.mcInfo = mcInfo
    }

    public launchProfile(): void {

    }
    public deleteProfile() {

    }
}

export interface MinecraftProcess {
    pid: number
    info: MinecraftInfo
}

export interface MinecraftInfo {
    client: string
    version: string
    cmd: string
    cwd: string
}