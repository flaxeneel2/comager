<script>
    import DockerImagesManager from "../../assets/ts/DockerImagesManager";
    import {onMount,onDestroy,getContext} from "svelte";
    import DeleteImageConfirmationModal from "./DeleteImageConfirmationModal.svelte";
    import modals from "../../lib/util/modals";
    import util from "../../lib/util/functions"
    import AddImageModal from "./AddImageModal.svelte";
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
        })
    })

    onDestroy(() => {
        dockerImagesManager.unload();
    })

    const { open } = getContext("simple-modal")

    function openDeletionConfirmation(imageName) {
        open(
            DeleteImageConfirmationModal,
            {
                dockerImagesManager: dockerImagesManager,
                imageName: imageName
            },
            modals.getDarkThemeStyle()
        )
    }
    function openAddImageModal() {
        open(
            AddImageModal,
            {
                dockerImagesManager: dockerImagesManager
            },
            modals.getDarkThemeStyle()
        )
    }
</script>
<div>
    <div class="add-image btn" on:click={openAddImageModal}>
        Add a new image
    </div>
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
        <div class="images">
            {#each images as image}
                <div class="gray box">
                    <h3>{image.RepoTags[0]}</h3>
                    <p>ID: {image.Id.split(":")[1].slice(0, 12)}</p>
                    <p>Number of containers: {image.Containers === -1 ? "0" : image.Containers}</p>
                    <p>Creation Date: {new Date(image.Created*1000).toString()}</p>
                    <p>Size: {util.resolveSizeToHighestUnit(image.Size)}</p>
                    <div class="trash btn" on:click={() => openDeletionConfirmation(image.RepoTags[0])}>
                        Delete
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>