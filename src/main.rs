use spacetraders_client::api::{
    get_agent_details,
    get_waypoint_details,
    list_contracts,
    accept_contract,
    find_shipyards,
    get_shipyard_details,
    purchase_ship,
    list_ships,
    find_asteroids,
    orbit_ship,
    navigate_ship,
    extract_resources
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
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
                println!("Status: {}", if contract.accepted { "Accepted" } else { "Not Accepted" });
                println!("Fulfilled: {}", contract.fulfilled);
                println!("Deadline to Accept: {}", contract.deadline_to_accept);
                println!("Payment:");
                println!("  - On Accept: {} credits", contract.terms.payment.on_accepted);
                println!("  - On Fulfill: {} credits", contract.terms.payment.on_fulfilled);
                
                if let Some(deliveries) = &contract.terms.deliver {
                    println!("\nDelivery Requirements:");
                    for delivery in deliveries {
                        println!("  - {} units of {} to {}", 
                            delivery.units_required,
                            delivery.trade_symbol,
                            delivery.destination_symbol);
                        println!("    Progress: {}/{}", 
                            delivery.units_fulfilled,
                            delivery.units_required);
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
                                println!("Payment received: {} credits", 
                                    accepted.data.contract.terms.payment.on_accepted);
                                println!("New balance: {} credits", 
                                    accepted.data.agent.credits);
                            },
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
                                        },
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
            println!("\nYour Ships:");
            for ship in &response.data {
                println!("\nShip Symbol: {}", ship.symbol);
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
                        println!("  From: {} ({}, {})", 
                            departure.symbol, departure.x, departure.y);
                    }
                    if let Some(destination) = &route.destination {
                        println!("  To: {} ({}, {})", 
                            destination.symbol, destination.x, destination.y);
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
                            },
                            Err(e) => println!("Error entering orbit: {}", e),
                        }
                    }
                }
                
                if ship.registration.role == "EXCAVATOR" {
                    println!("\nMining Operations for {}", ship.symbol);
                    println!("Current status: {}", ship.nav.status);
                    
                    // If we're not at the asteroid yet
                    if ship.nav.waypoint_symbol != "X1-PS34-ED5C" {
                        println!("Ship needs to navigate to asteroid. Begin navigation? (y/n)");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input)?;
                        
                        if input.trim().to_lowercase() == "y" {
                            // Make sure we're in orbit first
                            if ship.nav.status != "IN_ORBIT" {
                                println!("Attempting to enter orbit...");
                                match orbit_ship(&ship.symbol).await {
                                    Ok(_) => {
                                        println!("Ship entered orbit successfully.");
                                        // Now try to navigate
                                        match navigate_ship(&ship.symbol, "X1-PS34-ED5C").await {
                                            Ok(nav_response) => {
                                                println!("Navigation started!");
                                                if let Some(route) = &nav_response.data.nav.route {
                                                    if let Some(arrival) = &route.arrival_time {
                                                        println!("Estimated arrival: {}", arrival);
                                                    }
                                                }
                                                println!("Navigation status: {}", nav_response.data.nav.status);
                                                println!("Fuel remaining: {}/{}", 
                                                    nav_response.data.fuel.current,
                                                    nav_response.data.fuel.capacity);
                                            },
                                            Err(e) => println!("Navigation error: {}", e),
                                        }
                                    },
                                    Err(e) => println!("Error entering orbit: {}", e),
                                }
                            } else {
                                // Already in orbit, just navigate
                                match navigate_ship(&ship.symbol, "X1-PS34-ED5C").await {
                                    Ok(nav_response) => {
                                        println!("Navigation started!");
                                        if let Some(route) = &nav_response.data.nav.route {
                                            if let Some(arrival) = &route.arrival_time {
                                                println!("Estimated arrival: {}", arrival);
                                            }
                                        }
                                        println!("Navigation status: {}", nav_response.data.nav.status);
                                        println!("Fuel remaining: {}/{}", 
                                            nav_response.data.fuel.current,
                                            nav_response.data.fuel.capacity);
                                    },
                                    Err(e) => println!("Navigation error: {}", e),
                                }
                            }
                        }
                    }
                    // We're at the asteroid
                    println!("At asteroid. Begin mining sequence? (y/n)");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    
                    if input.trim().to_lowercase() == "y" {
                        // Make sure we're in orbit
                        if ship.nav.status != "IN_ORBIT" {
                            match orbit_ship(&ship.symbol).await {
                                Ok(_) => println!("Ship entered orbit successfully."),
                                Err(e) => println!("Error entering orbit: {}", e),
                            }
                        }
                        
                        // Begin extraction
                        println!("Beginning extraction sequence...");
                        loop {
                            match extract_resources(&ship.symbol).await {
                                Ok(extract_response) => {
                                    println!("Extraction successful!");
                                    println!("Extracted {} units of {}", 
                                        extract_response.data.extraction.yield_data.units,
                                        extract_response.data.extraction.yield_data.symbol);
                                    println!("Cargo: {}/{} units", 
                                        extract_response.data.cargo.units,
                                        extract_response.data.cargo.capacity);
                                    println!("Cooldown: {} seconds", 
                                        extract_response.data.cooldown.remaining_seconds);
                                    
                                    if extract_response.data.cargo.units >= extract_response.data.cargo.capacity {
                                        println!("Cargo hold is full!");
                                        break;
                                    }
                                    
                                    println!("\nWaiting for cooldown to expire...");
                                    tokio::time::sleep(std::time::Duration::from_secs(
                                        extract_response.data.cooldown.remaining_seconds as u64 + 1
                                    )).await;
                                },
                                Err(e) => {
                                    println!("Extraction error: {}", e);
                                    println!("Would you like to try again? (y/n)");
                                    let mut input = String::new();
                                    std::io::stdin().read_line(&mut input)?;
                                    if input.trim().to_lowercase() != "y" {
                                        break;
                                    }
                                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                                },
                            }
                        }
                    }
                }
                println!("\n----------");
            }
        }
        Err(e) => println!("Error listing ships: {}", e),
    }
    
    // Find asteroids in the system
    let system_symbol = "X1-PS34";  // Your current system
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