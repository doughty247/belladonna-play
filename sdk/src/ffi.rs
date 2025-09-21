//! C FFI surface for belladonna-sdk (feature `ffi`).
//! 
//! This provides a C-compatible interface for the Belladonna SDK,
//! suitable for integration with game engines and other C/C++ applications.
#![allow(clippy::missing_safety_doc)]
use super::{SdkHandle, InitConfig, EntitlementCode};
use std::os::raw::c_int;

const ABI_VERSION: u32 = 1;

#[repr(C)]
pub struct bd_handle {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct bd_status_out {
    pub integrity_enabled: i32,
    pub reserved: u32,
}

#[repr(C)]
pub struct bd_entitlement_result {
    pub entitled: i32,
}

// Error codes (sync with header)
const BD_OK: i32 = 0;
const BD_ERR_INIT: i32 = 1;
const BD_ERR_INTEGRITY: i32 = 2;
const BD_ERR_ENTITLEMENT: i32 = 3;
const BD_ERR_ENTITLEMENT_EXPIRED: i32 = 4;
const BD_ERR_SIGNATURE_INVALID: i32 = 5;
const BD_ERR_CACHE_CORRUPT: i32 = 6;
const BD_ERR_NETWORK: i32 = 7;
const BD_ERR_INVALID_ARG: i32 = 100;
const BD_ERR_BUFFER_TOO_SMALL: i32 = 101;

// Logger callback (global optional)
static mut LOGGER_CB: Option<extern "C" fn(*const i8)> = None;

fn log_line(line: &str) {
    unsafe {
        if let Some(cb) = LOGGER_CB {
            if let Ok(cstr) = std::ffi::CString::new(line) {
                cb(cstr.as_ptr());
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn bd_get_abi_version() -> u32 {
    ABI_VERSION
}

#[no_mangle]
pub extern "C" fn bd_get_version_str() -> *const i8 {
    static VER: &str = env!("CARGO_PKG_VERSION");
    VER.as_ptr() as *const i8
}

#[no_mangle]
pub unsafe extern "C" fn bd_init(
    handle_out: *mut *mut bd_handle,
    auto_integrity: i32,
) -> i32 {
    if handle_out.is_null() {
        return BD_ERR_INVALID_ARG;
    }
    match SdkHandle::init(InitConfig {
        auto_integrity: auto_integrity != 0,
    }) {
        Ok(h) => {
            let boxed = Box::new(h);
            *handle_out = Box::into_raw(boxed) as *mut bd_handle;
            BD_OK
        }
        Err(_) => BD_ERR_INIT,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bd_shutdown(handle: *mut bd_handle) -> i32 {
    if handle.is_null() {
        return BD_ERR_INVALID_ARG;
    }
    drop(Box::from_raw(handle as *mut SdkHandle));
    BD_OK
}

fn handle_mut<'a>(h: *mut bd_handle) -> Result<&'a mut SdkHandle, i32> {
    if h.is_null() {
        Err(BD_ERR_INVALID_ARG)
    } else {
        Ok(unsafe { &mut *(h as *mut SdkHandle) })
    }
}

#[no_mangle]
pub unsafe extern "C" fn bd_enable_integrity(h: *mut bd_handle) -> i32 {
    match handle_mut(h) {
        Ok(s) => match s.enable_integrity() {
            Ok(_) => {
                log_line("integrity_enabled");
                BD_OK
            }
            Err(_) => BD_ERR_INTEGRITY,
        },
        Err(c) => c,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bd_disable_integrity(h: *mut bd_handle) -> i32 {
    match handle_mut(h) {
        Ok(s) => {
            s.disable_integrity();
            log_line("integrity_disabled");
            BD_OK
        }
        Err(_) => BD_ERR_INVALID_ARG,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bd_check_entitlement(
    h: *mut bd_handle,
    user: *const i8,
    out: *mut bd_entitlement_result,
) -> i32 {
    if h.is_null() || user.is_null() || out.is_null() {
        return BD_ERR_INVALID_ARG;
    }

    let user_str = match std::ffi::CStr::from_ptr(user).to_str() {
        Ok(s) => s,
        Err(_) => return BD_ERR_INVALID_ARG,
    };

    match handle_mut(h) {
        Ok(s) => {
            let result = s.check_entitlement(user_str);
            (*out).entitled = if result.entitled { 1 } else { 0 };
            match result.code {
                EntitlementCode::Ok => BD_OK,
                EntitlementCode::NotEntitled => BD_ERR_ENTITLEMENT,
                EntitlementCode::Expired => BD_ERR_ENTITLEMENT_EXPIRED,
                EntitlementCode::SignatureInvalid => BD_ERR_SIGNATURE_INVALID,
                EntitlementCode::CacheCorrupt => BD_ERR_CACHE_CORRUPT,
                EntitlementCode::NetworkError => BD_ERR_NETWORK,
                EntitlementCode::Error => BD_ERR_ENTITLEMENT,
            }
        }
        Err(c) => c,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bd_status(
    h: *mut bd_handle,
    out: *mut bd_status_out,
) -> i32 {
    if h.is_null() || out.is_null() {
        return BD_ERR_INVALID_ARG;
    }

    match handle_mut(h) {
        Ok(s) => {
            (*out).integrity_enabled = if s.is_integrity_enabled() { 1 } else { 0 };
            (*out).reserved = 0;
            BD_OK
        }
        Err(c) => c,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bd_report_event(
    _h: *mut bd_handle,
    _key: *const i8,
    _value: *const i8,
) -> i32 {
    // Stub implementation
    BD_OK
}

#[no_mangle]
pub unsafe extern "C" fn bd_set_logger(
    logger: Option<extern "C" fn(*const i8)>,
) -> i32 {
    LOGGER_CB = logger;
    BD_OK
}

#[no_mangle]
pub unsafe extern "C" fn bd_export_metrics(
    out_json: *mut i8,
    capacity: u32,
    written: *mut u32,
) -> i32 {
    if out_json.is_null() || written.is_null() {
        return BD_ERR_INVALID_ARG;
    }

    let stub_metrics = r#"{"status":"stub","note":"Requires Belladonna Play runtime"}"#;
    let required_size = stub_metrics.len() + 1; // +1 for null terminator

    if capacity < required_size as u32 {
        *written = required_size as u32;
        return BD_ERR_BUFFER_TOO_SMALL;
    }

    std::ptr::copy_nonoverlapping(
        stub_metrics.as_ptr() as *const i8,
        out_json,
        stub_metrics.len(),
    );
    *out_json.add(stub_metrics.len()) = 0; // null terminator
    *written = stub_metrics.len() as u32;
    BD_OK
}

#[no_mangle]
pub unsafe extern "C" fn bd_check_entitlement(h: *mut bd_handle, user: *const i8, out: *mut bd_entitlement_result) -> i32 {
    if user.is_null() || out.is_null() { return BD_ERR_INVALID_ARG; }
    let s = match handle_mut(h) { Ok(s)=>s, Err(c)=> return c };
    let c_str = std::ffi::CStr::from_ptr(user);
    let user_str = match c_str.to_str() { Ok(v)=>v, Err(_)=> return BD_ERR_ENTITLEMENT };
    let res = s.check_entitlement(user_str);
    log_line(&format!("entitlement user={} ok={} code={:?}", user_str, res.entitled, res.code));
    unsafe { (*out).entitled = if res.entitled {1} else {0}; }
    match res.code {
        EntitlementCode::Ok => BD_OK,
        EntitlementCode::NotEntitled => BD_ERR_ENTITLEMENT,
        EntitlementCode::Expired => BD_ERR_ENTITLEMENT_EXPIRED,
        EntitlementCode::SignatureInvalid => BD_ERR_SIGNATURE_INVALID,
        EntitlementCode::CacheCorrupt => BD_ERR_CACHE_CORRUPT,
        EntitlementCode::NetworkError => BD_ERR_NETWORK,
        EntitlementCode::Error => BD_ERR_ENTITLEMENT,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bd_status(h: *mut bd_handle, out: *mut bd_status_out) -> i32 {
    if out.is_null() { return BD_ERR_INVALID_ARG; }
    let s = match handle_mut(h) { Ok(s)=>s, Err(c)=> return c };
    // Lock to inspect integrity flag
    let enabled = { let g = s.inner.lock(); g.integrity.is_some() };
    (*out).integrity_enabled = if enabled {1} else {0};
    (*out).reserved = 0;
    BD_OK
}

#[no_mangle]
pub unsafe extern "C" fn bd_report_event(_h: *mut bd_handle, _key: *const i8, _value: *const i8) -> i32 { BD_OK }

#[no_mangle]
pub unsafe extern "C" fn bd_set_logger(cb: Option<extern "C" fn(*const i8)>) -> i32 { LOGGER_CB = cb; BD_OK }

#[no_mangle]
pub unsafe extern "C" fn bd_export_metrics(buf: *mut i8, capacity: u32, written: *mut u32) -> i32 {
    if buf.is_null() || written.is_null() { return BD_ERR_INVALID_ARG; }
    let snap = belladonna_play::metrics::snapshot();
    let json = match serde_json::to_string(&snap) { Ok(s)=>s, Err(_)=> return BD_ERR_INTEGRITY };
    let bytes = json.as_bytes();
    unsafe { *written = bytes.len() as u32; }
    if bytes.len() as u32 > capacity { return BD_ERR_BUFFER_TOO_SMALL; }
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf as *mut u8, bytes.len()); }
    BD_OK
}

// Basic FFI smoke tests (only when built as tests + feature enabled)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ffi_init_cycle() {
        let mut handle: *mut bd_handle = std::ptr::null_mut();
        let rc = unsafe { bd_init(&mut handle as *mut *mut bd_handle, 1) };
        assert_eq!(rc, 0);
        assert!(!handle.is_null());
        let mut st = bd_status_out { integrity_enabled: 0, reserved:0 };
        let rc2 = unsafe { bd_status(handle, &mut st as *mut bd_status_out) }; assert_eq!(rc2,0);
        assert_eq!(st.integrity_enabled,1);
        let rc3 = unsafe { bd_shutdown(handle) }; assert_eq!(rc3,0);
    }
}

// Verify an envelope manifest blob (JSON). Expected envelope structure:
// { "manifest": <manifest_json_string>, "signed_payload_b64": <b64(manifest)>, "signature_b64": <b64(sig)>, "pubkey_b64": <b64(pubkey)> }
// manifest JSON should include per-file entries with `path` and `sha256` hex string.
#[no_mangle]
pub unsafe extern "C" fn bd_verify_manifest(_h: *mut bd_handle, manifest_ptr: *const u8, manifest_len: usize) -> c_int {
    if manifest_ptr.is_null() || manifest_len == 0 { return BD_ERR_INVALID_ARG; }
    let bytes = slice::from_raw_parts(manifest_ptr, manifest_len);
    let s = match std::str::from_utf8(bytes) { Ok(v)=>v, Err(_) => return BD_ERR_INVALID_ARG };

    // parse envelope JSON
    let v: JsonValue = match serde_json::from_str(s) { Ok(v)=>v, Err(_) => return BD_ERR_SIGNATURE_INVALID };
    let signed_b64 = match v.get("signed_payload_b64") { Some(x)=> x.as_str().unwrap_or(""), None=>"" };
    let sig_b64 = match v.get("signature_b64") { Some(x)=> x.as_str().unwrap_or(""), None=>"" };
    let pk_b64 = match v.get("pubkey_b64") { Some(x)=> x.as_str().unwrap_or(""), None=>"" };

    if signed_b64.is_empty() || sig_b64.is_empty() || pk_b64.is_empty() { return BD_ERR_SIGNATURE_INVALID; }

    let signed = match B64.decode(signed_b64) { Ok(b)=>b, Err(_) => return BD_ERR_SIGNATURE_INVALID };
    let sig_bytes = match B64.decode(sig_b64) { Ok(b)=>b, Err(_) => return BD_ERR_SIGNATURE_INVALID };
    let pk_bytes = match B64.decode(pk_b64) { Ok(b)=>b, Err(_) => return BD_ERR_SIGNATURE_INVALID };

    if sig_bytes.len() != 64 || pk_bytes.len() != 32 { return BD_ERR_SIGNATURE_INVALID; }

    let pk = match PublicKey::from_bytes(&pk_bytes) { Ok(k)=>k, Err(_) => return BD_ERR_SIGNATURE_INVALID };
    let sig = match Signature::from_bytes(&sig_bytes) { Ok(s)=>s, Err(_) => return BD_ERR_SIGNATURE_INVALID };

    if pk.verify(&signed, &sig).is_err() { return BD_ERR_SIGNATURE_INVALID; }

    // verify inner manifest file hashes if present
    let manifest_json = match v.get("manifest") { Some(m)=> { if m.is_string() { m.as_str().unwrap_or("") } else { "" } } , None=>"" };
    if manifest_json.is_empty() { return BD_OK; } // nothing else to verify

    let mi: JsonValue = match serde_json::from_str(manifest_json) { Ok(j)=>j, Err(_) => return BD_ERR_SIGNATURE_INVALID };
    if let Some(files) = mi.get("files") {
        if let Some(arr) = files.as_array() {
            for entry in arr.iter() {
                if let (Some(path_v), Some(sha_v)) = (entry.get("path"), entry.get("sha256")) {
                    if let (Some(path), Some(sha_hex)) = (path_v.as_str(), sha_v.as_str()) {
                        // compute on-disk file hash if file exists
                        if let Ok(mut f) = std::fs::File::open(path) {
                            let mut hasher = Sha256::new();
                            let mut buf = [0u8; 8192];
                            loop {
                                match f.read(&mut buf) {
                                    Ok(0) => break,
                                    Ok(n) => { hasher.update(&buf[..n]); },
                                    Err(_) => return BD_ERR_SIGNATURE_INVALID,
                                }
                            }
                            let got = hasher.finalize();
                            let got_hex = hex::encode(got);
                            if got_hex != sha_hex.to_lowercase() { return BD_ERR_SIGNATURE_INVALID; }
                        }
                    }
                }
            }
        }
    }

    BD_OK
}