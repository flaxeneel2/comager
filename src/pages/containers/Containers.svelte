<script>
    import DockerContainersManager from "../../assets/ts/DockerContainersManager";
    import {onMount,onDestroy,getContext} from "svelte";
    import modals from "../../lib/util/modals"
    import DeleteContainerConfirmationModal from "./DeleteContainerConfirmationModal.svelte";
    import CreateContainerModal from "./CreateContainerModal.svelte";
    import ContainerDetails from "./ContainerDetails.svelte";
    let containers;
    let error = "";
    let errorMessage = "";
    let progressMessage = "";
    let successMessage = ""
    let dockerContainersManager = new DockerContainersManager();
    /* Run when the component is loaded completely */
    onMount(() => {
        dockerContainersManager.updateContainersPeriodically();
        /* Handle errors */
        dockerContainersManager.addEventListener("containersFetchError", (event) => {
            let errorData = event.detail;
            if(errorData.error_msg) {
                error = errorData.error_msg
            } else error = errorData.error
        })
        /* Handle updates */
        dockerContainersManager.addEventListener("containersFetchSuccess", (event) => {
            containers = event.detail
            console.log(containers)
        })
    })

    onDestroy(() => {
        dockerContainersManager.unload();
    })

    const { open } = getContext("simple-modal")

    function openDeletionConfirmation(containerId) {
        open(
            DeleteContainerConfirmationModal,
            {
                dockerContainersManager: dockerContainersManager,
                containerId: containerId
            },
            modals.getDarkThemeStyle()
        )
    }
    function openCreateContainerModal() {
        open(
            CreateContainerModal,
            {
                dockerContainersManager: dockerContainersManager
            },
            modals.getDarkThemeStyle()
        )
    }
    async function startContainer(containerId) {

        await dockerContainersManager.startContainer(containerId)
            .then(() => {
                progressMessage = ""
                successMessage = "Container started!"
                setTimeout(() => successMessage = "", 1000)
            })
            .catch((e) => {
                errorMessage = `${e.error} ${e.error_msg ? e.error_msg : ""}`
            })
    }
    async function stopContainer(containerId) {
        await dockerContainersManager.stopContainer(containerId)
            .then(() => {
                progressMessage = ""
                successMessage = "Container stopped!"
                setTimeout(() => successMessage = "", 1000)
            })
            .catch((e) => {
                errorMessage = `${e.error} ${e.error_msg ? e.error_msg : ""}`
            })

    }
    async function restartContainer(containerId) {
        await dockerContainersManager.restartContainer(containerId)
            .then(() => {
                progressMessage = ""
                successMessage = "Container restarted!"
                setTimeout(() => successMessage = "", 1000)
            })
            .catch((e) => {
                errorMessage = `${e.error} ${e.error_msg ? e.error_msg : ""}`
            })

    }

    function openContainerDetailsModal(containerId) {
        dockerContainersManager.unload();
        open(
            ContainerDetails,
            {
                dockerContainersManager: dockerContainersManager,
                containerId: containerId
            },
            {
                styleWindow: {
                    border: "3px solid #454545",
                    backgroundColor: "#1a1a1a",
                    color: "aliceblue",
                    width: "95%"
                }
            }
        )
    }
</script>
<div>
    <div class="add-image btn" on:click={openCreateContainerModal}>
        Create a new container
    </div>
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
    {#if !containers && error === ""}
        <div class="banner progress">
            Loading containers
        </div>
    {:else if error !== ""}
        <div class="banner error">
            {`${error.error} ${error.error_msg ? error.error_msg : ""}`}
        </div>
    {:else if containers.length === 0}
        <div class="banner error">
            There are no containers! Create a container by pressing the button above!
        </div>
    {:else}
        <div class="images">
            {#each containers as container}
                <div class="gray box">
                    <p>ID: {container.Id.slice(0, 12)}</p>
                    <p>Names: {container.Names.join(", ")}</p>
                    <p>Creation Date: {new Date(container.Created*1000).toString()}</p>
                    <p>Image: {container.Image}</p>
                    <p>Status: {container.Status}</p>
                    <p>Commands: {container.Command}</p>
                    <div class="start-button btn" on:click={async () => await startContainer(container.Id)}>
                        Start Container
                    </div>
                    <div class="restart-button btn" on:click={async () => await restartContainer(container.Id)}>
                        Restart Container
                    </div>
                    <div class="stop-button btn" on:click={async () => await stopContainer(container.Id)}>
                        Stop Container
                    </div>
                    <div class="stop-button btn trash" on:click={() => openDeletionConfirmation(container.Id)}>
                        Delete
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>