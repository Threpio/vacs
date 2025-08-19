export type AudioDevices = {
    selected: string;
    default: string;
    all: string[];
};

export type AudioVolumes = {
    input: number;
    output: number;
    click: number;
    chime: number;
}

export type AudioHosts = {
    selected: string;
    all: string[];
}