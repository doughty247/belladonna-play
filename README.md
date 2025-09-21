# Belladonna Play SDK

Open-source SDK interface for integrating Belladonna Play DRM and anti-cheat functionality into games and interactive applications.

## Overview

This repository contains the **open-source SDK interface** for Belladonna Play. It provides the public APIs, integration examples, and documentation that game developers use to integrate with Belladonna Play, but the actual DRM and anti-cheat functionality requires the separate closed-source Belladonna Play runtime library.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Your Game/Engine                         │
├─────────────────────────────────────────────────────────────┤
│              Belladonna Play SDK (Open Source)             │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐  │
│  │   Rust SDK API  │  │   C/C++ API     │  │   Godot     │  │
│  │                 │  │                 │  │  Extension  │  │
│  └─────────────────┘  └─────────────────┘  └─────────────┘  │
├─────────────────────────────────────────────────────────────┤
│            Belladonna Play Runtime (Closed Source)         │
│        ┌─────────────┐  ┌─────────────────────────────┐     │
│        │     DRM     │  │        Anti-Cheat           │     │
│        │ Components  │  │       Components            │     │
│        └─────────────┘  └─────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

## Repository Contents

- **`sdk/`** - Core Rust SDK with C ABI bindings
- **`godot-extension/`** - Godot engine integration examples  
- **`examples/`** - Integration examples and sample code
- **`docs/`** - Detailed documentation and guides

## Quick Start

### Rust Integration

```rust
use belladonna_sdk::{InitConfig, SdkHandle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the SDK
    let sdk = SdkHandle::init(InitConfig { 
        auto_integrity: true 
    })?;
    
    // Check user entitlement
    let result = sdk.check_entitlement("player123");
    if result.entitled {
        println!("Player is entitled to play");
    } else {
        println!("Player is not entitled: {:?}", result.code);
    }
    
    // Enable integrity monitoring
    sdk.enable_integrity()?;
    
    Ok(())
}
```

### C/C++ Integration

```cpp
#include "belladonna_sdk.h"

int main() {
    bd_handle* handle;
    
    // Initialize SDK with integrity monitoring
    if (bd_init(&handle, 1) != 0) {
        return 1;
    }
    
    // Check entitlement
    bd_entitlement_result result;
    int status = bd_check_entitlement(handle, "player123", &result);
    
    if (status == 0 && result.entitled) {
        printf("Player is entitled\\n");
    } else {
        printf("Player is not entitled\\n");
    }
    
    // Clean shutdown
    bd_shutdown(handle);
    return 0;
}
```

### Godot Integration

```gdscript
extends Node

func _ready():
    var belladonna = BelladonnaRuntime.new()
    
    # Check SDK version
    print("Belladonna SDK Version: ", belladonna.version())
    
    # Verify game assets
    if belladonna.verify_manifest("res://assets/manifest.json"):
        print("Asset manifest verified successfully")
    else:
        print("Asset verification failed: ", belladonna.get_last_error())
```

## Features

### SDK Interface (Open Source)
- ✅ **High-level API** - Clean, ergonomic interfaces for common operations
- ✅ **Cross-language bindings** - C ABI for integration with any language
- ✅ **Godot integration** - Native GDScript bindings and examples
- ✅ **Example implementations** - Cryptographic verification examples
- ✅ **Documentation** - Comprehensive guides and API documentation

### Runtime Features (Requires Commercial License)
- 🔒 **DRM Protection** - Asset encryption and entitlement verification
- 🔒 **Anti-Cheat** - Process integrity monitoring and threat detection
- 🔒 **Asset Streaming** - Secure asset loading with signature verification
- 🔒 **Mod Verification** - Cryptographic signing for approved modifications
- 🔒 **Analytics** - Detailed security metrics and observability

## Integration Patterns

### 1. Basic DRM Integration

```rust
// Initialize with DRM only
let sdk = SdkHandle::init(InitConfig { auto_integrity: false })?;

// Check if player owns the game
match sdk.check_entitlement(&player_id) {
    EntitlementResult { entitled: true, .. } => {
        // Player owns the game - proceed
        start_game();
    }
    EntitlementResult { code: EntitlementCode::Expired, .. } => {
        // License expired
        show_purchase_dialog();
    }
    _ => {
        // Not entitled or other error
        show_demo_mode();
    }
}
```

### 2. Anti-Cheat Integration  

```rust
// Enable integrity monitoring
let sdk = SdkHandle::init(InitConfig { auto_integrity: true })?;

// Check integrity status
if sdk.is_integrity_enabled() {
    println!("Anti-cheat protection active");
} else {
    println!("Running in offline mode");
}
```

### 3. Asset Protection

```c
// Load protected game asset
char* asset_data;
size_t asset_size;

int result = bd_decrypt_asset_to_memory(
    "assets/level1.enc", 
    &asset_data, 
    &asset_size
);

if (result == BD_OK) {
    // Use decrypted asset data
    load_level_data(asset_data, asset_size);
    
    // Always free encrypted asset memory
    bd_free_memory(asset_data);
}
```

## Building

### Prerequisites

- Rust 1.70+ with Cargo
- C/C++ compiler (for C bindings)
- Godot 4.0+ (for Godot extension)

### Build SDK

```bash
cd sdk/
cargo build --release
```

### Build with C ABI

```bash
cd sdk/
cargo build --release --features ffi
```

### Build Godot Extension

```bash
cd godot-extension/
cargo build --release --features godot-bindings
```

## Runtime Requirements

This SDK interface alone provides **example implementations** and **stub functionality** only. For production DRM and anti-cheat functionality, you need:

1. **Belladonna Play Runtime License** - Commercial license for the closed-source runtime
2. **Runtime Library** - The `belladonna-play` runtime library for your target platform
3. **Integration Support** - Professional integration assistance available

## Documentation

- **[Integration Guide](docs/integration-guide.md)** - Step-by-step integration walkthrough
- **[API Reference](docs/api-reference.md)** - Complete API documentation  
- **[Examples](examples/)** - Working integration examples
- **[FAQ](docs/faq.md)** - Common questions and troubleshooting

## Security Model

### Open Source Boundaries

This SDK provides:
- ✅ Public API interfaces and type definitions
- ✅ Example cryptographic verification code
- ✅ Integration patterns and documentation
- ✅ Stub implementations for development/testing

This SDK does **NOT** provide:
- ❌ Actual DRM protection mechanisms
- ❌ Anti-cheat detection algorithms  
- ❌ Production cryptographic keys
- ❌ Bypass/circumvention protection

### Security Through Obscurity

The Belladonna Play security model follows industry best practices:

- **Open interfaces** - APIs and integration patterns are transparent
- **Closed implementation** - Security-critical algorithms remain proprietary
- **Verified builds** - Runtime binaries are cryptographically signed
- **Key isolation** - Production keys never appear in open-source code

## Support

### Community Support (Open Source)
- **GitHub Issues** - Bug reports and feature requests for SDK interface
- **Documentation** - Comprehensive guides and examples
- **Community Forums** - Developer discussions and integration help

### Commercial Support (Runtime)
- **Professional Integration** - Dedicated integration engineering support
- **Priority Bug Fixes** - Expedited resolution for runtime issues
- **Custom Development** - Tailored solutions for specific requirements
- **24/7 Monitoring** - Production security monitoring and response

## Licensing

### SDK License (MIT)
The SDK interface, examples, and documentation in this repository are licensed under the MIT License.

### Runtime License (Commercial)
The Belladonna Play runtime library requires a separate commercial license. Contact the maintainers for licensing information.

## Getting Started

1. **Try the Examples** - Start with the integration examples
2. **Read the Documentation** - Review the integration guide
3. **Build and Test** - Integrate the SDK interface in development mode
4. **Contact for Runtime** - Reach out for commercial runtime licensing
5. **Deploy with Confidence** - Launch with full DRM and anti-cheat protection

---

For runtime licensing and commercial support, please contact the maintainers through this repository.