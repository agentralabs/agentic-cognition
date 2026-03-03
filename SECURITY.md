# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

Please report security vulnerabilities to: security@agentralabs.tech

Do NOT open a public issue for security vulnerabilities.

We will respond within 48 hours and provide a fix within 7 days for critical issues.

## Security Measures

- blake3 checksums for .acog file integrity
- Atomic file operations (temp + rename) prevent corruption
- Per-project isolation prevents cross-contamination
- Token-based authentication for server mode
- Constant-time token comparison
- No network access in core library
- Input validation on all boundaries
