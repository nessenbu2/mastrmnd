import {useParams} from "react-router-dom";
import {getClientState, getLibrary} from "../api/clients";
import {useEffect, useState} from "react";
import ClientState, {Artist, Song} from "./types";
import Library from "./Library";


export default function ClientPage() {
    const { clientName } = useParams()
    const [client, setClient] = useState<ClientState | null>(null)
    const [library, setLibrary] = useState<Array<Artist>>([])
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

    const onPlay = (song: Song) => {
        console.log("would play song: " + song.name);
        console.log("client name: " + clientName);
    }

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
                    <Library artists={library} handlePlay={onPlay}/>
                )}
            </div>
        </div>
    )
}