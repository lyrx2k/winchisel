use crate::{
    AppSettings,
    app_definitions::get_all_apps,
    download_definitions::{DownloadApp, DownloadCategory, get_all_downloads},
    i18n::t,
    save_app_settings,
};
use eframe::egui;
use egui_notify::Toasts;
use iconflow::{Pack, Size, Style, try_icon};
use std::collections::HashSet;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

mod debloater;
mod downloads;
#[path = "app/extras.rs"]
mod extras;
mod home;
#[path = "app/latency.rs"]
mod latency_tab;
#[path = "app/performance.rs"]
mod performance_tab;
#[path = "app/privacy_security.rs"]
mod privacy_security;
#[path = "app/processes.rs"]
mod processes_tab;
#[path = "app/repair.rs"]
mod repair;
#[path = "app/restore_point.rs"]
mod restore_point;
mod settings;
#[path = "app/system_info.rs"]
mod system_info;
#[path = "app/ui_shell.rs"]
mod ui_shell;
#[path = "app/update.rs"]
mod update;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tab {
    Home,
    Debloater,
    Downloads,
    Performance,
    Processes,
    Latency,
    SecurityPrivacy,
    Settings,
    Extras,
}

#[derive(Clone)]
struct DownloadsState {
    downloads_query: String,
    downloads_items: Vec<DownloadApp>,
    downloads_selected: Vec<bool>,
    downloads_installed: Vec<bool>,
    downloads_view_mode: usize,
    downloads_loading: bool,
    downloads_filter_cache_query: String,
    downloads_filter_cache_view_mode: usize,
    downloads_filter_cache_all: Vec<usize>,
    downloads_filter_cache_categories: Vec<Vec<usize>>,
}

#[derive(Clone)]
pub(crate) struct HomeState {
    computer_name: String,
    os_version: String,
    kernel_version: String,
    bios_version: String,
    bios_date: String,
    cpu_brand: String,
    cpu_cores: String,
    gpu_name: String,
    gpu_vram: String,
    gpu_driver_version: String,
    motherboard: String,
    windows_build: String,
    memory_total: String,
    memory_used: String,
    memory_total_gb: f64,
    memory_used_gb: f64,
    storage_total: String,
    storage_used: String,
    storage_total_gb: f64,
    storage_used_gb: f64,
    cpu_usage: String,
    cpu_usage_percent: f32,
    cpu_speed: String,
    ram_details: String,
    display_info: String,
    uptime: String,
}

struct AppState {
    settings: AppSettings,
    is_admin: bool,
    active_tab: Tab,
    update_status: String,
    update_check_loading: bool,
    update_check_started: bool,
    last_saved_settings: AppSettings,
    debloater: debloater::DebloaterState,
    downloads: DownloadsState,
    performance: performance_tab::PerformanceState,
    privacy_security: privacy_security::PrivacySecurityState,
    home: HomeState,
    cpu: processes_tab::CpuState,
    latency: latency_tab::LatencyState,
    extras: ExtrasState,
    home_last_refresh: Option<Instant>,
}

#[derive(Clone)]
struct ExtrasState {
    brave_debloat_enabled: bool,
    brave_debloat_loaded: bool,
    edge_debloat_enabled: bool,
    edge_debloat_loaded: bool,
    widgets_removed_enabled: bool,
    widgets_removed_loaded: bool,
    ctfmon_blocked_enabled: bool,
    ctfmon_blocked_loaded: bool,
    ctfmon_service_dll_enabled: bool,
    ctfmon_service_dll_open: bool,
    timer_resolution_enabled: bool,
    timer_resolution_loaded: bool,
    ipv6_preferred_enabled: bool,
    ipv6_preferred_loaded: bool,
    teredo_disabled_enabled: bool,
    teredo_disabled_loaded: bool,
    powershell7_telemetry_enabled: bool,
    powershell7_telemetry_loaded: bool,
    hpet_preferred_enabled: bool,
    hpet_preferred_loaded: bool,
}

struct DownloadsLoadResult {
    installed: Vec<bool>,
}

struct DownloadsLoadWorker {
    rx: Receiver<DownloadsLoadResult>,
}

struct DownloadScanCache {
    winget_ids: HashSet<String>,
    winget_names: HashSet<String>,
    registry_names: Vec<String>,
    cached_at: Instant,
}

static DOWNLOAD_SCAN_CACHE: LazyLock<Mutex<Option<DownloadScanCache>>> =
    LazyLock::new(|| Mutex::new(None));

struct DownloadInstallResult {
    ok: usize,
    fail: usize,
}

struct DownloadInstallWorker {
    rx: Receiver<DownloadInstallResult>,
}

struct RestorePointLoadWorker {
    rx: Receiver<RestorePointResult>,
}

struct RepairLoadWorker {
    rx: Receiver<RepairEvent>,
}

struct SettingsActionLoadWorker {
    rx: Receiver<SettingsActionEvent>,
}

struct ExtrasBoolWorker {
    rx: Receiver<Result<(), String>>,
}

struct ExtrasLoadWorker {
    rx: Receiver<ExtrasState>,
}

#[derive(Clone)]
enum RepairDialog {
    Progress { stage: String, log: Vec<String> },
    Result { title: String, message: String },
}

#[derive(Clone)]
enum SettingsActionDialog {
    Progress {
        title: String,
        stage: String,
        log: Vec<String>,
    },
    Result {
        title: String,
        message: String,
    },
}

enum RepairEvent {
    Stage(String),
    Log(String),
    Finished(RepairResult),
}

enum SettingsActionEvent {
    Stage(String),
    Log(String),
    Finished(SettingsActionResult),
}

struct RepairResult {
    success: bool,
    message: String,
}

struct SettingsActionResult {
    title: String,
    message: String,
}

#[derive(Clone)]
enum RestorePointDialog {
    Progress,
    Result { title: String, message: String },
}

struct RestorePointResult {
    success: bool,
    message: String,
}

static CPU_TREE_EXPANDED: Mutex<Option<HashSet<i32>>> = Mutex::new(None);

static CPU_PRIORITY_PENDING_PID: Mutex<i32> = Mutex::new(-1);

#[derive(Clone, Copy, PartialEq, Eq)]
enum DownloadAction {
    Install,
}

enum UpdateCheckResult {
    UpToDate,
    UpdateAvailable(String),
    Error(String),
}

#[derive(Clone)]
enum UpdateDialog {
    UpdateAvailable { latest_version: String },
    Error { message: String },
}

struct IconCache {
    warehouse: Option<char>,
    star: Option<char>,
    shield_check: Option<char>,
    badge_info: Option<char>,
    refresh_cw: Option<char>,
    heart: Option<char>,
    bug: Option<char>,
}

impl Default for IconCache {
    fn default() -> Self {
        use iconflow::{Pack, Size, Style, try_icon};
        Self {
            warehouse: try_icon(Pack::Lucide, "warehouse", Style::Regular, Size::Regular)
                .ok()
                .and_then(|i| char::from_u32(i.codepoint)),
            star: try_icon(Pack::Lucide, "star", Style::Regular, Size::Regular)
                .ok()
                .and_then(|i| char::from_u32(i.codepoint)),
            shield_check: try_icon(Pack::Lucide, "shield-check", Style::Regular, Size::Regular)
                .ok()
                .and_then(|i| char::from_u32(i.codepoint)),
            badge_info: try_icon(Pack::Lucide, "badge-info", Style::Regular, Size::Regular)
                .ok()
                .and_then(|i| char::from_u32(i.codepoint)),
            refresh_cw: try_icon(Pack::Lucide, "refresh-cw", Style::Regular, Size::Regular)
                .ok()
                .and_then(|i| char::from_u32(i.codepoint)),
            heart: try_icon(Pack::Lucide, "heart", Style::Regular, Size::Regular)
                .ok()
                .and_then(|i| char::from_u32(i.codepoint)),
            bug: try_icon(Pack::Lucide, "bug", Style::Regular, Size::Regular)
                .ok()
                .and_then(|i| char::from_u32(i.codepoint)),
        }
    }
}

pub struct WinchiselApp {
    state: AppState,
    system: sysinfo::System,
    initialized_style: bool,
    icon_cache: IconCache,
    debloater_load_worker: Option<debloater::DebloaterLoadWorker>,
    debloater_cache_ready: bool,
    pending_debloater_action: Option<debloater::DebloaterAction>,
    pending_download_action: Option<DownloadAction>,
    downloads_load_worker: Option<DownloadsLoadWorker>,
    downloads_install_worker: Option<DownloadInstallWorker>,
    downloads_cache_ready: bool,
    performance_load_worker: Option<performance_tab::PerformanceLoadWorker>,
    privacy_load_worker: Option<privacy_security::PrivacySecurityLoadWorker>,
    cpu_load_worker: Option<processes_tab::CpuLoadWorker>,
    latency_load_worker: Option<latency_tab::LatencyLoadWorker>,
    restore_point_load_worker: Option<RestorePointLoadWorker>,
    restore_point_dialog: Option<RestorePointDialog>,
    repair_load_worker: Option<RepairLoadWorker>,
    repair_dialog: Option<RepairDialog>,
    settings_action_load_worker: Option<SettingsActionLoadWorker>,
    settings_action_dialog: Option<SettingsActionDialog>,
    extras_teredo_worker: Option<ExtrasBoolWorker>,
    extras_hpet_worker: Option<ExtrasBoolWorker>,
    extras_load_worker: Option<ExtrasLoadWorker>,
    toasts: Toasts,
    show_log_window: bool,
    update_check_rx: Option<Receiver<UpdateCheckResult>>,
    pending_update_dialog: Option<UpdateDialog>,
    update_dialog_on_complete: bool,
    settings_save_due_at: Option<Instant>,
    settings_save_snapshot: AppSettings,
}

impl WinchiselApp {
    fn tr(&self, key: &str) -> &'static str {
        t(self.state.settings.language, key)
    }

    fn sidebar_tab_label(icon: &str, label: &str) -> String {
        if let Ok(icon) = try_icon(Pack::Lucide, icon, Style::Regular, Size::Regular) {
            let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
            format!("{glyph}  {label}")
        } else {
            label.to_string()
        }
    }

    pub fn new(settings: AppSettings, is_admin: bool) -> Self {
        let mut system = sysinfo::System::new_all();
        let debloater_items = get_all_apps(settings.language);
        let (debloater_load_worker, debloater_loading) =
            Self::spawn_debloater_worker(debloater_items.clone());
        let download_items = get_all_downloads(settings.language);
        let (downloads_load_worker, downloads_loading) =
            Self::spawn_downloads_worker(download_items.clone());
        let (performance_load_worker, performance_loaded) =
            Self::spawn_performance_worker("", settings.language);
        let (privacy_load_worker, privacy_loaded) =
            Self::spawn_privacy_security_worker(settings.language);
        Self {
            state: AppState {
                last_saved_settings: settings.clone(),
                debloater: debloater::DebloaterState {
                    debloater_query: String::new(),
                    debloater_items,
                    debloater_selected: Vec::new(),
                    debloater_installed: Vec::new(),
                    debloater_tab: 0,
                    debloater_view_mode: 0,
                    debloater_loading,
                    debloater_filter_cache_query: String::new(),
                    debloater_filter_cache_tab: usize::MAX,
                    debloater_filter_cache_view_mode: usize::MAX,
                    debloater_filter_cache_all: Vec::new(),
                },
                settings: settings.clone(),
                is_admin,
                active_tab: Tab::Home,
                update_status: t(settings.language, "update_ready").to_string(),
                update_check_loading: false,
                update_check_started: false,
                downloads: DownloadsState {
                    downloads_query: String::new(),
                    downloads_items: download_items,
                    downloads_selected: Vec::new(),
                    downloads_installed: Vec::new(),
                    downloads_view_mode: 0,
                    downloads_loading,
                    downloads_filter_cache_query: String::new(),
                    downloads_filter_cache_view_mode: usize::MAX,
                    downloads_filter_cache_all: Vec::new(),
                    downloads_filter_cache_categories: vec![Vec::new(); 16],
                },
                performance: performance_tab::PerformanceState {
                    performance_query: String::new(),
                    performance_loaded,
                    performance_groups: Default::default(),
                    performance_quick_action_index: 0,
                },
                privacy_security: privacy_security::PrivacySecurityState {
                    privacy_query: String::new(),
                    privacy_quick_action_index: 0,
                    groups: Default::default(),
                    loaded: privacy_loaded,
                },
                home: Self::build_home_state(&mut system, settings.language),
                cpu: processes_tab::CpuState {
                    cpu_filter_mode: 1,
                    cpu_visible_count: 0,
                    cpu_total_usage: "0.0%".to_string(),
                    cpu_processes_all: Vec::new(),
                    cpu_processes: Vec::new(),
                    cpu_selected_pid: -1,
                    cpu_selected_name: String::new(),
                    cpu_affinity_dialog_visible: false,
                    cpu_affinity_dialog_pid: -1,
                    cpu_affinity_dialog_name: String::new(),
                    cpu_affinity_dialog_mask: String::new(),
                    cpu_affinity_cores: Vec::new(),
                    cpu_last_refresh: None,
                    cpu_reload_ready_at: None,
                    cpu_reload_pending: false,
                    cpu_last_error: None,
                    cpu_pending_action: None,
                    cpu_realtime_confirm_visible: false,
                    cpu_realtime_pending_pid: -1,
                    cpu_realtime_pending_name: String::new(),
                },
                latency: latency_tab::LatencyState {
                    latency_loading: false,
                    latency_completion_pending: false,
                    latency_progress: 0,
                    latency_progress_display: 0.0,
                    latency_status: String::new(),
                    latency_lines: Vec::new(),
                    latency_pending_lines: Vec::new(),
                    latency_tick_next_at: None,
                    latency_completion_ready_at: None,
                },
                extras: ExtrasState {
                    brave_debloat_enabled: false,
                    brave_debloat_loaded: false,
                    edge_debloat_enabled: false,
                    edge_debloat_loaded: false,
                    widgets_removed_enabled: false,
                    widgets_removed_loaded: false,
                    ctfmon_blocked_enabled: false,
                    ctfmon_blocked_loaded: false,
                    ctfmon_service_dll_enabled: false,
                    ctfmon_service_dll_open: true,
                    timer_resolution_enabled: false,
                    timer_resolution_loaded: false,
                    ipv6_preferred_enabled: false,
                    ipv6_preferred_loaded: false,
                    teredo_disabled_enabled: false,
                    teredo_disabled_loaded: false,
                    powershell7_telemetry_enabled: false,
                    powershell7_telemetry_loaded: false,
                    hpet_preferred_enabled: false,
                    hpet_preferred_loaded: false,
                },
                home_last_refresh: None,
            },
            system,
            icon_cache: IconCache::default(),
            initialized_style: false,
            debloater_load_worker,
            debloater_cache_ready: false,
            pending_debloater_action: None,
            pending_download_action: None,
            downloads_load_worker,
            downloads_install_worker: None,
            downloads_cache_ready: false,
            performance_load_worker,
            privacy_load_worker,
            cpu_load_worker: None,
            latency_load_worker: None,
            restore_point_load_worker: None,
            restore_point_dialog: None,
            repair_load_worker: None,
            repair_dialog: None,
            settings_action_load_worker: None,
            settings_action_dialog: None,
            extras_teredo_worker: None,
            extras_hpet_worker: None,
            extras_load_worker: None,
            toasts: Toasts::default().with_anchor(egui_notify::Anchor::BottomRight),
            show_log_window: false,
            update_check_rx: None,
            pending_update_dialog: None,
            update_dialog_on_complete: false,
            settings_save_due_at: None,
            settings_save_snapshot: settings,
        }
    }

    fn init_style(&mut self, ctx: &egui::Context) {
        if self.initialized_style {
            return;
        }
        self.initialized_style = true;

        let mut style = egui::Style::default();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.window_margin = egui::Margin::same(10);
        style.visuals = egui::Visuals::dark();
        ctx.set_global_style(style);
        Self::install_icon_fonts(ctx);
    }

    fn sync_settings(&mut self) {
        let settings_changed = self.state.settings.check_updates_on_startup
            != self.settings_save_snapshot.check_updates_on_startup
            || self.state.settings.show_console != self.settings_save_snapshot.show_console
            || self.state.settings.language != self.settings_save_snapshot.language;

        if settings_changed {
            self.settings_save_snapshot = self.state.settings.clone();
            self.settings_save_due_at = Some(Instant::now() + Duration::from_millis(600));
        }

        let due = self
            .settings_save_due_at
            .is_some_and(|due_at| Instant::now() >= due_at);

        if due {
            save_app_settings(&self.state.settings);
            crate::set_console_visibility(self.state.settings.show_console);
            self.state.last_saved_settings = self.state.settings.clone();
            self.settings_save_due_at = None;
            self.toasts
                .info(self.tr("settings_saved"))
                .duration(Duration::from_secs_f64(2.5));
        }
    }

    fn ensure_download_selection(&mut self) {
        if self.state.downloads.downloads_selected.len()
            != self.state.downloads.downloads_items.len()
        {
            self.state.downloads.downloads_selected =
                vec![false; self.state.downloads.downloads_items.len()];
        }
        if self.state.downloads.downloads_installed.len()
            != self.state.downloads.downloads_items.len()
        {
            self.state.downloads.downloads_installed =
                vec![false; self.state.downloads.downloads_items.len()];
        }
        if self.state.downloads.downloads_filter_cache_categories.len() != 16 {
            self.state.downloads.downloads_filter_cache_categories = vec![Vec::new(); 16];
        }
    }

    fn spawn_downloads_worker(items: Vec<DownloadApp>) -> (Option<DownloadsLoadWorker>, bool) {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let cache_ttl = Duration::from_secs(600);
            let cached = DOWNLOAD_SCAN_CACHE.lock().ok().and_then(|guard| {
                guard.as_ref().and_then(|entry| {
                    (entry.cached_at.elapsed() < cache_ttl).then(|| {
                        (
                            entry.winget_ids.clone(),
                            entry.winget_names.clone(),
                            entry.registry_names.clone(),
                        )
                    })
                })
            });

            let (winget_ids, winget_names, registry_names) = if let Some(cache) = cached {
                cache
            } else {
                let winget_ids_handle = std::thread::spawn(Self::get_winget_installed_ids);
                let winget_names_handle = std::thread::spawn(Self::get_winget_installed_names);
                let registry_names_handle = std::thread::spawn(Self::get_registry_installed_names);

                let winget_ids = winget_ids_handle.join().unwrap_or_default();
                let winget_names = winget_names_handle.join().unwrap_or_default();
                let registry_names = registry_names_handle.join().unwrap_or_default();

                if let Ok(mut guard) = DOWNLOAD_SCAN_CACHE.lock() {
                    *guard = Some(DownloadScanCache {
                        winget_ids: winget_ids.clone(),
                        winget_names: winget_names.clone(),
                        registry_names: registry_names.clone(),
                        cached_at: Instant::now(),
                    });
                }

                (winget_ids, winget_names, registry_names)
            };

            let installed = items
                .iter()
                .map(|item| {
                    Self::is_download_installed(item, &winget_ids, &winget_names, &registry_names)
                })
                .collect();
            let _ = tx.send(DownloadsLoadResult { installed });
        });
        (Some(DownloadsLoadWorker { rx }), true)
    }

    fn start_downloads_load(&mut self) {
        if self.downloads_cache_ready
            || self.downloads_load_worker.is_some()
            || self.state.downloads.downloads_loading
        {
            return;
        }
        let (worker, loading) =
            Self::spawn_downloads_worker(self.state.downloads.downloads_items.clone());
        self.downloads_load_worker = worker;
        self.state.downloads.downloads_loading = loading;
    }

    fn poll_downloads_load(&mut self) {
        let Some(worker) = self.downloads_load_worker.as_ref() else {
            return;
        };
        match worker.rx.try_recv() {
            Ok(result) => {
                self.state.downloads.downloads_installed = result.installed;
                self.state.downloads.downloads_loading = false;
                self.downloads_cache_ready = true;
                self.downloads_load_worker = None;
                self.state.downloads.downloads_filter_cache_query.clear();
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.state.downloads.downloads_loading = false;
                self.downloads_load_worker = None;
            }
        }
    }

    fn ps_escape(value: &str) -> String {
        value.replace('\'', "''")
    }

    fn ps_lines(cmd: &str) -> Vec<String> {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let mut command = Command::new("powershell.exe");
        command.args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            cmd,
        ]);
        #[cfg(windows)]
        {
            command.creation_flags(CREATE_NO_WINDOW);
        }
        let output = match command.output() {
            Ok(output) => output,
            Err(_) => return Vec::new(),
        };
        if !output.status.success() {
            return Vec::new();
        }
        String::from_utf8(output.stdout)
            .unwrap_or_default()
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn get_winget_installed_ids() -> HashSet<String> {
        let cmd = r#"
$tmp = [System.IO.Path]::GetTempFileName() + '.json'
$null = winget export -o $tmp --accept-source-agreements --nowarn --disable-interactivity 2>$null
if (Test-Path $tmp) {
    try {
        $j = Get-Content $tmp -Raw | ConvertFrom-Json
        foreach ($src in $j.Sources) {
            foreach ($pkg in $src.Packages) {
                $pkg.PackageIdentifier
            }
        }
    } catch {}
    Remove-Item $tmp -Force -ErrorAction SilentlyContinue
}
"#;
        Self::ps_lines(cmd)
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect()
    }

    fn get_winget_installed_names() -> HashSet<String> {
        let cmd = r#"
$ErrorActionPreference = 'SilentlyContinue'
winget list --accept-source-agreements --disable-interactivity | Select-Object -Skip 2 | ForEach-Object {
    $line = $_.ToString().Trim()
    if ($line -ne '') {
        ($line -split '\s{2,}')[0]
    }
}
"#;
        Self::ps_lines(cmd)
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect()
    }

    fn get_registry_installed_names() -> Vec<String> {
        let cmd = r#"
$paths = [System.Collections.Generic.List[string]]@(
    'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall',
    'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall'
)
if (-not (Get-PSDrive -Name HKU -ErrorAction SilentlyContinue)) {
    New-PSDrive -Name HKU -PSProvider Registry -Root HKEY_USERS -ErrorAction SilentlyContinue | Out-Null
}
Get-ChildItem 'HKU:\' -ErrorAction SilentlyContinue | Where-Object { $_.PSChildName -notmatch '_Classes$' } | ForEach-Object {
    $paths.Add("HKU:\$($_.PSChildName)\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall")
    $paths.Add("HKU:\$($_.PSChildName)\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall")
}
foreach ($p in $paths) {
    if (Test-Path $p) {
        Get-ChildItem $p -ErrorAction SilentlyContinue | ForEach-Object {
            (Get-ItemProperty $_.PSPath -Name DisplayName -ErrorAction SilentlyContinue).DisplayName
        }
    }
}
        "#;
        Self::ps_lines(cmd)
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect()
    }

    fn is_download_installed(
        item: &DownloadApp,
        winget_ids: &HashSet<String>,
        winget_names: &HashSet<String>,
        registry_names: &[String],
    ) -> bool {
        if item
            .winget_ids
            .iter()
            .map(|id| id.to_lowercase())
            .any(|id| !id.is_empty() && winget_ids.contains(&id))
        {
            return true;
        }
        let name_lower = item.name.to_lowercase();
        if winget_names
            .iter()
            .any(|installed| installed == &name_lower)
        {
            return true;
        }
        registry_names
            .iter()
            .any(|installed| installed.contains(&name_lower) || name_lower.contains(installed))
    }

    fn category_label_download(&self, category: &DownloadCategory) -> String {
        match category {
            DownloadCategory::Browsers => self.tr("download_category_0").to_string(),
            DownloadCategory::DocumentViewers => self.tr("download_category_1").to_string(),
            DownloadCategory::MessagingEmailCalendar => self.tr("download_category_2").to_string(),
            DownloadCategory::OnlineStorageBackup => self.tr("download_category_3").to_string(),
            DownloadCategory::Multimedia => self.tr("download_category_4").to_string(),
            DownloadCategory::Imaging => self.tr("download_category_5").to_string(),
            DownloadCategory::CustomizationUtilities => self.tr("download_category_6").to_string(),
            DownloadCategory::Gaming => self.tr("download_category_7").to_string(),
            DownloadCategory::Compression => self.tr("download_category_8").to_string(),
            DownloadCategory::FileDiskManagement => self.tr("download_category_9").to_string(),
            DownloadCategory::RemoteAccess => self.tr("download_category_10").to_string(),
            DownloadCategory::OpticalDiscTools => self.tr("download_category_11").to_string(),
            DownloadCategory::OtherUtilities => self.tr("download_category_12").to_string(),
            DownloadCategory::PrivacySecurity => self.tr("download_category_13").to_string(),
            DownloadCategory::DevelopmentApps => self.tr("download_category_14").to_string(),
            DownloadCategory::RuntimesDependencies => self.tr("download_category_15").to_string(),
        }
    }
}

impl eframe::App for WinchiselApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.init_style(ui.ctx());
        ui.ctx().plugin_or_default::<egui_async::EguiAsyncPlugin>();
        let mut repaint_after: Option<Duration> = None;
        self.poll_latency_worker();
        self.ensure_debloater_selection();
        self.ensure_download_selection();
        self.poll_debloater_load();
        self.poll_downloads_load();
        self.poll_download_install();
        self.poll_performance_load();
        self.poll_cpu_load();
        self.poll_restore_point();
        self.poll_system_repair();
        self.poll_settings_actions();
        self.poll_update_check();
        if self.state.active_tab == Tab::Extras
            && !self.state.extras.brave_debloat_loaded
            && self.extras_load_worker.is_none()
        {
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let _ = tx.send(Self::load_extras_state());
            });
            self.extras_load_worker = Some(ExtrasLoadWorker { rx });
        }
        if let Some(worker) = self.extras_load_worker.as_ref() {
            match worker.rx.try_recv() {
                Ok(state) => {
                    let old_open = self.state.extras.ctfmon_service_dll_open;
                    self.state.extras = state;
                    self.state.extras.ctfmon_service_dll_open = old_open;
                    self.extras_load_worker = None;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    Self::bump_repaint_after(&mut repaint_after, Duration::from_millis(50));
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.extras_load_worker = None;
                }
            }
        }
        if self.state.settings.check_updates_on_startup
            && !self.state.update_check_started
            && !self.state.update_check_loading
        {
            self.state.update_check_started = true;
            self.start_update_check(true);
        }
        if self.settings_save_due_at.is_some() {
            Self::bump_repaint_after(&mut repaint_after, Duration::from_millis(50));
        }
        WinchiselApp::performance_tick(self, ui);
        WinchiselApp::privacy_tick(self, ui);
        if self.state.active_tab == Tab::Processes
            && self.state.cpu.cpu_last_refresh.is_none()
            && self.cpu_load_worker.is_none()
            && !self.state.cpu.cpu_reload_pending
        {
            self.request_cpu_reload();
        }
        if self.state.cpu.cpu_reload_pending
            && self.cpu_load_worker.is_none()
            && self
                .state
                .cpu
                .cpu_reload_ready_at
                .is_some_and(|ready_at| Instant::now() >= ready_at)
        {
            self.state.cpu.cpu_reload_pending = false;
            self.state.cpu.cpu_reload_ready_at = None;
            self.start_cpu_load();
        }
        if self.state.active_tab == Tab::Home {
            let refresh_due = self
                .state
                .home_last_refresh
                .map(|last| last.elapsed() >= Duration::from_secs(5))
                .unwrap_or(true);
            if refresh_due {
                let lang = self.state.settings.language;
                self.state.home = Self::build_home_state(&mut self.system, lang);
                self.state.home_last_refresh = Some(Instant::now());
                Self::bump_repaint_after(&mut repaint_after, Duration::from_millis(100));
            } else if let Some(last) = self.state.home_last_refresh {
                let elapsed = last.elapsed();
                let until_refresh = Duration::from_secs(5).saturating_sub(elapsed);
                let next = until_refresh.min(Duration::from_millis(250));
                Self::bump_repaint_after(&mut repaint_after, next);
            }
        }
        if self.state.debloater.debloater_loading || self.debloater_load_worker.is_some() {
            Self::bump_repaint_after(&mut repaint_after, Duration::from_millis(50));
        }
        if self.state.downloads.downloads_loading || self.downloads_load_worker.is_some() {
            Self::bump_repaint_after(&mut repaint_after, Duration::from_millis(50));
        }
        if WinchiselApp::performance_sidebar_loading(self) {
            Self::bump_repaint_after(&mut repaint_after, Duration::from_millis(50));
        }
        if WinchiselApp::privacy_sidebar_loading(self) {
            Self::bump_repaint_after(&mut repaint_after, Duration::from_millis(50));
        }
        if self.cpu_load_worker.is_some() {
            Self::bump_repaint_after(&mut repaint_after, Duration::from_millis(50));
        }
        if self.state.update_check_loading || self.update_check_rx.is_some() {
            Self::bump_repaint_after(&mut repaint_after, Duration::from_millis(50));
        }

        if let Some(delay) = repaint_after {
            ui.ctx().request_repaint_after(delay);
        }
        egui::Panel::top("top_bar").show_inside(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading(format!("Winchisel v{}", env!("CARGO_PKG_VERSION")));
                    ui.horizontal(|ui| {
                        if let Some(glyph) = self.icon_cache.shield_check {
                            ui.label(
                                egui::RichText::new(glyph.to_string())
                                    .color(egui::Color32::from_rgb(96, 181, 103)),
                            );
                        }
                        let admin = if self.state.is_admin {
                            self.tr("status_admin")
                        } else {
                            self.tr("status_standard")
                        };
                        ui.colored_label(egui::Color32::from_rgb(96, 181, 103), admin);
                        ui.separator();
                        if let Some(glyph) = self.icon_cache.badge_info {
                            ui.label(
                                egui::RichText::new(glyph.to_string())
                                    .color(egui::Color32::from_rgb(166, 166, 166)),
                            );
                        }
                        let status_color =
                            if self.state.update_status == self.tr("update_up_to_date") {
                                egui::Color32::from_rgb(96, 181, 103)
                            } else if self
                                .state
                                .update_status
                                .contains(self.tr("update_available_prefix"))
                            {
                                egui::Color32::from_rgb(226, 196, 84)
                            } else {
                                egui::Color32::from_rgb(166, 166, 166)
                            };
                        ui.colored_label(status_color, &self.state.update_status);
                    });
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let update_button_text = if let Some(glyph) = self.icon_cache.refresh_cw {
                        format!("{glyph}  {}", self.tr("check_updates"))
                    } else {
                        self.tr("check_updates").to_string()
                    };
                    if ui
                        .add_sized(
                            [128.0, 34.0],
                            egui::Button::new(update_button_text)
                                .fill(egui::Color32::from_rgb(35, 54, 80))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(10, 210, 254),
                                )),
                        )
                        .clicked()
                    {
                        self.start_update_check(true);
                    }
                    let donate_button_text = if let Some(glyph) = self.icon_cache.heart {
                        format!("{glyph}  {}", self.tr("donate"))
                    } else {
                        self.tr("donate").to_string()
                    };
                    if ui
                        .add_sized(
                            [102.0, 34.0],
                            egui::Button::new(donate_button_text)
                                .fill(egui::Color32::from_rgb(82, 48, 92))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(165, 106, 183),
                                )),
                        )
                        .clicked()
                    {
                        crate::open_url("https://ko-fi.com/tekkubot");
                    }
                    let bug_report_text = if let Some(glyph) = self.icon_cache.bug {
                        format!("{glyph}  {}", self.tr("bug_report"))
                    } else {
                        self.tr("bug_report").to_string()
                    };
                    if ui
                        .add_sized(
                            [118.0, 34.0],
                            egui::Button::new(bug_report_text)
                                .fill(egui::Color32::from_rgb(92, 36, 36))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(184, 78, 78),
                                )),
                        )
                        .clicked()
                    {
                        crate::open_url("https://github.com/lyrx2k/winchisel/issues");
                    }
                });
            });
        });

        egui::Panel::left("sidebar")
            .resizable(false)
            .exact_size(228.0)
            .show_inside(ui, |ui| {
                let tab_home = self.tr("home").to_string();
                let tab_debloater = self.tr("debloater").to_string();
                let tab_downloads = self.tr("downloads").to_string();
                let tab_performance = self.tr("performance").to_string();
                let tab_processes = self.tr("processes").to_string();
                let tab_latency = self.tr("latency").to_string();
                let tab_privacy_security = self.tr("privacy_security").to_string();
                let tab_settings = self.tr("settings").to_string();
                let tab_extras = self.tr("extras").to_string();
                let privacy_loading = WinchiselApp::privacy_sidebar_loading(self);
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.heading(self.tr("nav_title"));
                    ui.label(self.tr("nav_subtitle"));
                });
                ui.add_space(16.0);
                Self::tab_button(
                    ui,
                    &mut self.state.active_tab,
                    Tab::Home,
                    &Self::sidebar_tab_label("house", &tab_home),
                    false,
                );
                let performance_loading = WinchiselApp::performance_sidebar_loading(self);
                Self::tab_button(
                    ui,
                    &mut self.state.active_tab,
                    Tab::Debloater,
                    &Self::sidebar_tab_label("eraser", &tab_debloater),
                    self.state.debloater.debloater_loading || self.debloater_load_worker.is_some(),
                );
                Self::tab_button(
                    ui,
                    &mut self.state.active_tab,
                    Tab::Performance,
                    &Self::sidebar_tab_label("gauge", &tab_performance),
                    performance_loading,
                );
                Self::tab_button(
                    ui,
                    &mut self.state.active_tab,
                    Tab::SecurityPrivacy,
                    &Self::sidebar_tab_label("shield-check", &tab_privacy_security),
                    privacy_loading,
                );
                Self::tab_button(
                    ui,
                    &mut self.state.active_tab,
                    Tab::Downloads,
                    &Self::sidebar_tab_label("download", &tab_downloads),
                    self.state.downloads.downloads_loading || self.downloads_load_worker.is_some(),
                );
                Self::tab_button(
                    ui,
                    &mut self.state.active_tab,
                    Tab::Processes,
                    &Self::sidebar_tab_label("list-tree", &tab_processes),
                    self.cpu_load_worker.is_some() && self.state.cpu.cpu_last_refresh.is_none(),
                );
                Self::tab_button(
                    ui,
                    &mut self.state.active_tab,
                    Tab::Latency,
                    &Self::sidebar_tab_label("clock-3", &tab_latency),
                    false,
                );
                Self::tab_button(
                    ui,
                    &mut self.state.active_tab,
                    Tab::Extras,
                    &Self::sidebar_tab_label("folder-plus", &tab_extras),
                    false,
                );
                Self::tab_button(
                    ui,
                    &mut self.state.active_tab,
                    Tab::Settings,
                    &Self::sidebar_tab_label("settings-2", &tab_settings),
                    false,
                );
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add_space(4.0);
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| match self.state.active_tab {
                    Tab::Home => self.render_home_tab(ui),
                    Tab::Debloater => self.render_debloater_tab(ui),
                    Tab::Downloads => self.render_downloads_tab(ui),
                    Tab::Performance => self.render_performance_tab(ui),
                    Tab::Processes => self.render_processes_tab(ui),
                    Tab::Latency => self.render_latency_tab(ui),
                    Tab::SecurityPrivacy => self.render_privacy_security_tab(ui),
                    Tab::Settings => self.render_settings_tab(ui),
                    Tab::Extras => self.render_extras_tab(ui),
                });
        });

        self.poll_privacy_security_load();

        self.show_debloater_dialog(ui.ctx());

        if let Some(DownloadAction::Install) = self.pending_download_action {
            egui::Window::new(self.tr("download_confirm_title"))
                .collapsible(false)
                .resizable(false)
                .default_width(420.0)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ui.ctx(), |ui| {
                    let items = self.selected_download_items();
                    ui.label(self.tr("download_confirm_desc"));
                    ui.add_space(8.0);
                    egui::ScrollArea::vertical()
                        .max_height(220.0)
                        .show(ui, |ui| {
                            for item in &items {
                                ui.label(&item.name);
                            }
                        });
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button(self.tr("download_confirm_install")).clicked() {
                            self.pending_download_action = None;
                            self.start_download_install();
                        }
                        if ui.button(self.tr("download_cancel")).clicked() {
                            self.pending_download_action = None;
                        }
                    });
                });
        }

        self.show_update_dialog(ui.ctx());
        self.show_restore_point_dialog(ui.ctx());
        self.show_system_repair_dialog(ui.ctx());
        self.show_settings_action_dialog(ui.ctx());
        self.show_toast_layer(ui);
        self.show_log_window(ui.ctx());

        self.sync_settings();
    }

    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}

impl WinchiselApp {
    fn bump_repaint_after(slot: &mut Option<Duration>, next: Duration) {
        *slot = Some(slot.map_or(next, |cur| cur.min(next)));
    }

    fn load_extras_state() -> ExtrasState {
        ExtrasState {
            brave_debloat_enabled: Self::brave_debloat_enabled(),
            brave_debloat_loaded: true,
            edge_debloat_enabled: Self::edge_debloat_enabled(),
            edge_debloat_loaded: true,
            widgets_removed_enabled: Self::widgets_removed_enabled(),
            widgets_removed_loaded: true,
            ctfmon_blocked_enabled: Self::ctfmon_blocked_enabled(),
            ctfmon_blocked_loaded: true,
            ctfmon_service_dll_enabled: Self::ctfmon_service_dll_enabled(),
            ctfmon_service_dll_open: true,
            timer_resolution_enabled: Self::timer_resolution_enabled(),
            timer_resolution_loaded: true,
            ipv6_preferred_enabled: Self::ipv6_preferred_enabled(),
            ipv6_preferred_loaded: true,
            teredo_disabled_enabled: Self::teredo_disabled_enabled(),
            teredo_disabled_loaded: true,
            powershell7_telemetry_enabled: Self::powershell7_telemetry_enabled(),
            powershell7_telemetry_loaded: true,
            hpet_preferred_enabled: Self::hpet_preferred_enabled(),
            hpet_preferred_loaded: true,
        }
    }

    #[cfg(windows)]
    fn brave_debloat_enabled() -> bool {
        use winreg::enums::*;
        let root = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let Ok(key) =
            root.open_subkey_with_flags(r"SOFTWARE\Policies\BraveSoftware\Brave", KEY_READ)
        else {
            return false;
        };
        let read_dword = |name: &str| -> Option<u32> { key.get_value(name).ok() };
        read_dword("BraveRewardsDisabled") == Some(1)
            && read_dword("BraveWalletDisabled") == Some(1)
            && read_dword("BraveVPNDisabled") == Some(1)
            && read_dword("BraveAIChatEnabled") == Some(0)
            && read_dword("BraveStatsPingEnabled") == Some(0)
    }

    #[cfg(windows)]
    fn apply_brave_debloat(enabled: bool) -> Result<(), String> {
        use winreg::enums::*;
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let (key, _) = hklm
            .create_subkey(r"SOFTWARE\Policies\BraveSoftware\Brave")
            .map_err(|e| format!("failed to open Brave policy key: {e}"))?;

        let dword = |key: &winreg::RegKey, name: &str, value: u32| -> Result<(), String> {
            key.set_value(name, &value)
                .map_err(|e| format!("failed to set {name}: {e}"))
        };
        let remove = |key: &winreg::RegKey, name: &str| -> Result<(), String> {
            match key.delete_value(name) {
                Ok(_) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(e) => Err(format!("failed to remove {name}: {e}")),
            }
        };

        if enabled {
            dword(&key, "BraveRewardsDisabled", 1)?;
            dword(&key, "BraveWalletDisabled", 1)?;
            dword(&key, "BraveVPNDisabled", 1)?;
            dword(&key, "BraveAIChatEnabled", 0)?;
            dword(&key, "BraveStatsPingEnabled", 0)?;
            dword(&key, "BraveNewsDisabled", 1)?;
            dword(&key, "BraveTalkDisabled", 1)?;
            dword(&key, "TorDisabled", 1)?;
            dword(&key, "BraveP3AEnabled", 0)?;
            dword(&key, "UrlKeyedAnonymizedDataCollectionEnabled", 0)?;
            dword(&key, "SafeBrowsingExtendedReportingEnabled", 0)?;
            dword(&key, "MetricsReportingEnabled", 0)?;
        } else {
            for name in [
                "BraveRewardsDisabled",
                "BraveWalletDisabled",
                "BraveVPNDisabled",
                "BraveAIChatEnabled",
                "BraveStatsPingEnabled",
                "BraveNewsDisabled",
                "BraveTalkDisabled",
                "TorDisabled",
                "BraveP3AEnabled",
                "UrlKeyedAnonymizedDataCollectionEnabled",
                "SafeBrowsingExtendedReportingEnabled",
                "MetricsReportingEnabled",
            ] {
                remove(&key, name)?;
            }
        }
        Ok(())
    }

    #[cfg(windows)]
    fn edge_debloat_enabled() -> bool {
        use winreg::enums::*;
        let root = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let Ok(edge) = root.open_subkey_with_flags(r"SOFTWARE\Policies\Microsoft\Edge", KEY_READ)
        else {
            return false;
        };
        let Ok(edge_update) =
            root.open_subkey_with_flags(r"SOFTWARE\Policies\Microsoft\EdgeUpdate", KEY_READ)
        else {
            return false;
        };
        let Ok(blocklist) = root.open_subkey_with_flags(
            r"SOFTWARE\Policies\Microsoft\Edge\ExtensionInstallBlocklist",
            KEY_READ,
        ) else {
            return false;
        };
        let read_dword =
            |key: &winreg::RegKey, name: &str| -> Option<u32> { key.get_value(name).ok() };
        let read_string =
            |key: &winreg::RegKey, name: &str| -> Option<String> { key.get_value(name).ok() };

        read_dword(&edge_update, "CreateDesktopShortcutDefault") == Some(0)
            && read_dword(&edge, "PersonalizationReportingEnabled") == Some(0)
            && read_string(&blocklist, "1").as_deref() == Some("ofefcgjbeghpigppfmkologfjadafddi")
            && read_dword(&edge, "ShowRecommendationsEnabled") == Some(0)
            && read_dword(&edge, "HideFirstRunExperience") == Some(1)
            && read_dword(&edge, "UserFeedbackAllowed") == Some(0)
            && read_dword(&edge, "ConfigureDoNotTrack") == Some(1)
            && read_dword(&edge, "AlternateErrorPagesEnabled") == Some(0)
            && read_dword(&edge, "EdgeCollectionsEnabled") == Some(0)
            && read_dword(&edge, "EdgeShoppingAssistantEnabled") == Some(0)
            && read_dword(&edge, "MicrosoftEdgeInsiderPromotionEnabled") == Some(0)
            && read_dword(&edge, "ShowMicrosoftRewards") == Some(0)
            && read_dword(&edge, "WebWidgetAllowed") == Some(0)
            && read_dword(&edge, "DiagnosticData") == Some(0)
            && read_dword(&edge, "EdgeAssetDeliveryServiceEnabled") == Some(0)
            && read_dword(&edge, "WalletDonationEnabled") == Some(0)
            && read_dword(&edge, "DefaultBrowserSettingsCampaignEnabled") == Some(0)
    }

    #[cfg(windows)]
    fn widgets_removed_enabled() -> bool {
        Self::ps_lines(
            "Get-AppxPackage Microsoft.WidgetsPlatformRuntime -AllUsers; Get-AppxPackage MicrosoftWindows.Client.WebExperience -AllUsers",
        )
        .is_empty()
    }

    #[cfg(windows)]
    fn ctfmon_blocked_enabled() -> bool {
        use winreg::enums::*;
        let root = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let Ok(key) = root.open_subkey_with_flags(r"Software\Microsoft\Input", KEY_READ) else {
            return false;
        };
        let read_dword = |name: &str| -> Option<u32> { key.get_value(name).ok() };
        read_dword("InputServiceEnabled") == Some(0)
            && read_dword("InputServiceEnabledForCCI") == Some(0)
    }

    #[cfg(windows)]
    fn ctfmon_service_dll_enabled() -> bool {
        use winreg::enums::*;
        let root = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let Ok(key) = root.open_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Services\TextInputManagementService\Parameters",
            KEY_READ,
        ) else {
            return false;
        };
        key.get_value::<String, _>("ServiceDll")
            .map(|value| value.eq_ignore_ascii_case(r"%SystemRoot%\System32\MSCTF.DLL"))
            .unwrap_or(false)
    }

    #[cfg(windows)]
    fn timer_resolution_enabled() -> bool {
        use winreg::enums::*;
        let root = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let Ok(key) = root.open_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            KEY_READ,
        ) else {
            return false;
        };
        key.get_value::<u32, _>("GlobalTimerResolutionRequests")
            .map(|value| value == 1)
            .unwrap_or(false)
    }

    #[cfg(not(windows))]
    fn timer_resolution_enabled() -> bool {
        false
    }

    #[cfg(not(windows))]
    fn ctfmon_blocked_enabled() -> bool {
        false
    }

    #[cfg(not(windows))]
    fn ctfmon_service_dll_enabled() -> bool {
        false
    }

    #[cfg(not(windows))]
    fn widgets_removed_enabled() -> bool {
        false
    }

    #[cfg(windows)]
    fn apply_edge_debloat(enabled: bool) -> Result<(), String> {
        use winreg::enums::*;
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let (edge, _) = hklm
            .create_subkey(r"SOFTWARE\Policies\Microsoft\Edge")
            .map_err(|e| format!("failed to open Edge policy key: {e}"))?;
        let (edge_update, _) = hklm
            .create_subkey(r"SOFTWARE\Policies\Microsoft\EdgeUpdate")
            .map_err(|e| format!("failed to open EdgeUpdate policy key: {e}"))?;
        let (blocklist, _) = hklm
            .create_subkey(r"SOFTWARE\Policies\Microsoft\Edge\ExtensionInstallBlocklist")
            .map_err(|e| format!("failed to open Edge blocklist key: {e}"))?;

        let dword = |key: &winreg::RegKey, name: &str, value: u32| -> Result<(), String> {
            key.set_value(name, &value)
                .map_err(|e| format!("failed to set {name}: {e}"))
        };
        let string = |key: &winreg::RegKey, name: &str, value: &str| -> Result<(), String> {
            key.set_value(name, &value)
                .map_err(|e| format!("failed to set {name}: {e}"))
        };
        let remove = |key: &winreg::RegKey, name: &str| -> Result<(), String> {
            match key.delete_value(name) {
                Ok(_) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(e) => Err(format!("failed to remove {name}: {e}")),
            }
        };

        if enabled {
            dword(&edge_update, "CreateDesktopShortcutDefault", 0)?;
            dword(&edge, "PersonalizationReportingEnabled", 0)?;
            string(&blocklist, "1", "ofefcgjbeghpigppfmkologfjadafddi")?;
            dword(&edge, "ShowRecommendationsEnabled", 0)?;
            dword(&edge, "HideFirstRunExperience", 1)?;
            dword(&edge, "UserFeedbackAllowed", 0)?;
            dword(&edge, "ConfigureDoNotTrack", 1)?;
            dword(&edge, "AlternateErrorPagesEnabled", 0)?;
            dword(&edge, "EdgeCollectionsEnabled", 0)?;
            dword(&edge, "EdgeShoppingAssistantEnabled", 0)?;
            dword(&edge, "MicrosoftEdgeInsiderPromotionEnabled", 0)?;
            dword(&edge, "ShowMicrosoftRewards", 0)?;
            dword(&edge, "WebWidgetAllowed", 0)?;
            dword(&edge, "DiagnosticData", 0)?;
            dword(&edge, "EdgeAssetDeliveryServiceEnabled", 0)?;
            dword(&edge, "WalletDonationEnabled", 0)?;
            dword(&edge, "DefaultBrowserSettingsCampaignEnabled", 0)?;
        } else {
            remove(&edge_update, "CreateDesktopShortcutDefault")?;
            for name in [
                "PersonalizationReportingEnabled",
                "ShowRecommendationsEnabled",
                "HideFirstRunExperience",
                "UserFeedbackAllowed",
                "ConfigureDoNotTrack",
                "AlternateErrorPagesEnabled",
                "EdgeCollectionsEnabled",
                "EdgeShoppingAssistantEnabled",
                "MicrosoftEdgeInsiderPromotionEnabled",
                "ShowMicrosoftRewards",
                "WebWidgetAllowed",
                "DiagnosticData",
                "EdgeAssetDeliveryServiceEnabled",
                "WalletDonationEnabled",
                "DefaultBrowserSettingsCampaignEnabled",
            ] {
                remove(&edge, name)?;
            }
            remove(&blocklist, "1")?;
        }
        Ok(())
    }

    #[cfg(windows)]
    fn apply_widgets_removed(enabled: bool) -> Result<(), String> {
        #[cfg(windows)]
        use std::os::windows::process::CommandExt;
        use std::process::{Command, Stdio};
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let script = if enabled {
            r#"
$ErrorActionPreference = 'SilentlyContinue'
Get-Process *Widget* | Stop-Process
Get-AppxPackage Microsoft.WidgetsPlatformRuntime -AllUsers | Remove-AppxPackage -AllUsers
Get-AppxPackage MicrosoftWindows.Client.WebExperience -AllUsers | Remove-AppxPackage -AllUsers
Invoke-WinUtilExplorerUpdate -action "restart"
"#
        } else {
            r#"
$ErrorActionPreference = 'SilentlyContinue'
Add-AppxPackage -Register "C:\Program Files\WindowsApps\Microsoft.WidgetsPlatformRuntime*\AppxManifest.xml" -DisableDevelopmentMode
Add-AppxPackage -Register "C:\Program Files\WindowsApps\MicrosoftWindows.Client.WebExperience*\AppxManifest.xml" -DisableDevelopmentMode
Invoke-WinUtilExplorerUpdate -action "restart"
"#
        };

        let status = Command::new("powershell.exe")
            .args([
                "-NoLogo",
                "-NoProfile",
                "-NonInteractive",
                "-WindowStyle",
                "Hidden",
                "-Command",
                script,
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| format!("failed to run widgets command: {e}"))?;
        if status.success() {
            Ok(())
        } else {
            Err("widgets command failed".to_string())
        }
    }

    #[cfg(not(windows))]
    fn apply_widgets_removed(_enabled: bool) -> Result<(), String> {
        Err("Widgets tweak is only supported on Windows".to_string())
    }

    #[cfg(windows)]
    fn apply_ctfmon_blocked(enabled: bool) -> Result<(), String> {
        use winreg::enums::*;
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let (key, _) = hklm
            .create_subkey(r"Software\Microsoft\Input")
            .map_err(|e| format!("failed to open Microsoft Input key: {e}"))?;
        let value = if enabled { 0u32 } else { 1u32 };
        key.set_value("InputServiceEnabled", &value)
            .map_err(|e| format!("failed to set InputServiceEnabled: {e}"))?;
        key.set_value("InputServiceEnabledForCCI", &value)
            .map_err(|e| format!("failed to set InputServiceEnabledForCCI: {e}"))?;
        Ok(())
    }

    #[cfg(not(windows))]
    fn apply_ctfmon_blocked(_enabled: bool) -> Result<(), String> {
        Err("CTFMON tweak is only supported on Windows".to_string())
    }

    #[cfg(windows)]
    fn apply_ctfmon_service_dll(enabled: bool) -> Result<(), String> {
        use winreg::enums::*;
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let (key, _) = hklm
            .create_subkey(
                r"SYSTEM\CurrentControlSet\Services\TextInputManagementService\Parameters",
            )
            .map_err(|e| format!("failed to open TextInputManagementService parameters: {e}"))?;
        let value = if enabled {
            r"%SystemRoot%\System32\MSCTF.DLL"
        } else {
            r"%SystemRoot%\System32\TabSvc.dll"
        };
        key.set_value("ServiceDll", &value)
            .map_err(|e| format!("failed to set ServiceDll: {e}"))?;
        Ok(())
    }

    #[cfg(not(windows))]
    fn apply_ctfmon_service_dll(_enabled: bool) -> Result<(), String> {
        Err("CTFMON ServiceDll tweak is only supported on Windows".to_string())
    }

    #[cfg(windows)]
    fn apply_timer_resolution(enabled: bool) -> Result<(), String> {
        use winreg::enums::*;
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let (key, _) = hklm
            .create_subkey(r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel")
            .map_err(|e| format!("failed to open kernel key: {e}"))?;
        if enabled {
            key.set_value("GlobalTimerResolutionRequests", &1u32)
                .map_err(|e| format!("failed to set GlobalTimerResolutionRequests: {e}"))?;
        } else {
            match key.delete_value("GlobalTimerResolutionRequests") {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    return Err(format!(
                        "failed to remove GlobalTimerResolutionRequests: {e}"
                    ));
                }
            }
        }
        Ok(())
    }

    #[cfg(not(windows))]
    fn apply_timer_resolution(_enabled: bool) -> Result<(), String> {
        Err("Timer resolution tweak is only supported on Windows".to_string())
    }

    #[cfg(not(windows))]
    fn brave_debloat_enabled() -> bool {
        false
    }

    #[cfg(not(windows))]
    fn apply_brave_debloat(_enabled: bool) -> Result<(), String> {
        Err("Brave debloat is only supported on Windows".to_string())
    }

    #[cfg(not(windows))]
    fn edge_debloat_enabled() -> bool {
        false
    }

    #[cfg(not(windows))]
    fn apply_edge_debloat(_enabled: bool) -> Result<(), String> {
        Err("Edge debloat is only supported on Windows".to_string())
    }

    #[cfg(windows)]
    fn ipv6_preferred_enabled() -> bool {
        use winreg::enums::*;
        let root = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let Ok(key) = root.open_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters",
            KEY_READ,
        ) else {
            return false;
        };
        key.get_value::<u32, _>("DisabledComponents")
            .map(|value| value & 0x20 != 0)
            .unwrap_or(false)
    }

    #[cfg(windows)]
    fn apply_ipv6_preferred(enabled: bool) -> Result<(), String> {
        use winreg::enums::*;
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let (key, _) = hklm
            .create_subkey(r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters")
            .map_err(|e| format!("failed to open Tcpip6 parameters: {e}"))?;
        let current = key.get_value::<u32, _>("DisabledComponents").unwrap_or(0);
        let next = if enabled {
            current | 0x20
        } else {
            current & !0x20
        };
        key.set_value("DisabledComponents", &next)
            .map_err(|e| format!("failed to set DisabledComponents: {e}"))?;
        Ok(())
    }

    #[cfg(not(windows))]
    fn ipv6_preferred_enabled() -> bool {
        false
    }

    #[cfg(not(windows))]
    fn apply_ipv6_preferred(_enabled: bool) -> Result<(), String> {
        Err("IPv6 preference is only supported on Windows".to_string())
    }

    #[cfg(windows)]
    fn teredo_disabled_enabled() -> bool {
        use winreg::enums::*;
        let root = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let Ok(key) = root.open_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters",
            KEY_READ,
        ) else {
            return false;
        };
        key.get_value::<u32, _>("DisabledComponents")
            .map(|value| value & 0x01 != 0)
            .unwrap_or(false)
    }

    #[cfg(windows)]
    fn apply_teredo_disabled(enabled: bool) -> Result<(), String> {
        use std::os::windows::process::CommandExt;
        use std::process::Command;
        use std::process::Stdio;
        use winreg::enums::*;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let (key, _) = hklm
            .create_subkey(r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters")
            .map_err(|e| format!("failed to open Tcpip6 parameters: {e}"))?;
        let current = key.get_value::<u32, _>("DisabledComponents").unwrap_or(0);
        let next = if enabled {
            current | 0x01
        } else {
            current & !0x01
        };
        key.set_value("DisabledComponents", &next)
            .map_err(|e| format!("failed to set DisabledComponents: {e}"))?;
        let state = if enabled { "disabled" } else { "default" };
        let status = Command::new("netsh")
            .args(["interface", "teredo", "set", "state", state])
            .creation_flags(CREATE_NO_WINDOW)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| format!("failed to run netsh: {e}"))?;
        if status.success() {
            Ok(())
        } else {
            Err("netsh teredo command failed".to_string())
        }
    }

    #[cfg(not(windows))]
    fn teredo_disabled_enabled() -> bool {
        false
    }

    #[cfg(not(windows))]
    fn apply_teredo_disabled(_enabled: bool) -> Result<(), String> {
        Err("Teredo is only supported on Windows".to_string())
    }

    #[cfg(windows)]
    fn powershell7_telemetry_enabled() -> bool {
        use winreg::enums::*;
        let root = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let Ok(key) = root.open_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
            KEY_READ,
        ) else {
            return false;
        };
        key.get_value::<String, _>("POWERSHELL_TELEMETRY_OPTOUT")
            .map(|value| value == "1")
            .unwrap_or(false)
    }

    #[cfg(windows)]
    fn apply_powershell7_telemetry(enabled: bool) -> Result<(), String> {
        use winreg::enums::*;
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let (key, _) = hklm
            .create_subkey(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")
            .map_err(|e| format!("failed to open machine environment key: {e}"))?;
        if enabled {
            key.set_value("POWERSHELL_TELEMETRY_OPTOUT", &"1")
                .map_err(|e| format!("failed to set POWERSHELL_TELEMETRY_OPTOUT: {e}"))?;
        } else {
            match key.delete_value("POWERSHELL_TELEMETRY_OPTOUT") {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(format!("failed to remove POWERSHELL_TELEMETRY_OPTOUT: {e}")),
            }
        }
        Ok(())
    }

    #[cfg(not(windows))]
    fn powershell7_telemetry_enabled() -> bool {
        false
    }

    #[cfg(not(windows))]
    fn apply_powershell7_telemetry(_enabled: bool) -> Result<(), String> {
        Err("PowerShell 7 telemetry is only supported on Windows".to_string())
    }

    #[cfg(windows)]
    fn hpet_preferred_enabled() -> bool {
        #[cfg(windows)]
        use std::os::windows::process::CommandExt;
        use std::process::{Command, Stdio};
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let output = Command::new("bcdedit")
            .args(["/enum", "{current}"])
            .creation_flags(CREATE_NO_WINDOW)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output();
        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .find_map(|line| {
                        let trimmed = line.trim();
                        let (key, value) = trimmed.split_once(char::is_whitespace)?;
                        if !key.eq_ignore_ascii_case("useplatformclock") {
                            return None;
                        }
                        let value = value.trim().to_ascii_lowercase();
                        Some(matches!(value.as_str(), "false" | "no" | "0"))
                    })
                    .unwrap_or(false)
            }
            _ => false,
        }
    }

    #[cfg(windows)]
    fn apply_hpet_preferred(enabled: bool) -> Result<(), String> {
        #[cfg(windows)]
        use std::os::windows::process::CommandExt;
        use std::process::{Command, Stdio};
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let value = if enabled { "false" } else { "true" };
        let status = Command::new("bcdedit")
            .args(["/set", "useplatformclock", value])
            .creation_flags(CREATE_NO_WINDOW)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| format!("failed to run bcdedit: {e}"))?;
        if status.success() {
            Ok(())
        } else {
            Err("bcdedit useplatformclock command failed".to_string())
        }
    }

    #[cfg(not(windows))]
    fn hpet_preferred_enabled() -> bool {
        false
    }

    #[cfg(not(windows))]
    fn apply_hpet_preferred(_enabled: bool) -> Result<(), String> {
        Err("HPET toggle is only supported on Windows".to_string())
    }
}
