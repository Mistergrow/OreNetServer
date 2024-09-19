use {
    ore_api::consts::BUS_ADDRESSES,
    reqwest::Client,
    serde_json::{json, Value},
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_response::RpcPrioritizationFee},
    solana_sdk::{message::Message, pubkey::Pubkey},
    std::{collections::HashMap, str::FromStr},
    tracing::warn,
    url::Url,
    std::env,
    dotenv::dotenv,
};

pub const DEFAULT_PRIORITY_FEE: u64 = 11_000;

pub struct HeliusClient {
    pub api_url: String,
}

impl HeliusClient {
    async fn estimate_priority_fee(&self, blockhash: &str) -> Result<u64, String> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getPriorityFeeEstimate",
            "params": [
                {
                    "blockhash": blockhash
                }
            ]
        });

        tracing::info!("Requesting priority fee from Helius: {}", self.api_url);

        let client = Client::new();
        let response = client
            .post(&self.api_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status() != reqwest::StatusCode::OK {
            return Err(format!("Unexpected status code: {}", response.status()));
        }

        let fee_data = response.json::<Value>().await.map_err(|e| e.to_string())?;
        Ok(fee_data["result"]["priorityFeeEstimate"].as_u64().unwrap_or(DEFAULT_PRIORITY_FEE))
    }
}

pub struct QuickNodeClient {
    pub api_url: String,
}

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

impl QuickNodeClient {
    pub async fn qn_estimate_priority_fees(&self, last_n_blocks: u64) -> Result<serde_json::Value, String> {
        // Dynamische URL mit dem Endpunkt für QuickNode erstellen
        let url = format!("{}/v1/estimatePriorityFees", self.api_url);

        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));

        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBlockHeight",
        });

        tracing::info!("Sending POST request to QuickNode: {}", url);

        let client = Client::new();
        let response = client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to send request: {}", e);
                e.to_string()
            })?;

        tracing::info!("QuickNode response: {:?}", response);

        let fee_data = response.json::<Value>().await.map_err(|e| {
            tracing::error!("Failed to parse response JSON: {}", e);
            e.to_string()
        })?;

        Ok(fee_data)
    }
}

pub struct AlchemyClient {
    pub api_url: String,
}

impl AlchemyClient {
    async fn get_priority_fee(&self, blockhash: &str) -> Result<u64, String> {
        let url = format!("{}/getPriorityFee?blockhash={}", self.api_url, blockhash);
        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        let fee_data = response.json::<Value>().await.map_err(|e| e.to_string())?;
        Ok(fee_data["priority_fee"].as_u64().unwrap_or(DEFAULT_PRIORITY_FEE))
    }
}

enum FeeStrategy {
    Helius,
    Triton,
    LOCAL,
    Alchemy,
    Quiknode,
}

fn detect_fee_strategy(host: &str) -> FeeStrategy {
    if host.contains("helius") {
        FeeStrategy::Helius
    } else if host.contains("alchemy") {
        FeeStrategy::Alchemy
    } else if host.contains("triton") {
        FeeStrategy::Triton
    } else if host.contains("quiknode") {
        FeeStrategy::Quiknode
    } else {
        FeeStrategy::LOCAL
    }
}

pub async fn dynamic_fee(
    rpc_client: &RpcClient,
    dynamic_fee_url: Option<String>,
    helius_client: &HeliusClient,
    quiknode_client: &QuickNodeClient,
    alchemy_client: &AlchemyClient,
    priority_fee_cap: Option<u64>,
) -> Result<u64, String> {
    dotenv().ok();  // Lade Umgebungsvariablen

    let rpc_url = dynamic_fee_url.unwrap_or_else(|| rpc_client.url());

    let host = match Url::parse(&rpc_url) {
        Ok(parsed_url) => match parsed_url.host_str() {
            Some(host) => host.to_string(),
            None => return Err(format!("Invalid host in URL: {}", rpc_url)),
        },
        Err(err) => return Err(format!("Failed to parse URL: {}. Error: {}", rpc_url, err)),
    };

    let strategy = detect_fee_strategy(&host);

    match strategy {
        FeeStrategy::Helius => {
            let fee = get_fee_from_helius(rpc_client, helius_client, priority_fee_cap).await?;
            Ok(fee)
        }
        FeeStrategy::Alchemy => {
            let fee = get_fee_from_alchemy(rpc_client, alchemy_client).await?;
            Ok(fee)
        }
        FeeStrategy::Triton => {
            let fee = get_fee_from_triton().await?;
            Ok(fee)
        }
        FeeStrategy::LOCAL => {
            let fee = DEFAULT_PRIORITY_FEE;
            Ok(fee)
        }
        FeeStrategy::Quiknode => {
            let fee = get_fee_from_quiknode(quiknode_client).await?;
            Ok(fee)
        }
    }
}

async fn get_fee_from_helius(
    rpc_client: &RpcClient,
    helius_client: &HeliusClient,
    priority_fee_cap: Option<u64>,
) -> Result<u64, String> {
    let blockhash = rpc_client.get_latest_blockhash().await.map_err(|e| e.to_string())?;
    tracing::info!("Using blockhash: {}", blockhash);
    
    let fee_estimate = helius_client
        .estimate_priority_fee(&blockhash.to_string())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(priority_fee_cap.unwrap_or(fee_estimate as u64))
}

async fn get_fee_from_quiknode(quiknode_client: &QuickNodeClient) -> Result<u64, String> {
    let response = quiknode_client.qn_estimate_priority_fees(100).await.map_err(|e| e.to_string())?;
    Ok(response["per_compute_unit"]["medium"].as_u64().unwrap_or(DEFAULT_PRIORITY_FEE))
}

async fn get_fee_from_alchemy(
    rpc_client: &RpcClient,
    alchemy_client: &AlchemyClient,
) -> Result<u64, String> {
    let blockhash = rpc_client.get_latest_blockhash().await.map_err(|e| e.to_string())?;
    let fee_estimate = alchemy_client.get_priority_fee(&blockhash.to_string()).await.map_err(|e| e.to_string())?;
    
    Ok(fee_estimate as u64)
}

async fn get_fee_from_triton() -> Result<u64, String> {
    Ok(10_000)
}
