import React, { useEffect, useState } from 'react'
import { fetchClients } from "./api/clients";
import type { ClientState, ClientsResponse } from "./lib/types";

export default function App() {
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
          Array.isArray(result) ? (
            <table style={{ width: '100%', borderCollapse: 'collapse', marginTop: 8 }}>
              <thead>
                <tr>
                  <th style={{ textAlign: 'left', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Client</th>
                  <th style={{ textAlign: 'right', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Calls</th>
                  <th style={{ textAlign: 'left', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Last Message</th>
                  <th style={{ textAlign: 'right', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Last Seen (s)</th>
                  <th style={{ textAlign: 'left', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>State</th>
                </tr>
              </thead>
              <tbody>
                {result.sort((a: ClientState,b: ClientState)=> b.call_count - a.call_count).map((item: ClientState) => (
                  <tr key={item.name}>
                    <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px' }}>{item.name}</td>
                    <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px', textAlign: 'right' }}>{item.call_count}</td>
                    <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px' }}>{item.last_message}</td>
                    <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px', textAlign: 'right' }}>{item.last_seen_secs}</td>
                    <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px' }}>{typeof item.state === 'number' ? (item.state === 0 ? 'Idle' : (item.state === 1 ? 'Playing' : String(item.state))) : String(item.state)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          ) : (
            <pre style={{ background: '#f6f8fa', padding: 12, borderRadius: 6, overflowX: 'auto' }}>{JSON.stringify(result, null, 2)}</pre>
          )
        )}
      </div>
    </div>
  )
}
