import Button from "./ui/Button.tsx";
import "../styles/call-list.css";
import {useLayoutEffect, useMemo, useRef, useState} from "preact/hooks";

type CallListEntry = { in: boolean; time: string; name: string; number: string };
type CallListItem = { call?: CallListEntry };

function CallList() {
    const calls = useRef<CallListEntry[]>([
        {in: true, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: true, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: false, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: true, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: true, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: false, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: false, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: true, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: false, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: true, time: "14:49", name: "LOVV_CTR", number: "1234567"},
        {in: true, time: "14:49", name: "LOVV_CTR", number: "1234567"},
    ]);
    const [listContainerHeight, setListContainerHeight] = useState<number>(0);
    const listContainer = useRef<HTMLDivElement>(null);

    useLayoutEffect(() => {
        if (!listContainer.current) return;
        const observer = new ResizeObserver(entries => {
            for (const entry of entries) {
                setListContainerHeight(entry.contentRect.height);
            }
        });
        observer.observe(listContainer.current);
        return () => observer.disconnect();
    }, []);

    const callListItems = useMemo((): CallListItem[] => {
        const callListItems: CallListItem[] = [];

        if (listContainer.current) {
            const callListHeaderHeight = 1.75 * parseFloat(getComputedStyle(listContainer.current).fontSize);
            const callListItemHeight = 2.7 * parseFloat(getComputedStyle(listContainer.current).fontSize);

            const callListItemCount = (listContainerHeight - callListHeaderHeight) / callListItemHeight;

            for (let i = 0; i < callListItemCount; i++) {
                callListItems.push({call: calls.current[i]});
            }
        }

        return callListItems;
    }, [listContainerHeight, calls]);

    return (
        <div className="w-[37.5rem] h-full flex flex-col gap-3 p-3">
            <div
                ref={listContainer}
                className="h-full w-full grid grid-cols-[minmax(3rem,auto)_1fr_1fr_4rem] box-border gap-[1px] [&_div]:outline-1 [&_div]:outline-gray-500"
                style={{gridTemplateRows: `1.75rem repeat(${callListItems.length},1fr)`}}
            >
                <div className="col-span-2 bg-gray-300 flex justify-center items-center font-bold">Name</div>
                <div className="bg-gray-300 flex justify-center items-center font-bold">Number</div>
                <div className="!outline-0"></div>

                {callListItems.map((item, idx) => {
                    const rowSpan = `span ${callListItems.length - 2} / span ${callListItems.length - 2}`;
                    const lastElement =
                        idx === 0 ? (
                            <div className="relative bg-gray-300">
                                <svg
                                    className="absolute h-[85%] max-w-[85%] top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2"
                                    viewBox="0 0 125 89" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M62.5 0L120 60H5L62.5 0Z" fill="#6A7282"/>
                                    <path
                                        d="M63.2217 26.3076L120.722 86.3076L122.344 88H2.65625L4.27832 86.3076L61.7783 26.3076L62.5 25.5547L63.2217 26.3076Z"
                                        fill="#6A7282" stroke="#D1D5DC" stroke-width="2"/>
                                </svg>
                            </div>
                        ) : idx === 1 ? (
                            <div className="bg-gray-300" style={{gridRow: rowSpan}}>
                                <div className="relative h-full w-full px-4 py-13">
                                    <div
                                        className="h-full w-full border border-b-gray-100 border-r-gray-100 border-l-gray-700 border-t-gray-700 !outline-none flex flex-col-reverse">
                                        {/*<div className="w-full bg-blue-600" style={{height: `calc(100% * ${position})`}}></div>*/}
                                    </div>
                                    {/*<div*/}
                                    {/*    className={clsx(*/}
                                    {/*        "dotted-background absolute translate-y-[-50%] left-0 w-full aspect-square shadow-[0_0_0_1px_#364153] !outline-none rounded-md cursor-pointer bg-blue-600 border",*/}
                                    {/*        true && "border-t-blue-200 border-l-blue-200 border-r-blue-900 border-b-blue-900",*/}
                                    {/*        false && "border-b-blue-200 border-r-blue-200 border-l-blue-900 border-t-blue-900 shadow-none",*/}
                                    {/*    )}*/}
                                    {/*    style={{top: `calc(2.25rem + (1 - ${1}) * (100% - 4.5rem))`}}>*/}
                                    {/*</div>*/}
                                </div>
                            </div>
                        ) : idx === callListItems.length - 1 ? (
                            <div className="relative bg-gray-300">
                                <svg
                                    className="absolute h-[85%] max-w-[85%] top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2 rotate-180"
                                    viewBox="0 0 125 89" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M62.5 0L120 60H5L62.5 0Z" fill="#6A7282"/>
                                    <path
                                        d="M63.2217 26.3076L120.722 86.3076L122.344 88H2.65625L4.27832 86.3076L61.7783 26.3076L62.5 25.5547L63.2217 26.3076Z"
                                        fill="#6A7282" stroke="#D1D5DC" stroke-width="2"/>
                                </svg>
                            </div>
                        ) : <></>;

                    return (
                        <>
                            <div className="bg-yellow-50 p-0.5 text-center flex flex-col justify-between leading-4">
                                <p>{item.call?.in !== undefined ? (item.call.in ? "IN" : "OUT") : ""}</p>
                                <p className="tracking-wider font-semibold">{item.call?.time ?? ""}</p>
                            </div>
                            <div className="bg-yellow-50 px-0.5 flex items-center font-semibold">
                                {item.call?.name ?? ""}
                            </div>
                            <div className="bg-yellow-50 px-0.5 flex items-center font-semibold">
                                {item.call?.number ?? ""}
                            </div>
                            {lastElement}
                        </>
                    );
                })}
            </div>
            <div className="w-full shrink-0 flex flex-row justify-between pr-16 [&_button]:h-15 [&_button]:rounded">
                <Button color="gray">
                    <p>Delete<br/>List</p>
                </Button>
                <Button color="gray" className="w-56 text-xl">Call</Button>
            </div>
        </div>
    );
}

export default CallList;