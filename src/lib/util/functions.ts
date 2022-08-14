export default {
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
    },

    /**
     * Get file name from path
     * @param path
     * @returns {string|string}
     */
    getFileFromPath(path) {
        let nodes = path.replaceAll("\\", "/").split("/")
        return nodes[nodes.length-1] || "No file selected."
    },

    /**
     * Generate a random string
     * @param stringLength The length of the string to generate
     */
    generateRandomString(stringLength) {
        let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        let returner = ""
        for(let index = 0; index<stringLength; index++) {
            returner += characters[Math.floor(Math.random() * characters.length)]
        }
        return returner
    }
}