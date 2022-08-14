<script lang="ts">
    import DockerContainersManager from "../../assets/ts/DockerContainersManager";
    export let dockerContainersManager: DockerContainersManager;

    let errorMessage = ""
    let progressMessage = ""
    let successMessage = ""

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
</div>
