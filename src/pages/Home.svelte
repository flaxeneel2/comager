<script lang="ts">
    import DockerInfoManager from "../assets/ts/DockerInfoManager";
    import modalUtil from "../lib/util/modals"
    import {onMount,onDestroy,getContext} from "svelte";
    import ConnectionModal from "./NewConnectionModal.svelte"
    import type {DockerError} from "../types";
    const { open } = getContext("simple-modal");
    let fieldMessage = "Loading..."
    let dockerInfoManager = new DockerInfoManager();
    onMount(() => {
        dockerInfoManager.updateFieldsPeriodically();
        dockerInfoManager.addEventListener("infoFetchError", (e: CustomEvent) => {
            let detail: DockerError = e.detail
            if(detail.error_msg) {
                fieldMessage = detail.error_msg
            } else fieldMessage = detail.error
        })
    })
    onDestroy(() => {
        dockerInfoManager.unload();
    })
    function newConnection() {
        open(
            ConnectionModal,
            {},
            modalUtil.getDarkThemeStyle()
        )
    }
</script>

<div>
    <div class="connect btn" on:click={newConnection}>
        Connect to a docker daemon
    </div>
    <div class="highlights">
        <div class="gray box">
            <h3>Kernel</h3>
            <p id="kernel">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>Kernel Version</h3>
            <p id="kernelVersion">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>Host Name</h3>
            <p id="host">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>Docker version</h3>
            <p id="dockerVersion">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>Operating System</h3>
            <p id="operatingSystem">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>Architecture</h3>
            <p id="architecture">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>Total RAM</h3>
            <p id="memory">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>CPU core count</h3>
            <p id="threadCount">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>Number of Images</h3>
            <p id="imageCount">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>Number of Containers</h3>
            <p id="containerCount">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>Swap Limit</h3>
            <p id="swapStatus">{fieldMessage}</p>
        </div>
        <div class="gray box">
            <h3>IPv4 forwarding</h3>
            <p id="ipv4ForwardingStatus">{fieldMessage}</p>
        </div>
    </div>
</div>