import ClientState from "./types";
import { incClient, toggleState } from "../api/clients";
import {useNavigate} from "react-router-dom";

type Props = { state: ClientState };

export default function ClientRow({ state } : Props) {

    const navigate = useNavigate();
    const routeToClientPage = () => {
        navigate(`/client/${state.name}`)
    }

    const handleToggle = () => {
        toggleState(state.name).then()
    }

    const handleInc= () => {
        incClient(state.name).then()
    }

    return (
        <tr key={state.name}>
            <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px' }}>{state.name}</td>
            <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px' }}>
                <button onClick={() => routeToClientPage()} type={"submit"}>Edit</button>
            </td>
            <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px', textAlign: 'left' }}>
                <button onClick={() => handleToggle()} type={"submit"}>Toggle State</button>
            </td>
            <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px', textAlign: 'left' }}>
                <button onClick={() => handleInc()} type={"reset"}>Inc Count</button>
            </td>
            <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px', textAlign: 'right' }}>{state.call_count}</td>
            <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px' }}>{state.port}</td>
            <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px', textAlign: 'right' }}>{state.last_seen_secs}</td>
            <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px' }}>{state.state}</td>
        </tr>
    )
}