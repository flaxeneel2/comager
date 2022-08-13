<script>
    import {invoke} from "@tauri-apps/api/tauri";

    let options = [
        {
            id: 1,
            text: "HTTP"
        },
        {
            id: 2,
            text: "Socket"
        },
        {
            id: 3,
            text: "SSL"
        },
    ]
    let selected = 0;
    let connectionDetails = {
        timeout: 5
    }

    let errorMessage = ""
    let progressMessage = ""
    let successMessage = ""
    async function tryConnect() {
        successMessage = ""
        errorMessage = ""
        progressMessage = "Attempting to connect with given details."
        switch (selected) {
            case 0: {
                /* Should not be possible, but just in case it somehow happens */
                errorMessage = "Please select the connection option and try again!"
                break
            }
            case 1: {
                /* Validate inputs */
                if(!connectionDetails.address) {
                    progressMessage = ""
                    errorMessage = "Please provide a valid address!"
                    return
                }
                if(!connectionDetails.timeout) {
                    progressMessage = ""
                    errorMessage = "Please provide a valid timeout!"
                    return
                }
                /* Try to connect via HTTP */
                await invoke("create_docker_http_connection", {addr: connectionDetails.address, timeout: connectionDetails.timeout})
                    .then((reply) => {
                        progressMessage = "" //remove progress div
                        successMessage = `Success! Received reply: ${reply}` //make success div visible
                    })
                    .catch((error) => {
                        progressMessage = "" //remove progress div
                        errorMessage = `${error.error} ${error.error_msg ? error.error_msg : ""}` //make error div visible
                    })
                break
            }
            case 2: {
                /* Validate inputs */
                if(!connectionDetails.address) {
                    progressMessage = ""
                    errorMessage = "Please provide a valid address!"
                    return
                }
                if(!connectionDetails.timeout) {
                    progressMessage = ""
                    errorMessage = "Please provide a valid timeout!"
                    return
                }
                /* Try to connect via socket */
                await invoke("create_docker_socket_connection", {socketPath: connectionDetails.socketPath, timeout: connectionDetails.timeout})
                    .then((reply) => {
                        progressMessage = "" //remove progress div
                        successMessage = `Success! Received reply: ${reply}` //make success div visible
                    })
                    .catch((error) => {
                        progressMessage = "" //remove progress div
                        errorMessage = `${error.error} ${error.error_msg ? error.error_msg : ""}` //make error div visible
                    })
                break
            }
            case 3: {
                /* Validate inputs */
                if(!connectionDetails.address) {
                    progressMessage = ""
                    errorMessage = "Please provide a valid address!"
                    return
                }
                if(!connectionDetails.sslKey) {
                    progressMessage = ""
                    errorMessage = "Please provide a valid SSL key!"
                    return
                }
                if(!connectionDetails.sslCert) {
                    progressMessage = ""
                    errorMessage = "Please provide a valid SSL certificate!"
                    return
                }
                if(!connectionDetails.sslCa) {
                    progressMessage = ""
                    errorMessage = "Please provide a valid SSL CA!"
                    return
                }
                if(!connectionDetails.timeout) {
                    progressMessage = ""
                    errorMessage = "Please provide a valid timeout!"
                    return
                }
                console.log(connectionDetails.sslCa)
                /* Try to connect via SSL */
                await invoke("create_docker_ssl_connection", {
                    addr: connectionDetails.address,
                    sslKey: connectionDetails.sslKey,
                    sslCert: connectionDetails.sslCert,
                    sslCa: connectionDetails.sslCa,
                    timeout: connectionDetails.timeout
                })
                    .then((reply) => {
                        progressMessage = "" //remove progress div
                        successMessage = `Success! Received reply: ${reply}` //make success div visible
                    })
                    .catch((error) => {
                        console.log(error)
                        progressMessage = "" //remove progress div
                        errorMessage = `${error.error} ${error.error_msg ? error.error_msg : ""}` //make error div visible
                    })
                break
            }
            default: {
                /* Should not be possible, but just in case it somehow happens */
                errorMessage = "Unknown connection option selected!"
                selected = 0
                break
            }
        }
    }
    /**
     * Handle a file selection (for SSL)
     * @param filterName - Name of the filter
     * @param extensions - Extensions allowed
     * @param storeTo - Which variable name to store to
     * @returns {Promise<void>}
     */
    async function handleFileSelection(filterName, extensions, storeTo) {
        await invoke("open_file_selection_and_get_file_path", { filterName: filterName, extensions: extensions })
            .then((reply) => {
                console.log(reply)
                connectionDetails[storeTo] = reply
            })
            .catch(() => {
                errorMessage = "File selection dialogue closed!"
            })
    }

    /**
     * Get file name from path
     * @param path
     * @returns {string|string}
     */
    function getFileFromPath(path) {
        let nodes = path.replaceAll("\\", "/").split("/")
        return nodes[nodes.length-1] || "No file selected."
    }
</script>

<div class="modal">
    <h1 style="text-align: center">New Connection</h1>
    {#if errorMessage !== ""}
        <div class="highlight error">
            <p>{errorMessage}</p>
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
    <h3>Connection type</h3>
    <select bind:value={selected} style="width: 50%">
        {#each options as option}
            <option value={option.id}>
                {option.text}
            </option>
        {/each}
    </select>
    {#if selected !== 0}
        <hr>
    {/if}
    {#if selected === 1}
        <div class="field">
            <label>
                Daemon Address
                <input placeholder="http://docker.daemon" bind:value={connectionDetails.address}>
            </label>
        </div>
        <div class="field">
            <label>
                Timeout (in seconds)
                <input placeholder="5" type="number" bind:value={connectionDetails.timeout}>
            </label>
        </div>
    {:else if selected === 2}
        <div class="field">
            <label>
                Daemon Socket Path
                <input placeholder="/path/to/docker/socket" bind:value={connectionDetails.socketPath}>
            </label>
        </div>
        <div class="field">
            <label>
                Timeout (in seconds)
                <input placeholder="5" type="number" bind:value={connectionDetails.timeout}>
            </label>

        </div>
    {:else if selected === 3}
        <div class="field">
            <label>
                Daemon Address
                <input placeholder="tcp://daemon.address:2375/" bind:value={connectionDetails.address}>
            </label>
        </div>
        <div class="field">
            SSL Key ({connectionDetails.sslKey ? getFileFromPath(connectionDetails.sslKey) : "No file selected."})
            <div class="btn" style="float: right" on:click={ () => handleFileSelection(
                "Key Files",
                [
                    "key",
                    "keystore",
                    "jks",
                    "p12",
                    "pfx",
                    "pem"
                ],
                "sslKey"
            )}>Select File</div>
        </div>
        <div class="field">
            SSL certificate ({connectionDetails.sslCert ? getFileFromPath(connectionDetails.sslCert) : "No file selected."})
            <div class="btn" style="float: right" on:click={ () => handleFileSelection(
                "Certificate",
                [
                    "crt",
                    "cer",
                    "ca-bundle",
                    "p7b",
                    "p7c",
                    "p7s",
                    "pem",
                    "p12",
                    "pfx"
                ],
                "sslCert"
            )}>Select File</div>
        </div>
        <div class="field">
            SSL CA ({connectionDetails.sslCa ? getFileFromPath(connectionDetails.sslCa) : "No file selected."})
            <div class="btn" style="float: right" on:click={ () => handleFileSelection(
                "CA files",
                [
                    "crt",
                    "cer",
                    "ca-bundle",
                    "p7b",
                    "p7c",
                    "p7s",
                    "pem",
                    "p12",
                    "pfx"
                ],
                "sslCa"
            )}>Select File</div>
        </div>
        <div class="field">
            <label>
                Timeout (in seconds)
                <input placeholder="5" type="number" bind:value={connectionDetails.timeout}>
            </label>
        </div>
    {/if}
    {#if selected !== 0}
        <div class="btn connect-button" on:click={tryConnect}>
            Connect
        </div>
    {/if}

</div>