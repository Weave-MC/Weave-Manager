<script lang="ts">
    import type {Agent} from "../../../../scripts/types"
    import VerticalScroll from "../../../util/VerticalScroll.svelte";
    import ButtonBar from "../../../util/ButtonBar.svelte";

    const agentList: Agent[] = [
        {filePath: "", fileName: "Agent 1", disabled: false},
        {filePath: "", fileName: "Agent 2", disabled: true},
        {filePath: "", fileName: "Agent 3", disabled: false},
        {filePath: "", fileName: "Agent 4", disabled: false},
    ]

    function agentListCompare(a: Agent, b: Agent) {
        if (a.disabled && !b.disabled)
            return 1
        if (b.disabled && !a.disabled)
            return -1
        return 0
    }
    
    function toggleAgent(agent: Agent) {
        // reactive updating
        agentList[agentList.indexOf(agent)] = {filePath: agent.filePath, fileName: agent.fileName, disabled: !agent.disabled}
    }
</script>

<div id="agent-list" class="relative w-full h-full bg-surface rounded-xl p-2">
    <div class="relative w-full text-center">
        <div id="open-folder" class="absolute right-1.5 cursor-pointer">
            <i class="fa-regular fa-folder-open"></i>
        </div>
        <h1>Agents</h1>
    </div>
    <VerticalScroll columns={1} items={agentList.sort(agentListCompare)} let:prop={agent}>
        <div class="w-full h-[2.75rem] rounded-lg flex items-center justify-between p-2 {agent.disabled ? 'bg-base' : 'bg-surface'}">
            <div class="h-full flex items-center gap-2">
                <h1 class="text-lg">{agent.fileName}</h1>
                {#if agent.disabled}
                    <h1 class="text-disabled">Un-Linked</h1>
                {/if}
            </div>
            <ButtonBar buttons={[
                {label: agent.disabled ? "Link Agent" : "Un-Link Agent", action: () => toggleAgent(agent), icon: agent.disabled ? "fa-solid fa-link-slash" : "fa-solid fa-link"}
            ]}/>
        </div>
    </VerticalScroll>
</div>