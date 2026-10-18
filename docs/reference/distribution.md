# Developer preview distribution

ReproCrate publishes unsigned developer previews for the Rust CLI and desktop application. The GitHub Releases page is the official download location: https://github.com/sanskarIN/ReproCrate/releases.

| Platform | CLI archive | Desktop archive |
| --- | --- | --- |
| Linux x86-64 | `reprocrate-vX.Y.Z-linux-x86_64.tar.gz` | `reprocrate-desktop-vX.Y.Z-linux-x86_64.tar.gz` |
| Windows x86-64 | `reprocrate-vX.Y.Z-windows-x86_64.zip` | `reprocrate-desktop-vX.Y.Z-windows-x86_64.zip` |
| macOS ARM64 | `reprocrate-vX.Y.Z-macos-arm64.tar.gz` | `reprocrate-desktop-vX.Y.Z-macos-arm64.tar.gz` |

Replace X.Y.Z with the selected published version and download the matching `.sha256` for each archive. Starting at v0.8.0, `reprocrate verify-download` checks checksum integrity offline; see [integrity guide](download-integrity.md). A matching checksum does not prove authenticity, since sidecars are not signed.

Desktop executables are raw developer binaries, **not** signed or notarized installers. Windows depends on WebView2 availability, Linux on WebKitGTK and related libraries, and macOS on suitable native runtime support. Do not claim unsupported operating systems or architectures. Automated CI builds/test runs do not replace manual GUI testing on clean systems. Users should review any downloaded executable before running it, especially if local endpoint policies warn about unsigned code.

The source is EPL-2.0 licensed. [Dependency inventory and advisory guide](../security/dependency-audits.md) explains what is and is not yet audited. No self-updater, automatic downloads, telemetry or mandatory account is installed. Public documentation is also maintained as a self-contained static site in `site/`; the Pages workflow has no third-party runtime assets.
