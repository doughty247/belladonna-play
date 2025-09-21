//! Belladonna Play Godot Extension - Example Integration Patterns
//!
//! This module provides example code showing how to integrate Belladonna Play
//! DRM and anti-cheat functionality into Godot games. This is the open-source
//! SDK interface only - actual functionality requires the Belladonna Play runtime.
//!
//! # Features
//! - Example asset verification patterns
//! - Cryptographic signature verification examples  
//! - Godot-specific integration helpers
//! - C ABI for GDExtension compatibility

#![allow(clippy::missing_safety_doc)]

use std::os::raw::{c_int, c_uchar};
use std::slice;
use std::panic;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use sha2::{Digest, Sha256};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use serde_json::Value as JsonValue;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use zeroize::Zeroize;
use std::sync::Mutex;

const ABI_VERSION: u32 = 1;
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Error codes
const BD_OK: c_int = 0;
const BD_ERR_RUNTIME_NOT_AVAILABLE: c_int = -1000;
const BD_ERR_INVALID_ARGS: c_int = -1;
const BD_ERR_CRYPTO_FAIL: c_int = -2;
const BD_ERR_FILE_NOT_FOUND: c_int = -3;

// Example metrics (stub counters)
static mut BD_ENCRYPTED_ASSET_COUNT: u64 = 0;
static mut BD_PLAINTEXT_ASSET_COUNT: u64 = 0;
static mut BD_DECRYPT_FAIL_TOTAL: u64 = 0;
static mut BD_CHUNK_DECRYPT_TOTAL: u64 = 0;
static mut BD_CHUNK_DECRYPT_FAIL_TOTAL: u64 = 0;

static LAST_ERROR: Mutex<String> = Mutex::new(String::new());

fn set_last_error(err: &str) {
    if let Ok(mut last_err) = LAST_ERROR.lock() {
        last_err.clear();
        last_err.push_str(err);
    }
}

// --- Godot binding (feature gated) -------------------------------------------------------
#[cfg(feature = "godot-bindings")]
mod godot_binding {
    use godot::prelude::*;
    use super::{VERSION, BD_OK, bd_get_last_error};

    /// Belladonna Play runtime interface for Godot
    /// 
    /// This provides example integration patterns showing how games can
    /// integrate with Belladonna Play for DRM and anti-cheat functionality.
    #[derive(GodotClass)]
    #[class(base=Object, init)]
    pub struct BelladonnaRuntime;

    #[godot_api]
    impl BelladonnaRuntime {
        /// Get the SDK version
        #[func]
        fn version(&self) -> GString {
            GString::from(VERSION)
        }

        /// Get count of encrypted assets processed (example metric)
        #[func]
        fn encrypted_asset_count(&self) -> u64 {
            unsafe { super::BD_ENCRYPTED_ASSET_COUNT }
        }

        /// Get count of plaintext assets processed (example metric)
        #[func]
        fn plaintext_asset_count(&self) -> u64 {
            unsafe { super::BD_PLAINTEXT_ASSET_COUNT }
        }

        /// Get total decryption failures (example metric)
        #[func]
        fn decrypt_fail_total(&self) -> u64 {
            unsafe { super::BD_DECRYPT_FAIL_TOTAL }
        }

        /// Example: Verify a signed manifest file
        /// 
        /// This shows the pattern for verifying asset manifests with Ed25519 signatures.
        /// In the full runtime, this would integrate with the actual DRM system.
        #[func]
        fn verify_manifest(&self, manifest_path: GString) -> bool {
            let path = manifest_path.to_string();
            
            // Example verification logic - in practice this would use the runtime
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    // Example: Check if manifest has required fields
                    if let Ok(json) = serde_json::from_str::<JsonValue>(&content) {
                        json.get("signature").is_some() && json.get("assets").is_some()
                    } else {
                        super::set_last_error("Invalid manifest JSON format");
                        false
                    }
                }
                Err(e) => {
                    super::set_last_error(&format!("Failed to read manifest: {}", e));
                    false
                }
            }
        }

        /// Example: Enable asset gate (DRM protection)
        /// 
        /// In the full runtime, this would activate asset protection.
        #[func]
        fn enable_asset_gate(&self) -> bool {
            super::set_last_error("Asset gate requires Belladonna Play runtime");
            false // Stub - requires runtime
        }

        /// Get the last error message
        #[func]
        fn get_last_error(&self) -> GString {
            if let Ok(err) = super::LAST_ERROR.lock() {
                GString::from(err.as_str())
            } else {
                GString::from("Unknown error")
            }
        }
    }

    #[derive(GodotClass)]
    #[class(base=Object, init)]
    pub struct BelladonnaAssetLoader;

    #[godot_api]
    impl BelladonnaAssetLoader {
        /// Example: Load and verify an encrypted asset
        /// 
        /// This shows the pattern for loading DRM-protected game assets.
        #[func]
        fn load_encrypted_asset(&self, asset_path: GString) -> PackedByteArray {
            let path = asset_path.to_string();
            super::set_last_error("Encrypted assets require Belladonna Play runtime");
            
            // Stub implementation - return empty array
            PackedByteArray::new()
        }

        /// Example: Check if an asset is encrypted
        #[func]
        fn is_asset_encrypted(&self, asset_path: GString) -> bool {
            let path = asset_path.to_string();
            // Simple heuristic - check file extension
            path.ends_with(".enc") || path.ends_with(".encrypted")
        }
    }
}

// --- C ABI Export Functions -------------------------------------------------------

/// Get the ABI version
#[no_mangle]
pub extern "C" fn bd_get_abi_version() -> u32 {
    ABI_VERSION
}

/// Get the version string
#[no_mangle]
pub extern "C" fn bd_get_version() -> *const c_uchar {
    VERSION.as_ptr()
}

/// Get the last error message
#[no_mangle]
pub extern "C" fn bd_get_last_error() -> *const c_uchar {
    if let Ok(err) = LAST_ERROR.lock() {
        err.as_ptr()
    } else {
        b"Unknown error\0".as_ptr()
    }
}

/// Example: Enable asset gate protection
/// 
/// This is a stub implementation. The actual function would activate
/// DRM protection for game assets when the runtime is available.
#[no_mangle]
pub unsafe extern "C" fn bd_enable_asset_gate() -> c_int {
    set_last_error("Asset gate requires Belladonna Play runtime");
    BD_ERR_RUNTIME_NOT_AVAILABLE
}

/// Example: Decrypt asset to memory
/// 
/// This shows the signature for the asset decryption function.
/// The actual implementation requires the Belladonna Play runtime.
#[no_mangle]
pub unsafe extern "C" fn bd_decrypt_asset_to_memory(
    _asset_path: *const c_uchar,
    _out_buffer: *mut c_uchar,
    _buffer_size: usize,
    _actual_size: *mut usize,
) -> c_int {
    set_last_error("Asset decryption requires Belladonna Play runtime");
    BD_ERR_RUNTIME_NOT_AVAILABLE
}

/// Example: Verify asset signature
/// 
/// This demonstrates the pattern for verifying cryptographic signatures
/// on game assets. The full implementation would integrate with the DRM system.
#[no_mangle]
pub unsafe extern "C" fn bd_verify_asset_signature(
    asset_data: *const c_uchar,
    asset_size: usize,
    signature: *const c_uchar,
    signature_size: usize,
    public_key: *const c_uchar,
    public_key_size: usize,
) -> c_int {
    if asset_data.is_null() || signature.is_null() || public_key.is_null() {
        set_last_error("Invalid arguments to signature verification");
        return BD_ERR_INVALID_ARGS;
    }

    if signature_size != 64 || public_key_size != 32 {
        set_last_error("Invalid signature or public key size");
        return BD_ERR_INVALID_ARGS;
    }

    // Example Ed25519 signature verification
    match std::panic::catch_unwind(|| {
        let asset_slice = slice::from_raw_parts(asset_data, asset_size);
        let signature_bytes = slice::from_raw_parts(signature, signature_size);
        let public_key_bytes = slice::from_raw_parts(public_key, public_key_size);

        let pub_key = PublicKey::from_bytes(public_key_bytes).map_err(|_| "Invalid public key")?;
        let sig = Signature::from_bytes(signature_bytes).map_err(|_| "Invalid signature")?;

        // Hash the asset data
        let mut hasher = Sha256::new();
        hasher.update(asset_slice);
        let hash = hasher.finalize();

        // Verify signature
        pub_key.verify(&hash, &sig).map_err(|_| "Signature verification failed")?;

        Ok(())
    }) {
        Ok(Ok(())) => BD_OK,
        Ok(Err(e)) => {
            set_last_error(e);
            BD_ERR_CRYPTO_FAIL
        }
        Err(_) => {
            set_last_error("Panic during signature verification");
            BD_ERR_CRYPTO_FAIL
        }
    }
}

/// Example: Initialize Belladonna Play runtime
/// 
/// In the full system, this would initialize the DRM and anti-cheat systems.
#[no_mangle]
pub unsafe extern "C" fn bd_init_runtime(config_path: *const c_uchar) -> c_int {
    if config_path.is_null() {
        set_last_error("Invalid config path");
        return BD_ERR_INVALID_ARGS;
    }

    set_last_error("Runtime initialization requires Belladonna Play runtime library");
    BD_ERR_RUNTIME_NOT_AVAILABLE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_verification_example() {
        use rand::rngs::OsRng;
        use ed25519_dalek::{Keypair, Signer};

        // Generate a test keypair
        let keypair = Keypair::generate(&mut OsRng);
        let test_data = b"test asset data";

        // Sign the data
        let signature = keypair.sign(test_data);

        // Verify using our C function
        let result = unsafe {
            bd_verify_asset_signature(
                test_data.as_ptr(),
                test_data.len(),
                signature.to_bytes().as_ptr(),
                signature.to_bytes().len(),
                keypair.public.to_bytes().as_ptr(),
                keypair.public.to_bytes().len(),
            )
        };

        assert_eq!(result, BD_OK);
    }

    #[test]
    fn test_version_functions() {
        assert_eq!(bd_get_abi_version(), ABI_VERSION);
        
        let version_ptr = bd_get_version();
        let version_str = unsafe {
            std::ffi::CStr::from_ptr(version_ptr as *const i8)
                .to_str()
                .unwrap()
        };
        assert_eq!(version_str, VERSION);
    }
}