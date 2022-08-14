import {invoke} from "@tauri-apps/api/tauri";

export default class DockerContainersManager extends EventTarget {
    private loop: NodeJS.Timer
    constructor() {
        super();
    }

    /**
     * Update the containers once by sending the data to the listeners attached
     */
    async updateContainers() {
        await invoke("get_docker_containers")
            .then((d) => {
                this.dispatchEvent(new CustomEvent("containersFetchSuccess", {
                    detail: d
                }))
            })
            .catch(e => {
                this.dispatchEvent(new CustomEvent("containersFetchError", {
                    detail: e
                }))
            });
    }

    unload() {
        clearInterval(this.loop)
    }


    /**
     * Delete a container. Returns a promise that must be awaited.
     * @param containerId ID of container
     * @param force Whether to force delete
     * @param volumes Whether to delete connected volumes
     * @param links Whether to delete links
     * @returns Promise
     */
    deleteContainer(containerId, force, volumes, links) {
        return invoke("delete_docker_container", {
            containerId: containerId,
            force: force,
            volumes: volumes,
            links: links
        })
    }

    /**
     * Create a new container
     * @param containerName
     * @param uniqueId
     */
    createContainer(containerName, uniqueId) {

    }

    /**
     * Update containers every second
     */
    async updateContainersPeriodically() {
        await this.updateContainers();
        this.loop = setInterval(async () => await this.updateContainers(), 1000)
    }
}