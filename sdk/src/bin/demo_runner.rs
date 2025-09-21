// Demo Runner - Quick test of SDK functionality
// Run with: cargo run --bin demo_runner

use belladonna_sdk::{InitConfig, SdkHandle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Belladonna Play SDK - Quick Demo");
    println!("================================");
    
    let sdk = SdkHandle::init(InitConfig {
        auto_integrity: true,
        demo_mode: true,
    })?;
    
    println!("✓ SDK initialized (demo_mode={})", sdk.is_demo_mode());
    println!("✓ Integrity enabled: {}", sdk.is_integrity_enabled());
    
    // Test entitlement checks
    let test_users = ["demo_player_123", "unknown_user", "expired_user"];
    for user in &test_users {
        let result = sdk.check_entitlement(user);
        println!("User '{}': entitled={}", user, result.entitled);
    }
    
    // Add a new demo user
    sdk.add_demo_user("new_player", true)?;
    let result = sdk.check_entitlement("new_player");
    println!("New user 'new_player': entitled={}", result.entitled);
    
    println!("\n✓ Demo completed - Run './run_demo.sh' for interactive demo");
    
    Ok(())
}