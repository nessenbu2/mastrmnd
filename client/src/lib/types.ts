export enum State {
  Idle = "Idle",
  Playing = "Playing",
}

export type ClientState = {
  name: string;
  call_count: number;
  last_seen_secs: number;
  last_message: string;
  state: State | number | string;
};

export type ClientsResponse = ClientState[];

export default ClientState;
