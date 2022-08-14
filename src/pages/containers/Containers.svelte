<script>
    import DockerContainersManager from "../../assets/ts/DockerContainersManager";
    import {onMount,onDestroy,getContext} from "svelte";
    import modals from "../../lib/util/modals"
    import DeleteContainerConfirmationModal from "./DeleteContainerConfirmationModal.svelte";
    import CreateContainerModal from "./CreateContainerModal.svelte";
    let containers;
    let error = "";
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
</script>
<div>
    <div class="add-image btn" on:click={openCreateContainerModal}>
        Create a new container
    </div>
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
                    <div class="container-details btn" on:click={() => openDeletionConfirmation(container.Id)}>
                        View Details
                    </div>
                    <div class="trash btn" on:click={() => openDeletionConfirmation(container.Id)}>
                        Delete
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>