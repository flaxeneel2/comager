<script lang="ts">
    import DockerInfoManager from "../assets/ts/DockerInfoManager";
    import { getContext } from 'svelte';
    import ConnectionModal from "./NewConnectionModal.svelte"
    const { open } = getContext("simple-modal");
    let fieldMessage = "Loading..."
    window.onload = () => {
        let dockerInfoManager = new DockerInfoManager();
        dockerInfoManager.updateFieldsPeriodically();
        dockerInfoManager.addEventListener("infoFetchError", (e: CustomEvent) => {
            console.log(`Encountered error!`)
            console.log(e.detail)
            if(e.detail.error_msg) {
                fieldMessage = e.detail.error_msg
            } else fieldMessage = e.detail.error
        })
    }

    function cancelNewConnection() {

    }

    function reconnectWithNewConnection() {

    }

    function newConnection() {
        open(
            ConnectionModal,
            {
                cancelNewConnection,
                reconnectWithNewConnection
            },
            {
                styleWindow: {
                    border: "3px solid #454545",
                    backgroundColor: "#1a1a1a",
                    color: "aliceblue"
                }
            }
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