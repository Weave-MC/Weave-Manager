<script lang="ts">
    import type {Mod} from "../../../../scripts/types"
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import AgentList from "./AgentList.svelte";

    const modList: Mod[] = [
        {name: "Seraph", description: "Statistic Mod for Hypixel", version: "1.0.0", authors: ["exejar"], filePath: "", fileName: "Seraph-1.0.0", disabled: false},
        {name: "Raven Weave", description: "Raven ported to Weave", version: "1.0.0", authors: ["PianoPenguin"], filePath: "", fileName: "RavenWeave-1.0.0", disabled: false},
        {name: "Concentra", description: "OneConfig transformer", version: "1.0.0", authors: ["Candice"], filePath: "", fileName: "Concentra-1.0.0", disabled: false},
        {name: "NoHitDelay", description: "Removes hit delay", version: "1.0.0", authors: ["nils"], filePath: "", fileName: "NoHitDelay-1.0.0", disabled: false},
        {name: null, description: null, version: null, authors: null, filePath: "", fileName: "Weave-Mod-1", disabled: true},
        {name: null, description: null, version: null, authors: null, filePath: "", fileName: "Weave-Mod-2", disabled: false},
        {name: null, description: null, version: null, authors: null, filePath: "", fileName: "Weave-Mod-3", disabled: false},
        {name: null, description: null, version: null, authors: null, filePath: "", fileName: "Weave-Mod-4", disabled: false},
    ]

    function modListCompare(a: Mod, b: Mod) {
        if (a.disabled && !b.disabled)
            return 1
        if (b.disabled && !a.disabled)
            return -1
        return 0
    }

    function toggleMod(mod: Mod) {
        // reactive updating
        modList[modList.indexOf(mod)] = {
            name: mod.name,
            description: mod.description,
            version: mod.version,
            authors: mod.authors,
            filePath: mod.filePath,
            fileName: mod.fileName,
            disabled: !mod.disabled
        }
    }

    function showModInfo(mod: Mod) {

    }
</script>

<div id="mod-list" class="relative w-full h-full rounded-xl flex flex-col gap-3">
    <div id="enabled-mods" class="relative w-full h-[60%] bg-surface rounded-xl text-center p-2">
        <div class="relative w-full text-center">
            <div id="open-folder" class="absolute right-1.5 cursor-pointer">
                <i class="fa-regular fa-folder-open"></i>
            </div>
            <h1>Mods</h1>
        </div>
        <VerticalScroll items={modList.filter((mod) => !mod.disabled)} let:prop={mod}>
            <div class="w-full h-[2.5rem] bg-surface rounded-lg flex items-center justify-between p-2">
                <div id="enabled-mod-info" class="h-full flex items-center gap-2">
                    <h1>{mod.name}</h1>
                    <h1>{mod.version}</h1>
                </div>
                <div id="enabled-buttons" class="h-full flex items-center gap-2">
                    <button class="icon-button" on:click={() => toggleMod(mod)}>
                        <i class="fa-solid fa-minus"></i>
                    </button>
                    <button class="icon-button" on:click={() => showModInfo(mod)}>
                        <i class="fa-solid fa-info"></i>
                    </button>
                </div>
            </div>
        </VerticalScroll>
    </div>
    <div id="disabled-mods" class="relative w-full h-[40%] bg-surface rounded-xl text-center p-2">
        <div class="relative w-full text-center">
            <h1>Disabled</h1>
        </div>
        <VerticalScroll items={modList.filter((mod) => mod.disabled)} let:prop={mod}>
            <div class="w-full h-[2.5rem] bg-base rounded-lg flex items-center justify-between p-2">
                <div id="disabled-mod-info" class="h-full flex items-center gap-2">
                    <h1>{mod.name}</h1>
                    <h1>{mod.version}</h1>
                </div>
                <div id ="disabled-buttons" class="h-full flex items-center gap-2">
                    <button class="icon-button" on:click={() => toggleMod(mod)}>
                        <i class="fa-solid fa-plus"></i>
                    </button>
                    <button class="icon-button" on:click={() => showModInfo(mod)}>
                        <i class="fa-solid fa-info"></i>
                    </button>
                </div>
            </div>
        </VerticalScroll>


    </div>
</div>

<style>
    .icon-button {
        @apply w-7 h-7 bg-overlay rounded;
        font-size: 1rem;
    }
</style>