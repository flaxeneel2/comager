<script lang="ts">
    let containers;
    import { invoke } from '@tauri-apps/api/tauri'
    import { onDestroy } from 'svelte';
    async function updateContainers() {
        containers = await invoke("get_docker_containers").catch((err) => containers = err);
        console.log(containers)
    }
    updateContainers()
    let containerUpdater = setInterval(updateContainers, 1000)
    onDestroy(() => clearInterval(containerUpdater))
</script>

{#if !containers}
    <p>Loading containers...</p>
{:else if (containers.error)}
    <p>There was an error while trying to fetch images!</p>
    <p>Error code: {containers.error}</p>
    <p>Error message: {containers.error_msg || "No error message"}</p>
{:else if containers.length === 0}
    <p>There seems to be no containers running! Create a container by clicking "New" in the top navbar</p>
{:else}
    <div class="docker-container-list">
    {#each containers as container}
        <div class="docker-container-button">
            <h1>{container.Names[0]}</h1>
        </div>
    {/each}
    </div>
{/if}
