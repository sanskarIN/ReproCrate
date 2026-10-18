# Verify downloaded release archives (v0.8.0)

ReproCrate's releases provide executable archives for CLI and Tauri desktop, plus one `.sha256` sidecar per archive. The `verify-download` command reads only your two explicitly selected local files. It never extracts an archive, runs any downloaded program, performs network calls or writes to the archive.

## Example

Download a matching archive and checksum file **from the same official GitHub release**. On Linux:

```sh
reprocrate verify-download ./reprocrate-v0.8.0-linux-x86_64.tar.gz \
  --checksum ./reprocrate-v0.8.0-linux-x86_64.tar.gz.sha256
```

Windows example:

```powershell
reprocrate.exe verify-download .\reprocrate-v0.8.0-windows-x86_64.zip --checksum .\reprocrate-v0.8.0-windows-x86_64.zip.sha256
```

The command reads checksum sidecars up to **512 bytes**, requires exactly one record of 64 hexadecimal SHA-256 characters, two spaces and the **identical archive filename**. It streams the selected archive up to **1 GiB** without loading the entire file into memory. It rejects symlinks and other non-regular selected file types, mismatched filenames, malformed digest text and digest mismatches; on Unix it also checks opened-file identity.

Exit code 0 means the bytes matched the supplied checksum; a verification error returns nonzero. If verification fails, discard the archive and fetch a new copy from a trusted source. Do not skip integrity checks for unexpected filenames.

**Security limitation:** An attacker who changes both an archive and its checksum can produce a matching pair. SHA-256 does **not authenticate** a release's publisher, prove the binary is safe, provide a cryptographic signature or replace OS code signing. Verify the source of the download, and remember that these developer-preview binaries are unsigned. v0.8.0 introduces checking; earlier CLI versions do not support this subcommand.
