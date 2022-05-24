<script lang="ts">
    let images;
    import { invoke } from '@tauri-apps/api/tauri'
    import { onDestroy, getContext } from 'svelte';
    import AddImage from "./AddImage.svelte"
    import { fly } from "svelte/transition";
    let { open } = getContext("simple-modal")

    async function updateImages() {
        images = await invoke("get_docker_images").catch((err) => images = err);
    }
    updateImages()
    let imageUpdater = setInterval(updateImages, 1000)
    onDestroy(() => clearInterval(imageUpdater))

    //TODO: actually implement the add image menu popup
    let onOkay = async (image) => {
        console.log("called!")
        let btn = document.getElementById("addImage")
        btn.innerHTML = "Adding image..."
        await invoke("install_docker_image_from_repo", { repo: "Hi", imageName: image })
        btn.innerHTML = "Image added!"
        await new Promise(resolve => setTimeout(resolve, 10000))
        btn.innerHTML = "Add a new image"
    }
    let onClose = async () => {
        let btn = document.getElementById("addImage")
        btn.innerHTML = "Action cancelled!"
        await new Promise(resolve => setTimeout(resolve, 10000))
        btn.innerHTML = "Add a new image"
    }
    function openAddImageMenu() {
        open(
            AddImage,
            {
                onOkay,
                onClose
            },
            {
                transitionWindow: fly,
                transitionWindowProps: {
                    y: 100,
                    duration: 1000
                },
            }
        )
    }

    async function addImageTemp() {
        let btn = document.getElementById("addImage")
        btn.innerHTML = "Adding image..."
        await invoke("install_docker_image_from_repo", { repo: "Hi", imageName: "debian:latest" })
        btn.innerHTML = "Image added!"
        await new Promise(resolve => setTimeout(resolve, 10000))
        btn.innerHTML = "Add a new image"
    }
</script>
<div id="addImage" class="docker-image-new-button" on:click={openAddImageMenu}>Add a new image</div>
{#if !images}
    <p>Loading images...</p>
{:else if (images.error)}
    <p>There was an error while trying to fetch images!</p>
    <p>Error code: {images.error}</p>
    <p>Error message: {images.error_msg || "No error message"}</p>
{:else if images.length === 0}
    <p>No images found! You can add an image by clicking the "New Image" button!</p>
{:else}
    <div class="docker-image-list">
    {#each images as image}
        <div class="docker-image-button">
            <h1>{image.RepoTags[0].split(":")[0]}</h1>
            <p>{image.RepoTags[0].split(":")[1]}</p>
        </div>
    {/each}
    </div>
{/if}