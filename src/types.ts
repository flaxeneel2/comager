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

export interface DockerError {
    error: string,
    error_msg?: string
}