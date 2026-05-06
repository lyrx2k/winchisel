# Winchisel

Winchisel is a fast, native Windows tweaking app built in Rust with `egui`/`eframe`.
It focuses on practical system tweaks, cleaner defaults, and a compact UI for quickly
changing Windows settings without digging through multiple control panels.

## What It Does

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

1. Start Winchisel
2. Let it relaunch as Administrator if Windows asks for permission
3. Pick the tab you need from the sidebar
4. Use the recommended/default buttons when you want one-click safe changes
5. Use the individual switches and selectors for manual control
6. Check the Settings tab if you want to show the console or control update checks

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

- `src/main.rs` - app bootstrap, admin launch, settings loading, window setup
- `src/app.rs` - app orchestration and shared state
- `src/app/home.rs` - home tab
- `src/app/debloater.rs` - debloater tab
- `src/app/downloads.rs` - downloads tab
- `src/app/performance.rs` - performance tab
- `src/app/processes.rs` - processes tab
- `src/app/latency.rs` - latency tab
- `src/app/settings.rs` - settings tab
- `src/app/update.rs` - update dialog and update flow
- `src/app/restore_point.rs` - restore point handling
- `src/app/system_info.rs` - system information collection
- `src/app/ui_shell.rs` - shared UI layout helpers
- `src/performance.rs` - performance tweak catalog and tweak logic
- `src/latency.rs` - latency analysis core
- `src/updater.rs` - update checking

## Contributing

Contributions are welcome.
Please keep the UI consistent and make sure `cargo clippy --all-targets --all-features -- -D warnings` passes before opening a PR.

## License

[AGPL-3.0 license](https://github.com/lyrx2k/winchisel/blob/main/LICENSE)
