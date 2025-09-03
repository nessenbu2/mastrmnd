import {useParams} from "react-router-dom";
import {getClientState, getLibrary} from "../api/clients";
import {useEffect, useState} from "react";
import ClientState, {Artists} from "./types";


export default function ClientPage() {
    const { clientName } = useParams()
    const [client, setClient] = useState<ClientState | null>(null)
    const [library, setLibrary] = useState<Array<Artists>>([])
    const [error, setError] = useState<string | null>(null)
    const [libraryError, setLibraryError] = useState<string | null>(null)

    useEffect(() => {
        getClientState(clientName!, setError, setClient)
        const id = setInterval(() => { getClientState(clientName!, setError, setClient) }, 2000)
        return () => clearInterval(id)
    }, [])

    useEffect(() => {
        getLibrary(setLibrary, setLibrary)
        const id = setInterval(() => { getLibrary(setLibraryError, setLibrary) }, 30000)
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
            <div>
                {!libraryError && (
                    <p>Number of artists: {library.length}</p>
                )}
            </div>
        </div>
    )
}