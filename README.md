# Winchisel

Winchisel is a fast, native Windows tweaking app built in Rust with `egui`/`eframe`.
It focuses on practical system tweaks, cleaner defaults, and a compact UI for quickly
changing Windows settings without digging through multiple control panels.

## Features

- Home dashboard with system information
- Debloater tools for removing Windows components and apps
- Downloads tab for installing useful apps and tools
- Performance tab with gaming and system tweaks
- Processes tab for process management, priorities, affinity, and related actions
- Latency tab for USB latency analysis and device topology inspection
- Settings tab for app behavior and system protection options
- Automatic admin launch for system-level tweaks
- Update checking through GitHub Releases
- Restore point support before risky actions

## How To Use

1. Run `Winchisel.exe` and grant Administrator rights (required)
2. Select a tab from the sidebar:

| Tab | Purpose |
|-----|---------|
| **Home** | System overview |
| **Debloater** | Remove bloatware — use 🛡️ *Recommended* for safe defaults |
| **Downloads** | Install useful tools |
| **Performance** | Gaming/system tweaks |
| **Processes** | Manage priorities and affinity |
| **Latency** | USB latency analysis |
| **Settings** | App preferences, console, updates |

> 🛡️ *Recommended* buttons = safe, tested defaults. Manual toggles = full control, may need restart.

## Download

Get the latest release here:

[![Download Winchisel](https://img.shields.io/badge/Download-Latest%20Release-brightgreen?style=for-the-badge&logo=windows)](https://github.com/lyrx2k/winchisel/releases/latest)

## Important Notes

- Winchisel is Windows-only.
- Most tweak actions require Administrator rights.
- Some settings depend on the Windows build or system configuration.
- A restore point may be created before risky operations where supported.
- The app stores its own settings in `%APPDATA%\Winchisel\settings.json`.

## Configuration

Winchisel stores its settings in:

```text
%APPDATA%\Winchisel\settings.json
```

Available app settings:

- `check_updates_on_startup`
- `show_console`

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

[AGPL-3.0 license](https://github.com/lyrx2k/winchisel/blob/main/LICENSE)
