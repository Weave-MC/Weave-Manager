<script lang="ts">
    import type {Mod} from "../../../../scripts/types"
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import ButtonBar from "../../../util/ButtonBar.svelte";

    const modList: Mod[] = [
        {name: "Seraph", description: "Statistic Mod for Hypixel", version: "1.0.0", authors: ["exejar"], filePath: "", fileName: "Seraph-1.0.0", disabled: false},
        {name: "Raven Weave", description: "Raven ported to Weave", version: "1.0.0", authors: ["PianoPenguin"], filePath: "", fileName: "RavenWeave-1.0.0", disabled: false},
        {name: "Concentra", description: "OneConfig transformer", version: "1.0.0", authors: ["Candice"], filePath: "", fileName: "Concentra-1.0.0", disabled: false},
        {name: "NoHitDelay", description: "Removes hit delay", version: "1.0.0", authors: ["nils"], filePath: "", fileName: "NoHitDelay-1.0.0", disabled: false},
        {name: null, description: null, version: null, authors: null, filePath: "", fileName: "Weave-Mod-1", disabled: true},
        {name: null, description: null, version: null, authors: null, filePath: "", fileName: "Weave-Mod-2", disabled: false},
        {name: null, description: null, version: null, authors: null, filePath: "", fileName: "Weave-Mod-3", disabled: false},
        {name: null, description: null, version: null, authors: null, filePath: "", fileName: "Weave-Mod-4", disabled: false},
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
    <div id="enabled-mods" class="relative w-full h-[100%] bg-surface rounded-xl text-center p-2">
        <div class="relative w-full text-center">
            <div id="open-folder" class="absolute right-1.5 cursor-pointer">
                <i class="fa-regular fa-folder-open"></i>
            </div>
            <h1>Mods</h1>
        </div>
        <VerticalScroll items={modList.sort(modListCompare)} let:prop={mod}>
            <div class="w-full h-[2.5rem] rounded-lg flex items-center justify-between p-2 {mod.disabled ? 'bg-base' : 'bg-surface'}">
                <div id="mod-info" class="h-full flex items-center gap-2">
                    <h1>{mod.name}</h1>
                    <h1>{mod.version}</h1>
                </div>
                <div id="options" class="h-full flex items-center gap-2">
                    <ButtonBar class="gap-2" buttons={[
                        { label: mod.disabled ? "Enable Mod" : "Disable Mod", action: () => toggleMod(mod), icon: mod.disabled ? "fa-solid fa-plus" : "fa-solid fa-minus" },
                        { label: "Mod Info", action: () => showModInfo(mod), icon: "fa-solid fa-info" }
                    ]}/>
                </div>
            </div>
        </VerticalScroll>
    </div>
</div>