<script>
    import { getContext } from 'svelte';
    export let onOkay = () => {};
    export let onClose = () => {};
    let onChange = () => {}
    let { close } = getContext("simple-modal")
    let value;
    function _onCancel() {
        onClose();
        close();
    }

    function _onOkay() {
        onOkay(value);
        close();
    }


    $: onChange(value)
</script>

<div>
    <h3 class="import-image-header">Import image options</h3>
    <input class="import-image-textbox" placeholder="debian:latest" type="text" bind:value on:keydown={e => e.which === 13 && _onOkay()} />
</div>