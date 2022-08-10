export interface DockerData {
    Name: string,
    OSType: string,
    KernelVersion: string,
    ServerVersion: string,
    OperatingSystem: string,
    Architecture: string,
    MemTotal: number,
    NCPU: string,
    Images: string,
    Containers: string,
    SwapLimit: string,
    IPv4Forwarding: string
}

/*
this.hostName.innerText = dockerData.Name
        this.kernelName.innerText = dockerData.OSType
        this.kernelVersion.innerText = dockerData.KernelVersion
        this.dockerVersion.innerText = dockerData.ServerVersion
        this.operatingSystem.innerText = dockerData.OperatingSystem
        this.arch.innerText = dockerData.Architecture
        this.memory.innerText = `${((dockerData.MemTotal)/1024)/1024}MB`
        this.threadCount.innerText = dockerData.NCPU
        this.imageCount.innerText = dockerData.Images
        this.containerCount.innerText = dockerData.Containers
        this.swapStatus.innerText = dockerData.SwapLimit
        this.ipv4FwdStatus.innerText = dockerData.IPv4Forwarding
 */