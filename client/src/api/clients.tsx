
type HandleError = (error: string) => void;
import type { ClientsResponse } from "../lib/types";

type HandleSuccess = (result: ClientsResponse) => void;

export async function fetchClients(error: HandleError, success: HandleSuccess) {
    try {
        const resp = await fetch('http://localhost:8080/clients', {
            method: 'GET',
            headers: {
                'Accept': 'application/json, text/plain, */*'
            },
        })
        let body = await resp.json()
        if (!resp.ok) {
            error(body)
        } else {
            success(body)
        }
    } catch (e) {
        error(e.message || String(e))
    }
}
