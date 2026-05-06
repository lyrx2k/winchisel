#![windows_subsystem = "windows"]

mod app;
mod app_definitions;
mod download_definitions;
mod latency;
mod performance;
mod updater;

use crate::app::WinchiselApp;
use eframe::icon_data::from_png_bytes;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Default)]
pub struct GamingTweakRow {
    pub tweak_id: i32,
    pub category: i32,
    pub key: &'static str,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub is_editable: bool,
    pub is_child: bool,
    pub is_parent: bool,
    pub is_expanded: bool,
    pub warning_text: String,
    pub input_type: i32,
    pub options: Vec<String>,
    pub selected_index: i32,
    pub recommended_label: String,
    pub default_label: String,
    pub badge_recommended: bool,
    pub badge_default: bool,
    pub badge_custom: bool,
    pub is_new: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub check_updates_on_startup: bool,
    pub show_console: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            check_updates_on_startup: true,
            show_console: false,
        }
    }
}

pub fn save_app_settings(settings: &AppSettings) {
    let path = app_settings_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(path, json);
    }
}

fn main() {
    if !is_admin() {
        restart_as_admin();
        return;
    }

    let _ = egui_logger::builder().init();
    let settings = load_app_settings();
    set_console_visibility(settings.show_console);

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([1100.0, 650.0])
            .with_icon(
                from_png_bytes(include_bytes!("assets/logo.png"))
                    .expect("failed to load window icon"),
            ),
        centered: true,
        ..Default::default()
    };

    if let Err(err) = eframe::run_native(
        "Winchisel",
        native_options,
        Box::new(move |_cc| {
            let app = WinchiselApp::new(settings, is_admin());
            Ok(Box::new(app))
        }),
    ) {
        eprintln!("Winchisel failed to start: {err:?}");
    }
}

fn load_app_settings() -> AppSettings {
    let path = app_settings_path();
    match fs::read_to_string(path) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => AppSettings::default(),
    }
}

fn app_settings_path() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        return PathBuf::from(appdata)
            .join("Winchisel")
            .join("settings.json");
    }
    PathBuf::from("settings.json")
}

#[cfg(target_os = "windows")]
pub(crate) fn set_console_visibility(show: bool) {
    use windows::Win32::System::Console::{AllocConsole, FreeConsole, GetConsoleWindow};
    use windows::Win32::UI::WindowsAndMessaging::{
        DeleteMenu, DrawMenuBar, GetSystemMenu, MF_BYCOMMAND, SC_CLOSE, SW_HIDE, SW_SHOW,
        ShowWindow,
    };

    unsafe {
        let mut hwnd = GetConsoleWindow();
        if show {
            if hwnd.0.is_null() {
                let _ = AllocConsole();
                hwnd = GetConsoleWindow();
            }
            if !hwnd.0.is_null() {
                let hmenu = GetSystemMenu(hwnd, false);
                if !hmenu.0.is_null() {
                    let _ = DeleteMenu(hmenu, SC_CLOSE, MF_BYCOMMAND);
                    let _ = DrawMenuBar(hwnd);
                }
                let _ = ShowWindow(hwnd, SW_SHOW);
            }
        } else if !hwnd.0.is_null() {
            let _ = ShowWindow(hwnd, SW_HIDE);
            let _ = FreeConsole();
        }
    }
}

#[cfg(target_os = "windows")]
fn is_admin() -> bool {
    use windows::Win32::UI::Shell::IsUserAnAdmin;
    unsafe { IsUserAnAdmin().as_bool() }
}

#[cfg(target_os = "windows")]
pub(crate) fn open_url(url: &str) {
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;
    use windows::core::PCWSTR;

    let url_w: Vec<u16> = std::ffi::OsStr::new(url)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let open_w: Vec<u16> = "open".encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let _ = ShellExecuteW(
            None,
            PCWSTR(open_w.as_ptr()),
            PCWSTR(url_w.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            SW_SHOW,
        );
    }
}

#[cfg(target_os = "windows")]
fn restart_as_admin() {
    if let Ok(exe) = std::env::current_exe() {
        use std::os::windows::ffi::OsStrExt;
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;
        use windows::core::PCWSTR;

        let exe_w: Vec<u16> = exe
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let verb_w: Vec<u16> = "runas".encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
            let _ = ShellExecuteW(
                None,
                PCWSTR(verb_w.as_ptr()),
                PCWSTR(exe_w.as_ptr()),
                PCWSTR::null(),
                PCWSTR::null(),
                SW_SHOW,
            );
        }
    }
}
