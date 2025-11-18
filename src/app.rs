/// The main application state.
pub struct App {
    pub results: Vec<RpcResult>,
    pub should_quit: bool,
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
        }
    }

    /// Updates the RPC results and sorts them by health (healthy first), then by latency.
    pub fn update_results(&mut self, new_results: Vec<RpcResult>) {
        self.results = new_results;
        self.results.sort_by_key(|r| (!r.healthy, r.latency_ms));
    }
}
