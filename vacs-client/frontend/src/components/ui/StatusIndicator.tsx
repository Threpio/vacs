import {useState} from "preact/hooks";
import {clsx} from "clsx";

type Status = "green" | "yellow" | "red" | "gray";

const StatusColors: Record<Status, string> = {
    green: "bg-green-600 border-green-700",
    yellow: "bg-yellow-500 border-yellow-600",
    red: "bg-red-400 border-red-700",
    gray: "bg-gray-400 border-gray-600"
};

function StatusIndicator() {
    const [status, _setStatus] = useState<Status>("yellow");

    return (
        <div className={clsx("h-full aspect-square rounded-full border", StatusColors[status])}></div>
    );
}

export default StatusIndicator;