//! Belladonna License GUI - Friendly license key input with TPM integration

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::process::Command;

#[derive(Debug, Error)]
pub enum LicenseGuiError {
    #[error("GUI initialization failed: {0}")]
    InitFailed(String),
    
    #[error("License key validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("User cancelled license input")]
    UserCancelled,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseDialogConfig {
    pub title: String,
    pub message: String,
    pub placeholder: String,
    pub allow_trial: bool,
    pub require_hardware_binding: bool,
    pub tmp_validation: bool,
}

impl Default for LicenseDialogConfig {
    fn default() -> Self {
        Self {
            title: "Belladonna License Activation".to_string(),
            message: "Enter your license key to unlock the full game experience:".to_string(),
            placeholder: "Enter license key (e.g., BELL-XXXX-XXXX-XXXX)".to_string(),
            allow_trial: true,
            require_hardware_binding: true,
            tmp_validation: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseResult {
    pub key: String,
    pub license_type: String,
    pub hardware_fingerprint: Option<String>,
    pub tmp_validated: bool,
}

/// Show the friendly license key input dialog
pub async fn show_license_dialog() -> Result<LicenseResult, LicenseGuiError> {
    let config = LicenseDialogConfig::default();
    
    // Use zenity for Linux GUI dialog
    let output = Command::new("zenity")
        .args(&[
            "--entry",
            "--title", &config.title,
            "--text", &config.message,
            "--entry-text", &config.placeholder,
            "--width", "500",
            "--height", "200"
        ])
        .output()
        .await
        .map_err(|e| LicenseGuiError::InitFailed(format!("Failed to launch dialog: {}", e)))?;

    if !output.status.success() {
        return Err(LicenseGuiError::UserCancelled);
    }

    let license_key = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Handle empty input
    if license_key.is_empty() && config.allow_trial {
        let trial_choice = show_trial_dialog().await?;
        if trial_choice {
            return Ok(LicenseResult {
                key: "TRIAL-MODE".to_string(),
                license_type: "Trial".to_string(),
                hardware_fingerprint: None,
                tmp_validated: false,
            });
        } else {
            return Err(LicenseGuiError::UserCancelled);
        }
    }
    
    if license_key.is_empty() {
        return Err(LicenseGuiError::UserCancelled);
    }
    
    // Basic validation
    if license_key.len() < 10 {
        return Err(LicenseGuiError::ValidationFailed(
            "License key is too short".to_string()
        ));
    }
    
    Ok(LicenseResult {
        key: license_key,
        license_type: "Standard".to_string(),
        hardware_fingerprint: None,
        tmp_validated: false,
    })
}

async fn show_trial_dialog() -> Result<bool, LicenseGuiError> {
    let output = Command::new("zenity")
        .args(&[
            "--question",
            "--title", "Trial Mode Available", 
            "--text", "No license key entered. Would you like to start a 30-day trial?\n\n• Access to all game features\n• 30-day time limit\n• Can upgrade anytime",
            "--width", "400",
            "--height", "150"
        ])
        .output()
        .await
        .map_err(|e| LicenseGuiError::InitFailed(format!("Failed to show trial dialog: {}", e)))?;

    Ok(output.status.success())
}
