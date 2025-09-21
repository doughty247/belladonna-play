//! Belladonna SDK: Open-source interface for game integrations
//!
//! This is the open-source SDK interface for Belladonna Play. It provides
//! the public API that games and engines integrate with, but requires the
//! closed-source Belladonna Play runtime for actual DRM and anti-cheat functionality.
//!
//! # Usage
//!
//! ```rust
//! use belladonna_sdk::{InitConfig, SdkHandle};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let sdk = SdkHandle::init(InitConfig { auto_integrity: true, demo_mode: true })?;
//!     let ent = sdk.check_entitlement("player1");
//!     println!("entitled={}", ent.entitled);
//!     Ok(())
//! }
//! ```

use parking_lot::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use thiserror::Error;

#[cfg(feature = "license-gui")]
pub mod license_gui;

/// SDK initialization and runtime errors
#[derive(Debug, Error)]
pub enum SdkError {
    #[error("init_error: {0}")]
    Init(String),
    #[error("integrity_error: {0}")]
    Integrity(String),
    #[error("entitlement_error: {0}")]
    Entitlement(String),
    #[error("runtime_not_available: This SDK interface requires the Belladonna Play runtime library")]
    RuntimeNotAvailable,
}

/// Configuration for SDK initialization
#[derive(Debug, Clone, Default)]
pub struct InitConfig {
    /// Automatically enable integrity monitoring on initialization
    pub auto_integrity: bool,
    /// Enable demo mode with simulated functionality
    pub demo_mode: bool,
}

/// Entitlement check result codes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntitlementCode {
    /// User is properly entitled
    Ok,
    /// User is not entitled or license not found
    NotEntitled,
    /// License has expired
    Expired,
    /// Signature validation failed (format/crypto)
    SignatureInvalid,
    /// Cache corruption or monotonic violation
    CacheCorrupt,
    /// Network or IO error during license verification
    NetworkError,
    /// Generic error or fallback
    Error,
}

/// Result of an entitlement check
#[derive(Debug, Clone)]
pub struct EntitlementResult {
    /// Whether the user is entitled
    pub entitled: bool,
    /// Specific result code
    pub code: EntitlementCode,
}

struct Inner {
    integrity_enabled: bool,
    demo_mode: bool,
    demo_entitlements: std::collections::HashMap<String, bool>,
}

/// Main SDK handle for Belladonna Play integration
#[derive(Clone)]
pub struct SdkHandle {
    inner: Arc<Mutex<Inner>>,
}

impl SdkHandle {
    /// Initialize the SDK with the given configuration
    ///
    /// # Note
    /// This open-source SDK provides the interface only. For actual DRM and
    /// anti-cheat functionality, you need the closed-source Belladonna Play runtime.
    /// Use `demo_mode: true` to enable interactive demo functionality.
    pub fn init(cfg: InitConfig) -> Result<Self, SdkError> {
        let mut demo_entitlements = HashMap::new();
        
        // Pre-populate demo users for testing
        if cfg.demo_mode {
            demo_entitlements.insert("demo_player_123".to_string(), true);
            demo_entitlements.insert("valid_user".to_string(), true);
            demo_entitlements.insert("expired_user".to_string(), false);
            demo_entitlements.insert("test_player".to_string(), true);
        }
        
        let handle = Self {
            inner: Arc::new(Mutex::new(Inner {
                integrity_enabled: false,
                demo_mode: cfg.demo_mode,
                demo_entitlements,
            })),
        };
        
        if cfg.auto_integrity {
            let _ = handle.enable_integrity();
        }
        
        Ok(handle)
    }

    /// Enable integrity monitoring
    ///
    /// # Note
    /// In demo mode, provides simulated integrity monitoring.
    /// Actual integrity monitoring requires the Belladonna Play runtime library.
    pub fn enable_integrity(&self) -> Result<(), SdkError> {
        let mut g = self.inner.lock();
        g.integrity_enabled = true;
        
        if g.demo_mode {
            println!("Demo: Integrity monitoring enabled - simulating real-time protection");
        } else {
            println!("Integrity monitoring enabled (SDK interface mode - requires runtime)");
        }
        Ok(())
    }

    /// Disable integrity monitoring
    pub fn disable_integrity(&self) {
        let mut g = self.inner.lock();
        g.integrity_enabled = false;
        println!("Integrity monitoring disabled");
    }

    /// Check user entitlement
    ///
    /// # Note
    /// In demo mode, provides realistic entitlement simulation.
    /// Actual entitlement checking requires the Belladonna Play runtime library.
    pub fn check_entitlement(&self, user: &str) -> EntitlementResult {
        let g = self.inner.lock();
        
        if g.demo_mode {
            if let Some(&entitled) = g.demo_entitlements.get(user) {
                EntitlementResult {
                    entitled,
                    code: if entitled { 
                        EntitlementCode::Ok 
                    } else { 
                        EntitlementCode::Expired 
                    },
                }
            } else {
                EntitlementResult {
                    entitled: false,
                    code: EntitlementCode::NotEntitled,
                }
            }
        } else {
            println!("Entitlement check (SDK interface mode - requires runtime)");
            EntitlementResult {
                entitled: false,
                code: EntitlementCode::Error,
            }
        }
    }
    
    /// Check if integrity monitoring is currently enabled
    pub fn is_integrity_enabled(&self) -> bool {
        let g = self.inner.lock();
        g.integrity_enabled
    }
    
    /// Check if demo mode is active
    pub fn is_demo_mode(&self) -> bool {
        let g = self.inner.lock();
        g.demo_mode
    }
    
    /// Add a demo user entitlement (demo mode only)
    pub fn add_demo_user(&self, user: &str, entitled: bool) -> Result<(), SdkError> {
        let mut g = self.inner.lock();
        if !g.demo_mode {
            return Err(SdkError::Init("Demo user management requires demo_mode=true".to_string()));
        }
        g.demo_entitlements.insert(user.to_string(), entitled);
        println!("Demo: Added user '{}' with entitlement={}", user, entitled);
        Ok(())
    }
    
    /// Simulate integrity threat detection (demo mode only)
    pub fn simulate_threat(&self, threat_type: &str) -> Result<String, SdkError> {
        let g = self.inner.lock();
        if !g.demo_mode {
            return Err(SdkError::Init("Threat simulation requires demo_mode=true".to_string()));
        }
        
        let response = match threat_type {
            "debugger" => "Demo: Debugger detected - switching to degraded mode",
            "memory_scan" => "Demo: Memory scanning detected - enabling enhanced protection",
            "injection" => "Demo: Code injection attempt blocked - maintaining normal operation",
            _ => "Demo: Unknown threat type - no action taken",
        };
        
        println!("{}", response);
        Ok(response.to_string())
    }
}

impl Drop for SdkHandle {
    fn drop(&mut self) {
        self.disable_integrity();
    }
}

#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_integrity_toggle() {
        let sdk = SdkHandle::init(InitConfig {
            auto_integrity: false,
            demo_mode: false,
        })
        .unwrap();
        
        assert!(!sdk.is_integrity_enabled());
        sdk.enable_integrity().unwrap();
        assert!(sdk.is_integrity_enabled());
        sdk.disable_integrity();
        assert!(!sdk.is_integrity_enabled());
    }

    #[test]
    fn entitlement_stub() {
        let sdk = SdkHandle::init(InitConfig {
            auto_integrity: false,
            demo_mode: false,
        })
        .unwrap();
        
        let res = sdk.check_entitlement("user1");
        // In stub mode, should always return not entitled with error code
        assert!(!res.entitled);
        assert_eq!(res.code, EntitlementCode::Error);
    }
}
