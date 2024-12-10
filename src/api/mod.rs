// In src/api/mod.rs
mod api;
mod cargo_management;
mod contracts;
mod ships;
mod shipyard;

// Public re-exports
pub use self::{
    // From api module
    api::{
        get_agent_details, get_waypoint_details, Agent, ApiResponse, Waypoint, WaypointFaction,
        WaypointOrbital, WaypointTrait,
    },
    // From cargo_management module
    cargo_management::{
        analyze_markets, calculate_max_sellable_units, find_all_markets_in_system,
        get_contract_requirements, ContractRequirement, MarketAnalysis, MarketPriceInfo,
    },
    // From contracts module
    contracts::{accept_contract, list_contracts, AcceptContractResponse, Contract},
    // From ships module
    ships::{
        dock_ship, extract_resources, find_asteroids, get_market_data, get_ship_cargo, list_ships,
        navigate_ship, orbit_ship, refuel_ship, sell_cargo_item, CargoItem, CargoResponse,
        DockResponse, ExtractionResponse, Market, MarketTransaction, NavigationResponse, Ship,
    },
    // From shipyard module
    shipyard::{
        find_shipyards, get_shipyard_details, purchase_ship, Ship as ShipyardShip, Shipyard,
    },
};
