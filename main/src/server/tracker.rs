use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct Tracker {
    inner: Arc<Mutex<HashMap<String, u64>>>,
}

impl Tracker {
    pub fn new() -> Self { Self::default() }

    pub fn record(&self, client_id: String) {
        let mut map = self.inner.lock().unwrap();
        *map.entry(client_id).or_insert(0) += 1;
    }

    pub fn snapshot(&self) -> HashMap<String, u64> {
        self.inner.lock().unwrap().clone()
    }
}

// Helper to derive a client name from tonic::Request
pub fn extract_client_name<T>(req: &tonic::Request<T>) -> String {
    // Print all metadata key/value pairs for visibility
    // Note: avoid panics; bound output sizes
    let md = req.metadata();
    if !md.is_empty() {
        // Collect printable lines
        let mut lines: Vec<String> = Vec::new();
        for kv in md.iter() {
            use tonic::metadata::{KeyAndValueRef, ValueRef};
            match kv {
                KeyAndValueRef::Ascii(k, v) => {
                    let name = k.as_str();
                    match v.to_str() {
                        Ok(s) => {
                            let s_trunc = if s.len() > 256 { &s[..256] } else { s };
                            lines.push(format!("{}: {}", name, s_trunc));
                        }
                        Err(_) => {
                            // Should not happen for Ascii, but be safe
                            let bytes = v.as_bytes();
                            let n = bytes.len().min(32);
                            let hex: String = bytes[..n].iter().map(|b| format!("{:02x}", b)).collect();
                            let more = if bytes.len() > n { "…" } else { "" };
                            lines.push(format!("{}: <ascii-bytes {}: {}{}>", name, bytes.len(), hex, more));
                        }
                    }
                }
                KeyAndValueRef::Binary(k, v) => {
                    let name = k.as_str();
                    let bytes = v.as_encoded_bytes();
                    let n = bytes.len().min(32);
                    let hex: String = bytes[..n].iter().map(|b| format!("{:02x}", b)).collect();
                    let more = if bytes.len() > n { "…" } else { "" };
                    lines.push(format!("{}: <binary {} bytes: {}{}>", name, bytes.len(), hex, more));
                }
            }
        }
        if !lines.is_empty() {
            println!("gRPC metadata ({} entries):\n{}", lines.len(), lines.join("\n"));
        }
    }

    // Prefer explicit metadata header x-client-name, then x-client-id
    if let Some(val) = req.metadata().get("x-client-name") {
        if let Ok(s) = val.to_str() { return format!("name:{}", s); }
    }
    if let Some(val) = req.metadata().get("x-client-id") {
        if let Ok(s) = val.to_str() { return format!("name:{}", s); }
    }
    // Try to get peer addr from extensions (set by Tonic/Hyper)
    if let Some(peer) = req.extensions().get::<SocketAddr>() {
        return format!("peer:{}", peer);
    }
    // Fallback to empty/unknown
    "unknown".to_string()
}
