<script lang="ts">
    import {PAGES} from "../enums/pages.js";

    export let containers: any[]
    import Navbar from "./Navbar.svelte";
    import Home from "./Home.svelte"
    import ManageContainer from "./containers/ManageContainer.svelte";
    import Containers from "./containers/Containers.svelte";
    import Images from "./images/Images.svelte";
    import Connect from "./Connect.svelte"
    import Modal from 'svelte-simple-modal';
    import {writable} from "svelte/store";
    let pages = PAGES.HOME
    let images;
    let modal = writable(null)
</script>
<Navbar bind:page={pages}/>
<Modal show={$modal}>
    <application>
        {#if pages === PAGES.HOME}
            <Home/>
        {:else if pages === PAGES.CONTAINER}
            <ManageContainer/>
        {:else if pages === PAGES.CONTAINERS}
            <Containers/>
        {:else if pages === PAGES.IMAGES}
            <Images/>
        {:else if pages === PAGES.CONNECT}
            <Connect/>
        {/if}
    </application>
</Modal>
