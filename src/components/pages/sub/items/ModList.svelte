<script lang="ts">
    import type {Mod} from "../../../../scripts/types"
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import ButtonBar from "../../../util/ButtonBar.svelte";
    import {modList} from "../../../../scripts/stores";
    import {toggleMod} from "../../../../scripts/components";
    import {getModsDirectory} from "../../../../scripts/paths";
    import {open} from "@tauri-apps/api/shell";

    function modListCompare(a: Mod, b: Mod) {
        if (a.disabled && !b.disabled)
            return 1
        if (b.disabled && !a.disabled)
            return -1
        return 0
    }

    function showModInfo(mod: Mod) {

    }

    function getModName(mod: Mod): string {
        return mod.mod_info.name !== "undefined" ? mod.mod_info.name : mod.file_name
    }
    function getModVersion(mod: Mod): string {
        return mod.mod_info.version !== "undefined" ? mod.mod_info.version : ""
    }

    async function openModFolder() {
        await open(await getModsDirectory())
    }
</script>

<div id="mod-list" class="relative w-full h-full rounded-xl flex flex-col gap-3">
    <div id="enabled-mods" class="relative w-full h-[100%] bg-surface rounded-xl text-center p-2">
        <div class="relative w-full text-center">
            <button id="open-folder" class="absolute right-1.5 cursor-pointer" on:click={async () => await openModFolder()}>
                <i class="fa-regular fa-folder-open"></i>
            </button>
            <h1>Mods</h1>
        </div>
        <VerticalScroll items={$modList.sort(modListCompare)} let:prop={mod}>
            <div class="w-full h-[2.5rem] rounded-lg flex items-center justify-between p-2 {mod.disabled ? 'bg-base' : 'bg-surface'}">
                <div id="mod-info" class="h-full flex items-center gap-2">
                    <h1>{getModName(mod)}</h1>
                    <h1>{getModVersion(mod)}</h1>
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