import Button from "./Button.tsx";
import {useCallStore} from "../../stores/call-store.ts";

function RadioPrioButton() {
    const callDisplayType = useCallStore(state => state.callDisplay?.type);

    return (
        <Button color="cyan" className="text-lg w-46" disabled={!(callDisplayType === "accepted")}>
            <p>RADIO<br/>PRIO</p>
        </Button>
    );
}

export default RadioPrioButton;