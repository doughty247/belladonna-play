use belladonna_sdk::license_gui::{show_license_dialog, LicenseGuiError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Belladonna License GUI Test");
    println!("===============================");
    
    match show_license_dialog().await {
        Ok(result) => {
            println!("License validation successful!");
            println!("   Key: {}", result.key);
            println!("   Type: {}", result.license_type);
            println!("   Hardware Bound: {:?}", result.hardware_fingerprint);
            println!("   TPM Validated: {}", result.tmp_validated);
        },
        Err(LicenseGuiError::UserCancelled) => {
            println!("User cancelled license input");
        },
        Err(e) => {
            println!("License validation failed: {}", e);
        }
    }
    
    Ok(())
}
