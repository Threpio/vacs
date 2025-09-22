import {useLayoutEffect, useMemo, useRef, useState} from "preact/hooks";
import {useEventCallback} from "./event-callback-hook.ts";

export const HEADER_HEIGHT_REM = 1.75;
const CALL_ROW_HEIGHT_REM = 2.7;

type UseCallListOptions = {
    callsCount: number;
}

export function useCallList({callsCount}: UseCallListOptions) {
    const listContainer = useRef<HTMLDivElement>(null);
    const [listContainerHeight, setListContainerHeight] = useState<number>(0);
    const [scrollOffset, setScrollOffset] = useState<number>(0);
    const [selectedCall, setSelectedCall] = useState<number>(0);

    const {visibleCallIndices, maxScrollOffset} = useMemo((): {
        visibleCallIndices: number[],
        maxScrollOffset: number
    } => {
        let itemCount: number;

        if (listContainer.current) {
            const fontSize = parseFloat(getComputedStyle(listContainer.current).fontSize);
            const callListHeaderHeight = HEADER_HEIGHT_REM * fontSize;
            const callListItemHeight = CALL_ROW_HEIGHT_REM * fontSize;

            itemCount = Math.floor((listContainerHeight - callListHeaderHeight) / callListItemHeight);
        } else {
            itemCount = 11;
        }

        return {
            visibleCallIndices: Array.from({length: itemCount}, (_, i) =>
                scrollOffset + i
            ),
            maxScrollOffset: callsCount - itemCount
        };
    }, [listContainerHeight, callsCount, scrollOffset]);

    const onKeyDown = useEventCallback((event: KeyboardEvent) => {
        if (callsCount === 0) return;

        if (event.key === "ArrowUp") {
            const firstVisibleCallIndex = visibleCallIndices[0];
            const newSelectedCall = Math.max(selectedCall - 1, 0);

            if (newSelectedCall < firstVisibleCallIndex) {
                setScrollOffset(scrollOffset => Math.max(scrollOffset - 1, 0));
            }

            setSelectedCall(newSelectedCall);
        } else if (event.key === "ArrowDown") {
            const lastVisibleCallIndex = visibleCallIndices[visibleCallIndices.length - 1];
            const newSelectedCall = Math.min(selectedCall + 1, callsCount - 1);

            if (newSelectedCall > lastVisibleCallIndex) {
                setScrollOffset(scrollOffset => Math.min(scrollOffset + 1, maxScrollOffset));
            }

            setSelectedCall(newSelectedCall);
        }
    });

    useLayoutEffect(() => {
        if (!listContainer.current) return;
        const observer = new ResizeObserver(entries => {
            for (const entry of entries) {
                setListContainerHeight(entry.contentRect.height);
            }
        });
        observer.observe(listContainer.current);

        window.addEventListener("keydown", onKeyDown);

        return () => {
            observer.disconnect();
            window.removeEventListener("keydown", onKeyDown);
        };
    }, [onKeyDown]);

    return {listContainer, scrollOffset, setScrollOffset, selectedCall, setSelectedCall, visibleCallIndices, maxScrollOffset};
}