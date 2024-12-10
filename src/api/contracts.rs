use serde::Deserialize;
use crate::api::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct Contract {
    pub id: String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub terms: ContractTerms,
    pub accepted: bool,
    pub fulfilled: bool,
    #[serde(rename = "deadlineToAccept")]
    pub deadline_to_accept: String,
}

#[derive(Debug, Deserialize)]
pub struct ContractTerms {
    pub deadline: String,
    pub payment: ContractPayment,
    pub deliver: Option<Vec<ContractDelivery>>,
}

#[derive(Debug, Deserialize)]
pub struct ContractPayment {
    #[serde(rename = "onAccepted")]
    pub on_accepted: i32,
    #[serde(rename = "onFulfilled")]
    pub on_fulfilled: i32,
}

#[derive(Debug, Deserialize)]
pub struct ContractDelivery {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    #[serde(rename = "destinationSymbol")]
    pub destination_symbol: String,
    #[serde(rename = "unitsRequired")]
    pub units_required: i32,
    #[serde(rename = "unitsFulfilled")]
    pub units_fulfilled: i32,
}

#[derive(Debug, Deserialize)]
pub struct AcceptContractResponse {
    pub contract: Contract,
    pub agent: super::api::Agent,
}

pub async fn list_contracts() -> anyhow::Result<ApiResponse<Vec<Contract>>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spacetraders.io/v2/my/contracts")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn accept_contract(contract_id: &str) -> anyhow::Result<ApiResponse<AcceptContractResponse>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let url = format!("https://api.spacetraders.io/v2/my/contracts/{}/accept", contract_id);
    
    println!("Making request to: {}", url);
    
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({}))  // Send empty JSON object
        .send()
        .await?;
    
    println!("Response status: {}", response.status());
    
    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);
    
    let parsed_response: ApiResponse<AcceptContractResponse> = serde_json::from_str(&response_text)
        .map_err(|e| {
            println!("JSON parsing error: {}", e);
            e
        })?;
    
    Ok(parsed_response)
} 