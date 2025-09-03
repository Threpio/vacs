import {useCallback, useRef} from "preact/hooks";

export function useEventCallback<T extends (...args: any[]) => any>(fn: T): T {
    const fnRef = useRef(fn);

    fnRef.current = fn;

    return useCallback(((...args: Parameters<T>): ReturnType<T> => {
        return fnRef.current(...args);
    }) as T, []);
}