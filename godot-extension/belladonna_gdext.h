// belladonna_gdext.h - Public C header for Belladonna Godot extension / FFI layer
// SPDX-License-Identifier: MIT
#ifndef BELLADONNA_GDEXT_H
#define BELLADONNA_GDEXT_H
#ifdef __cplusplus
extern "C" {
#endif
#include <stdint.h>

// ABI version query (stable bump when breaking FFI changes occur)
uint32_t bd_get_abi_version(void);

// Initialization / shutdown (placeholders for future handle-based context)
int bd_init(void **out_handle, int auto_integrity);
int bd_shutdown(void *handle);

// Manifest verification (Ed25519 + optional file hash validation)
int bd_verify_manifest(void *handle, const unsigned char *manifest_json, size_t manifest_len);

// Decrypt an encrypted asset from bundle.enc to a temporary file. Writes NUL-terminated path into out_path.
int bd_decrypt_asset_to_temp(void *handle, const unsigned char *asset_id, unsigned char *out_path, size_t out_len);

// Retrieve last error code (thread local)
int bd_get_last_error(void);

// Error codes
#define BD_OK 0
#define BD_ERR_VERIFY_FAIL 1
#define BD_ERR_DECRYPT_FAIL 2
#define BD_ERR_IO 3
#define BD_ERR_INTERNAL 4
#define BD_ERR_MANIFEST_PARSE 5
#define BD_ERR_BOUNDS 6
#define BD_ERR_UNSUPPORTED_VERSION 7
#define BD_ERR_INVALID_ARG 100

#ifdef __cplusplus
}
#endif
#endif // BELLADONNA_GDEXT_H
