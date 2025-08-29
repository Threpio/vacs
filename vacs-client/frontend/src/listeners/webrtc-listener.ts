import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {useCallStore} from "../stores/call-store.ts";

export function setupWebrtcListeners()  {
    const { errorPeer } = useCallStore.getState().actions;

    const unlistenFns: (Promise<UnlistenFn>)[] = [];

    const init = () => {
        unlistenFns.push(
            listen<string>("webrtc:call-error", (event) => {
                console.log("webrtc:call-error", event.payload);
                errorPeer(event.payload);
            }),
        );
    };

    init();

    return () => {
        unlistenFns.forEach(fn => fn.then(f => f()));
    }
}