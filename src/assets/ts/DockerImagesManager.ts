import {invoke} from "@tauri-apps/api/tauri";

export default class DockerImagesManager extends EventTarget {
    private loop: NodeJS.Timer
    constructor() {
        super();
    }

    /**
     * Update the images once by sending the data to the listeners attached
     */
    async updateImages() {
        await invoke("get_docker_images")
            .then((d) => {
                this.dispatchEvent(new CustomEvent("imagesFetchSuccess", {
                    detail: d
                }))
            })
            .catch(e => {
                this.dispatchEvent(new CustomEvent("imagesFetchError", {
                    detail: e
                }))
            });
    }

    unload() {
        clearInterval(this.loop)
    }

    /**
     * Convert bytes to highest possible unit that is still above 1 of the unit
     * @param size - Size of image in bytes.
     */
    resolveSizeToHighestUnit(size: number) {
        let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"]
        let returner = `${size}B`
        let iteration = 0
        do {
            iteration++
            size = size/1024
            returner = `${size.toFixed(2)}${units[iteration]}`
        } while(Math.floor(size/1024)!==0)
        return returner
    }

    /**
     * Update images every second
     */
    async updateImagesPeriodically() {
        await this.updateImages();
        this.loop = setInterval(async () => await this.updateImages(), 1000)
    }
}