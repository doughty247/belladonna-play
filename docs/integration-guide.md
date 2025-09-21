# Belladonna Play SDK Integration Guide

This guide walks you through integrating Belladonna Play DRM and anti-cheat functionality into your game or application.

## Table of Contents

1. [Overview](#overview)
2. [Setup](#setup)
3. [Basic Integration](#basic-integration)
4. [Advanced Features](#advanced-features)
5. [Platform-Specific Notes](#platform-specific-notes)
6. [Troubleshooting](#troubleshooting)

## Overview

Belladonna Play provides a two-layer architecture:

- **SDK Layer (Open Source)** - Integration interfaces and example code
- **Runtime Layer (Commercial)** - Actual DRM and anti-cheat implementation

This guide covers SDK integration. Runtime deployment requires a separate commercial license.

## Setup

### Option 1: Rust Integration

Add to your `Cargo.toml`:

```toml
[dependencies]
belladonna-sdk = { git = "https://github.com/doughty247/belladonna-play", branch = "main" }
```

### Option 2: C/C++ Integration  

1. Download the SDK headers:
   - `belladonna_sdk.h`

2. Link against the SDK library:
   - `libbelladonna_sdk.so` (Linux)
   - `belladonna_sdk.dll` (Windows)
   - `libbelladonna_sdk.dylib` (macOS)

### Option 3: Godot Integration

1. Copy the GDExtension files to your project
2. Add to your `extension_list.cfg`:

```ini
[configuration]
entry_symbol = "belladonna_gdext_init"

[libraries]
linux.debug.x86_64 = "res://addons/belladonna-play/libbelladonna_gdext.so"
linux.release.x86_64 = "res://addons/belladonna-play/libbelladonna_gdext.so"
```

## Basic Integration

### 1. Initialize the SDK

```rust
use belladonna_sdk::{InitConfig, SdkHandle, SdkError};

fn init_belladonna() -> Result<SdkHandle, SdkError> {
    let config = InitConfig {
        auto_integrity: true, // Enable anti-cheat automatically
    };
    
    SdkHandle::init(config)
}
```

### 2. Check User Entitlement

```rust
fn check_player_license(sdk: &SdkHandle, player_id: &str) -> bool {
    let result = sdk.check_entitlement(player_id);
    
    match result.code {
        EntitlementCode::Ok => {
            println!("Player {} is entitled", player_id);
            true
        }
        EntitlementCode::NotEntitled => {
            println!("Player {} does not own the game", player_id);
            false
        }
        EntitlementCode::Expired => {
            println!("Player {} license has expired", player_id);
            false
        }
        _ => {
            println!("Entitlement check failed: {:?}", result.code);
            false
        }
    }
}
```

### 3. Handle Asset Protection

```rust
// Note: This is example code - actual asset decryption requires the runtime
fn load_protected_asset(asset_path: &str) -> Option<Vec<u8>> {
    // In production, this would decrypt the asset using the runtime
    println!("Loading protected asset: {}", asset_path);
    
    // Stub implementation - load unprotected version for development
    std::fs::read(asset_path).ok()
}
```

## Advanced Features

### Custom Error Handling

```rust
use belladonna_sdk::{SdkError, EntitlementCode};

fn handle_belladonna_error(error: SdkError) {
    match error {
        SdkError::Init(msg) => {
            eprintln!("SDK initialization failed: {}", msg);
            // Fallback to offline mode
        }
        SdkError::Integrity(msg) => {
            eprintln!("Integrity monitoring failed: {}", msg);
            // Continue with reduced security
        }
        SdkError::Entitlement(msg) => {
            eprintln!("Entitlement check failed: {}", msg);
            // Show demo mode or purchase dialog
        }
        SdkError::RuntimeNotAvailable => {
            println!("Running in development mode - runtime not available");
            // Continue with stub implementations
        }
    }
}
```

### Metrics and Monitoring

```rust
fn print_sdk_status(sdk: &SdkHandle) {
    if sdk.is_integrity_enabled() {
        println!("✅ Anti-cheat protection active");
    } else {
        println!("⚠️  Running without anti-cheat protection");
    }
    
    // In production, you would export metrics to your monitoring system
}
```

### Graceful Degradation

```rust
fn start_game_with_protection(player_id: &str) {
    match init_belladonna() {
        Ok(sdk) => {
            // Full protection mode
            if check_player_license(&sdk, player_id) {
                start_protected_game(sdk);
            } else {
                show_purchase_dialog();
            }
        }
        Err(SdkError::RuntimeNotAvailable) => {
            // Development/demo mode
            println!("Starting in development mode");
            start_unprotected_game();
        }
        Err(e) => {
            handle_belladonna_error(e);
            // Decide whether to continue or exit
        }
    }
}
```

## Platform-Specific Notes

### Linux

- Requires glibc ≥ 2.31
- Kernel ≥ 5.10 recommended for full feature support
- May require additional permissions for anti-cheat features

### Windows  

- Windows 10/11 supported
- User-mode implementation (no kernel drivers)
- Compatible with Windows Defender and other security software

### macOS

- macOS 10.15+ supported  
- Code signing requirements for production deployment
- Gatekeeper compatibility ensured

### Steam Deck / SteamOS

- Full compatibility with Steam Deck
- Proton/Wine support for Windows games
- Controller-friendly error dialogs

## C/C++ Example

```cpp
#include "belladonna_sdk.h"
#include <stdio.h>

int main() {
    bd_handle* sdk = NULL;
    
    // Initialize SDK
    int result = bd_init(&sdk, 1); // auto_integrity = true
    if (result != 0) {
        printf("Failed to initialize Belladonna SDK: %d\\n", result);
        return 1;
    }
    
    // Check entitlement
    bd_entitlement_result ent_result;
    result = bd_check_entitlement(sdk, "player123", &ent_result);
    
    if (result == 0 && ent_result.entitled) {
        printf("Player is entitled - starting game\\n");
        // start_game();
    } else {
        printf("Player is not entitled\\n");
        // show_demo_or_purchase();
    }
    
    // Get status
    bd_status_out status;
    bd_status(sdk, &status);
    printf("Integrity enabled: %s\\n", status.integrity_enabled ? "Yes" : "No");
    
    // Export metrics (example)
    char metrics_json[1024];
    uint32_t written;
    result = bd_export_metrics(metrics_json, sizeof(metrics_json), &written);
    if (result == 0) {
        printf("Metrics: %s\\n", metrics_json);
    }
    
    // Clean shutdown
    bd_shutdown(sdk);
    return 0;
}
```

## Godot Example

```gdscript
extends Node

var belladonna_runtime: BelladonnaRuntime
var belladonna_loader: BelladonnaAssetLoader

func _ready():
    belladonna_runtime = BelladonnaRuntime.new()
    belladonna_loader = BelladonnaAssetLoader.new()
    
    print("Belladonna SDK Version: ", belladonna_runtime.version())
    
    # Verify game assets
    if verify_game_assets():
        print("All assets verified - game ready to start")
        start_game()
    else:
        print("Asset verification failed - entering safe mode")
        start_safe_mode()

func verify_game_assets() -> bool:
    # Check main asset manifest
    if not belladonna_runtime.verify_manifest("res://assets/manifest.json"):
        print("Main manifest verification failed")
        return false
    
    # Check critical game assets
    var critical_assets = [
        "res://assets/game_logic.enc",
        "res://assets/player_data.enc"
    ]
    
    for asset_path in critical_assets:
        if belladonna_loader.is_asset_encrypted(asset_path):
            print("Critical asset is encrypted: ", asset_path)
            # In production, would decrypt here
        else:
            print("Warning: Critical asset not encrypted: ", asset_path)
    
    return true

func start_game():
    # Enable asset protection
    if belladonna_runtime.enable_asset_gate():
        print("Asset protection enabled")
    else:
        print("Running without asset protection: ", belladonna_runtime.get_last_error())
    
    # Continue with normal game startup
    get_tree().change_scene_to_file("res://scenes/MainMenu.tscn")

func start_safe_mode():
    # Limited functionality without full protection
    get_tree().change_scene_to_file("res://scenes/SafeMode.tscn")
```

## Troubleshooting

### Common Issues

**SDK Initialization Fails**
- Check that your platform is supported
- Verify file permissions for SDK libraries
- Ensure no conflicting security software

**Entitlement Check Always Fails**
- Verify you're using the development/stub mode correctly
- Check network connectivity (production mode)
- Validate user ID format

**Performance Issues**
- Disable debug logging in production
- Consider integrity monitoring overhead
- Profile asset decryption performance

### Debug Mode

Enable detailed logging:

```rust
// Set environment variable
std::env::set_var("BELLADONNA_LOG_LEVEL", "debug");

// Initialize with debug info
let sdk = SdkHandle::init(InitConfig { auto_integrity: true })?;
```

### Getting Help

1. **Check the FAQ** - Common issues and solutions
2. **GitHub Issues** - Report bugs in the SDK interface  
3. **Documentation** - API reference and examples
4. **Commercial Support** - Priority support with runtime license

## Next Steps

1. **Implement Basic Integration** - Start with entitlement checking
2. **Add Asset Protection** - Protect critical game assets
3. **Enable Anti-Cheat** - Add integrity monitoring
4. **Test Thoroughly** - Verify all codepaths work correctly
5. **Deploy with Runtime** - Contact for commercial runtime license

Remember: This SDK provides interfaces only. Production deployment requires the Belladonna Play runtime library.