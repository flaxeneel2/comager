import {invoke} from "@tauri-apps/api/tauri";
import type {DockerData} from "../../types";

export default class DockerInfoManager extends EventTarget {

    constructor() {
        super();
    }
    async updateFields() {
        console.log("hi")
        let dockerData: DockerData;
        await invoke("get_docker_daemon_info")
            .then((d: DockerData) => dockerData = d)
            .catch(e => {
                this.dispatchEvent(new CustomEvent("infoFetchError", {
                    detail: e
                }))
            });
        if(!dockerData) return;
        document.getElementById("host").innerText = dockerData.Name
        document.getElementById("kernel").innerText = dockerData.OSType
        document.getElementById("kernelVersion").innerText = dockerData.KernelVersion
        document.getElementById("dockerVersion").innerText = dockerData.ServerVersion
        document.getElementById("operatingSystem").innerText = dockerData.OperatingSystem
        document.getElementById("architecture").innerText = dockerData.Architecture
        document.getElementById("memory").innerText = `${((dockerData.MemTotal)/1024)/1024}MB`
        document.getElementById("threadCount").innerText = dockerData.NCPU
        document.getElementById("imageCount").innerText = dockerData.Images
        document.getElementById("containerCount").innerText = dockerData.Containers
        document.getElementById("swapStatus").innerText = dockerData.SwapLimit
        document.getElementById("ipv4ForwardingStatus").innerText = dockerData.IPv4Forwarding
    }

    async updateFieldsPeriodically() {
        await this.updateFields();
        setInterval(async () => await this.updateFields(), 1000)
    }
}