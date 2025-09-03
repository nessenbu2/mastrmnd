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
        error(String(e))
    }
}

export async function toggleState(clientName: String) {
    try {
        const resp = await fetch(`http://localhost:8080/clients/toggle/${clientName}`, {
            method: 'GET',
            headers: {
                'Accept': 'application/json, text/plain, */*'
            }
        })
        let body = await resp.json();
        if (!resp.ok){
            console.error(body);
        }
    }
    catch (e) {
        console.error(e);
    }
}

export async function incClient(clientName: String) {
    try {
        const resp = await fetch(`http://localhost:8080/clients/inc/${clientName}`, {
            method: 'GET',
            headers: {
                'Accept': 'application/json, text/plain, */*'
            }
        })
        let body = await resp.json();
        if (!resp.ok){
            console.error(body);
        }
    }
    catch (e) {
        console.error(e);
    }
}

export async function getClientState(clientName: String, error: HandleError, success: HandleSuccess) {
    try {
        const resp = await fetch(`http://localhost:8080/clients/state/${clientName}`, {
            method: 'GET',
            headers: {
                'Accept': 'application/json, text/plain, */*'
            }
        })
        let body = await resp.json()
        if (!resp.ok){
            error(body)
        } else {
            success(body)
        }
    } catch (e) {
        error(String(e))
    }
}
