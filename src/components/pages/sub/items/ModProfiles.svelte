<script lang="ts">
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import type {Mod, ModProfile} from "../../../../scripts/types";
    import ButtonBar from "../../../util/ButtonBar.svelte";
    import {modList, modProfiles} from "../../../../scripts/stores";
    import {loadModProfile, saveModProfile} from "../../../../scripts/components";
    import {getProfileDirectory} from "../../../../scripts/paths";
    import {removeFile} from "@tauri-apps/api/fs";
    import CreateModProfilePopUp from "../../../popups/CreateModProfilePopUp.svelte";

    let popup: CreateModProfilePopUp

    async function deleteProfile(profile: ModProfile) {
        const filePath = `${await getProfileDirectory()}/${profile.name}.mod`
        await removeFile(filePath)
    }
    
    function profileInfo(profile: ModProfile) {
        
    }

    function getEnabledMods(): Mod[] {
        return $modList.filter((e) => !e.disabled)
    }
</script>

<div id="mod-profiles" class="relative w-full h-full rounded-xl bg-surface text-center p-2">
    <div class="relative w-full text-center">
        <button id="create-profile" class="absolute right-1.5" on:click={() => popup.startCreateModProfile(getEnabledMods())}>
            <i class="fa-regular fa-plus"></i>
        </button>
        <h1>Mod Profiles</h1>
    </div>
    <VerticalScroll columns={1} items={[...$modProfiles.values()]} let:prop={profile}>
        <div class="w-full h-[3rem] bg-surface rounded-lg flex items-center justify-between p-2">
            <div id="profile-info" class="h-full flex items-center gap-2">
                <h1>{profile.name}</h1>
            </div>
            <ButtonBar class="gap-2" buttons={[
                {label: "Load Profile", action: () => loadModProfile(profile), icon: "fa-solid fa-arrow-right-from-bracket"},
                {label: "Save Profile", action: () => saveModProfile(profile), icon: "fa-solid fa-floppy-disk"},
                {label: "Profile Info", action: () => profileInfo(profile), icon: "fa-solid fa-info"}
            ]}/>
        </div>
    </VerticalScroll>
</div>
<CreateModProfilePopUp bind:this={popup}/>