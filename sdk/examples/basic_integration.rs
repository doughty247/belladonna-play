// Example: Basic Belladonna SDK Integration
// This demonstrates the core SDK functionality

use belladonna_sdk::{InitConfig, SdkHandle, EntitlementCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Belladonna Play SDK Example");
    println!("=============================");
    
    // Initialize the SDK with demo mode enabled
    println!("1. Initializing SDK...");
    let sdk = SdkHandle::init(InitConfig {
        auto_integrity: true,
        demo_mode: true,
    })?;
    println!("   SDK initialized successfully");
    println!("   Demo mode: {}", sdk.is_demo_mode());
    
    // Check integrity status
    println!("2. Checking integrity status...");
    if sdk.is_integrity_enabled() {
        println!("   Integrity monitoring active");
    } else {
        println!("   Integrity monitoring disabled");
    }
    
    // Check player entitlement
    println!("3. Checking player entitlement...");
    let player_id = "demo_player_123";
    let entitlement = sdk.check_entitlement(player_id);
    
    match entitlement.code {
        EntitlementCode::Ok => {
            println!("   Player '{}' is entitled to play", player_id);
        }
        EntitlementCode::NotEntitled => {
            println!("   Player '{}' is not entitled", player_id);
        }
        EntitlementCode::Error => {
            println!("   Entitlement check failed (requires runtime)");
        }
        other => {
            println!("   Entitlement status: {:?}", other);
        }
    }
    
    // Simulate game logic
    println!("4. Starting game simulation...");
    simulate_game_startup(&sdk)?;
    
    println!("5. Game completed successfully");
    println!("   Final integrity status: {}", 
             if sdk.is_integrity_enabled() { "Active" } else { "Inactive" });
    
    Ok(())
}

fn simulate_game_startup(sdk: &SdkHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Loading game assets...");
    
    // Simulate loading protected assets
    let protected_assets = vec![
        "game_logic.enc",
        "player_data.enc", 
        "anti_cheat_rules.enc"
    ];
    
    for asset in protected_assets {
        println!("      Loading: {}", asset);
        // In production, this would decrypt the asset using the runtime
        // For this example, we just simulate the process
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    println!("   All assets loaded successfully");
    
    // Simulate game running
    println!("   Running game loop...");
    for frame in 1..=5 {
        std::thread::sleep(std::time::Duration::from_millis(200));
        
        // Check integrity periodically (example)
        if frame % 3 == 0 && sdk.is_integrity_enabled() {
            println!("      Frame {}: Integrity check passed", frame);
        } else {
            println!("      Frame {}: Running normally", frame);
        }
    }
    
    println!("   🏁 Game loop completed");
    Ok(())
}

// Example of error handling
#[allow(dead_code)]
fn handle_production_scenario() {
    match SdkHandle::init(InitConfig { auto_integrity: true, demo_mode: false }) {
        Ok(sdk) => {
            println!("Production mode: Full DRM protection active");
            
            // In production, you'd check actual entitlements here
            let player = "real_player_id";
            let result = sdk.check_entitlement(player);
            
            if result.entitled {
                println!("Starting protected game for player: {}", player);
                // start_protected_game(sdk);
            } else {
                match result.code {
                    EntitlementCode::NotEntitled => {
                        println!("Showing purchase dialog...");
                        // show_purchase_dialog();
                    }
                    EntitlementCode::Expired => {
                        println!("License expired - showing renewal dialog...");
                        // show_license_renewal_dialog();
                    }
                    _ => {
                        println!("Entitlement check failed - showing error...");
                        // show_error_dialog();
                    }
                }
            }
        }
        Err(e) => {
            println!("Development mode: Running with SDK stubs");
            println!("Error: {:?}", e);
            // start_development_game();
        }
    }
}