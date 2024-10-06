interface DockerImagePull {
    image: string;
    tag: string;
    nodename: string;
}

interface DockerImageList {
    nodename: string[];
}

interface DockerImageRemove {
    nodename: string;
    name: string;
    tag: string;
}

interface DockerVolumesCreate {
    nodename: string;
    name: string;
    label: { [key: string]: string };
}

interface DockerVolumesDelete {
    nodename: string;
    name: string;
    force: boolean;
}

interface DockerVolumesList {
    nodename: string[];
}