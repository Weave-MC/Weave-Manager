<script lang="ts">
    import HeaderBar from "./components/HeaderBar.svelte";
    import SideBar from "./components/SideBar.svelte";

    let selectedPage: any
</script>

<main id="main" class="theme-moonlight absolute w-screen h-screen overflow-clip text-text bg-crust select-none">
    <HeaderBar/>
    <SideBar bind:selectedPage/>
    <div id="content" class="fixed top-14 left-14 right-0 bottom-0 flex flex-col p-3 gap-3">
        <svelte:component this={selectedPage}/>
    </div>
</main>