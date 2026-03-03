---
status: stable
---

# The .acog File Format

The `.acog` format is a custom binary format with integrity protection designed for long-term persistence of living user models.

## Binary Layout

```
Offset  Size     Field
------  -------  -----
0x00    4 bytes  Magic number: 0x41 0x43 0x4F 0x47 ("ACOG")
0x04    2 bytes  Format version (big-endian u16)
0x06    2 bytes  Feature flags (big-endian u16)
0x08    4 bytes  Body length in bytes (big-endian u32)
0x0C    32 bytes BLAKE3 checksum of the body
0x2C    variable JSON body (UTF-8)
```

### Magic Number

The first four bytes are always `ACOG` (0x41434F47). This allows file type detection without relying on the file extension.

### Version

The 2-byte version field tracks the format version. The current version is 1. Readers should reject files with a version higher than they support.

### Feature Flags

The 2-byte feature flags field indicates optional features present in the file:

| Bit | Flag | Description |
|---:|:---|:---|
| 0 | `COMPRESSED` | Body is zstd-compressed |
| 1 | `ENCRYPTED` | Body is encrypted (reserved for future use) |
| 2-15 | -- | Reserved, must be zero |

### Body Length

The 4-byte body length field gives the exact size of the JSON body in bytes. This enables readers to allocate the correct buffer size before reading.

### BLAKE3 Checksum

The 32-byte BLAKE3 checksum is computed over the JSON body. On every read, the reader computes the checksum of the loaded body and compares it to the stored checksum. A mismatch indicates corruption.

### JSON Body

The body contains the full living user model serialized as UTF-8 JSON. The body includes:

- Model metadata (ID, name, lifecycle stage, creation timestamp, heartbeat count)
- Beliefs array (each with ID, text, domain, confidence, crystallization, timestamps)
- Entanglement edges (belief-to-belief connections with type and weight)
- Shadow structures (shadow beliefs, projections, blindspots, defended regions)
- Drift history (timestamped drift events with domain and magnitude)
- Decision fingerprints (behavioral pattern records)
- Growth rings (archaeological identity snapshots)
- Consciousness map (region activity levels)

## Integrity Guarantees

### BLAKE3 Checksums

Every file write computes a BLAKE3 checksum over the JSON body and stores it in the header. Every file read verifies the checksum before returning the data. If the checksum does not match, the read fails with an integrity error.

BLAKE3 was chosen for its speed (3x faster than SHA-256 on modern hardware) and cryptographic strength.

### Atomic Writes

File writes use a temp-file-plus-rename strategy:

1. Write the complete file to a temporary file in the same directory
2. Sync the temporary file to disk (`fsync`)
3. Atomically rename the temporary file to the target path

This guarantees that a crash during write cannot produce a partial or corrupt file. The previous version of the file remains intact until the rename completes.

### File Locking

Concurrent access is managed through advisory file locks. The lock file is a sidecar `.acog.lock` file. Writers acquire an exclusive lock before writing. Readers can proceed without locks since atomic rename guarantees consistent reads.

## Capacity Metrics

| Duration | Approximate Beliefs | File Size |
|:---|---:|---:|
| 1 week | ~50 | ~10 KB |
| 1 month | ~200 | ~40 KB |
| 6 months | ~500 | ~100 KB |
| 1 year | ~1,000 | ~200 KB |
| 5 years | ~5,000 | ~1 MB |
| 10 years | ~10,000 | ~2 MB |
| 20 years | ~20,000 | ~4 MB |

No external database dependencies are required. The format is designed to remain usable for decades without schema migrations.

## Forward Compatibility

The format is designed for forward compatibility:

- Unknown feature flags are treated as errors (to prevent data loss)
- The JSON body uses optional fields: new fields can be added without breaking older readers
- Older readers skip unknown JSON keys gracefully
- The version field enables breaking changes when necessary, with explicit migration paths

## Portability

One `.acog` file holds the complete user model. It is:

- **Self-contained**: no external dependencies, no database, no cloud service
- **Restart-safe**: survives process crashes, machine reboots, and OS updates
- **Model-agnostic**: load it into any LLM and it instantly understands who the user is
- **Client-portable**: move it between Claude Desktop, Cursor, VS Code, or any MCP client
- **Human-auditable**: the JSON body can be extracted and inspected with standard tools
