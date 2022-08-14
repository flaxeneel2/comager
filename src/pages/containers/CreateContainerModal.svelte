<script lang="ts">
    import DockerContainersManager from "../../assets/ts/DockerContainersManager";
    export let dockerContainersManager: DockerContainersManager;

    let errorMessage = ""
    let progressMessage = ""
    let successMessage = ""

    let containerName = ""

    let creationDetails = {}

    let adding = false

    async function createContainer() {
        errorMessage = ""
        successMessage = ""
        progressMessage = "Creating container"
        if(!dockerContainersManager) {
            progressMessage = ""
            errorMessage = "There was an error! Please close and open this prompt again"
            return;
        }
        if(adding) {
            progressMessage = ""
            errorMessage = "The container is being created! Please wait."
            return;
        }
        adding = true
        await dockerContainersManager.createContainer(creationDetails)
            .then((_reply) => {
                progressMessage = ""
                successMessage = "Success! Container created."
                adding = false
            })
            .catch(async (error) => {
                progressMessage = ""
                errorMessage = `There was an error creating the container!`
                await new Promise(resolve => setTimeout(resolve, 1))
                document.getElementById("errorMsg").innerText = `${error.error}${error.error_msg ? `: ${error.error_msg}`: ""}`
                console.log(error)
                adding = false
            })

    }

</script>

<div class="modal">
    <h2 style="text-align: center">Create Container</h2>
    {#if errorMessage !== ""}
        <div class="highlight error">
            <p>{errorMessage}</p>
            <p id="errorMsg"></p>
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
    <p>Any values that are not set are given default values.</p>
    <div class="field">
        <label>
            Image
            <input type="text" bind:value={creationDetails.image}>
        </label>
    </div>
    <div class="field">
        <label>
            Container Name
            <input type="text" bind:value={creationDetails.name}>
        </label>
    </div>
    <div class="field">
        <label>
            CPU limit (%)
            <input type="number" bind:value={creationDetails.cpuPercentageLimit}>
        </label>
    </div>
    <div class="field">
        <label>
            RAM limit (in MiB)
            <input type="number" bind:value={creationDetails.memoryLimit}>
        </label>
    </div>
    <div class="field">
        <label>
            Command to run
            <input type="text" bind:value={creationDetails.command}>
        </label>
    </div>
    <div class="btn connect-button" on:click={createContainer}>
        Create Container
    </div>
</div>
