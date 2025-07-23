import {useAuthStore} from "../stores/auth-store.ts";
import {listen, UnlistenFn} from "@tauri-apps/api/event";

export function setupAuthListeners() {
    const { setAuthenticated, setUnauthenticated } = useAuthStore.getState();

    const unlistenFns: (Promise<UnlistenFn>)[] = [];

    const init = () => {
        const unlisten1 = listen<string>("auth:authenticated", (event) => {
            setAuthenticated(event.payload);
        });

        const unlisten2 = listen("auth:unauthenticated", () => {
            setUnauthenticated();
        });

        unlistenFns.push(unlisten1, unlisten2);
    };

    init();

    return () => {
        unlistenFns.forEach(fn => fn.then(f => f()));
    }
}