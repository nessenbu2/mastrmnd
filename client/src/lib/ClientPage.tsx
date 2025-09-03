import {useParams} from "react-router-dom";
import {getClientState} from "../api/clients";
import {useEffect, useState} from "react";
import ClientState from "./types";


export default function ClientPage() {
    const { clientName } = useParams()
    const [client, setClient] = useState<ClientState | null>(null)
    const [error, setError] = useState<string | null>(null)

    useEffect(() => {
        getClientState(clientName!, setError, setClient)
        const id = setInterval(() => { getClientState(clientName!, setError, setClient) }, 2000)
        return () => clearInterval(id)
    }, [])

    return (
        <div>
            {error && (
                <h2>Error: failed to load {clientName}</h2>
            )}
            {client !== null && !error && (
                <div>
                    <h2>Client: {client.name}</h2>
                    <p>State: {client.state}</p>
                </div>
            )}
        </div>
    )
}