> [!CAUTION]
> v1.0.0 is available here https://github.com/lejyfps/winchisel


# Winchisel

Winchisel is a fast, native Windows tweaking app built in Rust with `egui`/`eframe`.
It focuses on practical system tweaks, cleaner defaults, and a compact UI for quickly
changing Windows settings without digging through multiple control panels.

## Features

- Home dashboard with system information
- Debloater tools for removing Windows components and apps
- Privacy settings for disabling telemetry and tracking
- Security options for system hardening
- Downloads tab for installing useful apps and tools (via winget or vendor sites)
- Performance tab with gaming and system tweaks
- Processes tab for process management, priorities, affinity, and related actions
- Latency tab for USB latency analysis and device topology inspection
- Settings tab for app behavior and system protection options
- Automatic admin launch for system-level tweaks
- Auto-updater via GitHub Releases
- Restore point support before risky actions

## How To Use

1. Run `Winchisel.exe` and grant Administrator rights (required)
2. Select a tab from the sidebar:

| Tab | Purpose |
|-----|---------|
| **Home** | System overview |
| **Debloater** | Remove bloatware — use 🛡️ *Recommended* for safe defaults |
| **Privacy** | Disable telemetry and tracking |
| **Security** | System hardening and security tweaks |
| **Downloads** | Install tools (fetches from winget or vendor sites) |
| **Performance** | Gaming/system tweaks |
| **Processes** | Manage priorities and affinity |
| **Latency** | USB latency analysis |
| **Settings** | App preferences, console, updates |

> 🛡️ *Recommended* buttons = safe, tested defaults. Manual toggles = full control, may need restart.

## Important Notes

- Winchisel is Windows-only.
- Most tweak actions require Administrator rights.
- Some settings depend on the Windows build or system configuration.
- A restore point may be created before risky operations where supported.
- The app stores its own settings in `%APPDATA%\Winchisel\settings.json`.
- **Network usage**: Auto-updater and Downloads tab require internet access.

## Configuration

Winchisel stores its settings in:

```text
%APPDATA%\Winchisel\settings.json
```

## Project Structure

| Module | Purpose |
|--------|---------|
| `main.rs` | Bootstrap, admin elevation, settings |
| `app/` | UI tabs (home, debloater, performance, processes, latency, settings) |
| `performance.rs` | Tweak registry/catalog |
| `latency.rs` | USB device topology analysis |
| `updater.rs` | GitHub Releases integration |

## Contributing

Contributions are welcome.
Please keep the UI consistent and make sure `cargo clippy --all-targets --all-features -- -D warnings` passes before opening a PR.

## License

### Software License

Winchisel is licensed under the [AGPL-3.0](LICENSE).

In plain language:
- ✅ Free to use for everyone — individuals, businesses, IT professionals
- ✅ Use it to service and consult for clients
- ✅ Modify for your own internal use
- ✅ Fork and modify under AGPL-3.0 terms (must remain open source)
- ❌ Remove or circumvent license/copyright notices
- ❌ Rebrand redistributed versions as "Winchisel"

### Community Guidelines

We respectfully ask contributors and forkers:

1. **Consider upstream contributions** — Pull requests help everyone
2. **Use distinct branding** for public forks — "Winchisel" is reserved
3. **Credit original work** — Link back to this repository

These are social expectations, not legal requirements. The AGPL-3.0 governs all rights and obligations.

### Trademark

"Winchisel" and the Winchisel logo are trademarks of lyrx2k. Derivative works must use a different name and branding.
