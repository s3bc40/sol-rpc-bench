use futures::future::join_all;
use reqwest::Client;
use serde_json::json;
use tokio::time::timeout;

use crate::app::RpcResult;
use std::time::{Duration, Instant};

/// List of Solana  public RPC endpoints to benchmark
pub fn get_rpc_list() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Helius Public", "https://rpc.helius.xyz"),
        ("Ankr", "https://rpc.ankr.com/solana"),
        ("Chainstack", "https://solana-mainnet.public.blastapi.io"),
        ("PublicNode", "https://solana-rpc.publicnode.com"),
        ("Project Serum", "https://solana-api.projectserum.com"),
        ("Rpcpool", "https://api.rpcpool.com"),
        ("RunNode", "https://api.mainnet-beta.solana.com"),
        ("Triton 1", "https://solana-mainnet.rpc.extrnode.com"),
        ("GenesysGo", "https://ssc-dao.genesysgo.net"),
        ("QuickNode Public", "https://api.mainnet.solana.com"),
        ("Metaplex", "https://api.metaplex.solana.com"),
        ("Syndica Public", "https://solana-api.syndica.io"),
        ("Serum", "https://solana.publickey.com"),
        ("Mainnet Beta", "https://api.mainnet-beta.solana.com"),
        (
            "Figment Public",
            "https://solana--mainnet.datahub.figment.io",
        ),
        (
            "Blockdaemon",
            "https://try.blockdaemon.com/solana/mainnet/native",
        ),
        ("Cloudflare", "https://solana-mainnet.cloudflare-eth.com"),
        ("Allnodes", "https://solana-mainnet-rpc.allnodes.me"),
        ("Nodereal", "https://open-platform.nodereal.io/solana/"),
        ("Lava", "https://solana.lava.build"),
        ("Nodies", "https://lb.nodies.app/v1/solana-mainnet"),
        ("Shyft", "https://rpc.shyft.to"),
        ("SolanaFM", "https://api.solana.fm"),
        ("Magic Eden", "https://rpc-mainnet.magiceden.dev"),
        ("Triton One", "https://api.triton.one/rpc/solana"),
    ]
}

/// Benchmark a single RPC endpoint
pub async fn benchmark_rpc(name: &str, url: &str) -> RpcResult {
    // Create an HTTP client
    let client = Client::new();
    // Create a JSON-RPC request for `getHealth` method
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getHealth",
    });
    // Time how long the request takes
    let start = Instant::now();
    let response = timeout(
        Duration::from_secs(5),
        client.post(url).json(&request_body).send(),
    )
    .await;
    let elapsed_ms = start.elapsed().as_millis() as u64;

    // Return RpcResult with name, url, latency_ms, and healthy status
    match response {
        Ok(Ok(resp)) => {
            let healthy = resp.status().is_success();
            RpcResult {
                name: name.to_string(),
                url: url.to_string(),
                latency_ms: elapsed_ms,
                healthy,
            }
        }
        _ => RpcResult {
            name: name.to_string(),
            url: url.to_string(),
            latency_ms: elapsed_ms,
            healthy: false,
        },
    }
}

/// Benchmark all RPCs in parallel
pub async fn benchmark_all_rpcs() -> Vec<RpcResult> {
    // Get the RPC list
    let rpc_list = get_rpc_list();
    // Create a Vec of futures (one for each RPC)
    let futures = rpc_list
        .into_iter()
        .map(|(name, url)| benchmark_rpc(name, url));
    // Run them in parallel
    join_all(futures).await
}
