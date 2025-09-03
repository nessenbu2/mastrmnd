use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub enum State {
    Idle,
    Playing,
}

impl From<i32> for State {
    fn from(v: i32) -> Self {
        match v {
            1 => State::Playing,
            _ => State::Idle,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientState {
    pub name: String,
    pub call_count: u64,
    pub last_seen_secs: u64,
    pub last_message: String,
    pub state: State,
}

#[derive(Clone, Default)]
pub struct ClientStateStore {
    inner: Arc<Mutex<HashMap<String, ClientState>>>,
}

impl ClientStateStore {
    pub fn new() -> Self { Self::default() }

    pub fn record_register(&self, name: String, message: String, state: Option<i32>) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let mut map = self.inner.lock().unwrap();
        let entry = map.entry(name.clone()).or_insert_with(|| ClientState {
            name: name.clone(),
            call_count: 0,
            last_seen_secs: now,
            last_message: String::new(),
            state: State::Idle,
        });
        entry.call_count += 1;
        entry.last_seen_secs = now;
        entry.last_message = message;
        if let Some(s) = state { entry.state = State::from(s); }
    }

    pub fn snapshot_counts(&self) -> HashMap<String, u64> {
        let map = self.inner.lock().unwrap();
        map.iter().map(|(k,v)| (k.clone(), v.call_count)).collect()
    }

    pub fn snapshot(&self) -> Vec<ClientState> {
        self.inner.lock().unwrap().values().cloned().collect()
    }

    pub fn get_state(&self, name: String) -> Option<ClientState> {
        self.inner.lock().unwrap().get(&name).cloned()
    }

    pub fn inc(&self, name: String) {
        self.inner.lock().unwrap().entry(name).and_modify(|entry| {entry.call_count += 1});
    }

    pub fn toggle_state(&self, name: String) {
        self.inner.lock().unwrap().entry(name).and_modify(|entry| {
            if entry.state == State::Idle {
                entry.state = State::Playing
            } else {
                entry.state = State::Idle
            }
        });
    }
}
