<script lang="ts">
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import {ModProfile} from "../../../../scripts/types";

    const modProfiles: ModProfile[] = [
        new ModProfile("Profile 1", [{name: "Seraph", description: "Statistic Mod for Hypixel", version: "1.0.0", authors: ["exejar"], filePath: "", fileName: "Seraph-1.0.0", disabled: false}]),
        new ModProfile("Profile 2", [{name: "Seraph", description: "Statistic Mod for Hypixel", version: "1.0.0", authors: ["exejar"], filePath: "", fileName: "Seraph-1.0.0", disabled: true}]),
        new ModProfile("Profile 3", [{name: "Seraph", description: "Statistic Mod for Hypixel", version: "1.0.0", authors: ["exejar"], filePath: "", fileName: "Seraph-1.0.0", disabled: false}]),
        new ModProfile("Profile 4", [{name: "Seraph", description: "Statistic Mod for Hypixel", version: "1.0.0", authors: ["exejar"], filePath: "", fileName: "Seraph-1.0.0", disabled: true}]),
        new ModProfile("Profile 5", [{name: "Seraph", description: "Statistic Mod for Hypixel", version: "1.0.0", authors: ["exejar"], filePath: "", fileName: "Seraph-1.0.0", disabled: false}]),
        new ModProfile("Profile 6", [{name: "Seraph", description: "Statistic Mod for Hypixel", version: "1.0.0", authors: ["exejar"], filePath: "", fileName: "Seraph-1.0.0", disabled: true}])
    ]
    
    function loadProfile(profile: ModProfile) {
        
    }
    
    function saveProfile(profile: ModProfile) {
        
    }
    
    function profileInfo(profile: ModProfile) {
        
    }
</script>

<div id="mod-profiles" class="relative w-full h-full rounded-xl bg-surface text-center p-2">
    <div class="relative w-full text-center">
        <h1>Mod Profiles</h1>
    </div>
    <VerticalScroll columns={1} items={modProfiles} let:prop={profile}>
        <div class="w-full h-[3.25rem] bg-surface rounded-lg flex items-center justify-between p-2">
            <div id="profile-info" class="h-full flex items-center gap-2">
                <h1>{profile.name}</h1>
            </div>
            <div id="buttons" class="h-full flex items-center gap-2">
                <button id="load-profile" class="icon-button" on:click={() => loadProfile(profile)}>
                    <i class="fa-solid fa-arrow-right-from-bracket"></i>
                </button>
                <button id="save-profile" class="icon-button" on:click={() => saveProfile(profile)}>
                    <i class="fa-solid fa-floppy-disk"></i>
                </button>
                <button id="profile-info" class="icon-button" on:click={() => profileInfo(profile)}>
                    <i class="fa-solid fa-info"></i>
                </button>
            </div>
        </div>
    </VerticalScroll>
</div>

<style>
    .icon-button {
        @apply w-8 h-8 bg-overlay rounded;
        font-size: 1rem;
    }
</style>