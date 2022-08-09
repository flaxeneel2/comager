<script lang="ts">
    import BurgerMenu from "../../lib/menus/hamburger/BurgerMenu.svelte";
    import {Page} from "../../enums";
    let openStatus = false;
    export let currentlyOpen = Page.HOME;

</script>
<div class="menu">
    <div class="burger-button">
        <BurgerMenu burgerColor={'aliceblue'} duration={'0.1'} backgroundColor={'#111111'} bind:open={openStatus}>
            <p on:click={() => { openStatus = false; currentlyOpen = Page.HOME }}>Home</p>
            <p on:click={() => { openStatus = false; currentlyOpen = Page.CONTAINERS }}>Containers</p>
            <p on:click={() => { openStatus = false; currentlyOpen = Page.IMAGES     }}>Images</p>
            <p on:click={() => { openStatus = false; currentlyOpen = Page.NETWORKS   }}>Networks</p>
        </BurgerMenu>
    </div>
</div>