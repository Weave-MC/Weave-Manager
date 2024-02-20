<script lang="ts">
    import type {Agent} from "../../../../scripts/types"
    import VerticalScroll from "../../../util/VerticalScroll.svelte";

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
        <div class="{agent.disabled ? 'bg-base' : 'bg-surface'} relative w-full h-[2.5rem] rounded-lg flex justify-between items-center overflow-clip p-2">
            <div class="w-full h-full left-2 flex justify-start gap-2 items-center">
                <h1 class="text-lg">{agent.fileName}</h1>
                {#if agent.disabled}
                    <h1 class="text-disabled">Un-Linked</h1>
                {/if}
            </div>
            <div class="tooltip absolute h-9 right-12 p-2 bg-overlay rounded-lg flex justify-center items-center opacity-0">
                {agent.disabled ? 'Link' : 'Un-Link'}
            </div>
            <button id="state" class="w-8 h-7 bg-overlay rounded-lg {agent.disabled ? 'disabled' : 'enabled'}" on:click={() => toggleAgent(agent)}>
                {#if agent.disabled}
                    <i class="fa-solid fa-link-slash"></i>
                {:else}
                    <i class="fa-solid fa-link"></i>
                {/if}
            </button>
        </div>
    </VerticalScroll>
</div>