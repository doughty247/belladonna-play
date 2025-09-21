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
//!     let sdk = SdkHandle::init(InitConfig { auto_integrity: true })?;
//!     let ent = sdk.check_entitlement("player1");
//!     println!("entitled={}", ent.entitled);
//!     Ok(())
//! }
//! ```

use parking_lot::Mutex;
use std::sync::Arc;
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
    pub fn init(cfg: InitConfig) -> Result<Self, SdkError> {
        let handle = Self {
            inner: Arc::new(Mutex::new(Inner {
                integrity_enabled: false,
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
    /// This is a stub implementation. Actual integrity monitoring requires
    /// the Belladonna Play runtime library.
    pub fn enable_integrity(&self) -> Result<(), SdkError> {
        let mut g = self.inner.lock();
        g.integrity_enabled = true;
        println!("Integrity monitoring enabled (SDK interface mode - requires runtime)");
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
    /// This is a stub implementation. Actual entitlement checking requires
    /// the Belladonna Play runtime library.
    pub fn check_entitlement(&self, _user: &str) -> EntitlementResult {
        println!("Entitlement check (SDK interface mode - requires runtime)");
        // Stub implementation - returns error indicating runtime needed
        EntitlementResult {
            entitled: false,
            code: EntitlementCode::Error,
        }
    }
    
    /// Check if integrity monitoring is currently enabled
    pub fn is_integrity_enabled(&self) -> bool {
        let g = self.inner.lock();
        g.integrity_enabled
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
        })
        .unwrap();
        
        let res = sdk.check_entitlement("user1");
        // In stub mode, should always return not entitled with error code
        assert!(!res.entitled);
        assert_eq!(res.code, EntitlementCode::Error);
    }
}
