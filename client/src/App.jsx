import React, { useEffect, useState } from 'react'

export default function App() {
  const [result, setResult] = useState(null)
  const [error, setError] = useState(null)
  const [loading, setLoading] = useState(false)

  const callBackend = async () => {
    setLoading(true)
    setError(null)
    try {
      const resp = await fetch('http://localhost:8080/clients', {
        method: 'GET',
        headers: {
          'Accept': 'application/json, text/plain, */*'
        },
      })
      const contentType = resp.headers.get('content-type') || ''
      let body
      if (contentType.includes('application/json')) {
        body = await resp.json()
      } else {
        body = await resp.text()
      }
      if (!resp.ok) {
        throw new Error(`HTTP ${resp.status}: ${typeof body === 'string' ? body : JSON.stringify(body)}`)
      }
      setResult(body)
    } catch (e) {
      setError(e.message || String(e))
      setResult(null)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    // Auto-call on load; and poll every 2s
    callBackend()
    const id = setInterval(() => { callBackend() }, 2000)
    return () => clearInterval(id)
  }, [])

  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', padding: 24 }}>
      <h1>mastrmnd client</h1>
      <p>Tracked gRPC clients (by id/peer) with call counts from http://localhost:8080/clients.</p>
      <button onClick={callBackend} disabled={loading}>
        {loading ? 'Calling…' : 'Call backend'}
      </button>
      <div style={{ marginTop: 16 }}>
        {error && (
          <div style={{ color: 'crimson' }}>
            <strong>Error:</strong> {error}
          </div>
        )}
        {result !== null && !error && (
          Array.isArray(result) ? null : (
            <table style={{ width: '100%', borderCollapse: 'collapse', marginTop: 8 }}>
              <thead>
                <tr>
                  <th style={{ textAlign: 'left', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Client</th>
                  <th style={{ textAlign: 'right', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Calls</th>
                </tr>
              </thead>
              <tbody>
                {Object.entries(result).sort((a,b)=> b[1]-a[1]).map(([id,count]) => (
                  <tr key={id}>
                    <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px' }}>{id}</td>
                    <td style={{ borderBottom: '1px solid #eee', padding: '4px 8px', textAlign: 'right' }}>{count}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        )}
      </div>
    </div>
  )
}
