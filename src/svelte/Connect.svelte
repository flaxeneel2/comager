<script>
    import { getContext } from 'svelte';
    import Select from 'svelte-select';
    export let onOkay = () => {};
    export let onClose = () => {};
    let onChange = () => {}
    let { close } = getContext("simple-modal")
    let data = {}
    function _onCancel() {
        onClose();
        close();
    }

    let connTypes = [
        {
            value: 1,
            label: "HTTP"
        },
        {
            value: 2,
            label: "SSL (HTTPS)"
        },
        {
            value: 3,
            label: "Socket"
        },
        {
            value: 4,
            label: "Local"
        }
    ]

    function _onOkay() {
        onOkay(data);
        close();
    }
</script>

<div>
    <h3 class="import-image-header">New connection details</h3>
    <div class="connect-dropdown-selection">
        <Select items={connTypes}/>
    </div>
</div>