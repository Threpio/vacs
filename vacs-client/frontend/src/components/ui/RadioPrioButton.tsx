import Button from "./Button.tsx";
import {useCallStore} from "../../stores/call-store.ts";
import {useAsyncDebounce} from "../../hooks/debounce-hook.ts";
import {invokeSafe} from "../../error.ts";
import {useState} from "preact/hooks";

function RadioPrioButton() {
    const [muted, setMuted] = useState<boolean>(false);
    const callDisplayType = useCallStore(state => state.callDisplay?.type);

    const handleOnClick = useAsyncDebounce(async () => {
        void invokeSafe("audio_set_input_muted", {muted: !muted});
        setMuted(muted => !muted);
    });

    return (
        <Button
            color={muted ? "blue" : "cyan"} className="text-lg w-46" disabled={!(callDisplayType === "accepted")}
            onClick={handleOnClick}
        >
            <p>RADIO<br/>PRIO</p>
        </Button>
    );
}

export default RadioPrioButton;