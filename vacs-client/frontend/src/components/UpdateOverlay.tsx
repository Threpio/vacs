import {clsx} from "clsx";
import {useUpdateStore} from "../stores/update-store.ts";
import Button from "./ui/Button.tsx";
import {useCallback, useEffect} from "preact/hooks";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {invoke} from "@tauri-apps/api/core";
import {useAsyncDebounceState} from "../hooks/debounce-hook.ts";

function UpdateOverlay() {
    const overlayVisible = useUpdateStore(state => state.overlayVisible);
    const dialogVisible = useUpdateStore(state => state.dialogVisible);
    const newVersion = useUpdateStore(state => state.newVersion);
    const {
        setVersions: setUpdateVersions,
        openDialog: openUpdateDialog,
        closeOverlay: closeUpdateOverlay
    } = useUpdateStore(state => state.actions);

    const [handleUpdateClick, updating] = useAsyncDebounceState(async () => {
        try {
            await invoke("app_update");
        } catch (e) {
            // TODO: Open fatal error overlay
        }
    });

    useEffect(() => {
        const checkForUpdate = async () => {
            try {
                const checkUpdateResult = await invoke<{
                    currentVersion: string,
                    newVersion?: string,
                    required: boolean
                }>("app_check_for_update");
                setUpdateVersions(checkUpdateResult.currentVersion, checkUpdateResult.newVersion);

                if (checkUpdateResult.required) {
                    openUpdateDialog();
                } else {
                    closeUpdateOverlay();
                }
            } catch (e) {
                console.error(e);
                // TODO: Open fatal error overlay
            }
        };
        void checkForUpdate();
    });

    const handleKeyDown = useCallback((event: KeyboardEvent) => {
        event.preventDefault();
    }, []);

    useEffect(() => {
        if (overlayVisible) {
            document.addEventListener("keydown", handleKeyDown);
        } else {
            document.removeEventListener("keydown", handleKeyDown);
        }
    }, [overlayVisible]);

    return overlayVisible ? (
        <div
            className={clsx("z-40 absolute top-0 left-0 w-full h-full flex justify-center items-center", dialogVisible && "bg-[rgba(0,0,0,0.5)]")}
        >
            {dialogVisible &&
                <div
                    className="bg-gray-300 border-4 border-t-blue-500 border-l-blue-500 border-b-blue-700 border-r-blue-700 rounded w-100 py-2">
                    <p className="w-full text-center text-lg font-semibold wrap-break-word">Mandatory update</p>
                    <p className="w-full text-center wrap-break-word mb-2">
                        In order to continue using VACS, you will need to update to version v{newVersion}.<br/>
                        Do you want to download and install the update?<br/>This will restart the application.
                    </p>
                    <div
                        className={clsx("w-full flex flex-row gap-2 justify-center items-center mb-2", updating && "brightness-90 [&>button]:cursor-not-allowed")}>
                        <Button color="red" className="px-3 py-1" muted={true} disabled={updating}
                                onClick={() => getCurrentWindow().close()}>Quit</Button>
                        <Button color="green" className="px-3 py-1" onClick={handleUpdateClick}
                                disabled={updating}>Update</Button>
                    </div>
                    {/*TODO: Add progress bar*/}
                    {updating && <p className="w-full text-center font-semibold">Updating...</p>}
                </div>
            }
        </div>
    ) : <></>;
}

export default UpdateOverlay;