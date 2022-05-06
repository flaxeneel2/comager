<script lang="ts">
    let images;
    import { invoke } from '@tauri-apps/api/tauri'
    import { onDestroy } from 'svelte';
    async function updateImages() {
        images = await invoke("get_docker_images").catch((err) => images = err);
        console.log(images)
    }
    updateImages()
    let imageUpdater = setInterval(updateImages, 1000)
    onDestroy(() => clearInterval(imageUpdater))

    //TODO: actually implement the add image menu popup
    function openAddImageMenu() {

    }
</script>
<div class="docker-image-new-button">Add a new image</div>
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