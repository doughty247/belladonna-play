# Belladonna Play SDK

Open-source SDK interface for integrating Belladonna Play DRM and anti-cheat functionality into games and interactive applications. Includes a fully functional demo system for testing and integration validation.

## Overview

This repository contains the **open-source SDK interface** for Belladonna Play. It provides the public APIs, integration examples, and documentation that game developers use to integrate with Belladonna Play, but the actual DRM and anti-cheat functionality requires the separate closed-source Belladonna Play runtime library.

**Platform Support:** Currently supports **Linux** with Windows support highly prioritized for future releases. Our Hardware Abstraction Layer (HAL) architecture enables rapid cross-platform deployment.

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
    // Initialize the SDK with demo mode
    let sdk = SdkHandle::init(InitConfig { 
        auto_integrity: true,
        demo_mode: true,
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
    
    // Initialize SDK with integrity monitoring and demo mode
    if (bd_init_with_demo(&handle, 1, 1) != 0) {
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
- **High-level API** - Clean, ergonomic interfaces for common operations
- **Cross-language bindings** - C ABI for integration with any language
- **Godot integration** - Native GDScript bindings and examples
- **Example implementations** - Cryptographic verification examples
- **Documentation** - Comprehensive guides and API documentation
- **Interactive Demo System** - Fully functional demo mode with realistic entitlement simulation
- **Demo Runner** - Easy-to-use script for testing integration patterns

### Runtime Features (Requires Commercial License)
- **DRM Protection** - Asset encryption and entitlement verification
- **Anti-Cheat** - Process integrity monitoring and threat detection
- **Asset Streaming** - Secure asset loading with signature verification
- **Mod Verification** - Cryptographic signing for approved modifications
- **Analytics** - Detailed security metrics and observability

## Integration Patterns

### 1. Basic DRM Integration

```rust
// Initialize with DRM and demo mode
let sdk = SdkHandle::init(InitConfig { 
    auto_integrity: false,
    demo_mode: true,
})?;

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
// Enable integrity monitoring with demo
let sdk = SdkHandle::init(InitConfig { 
    auto_integrity: true,
    demo_mode: true,
})?;

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

## Demo System

The SDK includes a comprehensive demo system that provides realistic functionality without requiring a commercial license:

### Demo Features
- **Interactive Demo Runner** - Menu-driven exploration of SDK capabilities
- **Entitlement Simulation** - Realistic user validation with predefined test accounts
- **Integrity Monitoring** - Simulated threat detection and response patterns
- **Asset Verification** - Cryptographic verification examples
- **User Management** - Dynamic user creation and entitlement testing
- **Integration Examples** - Live demonstrations across Rust, C/C++, and Godot

### Running Demos

```bash
# Interactive demo system
./run_demo.sh

# Or run specific examples
cargo run --example interactive_demo
cargo run --example basic_integration
```

### Demo Users (Pre-configured)
- `demo_player_123` - Entitled user
- `valid_user` - Entitled user  
- `expired_user` - Expired license
- `test_player` - Entitled user

## Building

### Prerequisites

**Supported Platforms:**
- **Linux** (x86_64) - Full SDK and runtime support
- **Windows** - SDK development support only (runtime planned)
- **macOS** - SDK development support only (runtime planned)

**Development Requirements:**
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

This SDK interface provides **demo functionality** for development and testing. For production DRM and anti-cheat functionality, you need:

1. **Belladonna Play Runtime License** - Commercial license for the closed-source runtime
2. **Runtime Library** - The `belladonna-play` runtime library (currently Linux x86_64 only)
3. **Platform Compatibility** - Linux deployment environment for production use
4. **Integration Support** - Professional integration assistance available

**Note:** Windows runtime support is highly prioritized and enabled by our HAL architecture for rapid deployment once available.

## Documentation

- **[Integration Guide](docs/integration-guide.md)** - Step-by-step integration walkthrough
- **[API Reference](docs/api-reference.md)** - Complete API documentation  
- **[Examples](examples/)** - Working integration examples
- **[FAQ](docs/faq.md)** - Common questions and troubleshooting

## Security Model

### Open Source Boundaries

This SDK provides:
- Public API interfaces and type definitions
- Example cryptographic verification code
- Integration patterns and documentation
- Realistic demo implementations with simulated functionality
- Interactive demo system for integration validation and testing

This SDK does **NOT** provide:
- Actual DRM protection mechanisms
- Anti-cheat detection algorithms  
- Production cryptographic keys
- Bypass/circumvention protection

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

## FAQ

### Platform Support

**Q: What platforms are currently supported?**
A: Belladonna Play currently supports **Linux** (x86_64). The SDK interface compiles and runs on Linux systems, and the commercial runtime library is available for Linux production deployments.

**Q: When will Windows support be available?**
A: Windows support is **highly prioritized** and planned for the next major release. Our Hardware Abstraction Layer (HAL) architecture is specifically designed to enable rapid cross-platform deployment, making Windows implementation straightforward once development resources are allocated.

**Q: Will macOS be supported?**
A: macOS support is possible in the future as our HAL architecture is designed for cross-platform deployment. However, due to lack of development hardware, macOS support is not possible at this time. Windows support takes priority given available development resources.

**Q: Can I develop on Windows and deploy on Linux?**
A: Yes! The SDK interface can be used for development and testing on any platform where Rust runs. You can develop your integration on Windows using demo mode, then deploy to Linux for production with the runtime library.

### Demo System

**Q: What can I do with the demo system without a commercial license?**
A: The demo system provides comprehensive functionality including:
- Realistic entitlement simulation with pre-configured test users
- Interactive threat detection demonstrations
- Asset verification examples
- User management and testing capabilities
- Integration pattern validation across multiple languages

**Q: How realistic is the demo system compared to the production runtime?**
A: The demo system accurately simulates the behavior patterns, API responses, and integration requirements of the production runtime. While it doesn't provide actual DRM protection, it gives developers a complete understanding of how their integration will work in production.

**Q: Can I ship games with just the demo system?**
A: No, the demo system is for development and testing only. Production DRM and anti-cheat functionality requires the commercial Belladonna Play runtime library.

### Integration

**Q: What programming languages are supported?**
A: Belladonna Play provides native integration for:
- **Rust** - Direct SDK integration
- **C/C++** - Through our C ABI layer
- **Godot** - Native GDScript bindings and examples
- **Other languages** - Any language that can interface with C libraries

**Q: How long does integration typically take?**
A: Basic integration can be completed in a few hours to a day using our examples and documentation. Complex custom integrations may take several days depending on specific requirements.

**Q: Do you provide integration support?**
A: Yes! We offer comprehensive integration support including documentation, examples, community forums for the open-source SDK, and professional integration services for commercial runtime deployments.

**Q: What game engines are supported?**
A: Current and planned engine support:
- **Godot** - Native GDScript bindings and examples (available now)
- **Unity** - Planned next priority with C# integration layer
- **Unreal Engine** - Planned following Unity support
- **Custom Engines** - Full support through C/C++ API integration

Engine support priorities may change based on community feedback and demand.

**Q: How do I integrate Belladonna Play into a custom engine?**
A: Custom engine integration is fully supported and typically involves:

1. **Link the C Library** - Include `libbelladonna_sdk.so` and `belladonna_sdk.h`
2. **Initialize at Engine Startup** - Call `bd_init_with_demo()` during engine initialization
3. **Hook Asset Loading** - Integrate `bd_verify_manifest()` into your asset pipeline
4. **Add Entitlement Checks** - Call `bd_check_entitlement()` at appropriate game entry points
5. **Enable Integrity Monitoring** - Use `bd_enable_integrity()` for anti-cheat protection

```cpp
// Example custom engine integration
#include "belladonna_sdk.h"

class GameEngine {
    bd_handle* belladonna_handle;
    
    bool initialize() {
        // Initialize Belladonna during engine startup
        if (bd_init_with_demo(&belladonna_handle, 1, 1) != 0) {
            return false;
        }
        return true;
    }
    
    bool loadGame(const std::string& player_id) {
        // Check entitlement before starting game
        bd_entitlement_result result;
        if (bd_check_entitlement(belladonna_handle, player_id.c_str(), &result) == 0 && result.entitled) {
            return startGameSession();
        }
        return showPurchaseDialog();
    }
};
```

The C ABI provides complete access to all SDK functionality with minimal integration overhead.

### Licensing

**Q: How does the licensing model work?**
A: The SDK interface is open-source (MIT License) and free to use. The production runtime library requires a separate commercial license with pricing based on your specific use case and deployment scale.

**Q: Can I evaluate the system before purchasing a runtime license?**
A: Absolutely! The demo system is specifically designed for comprehensive evaluation. You can fully validate your integration, test all features, and ensure compatibility before committing to a commercial runtime license.

**Q: Is the source code for the runtime available?**
A: The runtime library is closed-source to maintain security effectiveness. However, the complete SDK interface, integration patterns, and examples are open-source to ensure transparency in the integration process.

### Technical

**Q: What are the system requirements?**
A: Minimum requirements:
- **Development:** Rust 1.70+, C/C++ compiler
- **Runtime (Linux):** Linux kernel 4.15+, x86_64 architecture
- **Memory:** 64MB RAM for SDK operations
- **Storage:** 10MB for SDK interface, additional space for runtime library

**Q: Does Belladonna Play impact game performance?**
A: The SDK interface has minimal performance impact. The commercial runtime is optimized for production gaming with negligible performance overhead in normal operation.

**Q: How do I get help if I encounter issues?**
A: Multiple support channels are available:
- **GitHub Issues** - For SDK interface bugs and feature requests
- **Documentation** - Comprehensive integration guides and API references
- **Community Forums** - Developer discussions and community help
- **Commercial Support** - Professional support for runtime license holders

---

For runtime licensing and commercial support, please contact the maintainers through this repository.