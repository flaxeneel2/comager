<script lang="ts">
    import {PAGES} from "../enums/pages.js";
    export let page = PAGES.HOME
</script>
<navbar>
    <div class="navbar-button" on:click={() => {page = PAGES.HOME}}>Home</div>
    <div class="navbar-button" on:click={() => {page = PAGES.CONTAINERS}}>Containers</div>
    <div class="navbar-button" on:click={() => page = PAGES.IMAGES}>Images</div>
    <div class="navbar-button new-button" on:click={() => {page = PAGES.CONNECT}}>Connect</div>
</navbar>