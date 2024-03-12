<script lang="ts">
    import type {Agent} from "../../../../scripts/types"
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import ButtonBar from "../../../util/ButtonBar.svelte";
    import {agentList} from "../../../../scripts/stores";
    import {renameFile} from "@tauri-apps/api/fs";
    import {getAgentsDirectory} from "../../../../scripts/paths";
    import {open} from "@tauri-apps/api/shell";

    function agentListCompare(a: Agent, b: Agent) {
        if (a.disabled && !b.disabled)
            return 1
        if (b.disabled && !a.disabled)
            return -1
        return 0
    }


    async function toggleAgent(agent: Agent) {
        agent.disabled = !agent.disabled

        if (agent.disabled)
            await renameFile(agent.file_path, `${agent.file_path}.disabled`)
        else
            await renameFile(`${agent.file_path}.disabled`, agent.file_path)

        $agentList[$agentList.indexOf(agent)] = agent
    }

    async function openAgentsFolder() {
        await open(await getAgentsDirectory())
    }
</script>

<div id="agent-list" class="relative w-full h-full bg-surface rounded-xl p-2">
    <div class="relative w-full text-center">
        <button id="open-folder" class="absolute right-1.5 cursor-pointer" on:click={async () => await openAgentsFolder()}>
            <i class="fa-regular fa-folder-open"></i>
        </button>
        <h1>Agents</h1>
    </div>
    <VerticalScroll columns={1} items={$agentList.sort(agentListCompare)} let:prop={agent}>
        <div class="w-full h-[2.75rem] rounded-lg flex items-center justify-between p-2 {agent.disabled ? 'bg-base' : 'bg-surface'}">
            <div class="h-full flex items-center gap-2">
                <h1 class="text-lg">{agent.file_name}</h1>
            </div>
            <ButtonBar buttons={[
                {label: agent.disabled ? "Link Agent" : "Un-Link Agent", action: () => toggleAgent(agent), icon: agent.disabled ? "fa-solid fa-link-slash" : "fa-solid fa-link"}
            ]}/>
        </div>
    </VerticalScroll>
</div>