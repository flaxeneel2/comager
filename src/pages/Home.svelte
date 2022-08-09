<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    (async () => {
        let info = await invoke("get_docker_daemon_info")
        console.log(info)
    })();
</script>

<div>
    <div>

    </div>
</div>