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
     * Delete an image. Returns a promise that must be awaited.
     * @param imageName
     * @param force
     * @returns Promise
     */
    deleteImage(imageName, force) {
        return invoke("delete_docker_image", { imageName: imageName,  force: force})
    }

    /**
     * Update images every second
     */
    async updateImagesPeriodically() {
        await this.updateImages();
        this.loop = setInterval(async () => await this.updateImages(), 1000)
    }
}