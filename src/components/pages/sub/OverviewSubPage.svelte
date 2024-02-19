<script lang="ts">
    import type {MinecraftInfo, MinecraftProcess} from "../../../scripts/types";

    // <pid, MinecraftProcess>
    let processMap = new Map<number, MinecraftProcess>([
        [5728, {pid: 5728, info: {client: 'Lunar', cwd: 'dummy', cmd: 'dummy', version: '1.8.9'}}],
        [93415, {pid: 93415, info: {client: 'Forge', cwd: 'dummy', cmd: 'dummy', version: '1.7.10'}}],
        [689, {pid: 689, info: {client: 'Feather', cwd: 'dummy', cmd: 'dummy', version: '1.12.2'}}],
        [52746, {pid: 52746, info: {client: 'Vanilla', cwd: 'dummy', cmd: 'dummy', version: '1.20.1'}}],
        [8179, {pid: 8179, info: {client: 'Badlion', cwd: 'dummy', cmd: 'dummy', version: '1.16.5'}}],
        [64302, {pid: 64302, info: {client: 'Other', cwd: 'dummy', cmd: 'dummy', version: '1.11'}}]
    ])

    import VerticalScroll from "../../util/VerticalScroll.svelte";



    async function killProcess(pid: number) {

    }

    async function showProcessInfo(mcProcess: MinecraftProcess) {

    }

    async function createLaunchProfile(mcProcess: MinecraftProcess) {

    }

    const longString = "[14:00:01] [Genesis/main/INFO] Found external file: OptiFine_v1_8.jar\n" +
        "[14:00:01] [Genesis/main/INFO] Found classpath URL: common-0.1.0-SNAPSHOT-all.jar with SHA-1 hash 3d1ce015144d9e8c67afb6c7bea44d465088760f\n" +
        "[14:00:01] [Genesis/main/INFO] Found classpath URL: genesis-0.1.0-SNAPSHOT-all.jar with SHA-1 hash f4fe0763ee5612babf600ff896c2fcb2817bb441\n" +
        "[14:00:01] [Genesis/main/INFO] Found classpath URL: legacy-0.1.0-SNAPSHOT-all.jar with SHA-1 hash 95af94ca8287d1c63c3d6760794b7c73485a061d\n" +
        "[14:00:01] [Genesis/main/INFO] Found classpath URL: lunar-lang.jar with SHA-1 hash 8ac3190e3564dd237aa2e33e9da303b906319101\n" +
        "[14:00:01] [Genesis/main/INFO] Found classpath URL: lunar-emote.jar with SHA-1 hash b04f3ee8f5e43fa3b162981b50bb72fe1acabb33\n" +
        "[14:00:01] [Genesis/main/INFO] Found classpath URL: lunar.jar with SHA-1 hash c4bf1454c42c8bb098a3d81ccd4c98f7f137c6f8\n" +
        "[14:00:01] [Genesis/main/INFO] Found classpath URL: optifine-0.1.0-SNAPSHOT-all.jar with SHA-1 hash 8757e4990af8e16689617da7a0ed5b22deb522c2\n" +
        "[14:00:02] [Genesis/GameBootstrap/main/INFO] Launching Minecraft 1.8.9 v1_8 (parsed from 1.8.9)\n" +
        "[14:00:02] [Genesis/GameBootstrap/main/INFO] Config hash: 988e8644  File hash: e02e703f\n" +
        "[14:00:02] [Genesis/Baker/main/INFO] Found class cache: .\\cache\\988e8644\\e02e703f\\bake.cache\n" +
        "[14:00:02] [Genesis/Baker/main/INFO] Loading baked classes...\n" +
        "[14:00:02] [Genesis/Baker/main/INFO] Loaded 11698 baked classes.\n" +
        "LUNARCLIENT_STATUS_PREINIT\n" +
        "[14:00:02] [Genesis/GameBootstrap/main/INFO] Starting game!\n" +
        "[Weave] Initializing Weave\n" +
        "[14:00:02] [OptiFine/main/INFO] Using OptiFine namespace \n" +
        "[14:00:02] [OptiFine/main/INFO] Loaded 1050 patches.\n" +
        "[14:00:02] [OptiFine/main/INFO] Loaded 72 assets.\n" +
        "WARN StatusConsoleListener The use of package scanning to locate plugins is deprecated and will be removed in a future release\n" +
        "[14:00:03] [IchorPipeline/main/WARN] MappingSet for com.moonsworth.lunar.RCICHCHORIROCHICROIOOOIOH.RCICCIHHORIHRRIOICRCOIHRR.RROORRROHCICHHRRCROOIRIRR.CCICCHORRCRRRIOORCRCIOIRI has 0 field mappings.\n" +
        "WARN StatusConsoleListener The use of package scanning to locate plugins is deprecated and will be removed in a future release\n" +
        "WARN StatusConsoleListener The use of package scanning to locate plugins is deprecated and will be removed in a future release\n" +
        "WARN StatusConsoleListener The use of package scanning to locate plugins is deprecated and will be removed in a future release\n" +
        "[14:00:04] [mixin/INFO] SpongePowered MIXIN Subsystem Version=0.8.5 Source=Unknown Service=Weave Env=CLIENT\n" +
        "[14:00:05] [mixin/ERROR] Initialising mixin subsystem after game pre-init phase! Some mixins may be skipped.\n" +
        "[Weave] Loading C:\\Users\\Max\\.weave\\mods\\LunarCPSHook-1.0.jar\n" +
        "java.lang.reflect.InvocationTargetException\n" +
        "\tat java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke0(Native Method)\n" +
        "\tat java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke(Unknown Source)\n" +
        "\tat java.base/jdk.internal.reflect.DelegatingMethodAccessorImpl.invoke(Unknown Source)\n" +
        "\tat java.base/java.lang.reflect.Method.invoke(Unknown Source)\n" +
        "\tat net.weavemc.loader.bootstrap.AgentKt$premain$2.transform(Agent.kt:96)\n" +
        "\tat net.weavemc.loader.bootstrap.SafeTransformer$DefaultImpls.transform(SafeTransformer.kt:18)\n" +
        "\tat net.weavemc.loader.bootstrap.AgentKt$premain$2.transform(Agent.kt:87)\n" +
        "\tat java.instrument/java.lang.instrument.ClassFileTransformer.transform(Unknown Source)\n" +
        "\tat java.instrument/sun.instrument.TransformerManager.transform(Unknown Source)\n" +
        "\tat java.instrument/sun.instrument.InstrumentationImpl.transform(Unknown Source)\n" +
        "\tat java.base/java.lang.ClassLoader.defineClass1(Native Method)\n" +
        "\tat java.base/java.lang.ClassLoader.defineClass(Unknown Source)\n" +
        "\tat java.base/java.lang.ClassLoader.defineClass(Unknown Source)\n" +
        "\tat com.moonsworth.lunar.genesis.RCROHRIHRCRRRCHCOCCCCIHOR.findClass(Unknown Source)\n" +
        "\tat com.moonsworth.lunar.genesis.RCROHRIHRCRRRCHCOCCCCIHOR.loadClass(Unknown Source)\n" +
        "\tat java.base/java.lang.ClassLoader.loadClass(Unknown Source)\n" +
        "\tat java.base/java.lang.Class.forName0(Native Method)\n" +
        "\tat java.base/java.lang.Class.forName(Unknown Source)\n" +
        "\tat com.moonsworth.lunar.genesis.ClientGameBootstrap.apply(Unknown Source)\n" +
        "\tat java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke0(Native Method)\n" +
        "\tat java.base/jdk.internal.reflect.NativeMethodAccessorImpl.invoke(Unknown Source)\n" +
        "\tat java.base/jdk.internal.reflect.DelegatingMethodAccessorImpl.invoke(Unknown Source)\n" +
        "\tat java.base/java.lang.reflect.Method.invoke(Unknown Source)\n" +
        "\tat com.moonsworth.lunar.genesis.IOHHIIRCOCOHOHRIRHCORIOCI.RCICCIHHORIHRRIOICRCOIHRR(Unknown Source)\n" +
        "\tat com.moonsworth.lunar.genesis.Genesis.run(Unknown Source)\n" +
        "\tat com.moonsworth.lunar.genesis.Genesis.main(Unknown Source)\n" +
        "Caused by: org.spongepowered.asm.launch.MixinInitialisationError: Error initialising mixin config lunarcpshook.mixins.json\n" +
        "\tat Genesis//org.spongepowered.asm.mixin.transformer.Config.create(Unknown Source)\n" +
        "\tat Genesis//org.spongepowered.asm.mixin.Mixins.createConfiguration(Unknown Source)\n" +
        "\tat Genesis//org.spongepowered.asm.mixin.Mixins.addConfiguration(Unknown Source)\n" +
        "\tat Genesis//net.weavemc.loader.WeaveLoader.init(WeaveLoader.kt:63)\n" +
        "\t... 26 more\n" +
        "Caused by: java.lang.IllegalArgumentException: The specified resource 'lunarcpshook.mixins.json' was invalid or could not be read\n" +
        "\tat Genesis//org.spongepowered.asm.mixin.transformer.MixinConfig.create(Unknown Source)\n" +
        "\tat Genesis//org.spongepowered.asm.mixin.transformer.Config.create(Unknown Source)\n" +
        "\t... 29 more\n"
</script>

<div class="w-full h-full flex flex-row gap-3">
    <div id="left" class="w-[50%] h-full flex flex-col gap-3">
        <div id="minecraft-processes" class="relative w-full h-[66%] bg-surface rounded-xl overflow-clip p-2">
            <div class="w-full h-8 text-center">
                <h1>Minecraft Processes</h1>
            </div>
            <VerticalScroll columns={1} items={[...processMap.values()]} let:prop={mcProcess}>
                <div class="bg-surface w-full h-[3.25rem] rounded-lg text-lg text-center flex justify-between overflow-clip py-2">
                    <div class="h-full w-full flex flex-row justify-between items-center px-2">
                        <h1 class="w-[33%] text-start">{mcProcess.pid}</h1>
                        <h1 class="w-[33%] text-center">{mcProcess.info.client}</h1>
                        <h1 class="w-[33%] text-end">{mcProcess.info.version}</h1>
                    </div>
                    <div class="h-full flex flex-row justify-between items-center px-2 gap-2"> <!-- border-l-2 border-overlay -->
                        <button class="icon-button" on:click={async() => killProcess(mcProcess.pid)}>
                            <i class="fa-solid fa-skull"></i>
                        </button>
                        <button class="icon-button" on:click={async() => createLaunchProfile(mcProcess)}>
                            <i class="fa-solid fa-plus"></i>
                        </button>
                        <button class="icon-button" on:click={async() => showProcessInfo(mcProcess)}>
                            <i class="fa-solid fa-info"></i>
                        </button>
                    </div>
                </div>
            </VerticalScroll>
        </div>
        <div id="analytics" class="w-full h-[33%] bg-surface rounded-xl text-center">
            <h1>Analytics</h1>
        </div>
    </div>
    <div id="right" class="w-[50%] h-full overflow-clip">
        <div id="console" class="relative w-full h-full bg-surface rounded-xl text-center overflow-clip p-2">
            <div class="relative w-full text-center">
                <div id="console-popout" class="absolute right-1.5 cursor-pointer">
                    <i class="fa-solid fa-arrow-up-right-from-square"></i>
                </div>
                <h1>Console Output</h1>
            </div>

            <div id="console-container" class="absolute bg-crust rounded-lg top-10 bottom-2 left-2 right-2 break-words py-2 pl-2 pr-1">
                <div id="target-process-container" class="w-full h-6 flex items-center pr-1 gap-2">
                    <div class="px-2 h-full bg-overlay rounded">
                        <h1>PName: Lunar</h1>
                    </div>
                    <div class="px-2 h-full bg-overlay rounded">
                        <h1>PID: 5728</h1>
                    </div>
                </div>
                <div id="console-content" class="absolute top-9 bottom-2 left-2 right-1 overflow-y-scroll gap-2 pr-1 text-sm text-start">
                    {longString}
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .icon-button {
        @apply w-8 h-8 bg-overlay rounded;
        font-size: 1rem;
    }
    #console-content {
        font-size: 0.8rem;
    }
    ::-webkit-scrollbar {
        @apply w-1;
    }
    ::-webkit-scrollbar-track {
        @apply bg-none;
    }
    ::-webkit-scrollbar-thumb {
        @apply bg-overlay rounded-xl relative;
    }
</style>

<!-- profile-name profile-info profile-launch profile-settings  -->