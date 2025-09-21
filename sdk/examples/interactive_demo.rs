// Interactive Demo System for Belladonna Play SDK
// This demonstrates the SDK's demo capabilities and integration patterns

use belladonna_sdk::{InitConfig, SdkHandle, EntitlementCode, SdkError};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("====================================");
    println!("  Belladonna Play SDK Demo System  ");
    println!("====================================");
    println!();
    println!("This interactive demo showcases SDK integration patterns");
    println!("and simulates real DRM/anti-cheat functionality.");
    println!();
    
    // Initialize SDK in demo mode
    println!("Initializing SDK in demo mode...");
    let sdk = SdkHandle::init(InitConfig {
        auto_integrity: true,
        demo_mode: true,
    })?;
    
    println!("SDK initialized successfully!");
    println!("Demo mode: {}", sdk.is_demo_mode());
    println!("Integrity monitoring: {}", sdk.is_integrity_enabled());
    println!();
    
    loop {
        print_menu();
        print!("Select option (1-7): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => demo_entitlement_check(&sdk)?,
            "2" => demo_integrity_simulation(&sdk)?,
            "3" => demo_threat_simulation(&sdk)?,
            "4" => demo_user_management(&sdk)?,
            "5" => demo_asset_verification(),
            "6" => show_integration_examples(),
            "7" => {
                println!("Exiting demo...");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
        
        println!();
        println!("Press Enter to continue...");
        let mut _input = String::new();
        io::stdin().read_line(&mut _input)?;
        println!();
    }
    
    Ok(())
}

fn print_menu() {
    println!("╔═══════════════════════════════════════╗");
    println!("║           DEMO MENU OPTIONS           ║");
    println!("╠═══════════════════════════════════════╣");
    println!("║ 1. Test Entitlement Checking         ║");
    println!("║ 2. Integrity Monitoring Demo         ║");
    println!("║ 3. Threat Detection Simulation       ║");
    println!("║ 4. User Management                   ║");
    println!("║ 5. Asset Verification                ║");
    println!("║ 6. Integration Examples               ║");
    println!("║ 7. Exit                              ║");
    println!("╚═══════════════════════════════════════╝");
}

fn demo_entitlement_check(sdk: &SdkHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Entitlement Checking Demo ===");
    println!();
    
    let test_users = vec![
        "demo_player_123",
        "valid_user", 
        "expired_user",
        "unknown_player",
        "test_player"
    ];
    
    for user in test_users {
        println!("Checking entitlement for: {}", user);
        let result = sdk.check_entitlement(user);
        
        match result.code {
            EntitlementCode::Ok => {
                println!("  ✓ User '{}' is ENTITLED - Access granted", user);
            }
            EntitlementCode::NotEntitled => {
                println!("  ✗ User '{}' is NOT ENTITLED - Access denied", user);
            }
            EntitlementCode::Expired => {
                println!("  ⚠ User '{}' has EXPIRED license - Renewal required", user);
            }
            _ => {
                println!("  ? User '{}' - Status unknown (requires runtime)", user);
            }
        }
        println!();
    }
    
    Ok(())
}

fn demo_integrity_simulation(sdk: &SdkHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Integrity Monitoring Demo ===");
    println!();
    
    println!("Current integrity status: {}", 
             if sdk.is_integrity_enabled() { "ACTIVE" } else { "DISABLED" });
    
    if sdk.is_integrity_enabled() {
        println!("Simulating integrity monitoring cycle...");
        
        // Simulate monitoring phases
        let phases = vec![
            "Memory region scan - checking for modifications",
            "Process enumeration - scanning for suspicious tools", 
            "Debugger detection - timing analysis complete",
            "Module verification - validating loaded libraries",
            "Stack analysis - checking for injection attempts",
        ];
        
        for (i, phase) in phases.iter().enumerate() {
            println!("  [{}/{}] {}", i + 1, phases.len(), phase);
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        
        println!("✓ Integrity monitoring cycle completed - No threats detected");
    } else {
        println!("Integrity monitoring is disabled");
        println!("Use sdk.enable_integrity() to activate protection");
    }
    
    Ok(())
}

fn demo_threat_simulation(sdk: &SdkHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Threat Detection Simulation ===");
    println!();
    
    let threats = vec![
        ("debugger", "Simulating debugger attachment"),
        ("memory_scan", "Simulating memory scanner activity"),
        ("injection", "Simulating code injection attempt"),
        ("unknown_threat", "Simulating unknown threat pattern"),
    ];
    
    for (threat_type, description) in threats {
        println!("{}", description);
        match sdk.simulate_threat(threat_type) {
            Ok(response) => println!("  Response: {}", response.replace("Demo: ", "")),
            Err(e) => println!("  Error: {}", e),
        }
        println!();
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
    
    Ok(())
}

fn demo_user_management(sdk: &SdkHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== User Management Demo ===");
    println!();
    
    print!("Enter username to add: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim();
    
    if username.is_empty() {
        println!("No username provided");
        return Ok(());
    }
    
    print!("Grant entitlement? (y/n): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let entitled = input.trim().to_lowercase() == "y";
    
    sdk.add_demo_user(username, entitled)?;
    
    // Test the new user
    println!("Testing new user...");
    let result = sdk.check_entitlement(username);
    println!("Entitlement check result: entitled={}, code={:?}", 
             result.entitled, result.code);
    
    Ok(())
}

fn demo_asset_verification() {
    println!("=== Asset Verification Demo ===");
    println!();
    
    let demo_assets = vec![
        ("game_data.pak", "Valid", true),
        ("level_01.enc", "Valid", true),
        ("modified_asset.dat", "Tampered", false),
        ("missing_signature.bin", "Unsigned", false),
        ("corrupt_data.enc", "Corrupted", false),
    ];
    
    println!("Verifying demo assets...");
    println!();
    
    for (asset, status, valid) in demo_assets {
        print!("Checking {}: ", asset);
        std::thread::sleep(std::time::Duration::from_millis(200));
        
        if valid {
            println!("✓ {} - Signature valid", status);
        } else {
            println!("✗ {} - Verification failed", status);
        }
    }
    
    println!();
    println!("Asset verification demonstrates cryptographic validation");
    println!("that would be performed by the runtime library.");
}

fn show_integration_examples() {
    println!("=== Integration Examples ===");
    println!();
    
    println!("1. Basic Rust Integration:");
    println!("```rust");
    println!("let sdk = SdkHandle::init(InitConfig {{");
    println!("    auto_integrity: true,");
    println!("    demo_mode: true,");
    println!("}}).unwrap();");
    println!("let result = sdk.check_entitlement(\"user123\");");
    println!("```");
    println!();
    
    println!("2. C/C++ Integration:");
    println!("```cpp");
    println!("#include \"belladonna_sdk.h\"");
    println!("bd_handle* handle;");
    println!("bd_init_with_demo(&handle, 1, 1);");
    println!("bd_entitlement_result result;");
    println!("bd_check_entitlement(handle, \"user123\", &result);");
    println!("```");
    println!();
    
    println!("3. Godot Integration:");
    println!("```gdscript");
    println!("var belladonna = BelladonnaRuntime.new()");
    println!("belladonna.init_demo_mode(true)");
    println!("var entitled = belladonna.check_entitlement(\"player1\")");
    println!("```");
    println!();
    
    println!("These examples show how to integrate the SDK across");
    println!("different programming languages and engines.");
}