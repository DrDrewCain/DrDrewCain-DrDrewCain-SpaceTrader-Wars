use serde::{Deserialize, Serialize};
use crate::api::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct Waypoint {
    pub symbol: String,
    pub r#type: String,
    pub x: i32,
    pub y: i32,
    pub traits: Vec<WaypointTrait>,
}

#[derive(Debug, Deserialize)]
pub struct WaypointTrait {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Shipyard {
    pub symbol: String,
    #[serde(rename = "shipTypes")]
    pub ship_types: Vec<ShipType>,
    pub ships: Option<Vec<Ship>>,
}

#[derive(Debug, Deserialize)]
pub struct ShipType {
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Ship {
    pub r#type: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "purchasePrice")]
    pub purchase_price: i32,
    pub frame: ShipFrame,
    pub reactor: ShipReactor,
    pub engine: ShipEngine,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
}

#[derive(Debug, Deserialize)]
pub struct ShipFrame {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ShipReactor {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ShipEngine {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ShipModule {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ShipMount {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct PurchaseShipRequest {
    #[serde(rename = "shipType")]
    pub ship_type: String,
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
}

pub async fn find_shipyards(system_symbol: &str) -> anyhow::Result<ApiResponse<Vec<Waypoint>>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.spacetraders.io/v2/systems/{}/waypoints?traits=SHIPYARD",
        system_symbol
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

pub async fn get_shipyard_details(system_symbol: &str, waypoint_symbol: &str) -> anyhow::Result<ApiResponse<Shipyard>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.spacetraders.io/v2/systems/{}/waypoints/{}/shipyard",
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

pub async fn purchase_ship(ship_type: &str, waypoint_symbol: &str) -> anyhow::Result<ApiResponse<Ship>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let request = PurchaseShipRequest {
        ship_type: ship_type.to_string(),
        waypoint_symbol: waypoint_symbol.to_string(),
    };
    
    let response = client
        .post("https://api.spacetraders.io/v2/my/ships")
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await?;
        
    println!("Purchase response status: {}", response.status());
    
    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);
    
    let parsed_response: ApiResponse<Ship> = serde_json::from_str(&response_text)
        .map_err(|e| {
            println!("JSON parsing error: {}", e);
            e
        })?;

    Ok(parsed_response)
} 