//! Belladonna Play fork scaffolding.
//!
//! High-level modules (DRM, anti-cheat, engine plugins) will progressively wrap
//! the lower-level primitives exported by the core privilege bridge crate.

pub use olivine_bridge as core; // core crate library name

pub mod drm {
    //! DRM entitlement + offline window stub module.
    use thiserror::Error;
    #[derive(Debug, Error)]
    pub enum DrmError { #[error("unimplemented")] Unimplemented }
    pub type Result<T> = std::result::Result<T, DrmError>;
    pub fn check_entitlement(_user: &str) -> Result<bool> { Err(DrmError::Unimplemented) }
}

pub mod anticheat {
    //! Anti-cheat runtime stub module.
    use thiserror::Error;
    #[derive(Debug, Error)] pub enum AcError { #[error("unimplemented")] Unimplemented }
    pub type Result<T> = std::result::Result<T, AcError>;
    pub fn session_status() -> Result<&'static str> { Err(AcError::Unimplemented) }
}

pub mod sdk {
    //! Developer-facing simplified API surface (early draft).
    use super::{drm, anticheat};
    pub fn init() -> &'static str { "belladonna_play_init_stub" }
    pub fn check_entitlement(user: &str) -> bool { drm::check_entitlement(user).unwrap_or(false) }
    pub fn status() -> &'static str { anticheat::session_status().unwrap_or("uninit") }
}
