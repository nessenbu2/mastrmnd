import {useEffect, useState} from "react";
import {ClientsResponse} from "./types";
import {fetchClients} from "../api/clients";
import ClientTable from "./ClientTable";

export default function AllClients() {
    const [result, setResult] = useState<ClientsResponse | null>(null)
    const [error, setError] = useState<string | null>(null)

    useEffect(() => {
        fetchClients(setError, setResult)
        const id = setInterval(() => { fetchClients(setError, setResult) }, 2000)
        return () => clearInterval(id)
    }, [])

    return (
        <div style={{ fontFamily: 'system-ui, sans-serif', padding: 24 }}>
            <h1>mastrmnd client</h1>
            <div style={{ marginTop: 16 }}>
                {error && (
                    <div style={{ color: 'crimson' }}>
                        <strong>Error:</strong> {error}
                    </div>
                )}
                {result !== null && !error && (
                    <ClientTable clients={result}/>
                )}
            </div>
        </div>
)
}
