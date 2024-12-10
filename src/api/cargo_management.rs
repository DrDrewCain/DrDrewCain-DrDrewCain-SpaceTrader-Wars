// In a new file: src/api/cargo_management.rs
use super::{get_market_data, get_waypoint_details, list_contracts, ships::CargoItem};
use anyhow::Result;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ContractRequirement {
    pub trade_symbol: String,
    pub required_units: i32,
    pub fulfilled_units: i32,
    pub destination: String,
}

#[derive(Debug, Clone)]
pub struct MarketPriceInfo {
    pub waypoint: String,
    pub sell_price: i32,
    pub trade_volume: i32,
    pub distance: f64,
}
#[derive(Debug)]
pub struct MarketAnalysis {
    pub prices: HashMap<String, Vec<MarketPriceInfo>>,
    pub requirements: Vec<ContractRequirement>,
}

impl MarketAnalysis {
    pub fn get_best_market_for_item(&self, item_symbol: &str) -> Option<&MarketPriceInfo> {
        self.prices
            .get(item_symbol)?
            .iter()
            .max_by_key(|m| (m.sell_price as f64 - m.distance * 100.0) as i32)
    }

    pub fn get_contract_requirement(&self, item_symbol: &str) -> Option<&ContractRequirement> {
        self.requirements
            .iter()
            .find(|req| req.trade_symbol == item_symbol)
    }
}

pub async fn get_contract_requirements() -> Result<Vec<ContractRequirement>> {
    let mut requirements = Vec::new();

    match list_contracts().await {
        Ok(response) => {
            for contract in &response.data {
                if contract.accepted && !contract.fulfilled {
                    if let Some(deliveries) = &contract.terms.deliver {
                        for delivery in deliveries {
                            let remaining = delivery.units_required - delivery.units_fulfilled;
                            if remaining > 0 {
                                requirements.push(ContractRequirement {
                                    trade_symbol: delivery.trade_symbol.clone(),
                                    required_units: remaining,
                                    fulfilled_units: delivery.units_fulfilled,
                                    destination: delivery.destination_symbol.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to get contract requirements: {}", e);
        }
    }

    Ok(requirements)
}

pub async fn find_all_markets_in_system(system_symbol: &str) -> Result<Vec<String>> {
    let mut market_waypoints = Vec::new();

    match get_waypoint_details(system_symbol, &format!("{}-", system_symbol)).await {
        Ok(response) => {
            for waypoint in response.data.traits.iter() {
                if waypoint.symbol == "MARKETPLACE" {
                    market_waypoints.push(waypoint.symbol.clone());
                }
            }
        }
        Err(e) => {
            error!("Failed to get system waypoints: {}", e);
        }
    }

    Ok(market_waypoints)
}

pub async fn analyze_markets(
    system_symbol: &str,
    current_x: i32,
    current_y: i32,
    items: &[String],
) -> Result<MarketAnalysis> {
    let mut all_prices: HashMap<String, Vec<MarketPriceInfo>> = HashMap::new();

    if let Ok(markets) = find_all_markets_in_system(system_symbol).await {
        for market_waypoint in markets {
            if let Ok(waypoint_details) =
                get_waypoint_details(system_symbol, &market_waypoint).await
            {
                let distance = (((waypoint_details.data.x - current_x).pow(2)
                    + (waypoint_details.data.y - current_y).pow(2))
                    as f64)
                    .sqrt();

                if let Ok(market_data) = get_market_data(system_symbol, &market_waypoint).await {
                    for trade_good in market_data.data.trade_goods {
                        if items.contains(&trade_good.symbol) {
                            let price_info = MarketPriceInfo {
                                waypoint: market_waypoint.clone(),
                                sell_price: trade_good.sell_price,
                                trade_volume: trade_good.trade_volume,
                                distance,
                            };

                            all_prices
                                .entry(trade_good.symbol)
                                .or_insert_with(Vec::new)
                                .push(price_info);
                        }
                    }
                }
            }
        }
    }

    let requirements = get_contract_requirements().await?;

    Ok(MarketAnalysis {
        prices: all_prices,
        requirements,
    })
}

pub fn calculate_max_sellable_units(
    item: &CargoItem,
    contract_requirement: Option<&ContractRequirement>,
) -> i32 {
    if let Some(req) = contract_requirement {
        (item.units - req.required_units).max(0)
    } else {
        item.units
    }
}
