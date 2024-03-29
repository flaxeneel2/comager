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

    /**
     * Attach to a container
     * @param containerId
     * @param uniqueId
     */
    attachToContainers(containerId, uniqueId) {
        return invoke("set_container_up_for_live_stdio", {
            containerId: containerId,
            uniqueId: uniqueId
        })
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
     * @param creationDetails
     */
    createContainer(creationDetails) {
        return invoke("create_docker_container", {
            name: creationDetails.name,
            cpuPercentageLimit: (creationDetails.cpuPercentageLimit || 0),
            image: creationDetails.image,
            memoryLimit: ((creationDetails.memoryLimit*1024)*1024 || 0),
            command: creationDetails.command
        });
    }

    /**
     * Get the details of a container
     * @param containerId
     */
    getContainerDetails(containerId) {
        return invoke("get_docker_container_details", {
            containerId: containerId
        })
    }


    startContainer(containerId) {
        return invoke("start_docker_container", {
            containerId: containerId
        })
    }

    stopContainer(containerId) {
        return invoke("stop_docker_container", {
            containerId: containerId
        })
    }

    restartContainer(containerId) {
        return invoke("restart_docker_container", {
            containerId: containerId
        })
    }

    /**
     * Update containers every second
     */
    async updateContainersPeriodically() {
        await this.updateContainers();
        this.loop = setInterval(async () => await this.updateContainers(), 1000)
    }
}