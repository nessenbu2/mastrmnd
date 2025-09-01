
import type { ClientState } from "./lib/types";
import ClientRow from "./lib/ClientRow";

type Props = { clients: ClientState[] };

export default function ClientTable({ clients }: Props) {
  return (
      <div>
        <table style={{ width: '100%', borderCollapse: 'collapse', marginTop: 8 }}>
          <thead>
          <tr>
            <th style={{ textAlign: 'left', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Client</th>
            <th style={{ textAlign: 'left', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Toggle</th>
            <th style={{ textAlign: 'left', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Inc</th>
            <th style={{ textAlign: 'right', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Calls</th>
            <th style={{ textAlign: 'left', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Last Message</th>
            <th style={{ textAlign: 'right', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>Last Seen (s)</th>
            <th style={{ textAlign: 'left', borderBottom: '1px solid #ccc', padding: '4px 8px' }}>State</th>
          </tr>
          </thead>
          <tbody>
          {clients.map((state: ClientState) => (
              <ClientRow state={state}/>
          ))}
          </tbody>
        </table>
      </div>
  )

}