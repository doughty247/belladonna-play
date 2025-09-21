// Belladonna SDK C Header (ABI v1)
// NOTE: Experimental until version >= 0.1.0

#ifndef BELLADONNA_SDK_H
#define BELLADONNA_SDK_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct bd_handle bd_handle; // opaque

typedef struct bd_status_out {
    int32_t integrity_enabled;
    uint32_t reserved;
} bd_status_out;

typedef struct bd_entitlement_result {
    int32_t entitled;
} bd_entitlement_result;

// Error Codes
// 0  success
// 1  init failure
// 2  integrity failure
// 3  entitlement generic failure (not entitled / other)
// 4  entitlement expired (cache/file indicates past expiry)
// 5  signature invalid (format/crypto)
// 6  cache corrupt / monotonic violation
// 7  network / IO transient error
// 100 invalid/null handle/argument
// 101 buffer too small (metrics export) - written set to required size

uint32_t bd_get_abi_version(void);
const char* bd_get_version_str(void);

int32_t bd_init(bd_handle** out_handle, int32_t auto_integrity);
int32_t bd_shutdown(bd_handle* handle);
int32_t bd_enable_integrity(bd_handle* handle);
int32_t bd_disable_integrity(bd_handle* handle);
int32_t bd_check_entitlement(bd_handle* handle, const char* user, bd_entitlement_result* out);
int32_t bd_status(bd_handle* handle, bd_status_out* out);
int32_t bd_report_event(bd_handle* handle, const char* key, const char* value); // placeholder
int32_t bd_set_logger(void (*logger)(const char* line)); // registers a process-wide callback (UTF-8 lines)
// Export current metrics snapshot as JSON into caller buffer.
// Returns 0 success, 101 if capacity too small (written set to required size), 100 invalid args.
int32_t bd_export_metrics(char* out_json, uint32_t capacity, uint32_t* written);

#ifdef __cplusplus
}
#endif

#endif // BELLADONNA_SDK_H