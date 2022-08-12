<script>
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
    let connectionDetails = {}
</script>

<div class="modal">
    <h1 style="text-align: center">New Connection</h1>
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
            <label>
                SSL Key
                <input accept=".key,.keystore,.jks,.p12,.pfx,.pem" placeholder="/path/to/key.pem" type="file" bind:value={connectionDetails.sslKey}>
            </label>
        </div>
        <div class="field">
            <label>
                SSL certificate
                <input accept=".crt,.cer,.ca-bundle,.p7b,.p7c,.p7s,.pem,.p12,.pfx" placeholder="/path/to/cert.pem" type="file" bind:value={connectionDetails.sslCert}>
            </label>
        </div>
        <div class="field">
            <label>
                SSL CA
                <input accept=".crt,.cer,.ca-bundle,.p7b,.p7c,.p7s,.pem,.p12,.pfx" placeholder="/path/to/ca" type="file" bind:value={connectionDetails.sslCa}>
            </label>
        </div>
        <div class="field">
            <label>
                Timeout (in seconds)
                <input placeholder="5" type="number" bind:value={connectionDetails.timeout}>
            </label>
        </div>
    {/if}

</div>