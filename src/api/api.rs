pub use dotenv::dotenv;
pub use reqwest::Client;
pub use serde::Deserialize;
pub use std::env;
pub use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct Agent {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
    #[serde(rename = "startingFaction")]
    pub starting_faction: String,
    #[serde(rename = "shipCount")]
    pub ship_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct Waypoint {
    pub symbol: String,
    pub r#type: String,
    pub x: i32,
    pub y: i32,
    pub traits: Vec<WaypointTrait>,
    pub orbitals: Vec<WaypointOrbital>,
    pub faction: Option<WaypointFaction>,
}

#[derive(Debug, Deserialize)]
pub struct WaypointTrait {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct WaypointOrbital {
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct WaypointFaction {
    pub symbol: String,
}

pub async fn get_agent_details() -> Result<ApiResponse<Agent>> {
    dotenv().ok();
    let token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN must be set in .env");
    
    let client = Client::new();
    let response = client
        .get("https://api.spacetraders.io/v2/my/agent")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn get_waypoint_details(system_symbol: &str, waypoint_symbol: &str) -> Result<ApiResponse<Waypoint>> {
    dotenv().ok();
    let token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN must be set in .env");
    
    let client = Client::new();
    let url = format!(
        "https://api.spacetraders.io/v2/systems/{}/waypoints/{}", 
        system_symbol, 
        waypoint_symbol
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
