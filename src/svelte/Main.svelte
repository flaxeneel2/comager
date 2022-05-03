<script lang="ts">
    import {PAGES} from "../enums/pages.js";

    export let containers: any[]
    import Navbar from "./Navbar.svelte";
    import Home from "./Home.svelte"
    import ManageContainer from "./ManageContainer.svelte";
    import Containers from "./Containers.svelte";
    import Images from "./images/Images.svelte";
    let pages = PAGES.HOME
    let images;

</script>
<Navbar bind:page={pages}/>
<application>
    {#if pages === PAGES.HOME}
        <Home/>
    {:else if pages === PAGES.CONTAINER}
        <ManageContainer/>
    {:else if pages === PAGES.CONTAINERS}
        <Containers/>
    {:else if pages === PAGES.IMAGES}
        <Images/>
    {/if}
</application>
