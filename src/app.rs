use std::time::Instant;

/// The main application state.
pub struct App {
    pub results: Vec<RpcResult>,
    pub should_quit: bool,
    pub last_update: Option<Instant>,
}

/// Represents the result of an RPC call.
#[derive(Clone)]
pub struct RpcResult {
    pub name: String,
    pub url: String,
    pub latency_ms: u64,
    pub healthy: bool,
}

impl App {
    /// Creates a new instance of the application state.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            should_quit: false,
            last_update: None,
        }
    }

    /// Updates the RPC results and sorts them by health (healthy first), then by latency.
    pub fn update_results(&mut self, new_results: Vec<RpcResult>) {
        self.results = new_results;
        self.results.sort_by_key(|r| (!r.healthy, r.latency_ms));
        self.last_update = Some(Instant::now())
    }
}
