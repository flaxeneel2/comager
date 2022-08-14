<script lang="ts">
    import {getContext} from "svelte";
    import DockerContainersManager from "../../assets/ts/DockerContainersManager";

    export let dockerContainersManager: DockerContainersManager;
    export let containerId; //the id of the container to delete

    let { close } = getContext("simple-modal")

    let errorMessage = ""
    let progressMessage = ""
    let successMessage = ""

    let deleted = false

    let force = false;
    let volumes = false;
    let links = false;

    async function deleteContainer() {
        errorMessage = ""
        successMessage = ""
        progressMessage = "Deleting Container"
        if(!dockerContainersManager || !containerId) {
            progressMessage = ""
            errorMessage = "There was an error! Please close and open this prompt again"
            return;
        }
        if(deleted) {
            progressMessage = ""
            errorMessage = "The container is already deleted!"
            return;
        }
        deleted = true
        await dockerContainersManager.deleteContainer(
            containerId,
            (force.toString() === "true"),
            (volumes.toString() === "true"),
            (links.toString() === "true")
        )
            .then((_reply) => {
                progressMessage = ""
                successMessage = "Success! Container deleted."
                setTimeout(close, 1000)
            })
            .catch(async (error) => {
                progressMessage = ""
                errorMessage = "There was an error deleting the container!"
                await new Promise(resolve => setTimeout(resolve, 1))
                document.getElementById("error").innerText = `${error.error}${error.error_msg ? `: ${error.error_msg}` : ""}`
                console.log(error)
            })
    }

</script>

<div class="modal">
    <h2 style="text-align: center">Delete Container</h2>
    {#if errorMessage !== ""}
        <div class="highlight error">
            <p>{errorMessage}</p>
            <p id="error"></p>
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
            <input type="checkbox" bind:checked={force}>
        </label>
    </div>
    <div class="field">
        <label>
            Remove volumes
            <input type="checkbox" bind:checked={volumes}>
        </label>
    </div>
    <div class="field">
        <label>
            Remove links
            <input type="checkbox" bind:checked={links}>
        </label>
    </div>
    <div class="btn delete-button" on:click={deleteContainer}>
        Delete Container
    </div>
</div>
