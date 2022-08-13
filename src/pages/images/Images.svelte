<script>
    import {invoke} from "@tauri-apps/api/tauri";

    let images;
    let error = "";
    (async () => {
        await invoke("get_docker_images")
            .then()
    })();

</script>
<div>
    {#if !images}
        <div class="banner progress">
            Loading images
        </div>
    {:else if error !== ""}
        <div class="banner error">
            {`${error.error} ${error.error_msg ? error.error_msg : ""}`}
        </div>
    {:else if images.length === 0}
        <div class="banner error">
            There are no images! Add an image by pressing the button above!
        </div>
    {:else}
        <div class="highlights">
            {#each images as image}
                <div class="gray box">

                </div>
            {/each}
        </div>
    {/if}
</div>