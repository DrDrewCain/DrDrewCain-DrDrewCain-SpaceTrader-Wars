use serde::{Deserialize, Serialize};
use crate::api::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct Ship {
    pub symbol: String,
    pub nav: ShipNav,
    pub crew: ShipCrew,
    pub fuel: ShipFuel,
    pub cooldown: ShipCooldown,
    pub frame: ShipFrame,
    pub reactor: ShipReactor,
    pub engine: ShipEngine,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
    pub registration: ShipRegistration,
    pub cargo: ShipCargo,
}

#[derive(Debug, Deserialize)]
pub struct ShipNav {
    pub status: String,
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    pub route: Option<ShipRoute>,
}

#[derive(Debug, Deserialize)]
pub struct ShipRoute {
    pub destination: Option<ShipLocation>,
    pub departure: Option<ShipLocation>,
    #[serde(rename = "departureTime")]
    pub departure_time: Option<String>,
    #[serde(rename = "arrivalTime")]
    pub arrival_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ShipLocation {
    pub symbol: String,
    #[serde(rename = "type")]
    pub location_type: String,
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
pub struct ShipRegistration {
    pub name: String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct ShipCargo {
    pub capacity: i32,
    pub units: i32,
    pub inventory: Vec<CargoItem>,
}

#[derive(Debug, Deserialize)]
pub struct CargoItem {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub units: i32,
}

#[derive(Debug, Deserialize)]
pub struct ShipFuel {
    pub current: i32,
    pub capacity: i32,
    pub consumed: FuelConsumed,
}

#[derive(Debug, Deserialize)]
pub struct FuelConsumed {
    pub amount: i32,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct Waypoint {
    pub symbol: String,
    pub r#type: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize)]
pub struct NavigateRequest {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseShipResponse {
    pub ship: Ship,
    pub agent: super::api::Agent,
    pub transaction: ShipTransaction,
}

#[derive(Debug, Deserialize)]
pub struct ShipTransaction {
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "shipType")]
    pub ship_type: String,
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "agentSymbol")]
    pub agent_symbol: String,
    pub price: i32,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct ShipCrew {
    pub current: i32,
    pub capacity: i32,
    pub required: i32,
    pub rotation: String,
    pub morale: i32,
    pub wages: i32,
}

#[derive(Debug, Deserialize)]
pub struct ShipCooldown {
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "totalSeconds")]
    pub total_seconds: i32,
    #[serde(rename = "remainingSeconds")]
    pub remaining_seconds: i32,
}

#[derive(Debug, Deserialize)]
pub struct ShipFrame {
    pub symbol: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "moduleSlots")]
    pub module_slots: i32,
    #[serde(rename = "mountingPoints")]
    pub mounting_points: i32,
    #[serde(rename = "fuelCapacity")]
    pub fuel_capacity: i32,
    pub condition: f32,
    pub integrity: f32,
    pub requirements: Requirements,
}

#[derive(Debug, Deserialize)]
pub struct Requirements {
    #[serde(default)]
    pub power: i32,
    #[serde(default)]
    pub crew: i32,
    #[serde(default)]
    pub slots: i32,
}

#[derive(Debug, Deserialize)]
pub struct ShipReactor {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub condition: f32,
    pub integrity: f32,
    #[serde(rename = "powerOutput")]
    pub power_output: i32,
    pub requirements: Requirements,
}

#[derive(Debug, Deserialize)]
pub struct ShipEngine {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub condition: f32,
    pub integrity: f32,
    pub speed: i32,
    pub requirements: Requirements,
}

#[derive(Debug, Deserialize)]
pub struct ShipModule {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub capacity: Option<i32>,
    pub requirements: Requirements,
}

#[derive(Debug, Deserialize)]
pub struct ShipMount {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub strength: Option<i32>,
    pub requirements: Requirements,
}

#[derive(Debug, Serialize)]
pub struct PurchaseShipRequest {
    #[serde(rename = "shipType")]
    pub ship_type: String,
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    pub message: String,
    pub code: i32,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct NavigationResponse {
    pub nav: ShipNav,
    pub fuel: ShipFuel,
    #[serde(default)]
    pub events: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExtractionResponse {
    pub extraction: Extraction,
    pub cargo: ShipCargo,
    pub cooldown: ShipCooldown,
}

#[derive(Debug, Deserialize)]
pub struct Extraction {
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "yield")]
    pub yield_data: Yield,
}

#[derive(Debug, Deserialize)]
pub struct Yield {
    pub symbol: String,
    pub units: i32,
}

#[derive(Debug, Deserialize)]
pub struct CooldownError {
    pub error: CooldownErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct CooldownErrorDetail {
    pub message: String,
    pub code: i32,
    pub data: CooldownData,
}

#[derive(Debug, Deserialize)]
pub struct CooldownData {
    #[serde(rename = "cooldownRemaining")]
    pub cooldown_remaining: i32,
    #[serde(rename = "totalCooldown")]
    pub total_cooldown: i32,
    pub expiration: String,
}

pub async fn list_ships() -> anyhow::Result<ApiResponse<Vec<Ship>>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spacetraders.io/v2/my/ships")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn find_asteroids(system_symbol: &str) -> anyhow::Result<ApiResponse<Vec<Waypoint>>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.spacetraders.io/v2/systems/{}/waypoints?type=ENGINEERED_ASTEROID",
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

pub async fn orbit_ship(ship_symbol: &str) -> anyhow::Result<ApiResponse<Ship>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("https://api.spacetraders.io/v2/my/ships/{}/orbit", ship_symbol))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({}))
        .send()
        .await?;
        
    let status = response.status();
    println!("Orbit response status: {}", status);
    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);
    
    if !status.is_success() {
        if let Ok(error) = serde_json::from_str::<ErrorResponse>(&response_text) {
            println!("Error: {}", error.error.message);
            anyhow::bail!("API Error: {}", error.error.message);
        }
    }
    
    let parsed_response: ApiResponse<Ship> = serde_json::from_str(&response_text)
        .map_err(|e| {
            println!("JSON parsing error: {}", e);
            e
        })?;

    Ok(parsed_response)
}

pub async fn navigate_ship(ship_symbol: &str, waypoint_symbol: &str) -> anyhow::Result<ApiResponse<NavigationResponse>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let request = NavigateRequest {
        waypoint_symbol: waypoint_symbol.to_string(),
    };
    
    let response = client
        .post(&format!("https://api.spacetraders.io/v2/my/ships/{}/navigate", ship_symbol))
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await?;
        
    let status = response.status();
    println!("Navigation response status: {}", status);
    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);
    
    if !status.is_success() {
        if let Ok(error) = serde_json::from_str::<ErrorResponse>(&response_text) {
            println!("Error: {}", error.error.message);
            anyhow::bail!("API Error: {}", error.error.message);
        }
    }
    
    let parsed_response: ApiResponse<NavigationResponse> = serde_json::from_str(&response_text)
        .map_err(|e| {
            println!("JSON parsing error: {}", e);
            e
        })?;

    Ok(parsed_response)
}

pub async fn dock_ship(ship_symbol: &str) -> anyhow::Result<ApiResponse<Ship>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("https://api.spacetraders.io/v2/my/ships/{}/dock", ship_symbol))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn refuel_ship(ship_symbol: &str) -> anyhow::Result<ApiResponse<Ship>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("https://api.spacetraders.io/v2/my/ships/{}/refuel", ship_symbol))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn check_extraction_cooldown(ship_symbol: &str) -> anyhow::Result<Option<CooldownData>> {
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("https://api.spacetraders.io/v2/my/ships/{}/cooldown", ship_symbol))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    let status = response.status();
    let response_text = response.text().await?;
    
    if status.is_success() {
        if let Ok(cooldown_response) = serde_json::from_str::<ApiResponse<CooldownData>>(&response_text) {
            return Ok(Some(cooldown_response.data));
        }
    }
    
    Ok(None)
}

pub async fn extract_resources(ship_symbol: &str) -> anyhow::Result<ApiResponse<ExtractionResponse>> {
    // First check if ship is on cooldown
    if let Ok(Some(cooldown)) = check_extraction_cooldown(ship_symbol).await {
        println!("Ship is on cooldown for {} more seconds (expires at {})",
            cooldown.cooldown_remaining,
            cooldown.expiration);
        anyhow::bail!("Ship is on cooldown");
    }
    
    let token = std::env::var("BEARER_TOKEN")?;
    
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("https://api.spacetraders.io/v2/my/ships/{}/extract", ship_symbol))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({}))
        .send()
        .await?;
        
    let status = response.status();
    println!("Extraction response status: {}", status);
    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);
    
    if !status.is_success() {
        if let Ok(error) = serde_json::from_str::<CooldownError>(&response_text) {
            println!("Cooldown error: {}", error.error.message);
            println!("Remaining cooldown: {} seconds", error.error.data.cooldown_remaining);
            println!("Expires at: {}", error.error.data.expiration);
            anyhow::bail!("Ship is on cooldown");
        }
        
        if let Ok(error) = serde_json::from_str::<ErrorResponse>(&response_text) {
            println!("Error: {}", error.error.message);
            anyhow::bail!("API Error: {}", error.error.message);
        }
    }
    
    let parsed_response: ApiResponse<ExtractionResponse> = serde_json::from_str(&response_text)
        .map_err(|e| {
            println!("JSON parsing error: {}", e);
            e
        })?;

    Ok(parsed_response)
}

pub async fn purchase_ship(ship_type: &str, waypoint_symbol: &str) -> anyhow::Result<ApiResponse<PurchaseShipResponse>> {
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
    
    let parsed_response: ApiResponse<PurchaseShipResponse> = serde_json::from_str(&response_text)
        .map_err(|e| {
            println!("JSON parsing error: {}", e);
            e
        })?;

    Ok(parsed_response)
} 