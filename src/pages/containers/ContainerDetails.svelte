<script lang="ts">
    import DockerContainersManager from "../../assets/ts/DockerContainersManager";
    import {onDestroy, onMount} from "svelte";
    import util from "../../lib/util/functions"
    import {emit, listen} from "@tauri-apps/api/event";
    export let dockerContainersManager: DockerContainersManager;
    export let containerId;

    let errorMessage = ""
    let progressMessage = ""
    let successMessage = ""

    let containerDetails;
    let containerDetailsUpdateLoop;

    let uniqueId = util.generateRandomString(20)

    onMount(() => {
        containerDetailsUpdateLoop = setInterval(async () => {
            containerDetails = await dockerContainersManager.getContainerDetails(containerId)
        }, 500)
        dockerContainersManager.attachToContainers(containerId, uniqueId)

    })

    onDestroy(() => {
        clearInterval(containerDetailsUpdateLoop)
        emit(uniqueId, "END_TUNNEL|")
    })

    listen(uniqueId, (data) => {
        console.log(data)
    })

    function handleStartAttempt() {

    }

    function handleStopAttempt() {

    }

</script>

<div class="modal">
    <h2 style="text-align: center">Container details</h2>
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
    <div class="btn start-button" on:click={() => handleStartAttempt()}>
        Start Container
    </div>
    <div class="btn stop-button" on:click={() => handleStopAttempt()}>
        Stop Container
    </div>

</div>
