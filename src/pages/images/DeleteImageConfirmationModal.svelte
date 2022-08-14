<script lang="ts">
    import {getContext} from "svelte";
    import DockerImagesManager from "../../assets/ts/DockerImagesManager.js";

    export let dockerImagesManager: DockerImagesManager;
    export let imageName; //the name of the image to delete

    let { close } = getContext("simple-modal")

    let errorMessage = ""
    let progressMessage = ""
    let successMessage = ""

    let deleted = false

    let force = false;

    async function deleteImage() {
        progressMessage = "Deleting Image"
        if(!dockerImagesManager || !imageName) {
            progressMessage = ""
            errorMessage = "There was an error! Please close and open this prompt again"
            return;
        }
        if(deleted) {
            progressMessage = ""
            errorMessage = "The image is already deleted!"
            return;
        }
        await dockerImagesManager.deleteImage(imageName, (force.toString() === "true"))
            .then((_reply) => {
                progressMessage = ""
                successMessage = "Success! Image deleted."
                setTimeout(close, 1000)
            })
            .catch((error) => {
                progressMessage = ""
                errorMessage = "There was an error deleting the image!"
                console.log(error)
            })
    }

</script>

<div class="modal">
    <h2 style="text-align: center">Delete Image</h2>
    {#if errorMessage !== ""}
        <div class="highlight error">
            <p>{errorMessage}</p>
        </div>
    {/if}
    {#if progressMessage !== ""}
        <div class="highlight progress">
            <p>{progressMessage}</p>
        </div>
    {/if}
    {#if successMessage !== ""}
        <div class="highlight success">
            <p>{successMessage}</p>
        </div>
    {/if}
    <p>Deletions are permanent and cannot be undone.</p>
    <div class="field">
        <label>
            Force delete
            <input type="checkbox" bind:value={force}>
        </label>
    </div>
    <div class="btn delete-button" on:click={deleteImage}>
        Delete Image
    </div>
</div>
