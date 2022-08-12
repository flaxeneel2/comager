<script>
    export let cancelNewConnection = () => {}
    export let reconnectWithNewConnection = () => {}
    let options = [
        {
            id: 1,
            text: "Socket"
        },
        {
            id: 2,
            text: "HTTP"
        },
        {
            id: 3,
            text: "SSL"
        }
    ]
    let selection = "";
</script>

<div class="modal">
    <h1 style="text-align: center">New Connection</h1>
    <h3>Connection type</h3>
    <select value={'selection'} style="width: 50%">
        {#each options as option}
            <option value={option.id}>
                {option.text}
            </option>
        {/each}
    </select>
</div>