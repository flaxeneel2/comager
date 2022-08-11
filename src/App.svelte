<script lang="ts">
    import Menu from "./pages/components/Menu.svelte"
    import {Page} from "./enums"

    /* Required for modals */

    import Modal from "svelte-simple-modal"
    import { writable } from 'svelte/store';
    const modal = writable(null);

    /*                     */

    import Home from "./pages/Home.svelte";
    import Containers from "./pages/containers/Containers.svelte";
    import Images from "./pages/images/Images.svelte";
    import Networks from "./pages/networks/Networks.svelte";
    let currentlyOpen = Page.HOME;
</script>

<Modal modal={$modal}>
    <Menu bind:currentlyOpen/>

    <div class="data">
        {#if currentlyOpen === Page.HOME}
            <Home/>
        {:else if currentlyOpen === Page.CONTAINERS}
            <Containers/>
        {:else if currentlyOpen === Page.IMAGES}
            <Images/>
        {:else if currentlyOpen === Page.NETWORKS}
            <Networks/>
        {:else}
            <!--Just in case it somehow manages to not be any of the values that it is supposed to be, reset it to HOME-->
            {currentlyOpen = Page.HOME}
            <Home/>
        {/if}
    </div>
</Modal>
