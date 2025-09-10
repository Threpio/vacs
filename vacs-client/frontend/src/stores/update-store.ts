import {create} from "zustand/react";

type UpdateState = {
    overlayVisible: boolean,
    dialogVisible: boolean,
    currentVersion: string,
    newVersion?: string,
    actions: {
        setVersions: (currentVersion: string, newVersion?: string) => void,
        openDialog: () => void,
        closeOverlay: () => void,
    }
}

export const useUpdateStore = create<UpdateState>()((set) => ({
    overlayVisible: true,
    dialogVisible: false,
    currentVersion: "",
    newVersion: undefined,
    actions: {
        setVersions: (currentVersion: string, newVersion?: string) => {
            set({currentVersion, newVersion});
        },
        openDialog: () => {
            set({overlayVisible: true, dialogVisible: true});
        },
        closeOverlay: () => {
            set({overlayVisible: false, dialogVisible: false});
        }
    },
}));