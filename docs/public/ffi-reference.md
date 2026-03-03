---
status: stable
---

# FFI Reference

The `agentic-cognition-ffi` crate provides C-compatible bindings for using AgenticCognition from any language that supports C FFI, including Python, Node.js, Swift, Go, and Java.

## Header File

The C header declares opaque pointer types and functions:

```c
#ifndef AGENTIC_COGNITION_FFI_H
#define AGENTIC_COGNITION_FFI_H

#include <stdint.h>

typedef struct AcogEngine AcogEngine;
typedef struct AcogModel AcogModel;
typedef struct AcogResult AcogResult;

/* Engine lifecycle */
AcogEngine *acog_engine_new(const char *storage_path);
void acog_engine_free(AcogEngine *engine);

/* Model operations */
AcogResult *acog_model_create(AcogEngine *engine);
AcogResult *acog_model_heartbeat(AcogEngine *engine, const char *model_id, const char *context);
AcogResult *acog_model_vitals(AcogEngine *engine, const char *model_id);
AcogResult *acog_model_portrait(AcogEngine *engine, const char *model_id);
AcogResult *acog_model_soul(AcogEngine *engine, const char *model_id);

/* Belief operations */
AcogResult *acog_belief_add(AcogEngine *engine, const char *model_id,
                            const char *text, const char *domain, double confidence);
AcogResult *acog_belief_query(AcogEngine *engine, const char *model_id, const char *query);
AcogResult *acog_belief_graph(AcogEngine *engine, const char *model_id);

/* Shadow and prediction */
AcogResult *acog_shadow_map(AcogEngine *engine, const char *model_id);
AcogResult *acog_predict(AcogEngine *engine, const char *model_id, const char *query);

/* Result access */
const char *acog_result_json(const AcogResult *result);
int32_t acog_result_is_error(const AcogResult *result);
void acog_result_free(AcogResult *result);

#endif
```

## Memory Management

All FFI functions follow the allocate/free pattern:

1. The caller receives an opaque pointer handle from a creation function.
2. The caller must call the corresponding `_free` function when done.
3. Passing a null pointer to any `_free` function is safe (no-op).
4. All string parameters are borrowed -- the FFI layer copies them internally.
5. The `acog_result_json` function returns a pointer to a string owned by the result. It is valid until `acog_result_free` is called.

## Basic Usage (C)

```c
#include "agentic_cognition_ffi.h"
#include <stdio.h>

int main() {
    AcogEngine *engine = acog_engine_new(NULL); /* uses default storage */

    AcogResult *res = acog_model_create(engine);
    if (!acog_result_is_error(res)) {
        printf("Model created: %s\n", acog_result_json(res));
    }
    acog_result_free(res);

    acog_engine_free(engine);
    return 0;
}
```

## Python Bindings

Python bindings are available via `pip install agentic-cognition`. They wrap the FFI layer with Pythonic types:

```python
from agentic_cognition import CognitionEngine

engine = CognitionEngine()
model = engine.create_model()
engine.add_belief(model.id, "I prefer async communication", domain="work", confidence=0.8)
portrait = engine.portrait(model.id)
print(portrait)
```

## Node.js / WASM Bindings

WASM bindings are available through the `npm/wasm` package directory. They expose the same API surface as the FFI with JavaScript-native types.

## Thread Safety

The `AcogEngine` handle is `Send + Sync`. Multiple threads may share a single engine instance. Individual operations are internally serialized through the storage layer's atomic write guarantees.
