<script lang="ts">
    import { watch, watchImmediate } from "tauri-plugin-fs-watch-api"
    import {onMount} from "svelte";

    type Mod = {
        path: string;
        name: string;
        author: string;
        version: string;
        link: string;
    }

    let modList: Mod[] = []

    onMount(async () => {
        await watchImmediate(
            "C:/Users/exeja/.weave/mods",
            (event) => {
                const { type, paths } = event

                if (typeof type === "object") {
                    if ("create" in type) {
                        console.log(paths[0])
                    } else if ("remove" in type) {
                        console.log(paths[0])
                    }
                }
            }
        )
    })
</script>

<div id="mod-list" class="w-full h-full">
    <div id="mod-list-title" class="w-full h-8 flex justify-center items-center border-b-2 border-overlay">
        <h1 class="absolute">Weave Mods</h1>
        <div class="w-full flex justify-end px-2">
            <i class="fa-regular fa-folder-open"></i>
        </div>
    </div>
    <div id="content" class="w-full h-full pb-8 overflow-y-auto">
        <div id="list" class="w-full flex flex-col">
            {#each modList as mod}
                <div class="relative mod-item">

                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    .mod-item {
        @apply relative bg-surface w-full h-10 border-b-[0.0625rem] border-overlay flex flex-row justify-center items-center;
    }
</style>