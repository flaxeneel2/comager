<script>
    import DockerImagesManager from "../../assets/ts/DockerImagesManager";
    import {onMount,onDestroy} from "svelte";
    import * as types from "../../types"
    let images;
    let error = "";
    let dockerImagesManager = new DockerImagesManager();
    /* Run when the component is loaded completely */
    onMount(() => {
        console.log("called")
        dockerImagesManager.updateImagesPeriodically();
        /* Handle errors */
        dockerImagesManager.addEventListener("imagesFetchError", (event) => {
            let errorData = event.detail;
            if(errorData.error_msg) {
                error = errorData.error_msg
            } else error = errorData.error
        })
        /* Handle updates */
        dockerImagesManager.addEventListener("imagesFetchSuccess", (event) => {
            images = event.detail
            console.log(images)
        })
    })

    onDestroy(() => {
        dockerImagesManager.unload();
    })

</script>
<div>
    {#if !images && error === ""}
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
                    <h3>{image.RepoTags[0]}</h3>
                    <p>ID: {image.Id.split(":")[1].slice(0, 12)}</p>
                    <p>Number of containers: {image.Containers === -1 ? "0" : image.Containers}</p>
                    <p>Size: {dockerImagesManager.resolveSizeToHighestUnit(image.Size)}</p>
                </div>
            {/each}
        </div>
    {/if}
</div>