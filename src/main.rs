use anyhow::Result;
use env_logger;
use log::{debug, error, info};
use spacetraders_client::api::{
    accept_contract, analyze_markets, calculate_max_sellable_units, dock_ship,
    find_all_markets_in_system, find_asteroids, find_shipyards, get_agent_details,
    get_contract_requirements, get_market_data, get_ship_cargo, get_shipyard_details,
    get_waypoint_details, list_contracts, list_ships, navigate_ship, orbit_ship, purchase_ship,
    sell_cargo_item, CargoItem, ContractRequirement, MarketAnalysis, MarketPriceInfo, Ship,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    debug!("Starting SpaceTraders client");

    // Get agent details
    match get_agent_details().await {
        Ok(response) => {
            println!("\nAgent Details:");
            println!("==============");
            println!("Symbol: {}", response.data.symbol);
            println!("Credits: {}", response.data.credits);
            println!("Headquarters: {}", response.data.headquarters);
            println!("Ship Count: {}", response.data.ship_count);
        }
        Err(e) => println!("Error getting agent details: {}", e),
    }

    // Check contracts first
    println!("\nChecking Contracts:");
    println!("==================");
    match list_contracts().await {
        Ok(response) => {
            for contract in &response.data {
                println!("\nContract ID: {}", contract.id);
                println!("Type: {}", contract.contract_type);
                println!("Faction: {}", contract.faction_symbol);
                println!(
                    "Status: {}",
                    if contract.accepted {
                        "Accepted"
                    } else {
                        "Not Accepted"
                    }
                );
                println!("Fulfilled: {}", contract.fulfilled);
                println!("Deadline to Accept: {}", contract.deadline_to_accept);
                println!("Payment:");
                println!(
                    "  - On Accept: {} credits",
                    contract.terms.payment.on_accepted
                );
                println!(
                    "  - On Fulfill: {} credits",
                    contract.terms.payment.on_fulfilled
                );

                if let Some(deliveries) = &contract.terms.deliver {
                    println!("\nDelivery Requirements:");
                    for delivery in deliveries {
                        println!(
                            "  - {} units of {} to {}",
                            delivery.units_required,
                            delivery.trade_symbol,
                            delivery.destination_symbol
                        );
                        println!(
                            "    Progress: {}/{}",
                            delivery.units_fulfilled, delivery.units_required
                        );
                    }
                }

                if !contract.accepted {
                    println!("\nWould you like to accept this contract? (y/n)");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;

                    if input.trim().to_lowercase() == "y" {
                        match accept_contract(&contract.id).await {
                            Ok(accepted) => {
                                println!("Successfully accepted contract!");
                                println!(
                                    "Payment received: {} credits",
                                    accepted.data.contract.terms.payment.on_accepted
                                );
                                println!("New balance: {} credits", accepted.data.agent.credits);
                            }
                            Err(e) => println!("Error accepting contract: {}", e),
                        }
                    }
                }
            }
        }
        Err(e) => println!("Error listing contracts: {}", e),
    }

    // Then check shipyards
    let system_symbol = "X1-PS34";
    println!("\nChecking Shipyards in system {}:", system_symbol);
    println!("=================================");

    match find_shipyards(system_symbol).await {
        Ok(response) => {
            for waypoint in &response.data {
                println!("\nWaypoint: {}", waypoint.symbol);
                println!("Type: {}", waypoint.r#type);
                println!("Location: ({}, {})", waypoint.x, waypoint.y);

                match get_shipyard_details(system_symbol, &waypoint.symbol).await {
                    Ok(shipyard) => {
                        println!("\nAvailable ships at {}:", waypoint.symbol);
                        if let Some(ships) = shipyard.data.ships {
                            for ship in ships {
                                println!("\nShip type: {}", ship.r#type);
                                println!("Name: {}", ship.name);
                                println!("Price: {} credits", ship.purchase_price);
                                println!("Description: {}", ship.description);

                                println!("\nWould you like to purchase this ship? (y/n)");
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input)?;

                                if input.trim().to_lowercase() == "y" {
                                    match purchase_ship(&ship.r#type, &waypoint.symbol).await {
                                        Ok(purchase_response) => {
                                            println!("Successfully purchased ship!");
                                            println!("Ship details: {:?}", purchase_response.data);
                                        }
                                        Err(e) => println!("Error purchasing ship: {}", e),
                                    }
                                }
                            }
                        } else {
                            println!("No ships currently available at this shipyard");
                        }
                    }
                    Err(e) => println!("Error getting shipyard details: {}", e),
                }
            }
        }
        Err(e) => println!("Error finding shipyards: {}", e),
    }

    println!("\nChecking Ships and Asteroids:");
    println!("===========================");

    // List current ships
    match list_ships().await {
        Ok(response) => {
            info!("\nYour Ships:");
            for ship in &response.data {
                debug!("Processing ship: {}", ship.symbol);
                info!("\nShip Symbol: {}", ship.symbol);
                println!("Registration:");
                println!("  Name: {}", ship.registration.name);
                println!("  Role: {}", ship.registration.role);
                println!("  Faction: {}", ship.registration.faction_symbol);

                println!("\nNavigation:");
                println!("  Status: {}", ship.nav.status);
                println!("  System: {}", ship.nav.system_symbol);
                println!("  Waypoint: {}", ship.nav.waypoint_symbol);

                if let Some(route) = &ship.nav.route {
                    println!("\nRoute:");
                    if let Some(departure) = &route.departure {
                        println!(
                            "  From: {} ({}, {})",
                            departure.symbol, departure.x, departure.y
                        );
                    }
                    if let Some(destination) = &route.destination {
                        println!(
                            "  To: {} ({}, {})",
                            destination.symbol, destination.x, destination.y
                        );
                    }
                    if let Some(departure_time) = &route.departure_time {
                        println!("  Departure: {}", departure_time);
                    }
                    if let Some(arrival_time) = &route.arrival_time {
                        println!("  Arrival: {}", arrival_time);
                    }
                }

                println!("\nCargo:");
                println!("  {}/{} units", ship.cargo.units, ship.cargo.capacity);
                if !ship.cargo.inventory.is_empty() {
                    println!("  Inventory:");
                    for item in &ship.cargo.inventory {
                        println!("    - {} x{}", item.name, item.units);
                    }
                }

                println!("\nFuel:");
                println!("  {}/{} units", ship.fuel.current, ship.fuel.capacity);

                if ship.nav.status != "IN_ORBIT" {
                    println!("\nWould you like to put this ship into orbit? (y/n)");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;

                    if input.trim().to_lowercase() == "y" {
                        match orbit_ship(&ship.symbol).await {
                            Ok(orbit_response) => {
                                println!("Ship successfully entered orbit!");
                                println!("New status: {}", orbit_response.data.nav.status);
                            }
                            Err(e) => println!("Error entering orbit: {}", e),
                        }
                    }
                }

                // Handle mining operations
                handle_mining_operations(ship).await?;

                println!("\n----------");
            }
        }
        Err(e) => {
            error!("Error listing ships: {}", e);
            println!("Error listing ships: {}", e);
        }
    }

    // Find asteroids in the system
    let system_symbol = "X1-PS34"; // Your current system
    match find_asteroids(system_symbol).await {
        Ok(response) => {
            println!("\nEngineered Asteroids in system:");
            for asteroid in &response.data {
                println!("\nAsteroid: {}", asteroid.symbol);
                println!("Type: {}", asteroid.r#type);
                println!("Location: ({}, {})", asteroid.x, asteroid.y);
            }
        }
        Err(e) => println!("Error finding asteroids: {}", e),
    }

    Ok(())
}

async fn handle_cargo_sale(ship_symbol: &str, waypoint_symbol: &str) -> Result<()> {
    debug!(
        "Starting cargo sale for ship {} at waypoint {}",
        ship_symbol, waypoint_symbol
    );
    // First dock the ship
    match dock_ship(ship_symbol).await {
        Ok(_) => println!("Ship docked successfully."),
        Err(e) => {
            println!("Error docking ship: {}", e);
            return Ok(());
        }
    }

    // Get market data
    let system_symbol = waypoint_symbol
        .split('-')
        .take(2)
        .collect::<Vec<_>>()
        .join("-");
    match get_market_data(&system_symbol, waypoint_symbol).await {
        Ok(market_response) => {
            println!("\nMarket prices at {}:", waypoint_symbol);
            for good in &market_response.data.trade_goods {
                println!(
                    "{}: Buy {} / Sell {} (Volume: {})",
                    good.symbol, good.purchase_price, good.sell_price, good.trade_volume
                );
            }

            // Get ship's cargo
            match get_ship_cargo(ship_symbol).await {
                Ok(cargo_response) => {
                    println!(
                        "\nShip cargo ({}/{} units):",
                        cargo_response.data.units, cargo_response.data.capacity
                    );

                    for item in &cargo_response.data.inventory {
                        println!("{}: {} units", item.symbol, item.units);

                        println!("Would you like to sell this cargo? (y/n)");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input)?;

                        if input.trim().to_lowercase() == "y" {
                            match sell_cargo_item(ship_symbol, &item.symbol, item.units).await {
                                Ok(sell_response) => {
                                    println!(
                                        "Sold {} units of {} for {} credits",
                                        sell_response.data.transaction.units,
                                        sell_response.data.transaction.trade_symbol,
                                        sell_response.data.transaction.total_price
                                    );
                                    println!(
                                        "New credit balance: {}",
                                        sell_response.data.agent.credits
                                    );
                                }
                                Err(e) => println!("Error selling cargo: {}", e),
                            }
                        }
                    }
                }
                Err(e) => println!("Error getting cargo: {}", e),
            }
        }
        Err(e) => println!("Error getting market data: {}", e),
    }

    Ok(())
}

// Clean up the sell_selected_cargo function to show market prices
async fn sell_selected_cargo(ship_symbol: &str, inventory: &[CargoItem]) -> Result<()> {
    debug!("Starting selective cargo sale for ship {}", ship_symbol);

    // First get ship details to know current location
    let ships_response = list_ships().await?;
    let ship = ships_response
        .data
        .iter()
        .find(|s| s.symbol == ship_symbol)
        .ok_or_else(|| anyhow::anyhow!("Ship not found"))?;

    let system_symbol = &ship.nav.system_symbol;
    let current_waypoint = &ship.nav.waypoint_symbol;

    // Find nearest market
    if let Some(market_waypoint) = find_nearest_market(system_symbol, current_waypoint).await? {
        if market_waypoint.as_str() != current_waypoint {
            info!("Navigating to market at {}", market_waypoint);

            if ship.nav.status == "DOCKED" {
                orbit_ship(ship_symbol).await?;
            }

            navigate_ship(ship_symbol, &market_waypoint).await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }

        match dock_ship(ship_symbol).await {
            Ok(_) => {
                info!("Ship docked successfully at market");

                // Get market prices first
                if let Ok(market_prices) =
                    get_current_market_prices(system_symbol, &market_waypoint).await
                {
                    for item in inventory {
                        if let Some((_, price)) = market_prices
                            .iter()
                            .find(|(symbol, _)| symbol == &item.symbol)
                        {
                            println!("\nCargo item: {} ({} units)", item.name, item.units);
                            println!("Description: {}", item.description);
                            println!("Current market price: {} credits per unit", price);
                            println!("Potential value: {} credits", price * item.units);
                            println!("Would you like to sell this item? (y/n)");

                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input)?;

                            if input.trim().to_lowercase() == "y" {
                                println!(
                                    "How many units would you like to sell? (1-{}, or 'all')",
                                    item.units
                                );
                                let mut amount_input = String::new();
                                std::io::stdin().read_line(&mut amount_input)?;

                                let units_to_sell =
                                    match amount_input.trim().to_lowercase().as_str() {
                                        "all" => item.units,
                                        amount => {
                                            amount.parse::<i32>().unwrap_or(0).min(item.units)
                                        }
                                    };

                                if units_to_sell > 0 {
                                    info!(
                                        "Attempting to sell {} units of {}",
                                        units_to_sell, item.symbol
                                    );
                                    match sell_cargo_item(ship_symbol, &item.symbol, units_to_sell)
                                        .await
                                    {
                                        Ok(sell_response) => {
                                            println!(
                                                "Sold {} units of {} for {} credits",
                                                sell_response.data.transaction.units,
                                                sell_response.data.transaction.trade_symbol,
                                                sell_response.data.transaction.total_price
                                            );
                                            println!(
                                                "New credit balance: {}",
                                                sell_response.data.agent.credits
                                            );
                                        }
                                        Err(e) => {
                                            error!("Failed to sell {}: {}", item.symbol, e);
                                            println!("Error selling {}: {}", item.symbol, e);
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("\n{} cannot be sold at this market", item.name);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to dock ship: {}", e);
                println!("Error docking ship: {}", e);
            }
        }
    } else {
        println!("No suitable market found in the system");
    }

    Ok(())
}

async fn find_nearest_market(
    system_symbol: &str,
    current_waypoint: &str,
) -> Result<Option<String>> {
    // Get waypoint details to check for marketplace
    match get_waypoint_details(system_symbol, current_waypoint).await {
        Ok(response) => {
            let has_marketplace = response
                .data
                .traits
                .iter()
                .any(|trait_| trait_.symbol == "MARKETPLACE");

            if has_marketplace {
                return Ok(Some(current_waypoint.to_string()));
            }
        }
        Err(e) => {
            error!("Failed to get waypoint details: {}", e);
        }
    }

    // If current waypoint doesn't have a marketplace, look for nearest one
    // You would need to implement the logic to find other waypoints with markets
    // For now, we'll return None to indicate no market was found
    Ok(None)
}

async fn handle_mining_operations(ship: &Ship) -> Result<()> {
    debug!("Starting mining operations for ship: {}", ship.symbol);
    if ship.registration.role == "EXCAVATOR" {
        info!("\nMining Operations for {}", ship.symbol);
        info!("Current status: {}", ship.nav.status);

        // First handle any existing cargo
        if ship.cargo.units > 0 {
            println!("\nShip has cargo. Would you like to sell current cargo? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                // Check if the ship is docked
                if ship.nav.status != "DOCKED" {
                    println!("Ship needs to be docked to sell cargo.");
                    println!("Would you like to dock the ship? (y/n)");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;

                    if input.trim().to_lowercase() == "y" {
                        match dock_ship(&ship.symbol).await {
                            Ok(_) => {
                                println!("Ship docked successfully.");
                                // Pass the inventory to the function
                                sell_selected_cargo(&ship.symbol, &ship.cargo.inventory).await?;
                            }
                            Err(e) => {
                                error!("Failed to dock ship: {}", e);
                                println!("Error docking ship: {}", e);
                                return Ok(());
                            }
                        }
                    } else {
                        println!("Cannot sell cargo unless the ship is docked.");
                        return Ok(());
                    }
                } else {
                    // Pass the inventory when calling from docked state
                    sell_selected_cargo(&ship.symbol, &ship.cargo.inventory).await?;
                }
            }
        }

        // Rest of the mining operations code remains the same
        if ship.nav.status == "IN_ORBIT" {
            println!("\nWould you like to begin mining operations? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                // Proceed with mining operations
                // (orbit, navigate, extract sequence)
            }
        } else {
            println!("Ship must be in orbit to begin mining operations.");
        }
    }

    Ok(())
}

// current market prices - this will be helpful for selling decisions
async fn get_current_market_prices(
    system_symbol: &str,
    waypoint_symbol: &str,
) -> Result<Vec<(String, i32)>> {
    match get_market_data(system_symbol, waypoint_symbol).await {
        Ok(market_response) => {
            let prices: Vec<(String, i32)> = market_response
                .data
                .trade_goods
                .iter()
                .map(|good| (good.symbol.clone(), good.sell_price))
                .collect();
            Ok(prices)
        }
        Err(e) => {
            error!("Failed to get market prices: {}", e);
            Err(e)
        }
    }
}

async fn handle_ship_operations(ship: &Ship) -> Result<()> {
    debug!("Starting operations for ship: {}", ship.symbol);

    // Display ship info
    println!("\nShip Symbol: {}", ship.symbol);
    println!("Registration:");
    println!("  Name: {}", ship.registration.name);
    println!("  Role: {}", ship.registration.role);
    println!("  Faction: {}", ship.registration.faction_symbol);
    println!("\nNavigation:");
    println!("  Status: {}", ship.nav.status);
    println!("  System: {}", ship.nav.system_symbol);
    println!("  Waypoint: {}", ship.nav.waypoint_symbol);

    // Handle cargo management if there's cargo
    if ship.cargo.units > 0 {
        println!(
            "\nCurrent Cargo ({}/{} units):",
            ship.cargo.units, ship.cargo.capacity
        );
        for item in &ship.cargo.inventory {
            println!("  - {} x{}", item.name, item.units);
        }

        println!("\nWould you like to manage cargo (sell/transfer)? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            if ship.nav.status != "DOCKED" {
                println!("Ship needs to be docked to manage cargo.");
                println!("Would you like to dock the ship? (y/n)");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() == "y" {
                    match dock_ship(&ship.symbol).await {
                        Ok(_) => {
                            println!("Ship docked successfully.");
                            sell_selected_cargo(&ship.symbol, &ship.cargo.inventory).await?;
                        }
                        Err(e) => {
                            error!("Failed to dock ship: {}", e);
                            println!("Error docking ship: {}", e);
                            return Ok(());
                        }
                    }
                }
            } else {
                // If already docked, manage cargo
                sell_selected_cargo(&ship.symbol, &ship.cargo.inventory).await?;
            }
        }
    }

    Ok(())
}
