<script lang="ts">
    import DockerImagesManager from "../../assets/ts/DockerImagesManager.js";
    import {listen} from "@tauri-apps/api/event";
    import util from "../../lib/util/functions"
    export let dockerImagesManager: DockerImagesManager;

    let errorMessage = ""
    let progressMessage = ""
    let successMessage = ""

    let imageName = ""

    let adding = false

    async function addImage() {
        errorMessage = ""
        successMessage = ""
        progressMessage = "Adding Image"
        if(!dockerImagesManager) {
            progressMessage = ""
            errorMessage = "There was an error! Please close and open this prompt again"
            return;
        }
        if(adding) {
            progressMessage = ""
            errorMessage = "The image is being added! Please wait."
            return;
        }
        adding = true
        let uniqueId = `addImage-${util.generateRandomString(20)}`
        listen(uniqueId, (event) => {
            let extra = ""
            if(event.payload.progressDetail.current && event.payload.progressDetail.total) {
                console.log(`current: ${event.payload.progressDetail.current}, total: ${event.payload.progressDetail.total}`)
                extra = `: ${util.resolveSizeToHighestUnit(parseInt(event.payload.progressDetail.current))}/${util.resolveSizeToHighestUnit(parseInt(event.payload.progressDetail.total))}`
            }
            document.getElementById("progress").innerText = `${event.payload.status}${extra}`
        }).then(() =>{})
        await dockerImagesManager.addImageByName(imageName, uniqueId)
            .then((_reply) => {
                progressMessage = ""
                successMessage = "Success! Image added."
                adding = false
            })
            .catch(async (error) => {
                progressMessage = ""
                errorMessage = `There was an error adding the image!`
                await new Promise(resolve => setTimeout(resolve, 1))
                document.getElementById("errorMsg").innerText = `${error.error}${error.error_msg ? `: ${error.error_msg}`: ""}`
                console.log(error)
                adding = false
            })

    }

</script>

<div class="modal">
    <h2 style="text-align: center">Add Image</h2>
    {#if errorMessage !== ""}
        <div class="highlight error">
            <p>{errorMessage}</p>
            <p id="errorMsg"></p>
        </div>
    {/if}
    {#if progressMessage !== ""}
        <div class="highlight progress">
            <p>{progressMessage}</p>
            <p id="progress"></p>
        </div>
    {/if}
    {#if successMessage !== ""}
        <div class="highlight success">
            <p>{successMessage}</p>
        </div>
    {/if}
    <p>Both official and unofficial repositories are supported, but for unofficial repositories, the full link is required</p>
    <div class="field">
        <label>
            Image
            <input type="text" bind:value={imageName}>
        </label>
    </div>
    <div class="btn connect-button" on:click={addImage}>
        Add Image
    </div>
</div>
