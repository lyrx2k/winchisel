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
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};

mod debloater;
mod downloads;
mod home;
#[path = "app/latency.rs"]
mod latency_tab;
#[path = "app/performance.rs"]
mod performance_tab;
#[path = "app/processes.rs"]
mod processes_tab;
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
    Settings,
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
    memory_total: String,
    memory_used: String,
    storage_total: String,
    storage_used: String,
    cpu_usage: String,
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
    home: HomeState,
    cpu: processes_tab::CpuState,
    latency: latency_tab::LatencyState,
    home_last_refresh: Option<Instant>,
}

struct DownloadsLoadResult {
    installed: Vec<bool>,
}

struct DownloadsLoadWorker {
    rx: Receiver<DownloadsLoadResult>,
}

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
    UpToDate,
    UpdateAvailable { latest_version: String },
    Error { message: String },
}

pub struct WinchiselApp {
    state: AppState,
    initialized_style: bool,
    debloater_load_worker: Option<debloater::DebloaterLoadWorker>,
    debloater_cache_ready: bool,
    pending_debloater_action: Option<debloater::DebloaterAction>,
    pending_download_action: Option<DownloadAction>,
    downloads_load_worker: Option<DownloadsLoadWorker>,
    downloads_install_worker: Option<DownloadInstallWorker>,
    downloads_cache_ready: bool,
    performance_load_worker: Option<performance_tab::PerformanceLoadWorker>,
    cpu_load_worker: Option<processes_tab::CpuLoadWorker>,
    latency_load_worker: Option<latency_tab::LatencyLoadWorker>,
    restore_point_load_worker: Option<RestorePointLoadWorker>,
    restore_point_dialog: Option<RestorePointDialog>,
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
        let debloater_items = get_all_apps(settings.language);
        let (debloater_load_worker, debloater_loading) =
            Self::spawn_debloater_worker(debloater_items.clone());
        let download_items = get_all_downloads(settings.language);
        let (downloads_load_worker, downloads_loading) =
            Self::spawn_downloads_worker(download_items.clone());
        let (performance_load_worker, performance_loaded) =
            Self::spawn_performance_worker("", settings.language);
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
                home: Self::build_home_state(settings.language),
                cpu: processes_tab::CpuState {
                    cpu_filter_active_only: true,
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
                home_last_refresh: None,
            },
            initialized_style: false,
            debloater_load_worker,
            debloater_cache_ready: false,
            pending_debloater_action: None,
            pending_download_action: None,
            downloads_load_worker,
            downloads_install_worker: None,
            downloads_cache_ready: false,
            performance_load_worker,
            cpu_load_worker: None,
            latency_load_worker: None,
            restore_point_load_worker: None,
            restore_point_dialog: None,
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
            self.state.update_status = self.tr("settings_saved").to_string();
            self.settings_save_due_at = None;
            self.toasts
                .info(self.tr("settings_saved"))
                .duration(Duration::from_secs_f64(2.5));
        }
    }

    fn apply_language_change(&mut self) {
        let lang = self.state.settings.language;

        self.state.home = Self::build_home_state(lang);
        self.state.update_status = t(lang, "update_ready").to_string();

        self.state.debloater.debloater_items = get_all_apps(lang);
        self.state.debloater.debloater_selected =
            vec![false; self.state.debloater.debloater_items.len()];
        self.state.debloater.debloater_installed =
            vec![false; self.state.debloater.debloater_items.len()];
        self.state.debloater.debloater_filter_cache_query.clear();
        self.state.debloater.debloater_filter_cache_tab = usize::MAX;
        self.state.debloater.debloater_filter_cache_view_mode = usize::MAX;
        self.state.debloater.debloater_filter_cache_all.clear();
        self.debloater_cache_ready = false;
        self.state.debloater.debloater_loading = false;
        self.debloater_load_worker = None;
        self.start_debloater_load();

        self.state.downloads.downloads_items = get_all_downloads(lang);
        self.state.downloads.downloads_selected =
            vec![false; self.state.downloads.downloads_items.len()];
        self.state.downloads.downloads_installed =
            vec![false; self.state.downloads.downloads_items.len()];
        self.state.downloads.downloads_filter_cache_query.clear();
        self.state.downloads.downloads_filter_cache_all.clear();
        for bucket in &mut self.state.downloads.downloads_filter_cache_categories {
            bucket.clear();
        }
        self.downloads_cache_ready = false;
        self.state.downloads.downloads_loading = false;
        self.downloads_load_worker = None;
        self.start_downloads_load();

        self.state.performance.performance_loaded = false;
        self.performance_load_worker = None;
        self.start_performance_load();

        self.cpu_load_worker = None;
        self.state.cpu.cpu_visible_count = 0;
        self.state.cpu.cpu_total_usage = "0.0%".to_string();
        self.state.cpu.cpu_processes_all.clear();
        self.state.cpu.cpu_processes.clear();
        self.state.cpu.cpu_selected_pid = -1;
        self.state.cpu.cpu_selected_name.clear();
        self.state.cpu.cpu_last_refresh = None;
        self.state.cpu.cpu_last_error = None;
        self.state.cpu.cpu_reload_pending = false;
        self.state.cpu.cpu_reload_ready_at = None;
        self.start_cpu_load();

        self.latency_load_worker = None;
        self.state.latency.latency_loading = false;
        self.state.latency.latency_completion_pending = false;
        self.state.latency.latency_progress = 0;
        self.state.latency.latency_progress_display = 0.0;
        self.state.latency.latency_status.clear();
        self.state.latency.latency_lines.clear();
        self.state.latency.latency_pending_lines.clear();
        self.state.latency.latency_tick_next_at = None;
        self.state.latency.latency_completion_ready_at = None;

        self.restore_point_load_worker = None;
        self.restore_point_dialog = None;
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
            let winget_ids = Self::get_winget_installed_ids();
            let winget_names = Self::get_winget_installed_names();
            let registry_names = Self::get_registry_installed_names();
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
        self.poll_update_check();
        if self.state.settings.check_updates_on_startup
            && !self.state.update_check_started
            && !self.state.update_check_loading
        {
            self.state.update_check_started = true;
            self.start_update_check(true);
        }
        if self.settings_save_due_at.is_some() {
            repaint_after = Some(repaint_after.map_or(Duration::from_millis(50), |cur| {
                cur.min(Duration::from_millis(50))
            }));
        }
        WinchiselApp::performance_tick(self, ui);
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
                self.state.home = Self::build_home_state(self.state.settings.language);
                self.state.home_last_refresh = Some(Instant::now());
                repaint_after = Some(repaint_after.map_or(Duration::from_millis(100), |cur| {
                    cur.min(Duration::from_millis(100))
                }));
            } else if let Some(last) = self.state.home_last_refresh {
                let elapsed = last.elapsed();
                let until_refresh = Duration::from_secs(5).saturating_sub(elapsed);
                let next = until_refresh.min(Duration::from_millis(250));
                repaint_after = Some(repaint_after.map_or(next, |cur| cur.min(next)));
            }
        }
        if self.state.debloater.debloater_loading || self.debloater_load_worker.is_some() {
            repaint_after = Some(repaint_after.map_or(Duration::from_millis(50), |cur| {
                cur.min(Duration::from_millis(50))
            }));
        }
        if self.state.downloads.downloads_loading || self.downloads_load_worker.is_some() {
            repaint_after = Some(repaint_after.map_or(Duration::from_millis(50), |cur| {
                cur.min(Duration::from_millis(50))
            }));
        }
        if WinchiselApp::performance_sidebar_loading(self) {
            repaint_after = Some(repaint_after.map_or(Duration::from_millis(50), |cur| {
                cur.min(Duration::from_millis(50))
            }));
        }
        if self.cpu_load_worker.is_some() {
            repaint_after = Some(repaint_after.map_or(Duration::from_millis(50), |cur| {
                cur.min(Duration::from_millis(50))
            }));
        }
        if self.state.update_check_loading || self.update_check_rx.is_some() {
            repaint_after = Some(repaint_after.map_or(Duration::from_millis(50), |cur| {
                cur.min(Duration::from_millis(50))
            }));
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
                        if let Ok(icon) =
                            try_icon(Pack::Lucide, "shield-check", Style::Regular, Size::Regular)
                        {
                            let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
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
                        if let Ok(icon) =
                            try_icon(Pack::Lucide, "badge-info", Style::Regular, Size::Regular)
                        {
                            let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
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
                    let update_button_text = if let Ok(icon) =
                        try_icon(Pack::Lucide, "refresh-cw", Style::Regular, Size::Regular)
                    {
                        let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
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
                    let donate_button_text = if let Ok(icon) =
                        try_icon(Pack::Lucide, "heart", Style::Regular, Size::Regular)
                    {
                        let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
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
                    let bug_report_text = if let Ok(icon) =
                        try_icon(Pack::Lucide, "bug", Style::Regular, Size::Regular)
                    {
                        let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
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
                let tab_settings = self.tr("settings").to_string();
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
                    Tab::Downloads,
                    &Self::sidebar_tab_label("download", &tab_downloads),
                    self.state.downloads.downloads_loading || self.downloads_load_worker.is_some(),
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
                    Tab::Settings => self.render_settings_tab(ui),
                });
        });

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
        self.show_toast_layer(ui);
        self.show_log_window(ui.ctx());

        self.sync_settings();
    }

    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
