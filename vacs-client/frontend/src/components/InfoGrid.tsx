import {useAuthStore} from "../stores/auth-store.ts";
import "../styles/info-grid.css";

type InfoGridProps = {
    displayName: string
};

function InfoGrid(props: InfoGridProps) {
    const cid = useAuthStore(state => state.cid);
    const setAuthenticated = useAuthStore(state => state.setAuthenticated);
    const setUnauthenticated = useAuthStore(state => state.setUnauthenticated);

    return (
        <div className="grid grid-rows-2 w-full h-full" style={{ gridTemplateColumns: "25% 32.5% 42.5%" }}>
            <div className="info-grid-cell" onClick={() => setAuthenticated("test-cid")}>{cid}</div>
            <div className="info-grid-cell"></div>
            <div className="info-grid-cell"></div>
            <div className="info-grid-cell" onClick={() => setUnauthenticated()}>{props.displayName}</div>
            <div className="info-grid-cell"></div>
            <div className="info-grid-cell"></div>
        </div>
    );
}

export default InfoGrid;