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
        console.log(value)
        console.log(onOkay)
        onOkay(value);
        close();
    }


    $: onChange(value)
</script>

<input type="text" bind:value on:keydown={e => e.which === 13 && _onOkay()} />