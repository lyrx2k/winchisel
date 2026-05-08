use super::{
    SettingsActionDialog, SettingsActionEvent, SettingsActionLoadWorker, SettingsActionResult,
    WinchiselApp,
};
use eframe::egui;
use iconflow::Pack;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

impl WinchiselApp {
    pub(crate) fn render_settings_tab(&mut self, ui: &mut egui::Ui) {
        Self::page_shell(
            ui,
            self.tr("settings_title"),
            self.tr("settings_subtitle"),
            |ui| {
                let check_updates_startup = self.tr("check_updates_startup").to_string();
                let show_console = self.tr("show_console").to_string();
                Self::card_frame().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(Self::icon_text(
                                    Pack::Lucide,
                                    "settings-2",
                                    16.0,
                                    egui::Color32::from_rgb(160, 124, 220),
                                ));
                                ui.colored_label(
                                    egui::Color32::from_rgb(160, 124, 220),
                                    self.tr("settings_application"),
                                );
                            });
                            ui.label(self.tr("settings_saved_auto"));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui
                                .add_sized([116.0, 34.0], egui::Button::new(self.tr("open_logs")))
                                .clicked()
                            {
                                self.show_log_window = true;
                            }
                        });
                    });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "languages",
                            14.0,
                            egui::Color32::from_rgb(226, 226, 226),
                        ));
                        ui.label(self.tr("language"));
                        let current_language = self.state.settings.language;
                        egui::ComboBox::from_id_salt("language_combo")
                            .selected_text(match current_language {
                                crate::Language::English => "English",
                                crate::Language::German => "Deutsch",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::English,
                                    "English",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::German,
                                    "Deutsch",
                                );
                            });
                    });
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "refresh-cw",
                            14.0,
                            egui::Color32::from_rgb(226, 226, 226),
                        ));
                        ui.label(check_updates_startup);
                        Self::native_toggle_switch(
                            ui,
                            &mut self.state.settings.check_updates_on_startup,
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "monitor",
                            14.0,
                            egui::Color32::from_rgb(226, 226, 226),
                        ));
                        ui.label(show_console);
                        Self::native_toggle_switch(ui, &mut self.state.settings.show_console);
                    });
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "shield-check",
                            16.0,
                            egui::Color32::from_rgb(160, 124, 220),
                        ));
                        ui.colored_label(
                            egui::Color32::from_rgb(160, 124, 220),
                            self.tr("system_protection"),
                        );
                    });
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(Self::icon_text(
                                    Pack::Lucide,
                                    "archive-restore",
                                    14.0,
                                    egui::Color32::from_rgb(226, 226, 226),
                                ));
                                ui.label(self.tr("restore_point"));
                            });
                            ui.label(self.tr("restore_point_desc"));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let restore_button = egui::Button::new(self.tr("create_restore_point"))
                                .fill(egui::Color32::from_rgb(35, 54, 80))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(10, 210, 254),
                                ));
                            if ui.add_sized([168.0, 34.0], restore_button).clicked() {
                                self.start_restore_point();
                            }
                        });
                    });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(Self::icon_text(
                                    Pack::Lucide,
                                    "scan-search",
                                    14.0,
                                    egui::Color32::from_rgb(226, 226, 226),
                                ));
                                ui.label(self.tr("repair_window"));
                            });
                            ui.label(self.tr("repair_desc"));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let repair_button = egui::Button::new(self.tr("repair_run"))
                                .fill(egui::Color32::from_rgb(92, 64, 28))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(226, 196, 84),
                                ));
                            if ui.add_sized([168.0, 34.0], repair_button).clicked() {
                                self.start_system_repair();
                            }
                        });
                    });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(Self::icon_text(
                                    Pack::Lucide,
                                    "trash-2",
                                    14.0,
                                    egui::Color32::from_rgb(226, 226, 226),
                                ));
                                ui.label(self.tr("settings_disk_cleanup"));
                            });
                            ui.label(self.tr("settings_disk_cleanup_desc"));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let cleanup_button = egui::Button::new(self.tr("settings_disk_cleanup_run"))
                                .fill(egui::Color32::from_rgb(55, 74, 40))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(105, 185, 110),
                                ));
                            if ui.add_sized([168.0, 34.0], cleanup_button).clicked() {
                                self.start_settings_action(
                                    self.tr("settings_disk_cleanup").to_string(),
                                    SettingsActionKind::DiskCleanup,
                                );
                            }
                        });
                    });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(Self::icon_text(
                                    Pack::Lucide,
                                    "file-x",
                                    14.0,
                                    egui::Color32::from_rgb(226, 226, 226),
                                ));
                                ui.label(self.tr("settings_temp_files"));
                            });
                            ui.label(self.tr("settings_temp_files_desc"));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let temp_button = egui::Button::new(self.tr("settings_temp_files_run"))
                                .fill(egui::Color32::from_rgb(80, 54, 34))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(226, 152, 84),
                                ));
                            if ui.add_sized([168.0, 34.0], temp_button).clicked() {
                                self.start_settings_action(
                                    self.tr("settings_temp_files").to_string(),
                                    SettingsActionKind::TempFiles,
                                );
                            }
                        });
                    });
                });
            },
        );
    }
}

#[derive(Clone, Copy)]
pub(crate) enum SettingsActionKind {
    DiskCleanup,
    TempFiles,
}

impl WinchiselApp {
    pub(crate) fn start_settings_action(&mut self, title: String, kind: SettingsActionKind) {
        if self.settings_action_load_worker.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        let lang = self.state.settings.language;
        std::thread::spawn(move || {
            run_settings_action_worker(lang, kind, tx);
        });
        if !matches!(kind, SettingsActionKind::DiskCleanup) {
            self.settings_action_dialog = Some(SettingsActionDialog::Progress {
                title,
                stage: self.tr("repair_running").to_string(),
                log: Vec::new(),
            });
        } else {
            self.settings_action_dialog = None;
        }
        self.settings_action_load_worker = Some(SettingsActionLoadWorker { rx });
    }

    pub(crate) fn poll_settings_actions(&mut self) {
        let Some(worker) = self.settings_action_load_worker.as_ref() else {
            return;
        };
        loop {
            match worker.rx.try_recv() {
                Ok(event) => match event {
                    SettingsActionEvent::Stage(stage) => {
                        if let Some(SettingsActionDialog::Progress { stage: current, .. }) =
                            self.settings_action_dialog.as_mut()
                        {
                            *current = stage;
                        }
                    }
                    SettingsActionEvent::Log(line) => {
                        if let Some(SettingsActionDialog::Progress { log, .. }) =
                            self.settings_action_dialog.as_mut()
                        {
                            log.push(line);
                            if log.len() > 12 {
                                let excess = log.len() - 12;
                                log.drain(0..excess);
                            }
                        }
                    }
                    SettingsActionEvent::Finished(result) => {
                        self.settings_action_load_worker = None;
                        if result.title == self.tr("settings_disk_cleanup") {
                            self.toasts
                                .success(result.message.clone())
                                .duration(std::time::Duration::from_secs_f64(3.5));
                        } else {
                            self.settings_action_dialog = Some(SettingsActionDialog::Result {
                                title: result.title,
                                message: result.message,
                            });
                        }
                        break;
                    }
                },
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.settings_action_load_worker = None;
                    self.settings_action_dialog = Some(SettingsActionDialog::Result {
                        title: self.tr("repair_failed").to_string(),
                        message: self.tr("repair_failed_run").to_string(),
                    });
                    break;
                }
            }
        }
    }

    pub(crate) fn show_settings_action_dialog(&mut self, ctx: &egui::Context) {
        let Some(dialog) = self.settings_action_dialog.clone() else {
            return;
        };
        match dialog {
            SettingsActionDialog::Progress { title, stage, log } => {
                egui::Window::new(title)
                    .collapsible(false)
                    .resizable(true)
                    .default_width(560.0)
                    .default_height(360.0)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.add(egui::Spinner::new().size(22.0));
                                ui.vertical(|ui| {
                                    ui.label(stage);
                                    ui.label(self.tr("repair_running_desc"));
                                });
                            });
                            ui.add_space(12.0);
                            ui.separator();
                            ui.add_space(8.0);
                            ui.label(self.tr("repair_log_title"));
                            egui::ScrollArea::vertical()
                                .stick_to_bottom(true)
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    for line in log.iter() {
                                        ui.monospace(line);
                                    }
                                    if log.is_empty() {
                                        ui.label(self.tr("repair_log_waiting"));
                                    }
                                });
                        });
                    });
            }
            SettingsActionDialog::Result { title, message } => {
                egui::Window::new(title)
                    .collapsible(false)
                    .resizable(false)
                    .default_width(460.0)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.label(message);
                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            if ui.button(self.tr("settings_close")).clicked() {
                                self.settings_action_dialog = None;
                            }
                        });
                        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                            self.settings_action_dialog = None;
                        }
                    });
            }
        }
    }
}

fn run_settings_action_worker(
    lang: crate::Language,
    kind: SettingsActionKind,
    tx: mpsc::Sender<SettingsActionEvent>,
) {
    #[cfg(windows)]
    {
        let (title, script) = match kind {
            SettingsActionKind::DiskCleanup => (
                crate::i18n::t(lang, "settings_disk_cleanup").to_string(),
                r#"
cleanmgr.exe /d C: /VERYLOWDISK
Dism.exe /online /Cleanup-Image /StartComponentCleanup /ResetBase
"#,
            ),
            SettingsActionKind::TempFiles => (
                crate::i18n::t(lang, "settings_temp_files").to_string(),
                r#"
Remove-Item -Path "$Env:Temp\*" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path "$Env:SystemRoot\Temp\*" -Recurse -Force -ErrorAction SilentlyContinue
"#,
            ),
        };
        let _ = tx.send(SettingsActionEvent::Stage(title.clone()));
        let ok = run_powershell_logged(script, tx.clone());
        let result = if ok {
            SettingsActionResult {
                title,
                message: match kind {
                    SettingsActionKind::DiskCleanup => {
                        crate::i18n::t(lang, "settings_disk_cleanup_success").to_string()
                    }
                    SettingsActionKind::TempFiles => {
                        crate::i18n::t(lang, "settings_temp_files_success").to_string()
                    }
                },
            }
        } else {
            SettingsActionResult {
                title,
                message: match kind {
                    SettingsActionKind::DiskCleanup => {
                        crate::i18n::t(lang, "settings_disk_cleanup_failed").to_string()
                    }
                    SettingsActionKind::TempFiles => {
                        crate::i18n::t(lang, "settings_temp_files_failed").to_string()
                    }
                },
            }
        };
        let _ = tx.send(SettingsActionEvent::Finished(result));
    }
    #[cfg(not(windows))]
    {
        let _ = tx.send(SettingsActionEvent::Finished(SettingsActionResult {
            title: crate::i18n::t(lang, match kind {
                SettingsActionKind::DiskCleanup => "settings_disk_cleanup",
                SettingsActionKind::TempFiles => "settings_temp_files",
            })
            .to_string(),
            message: match kind {
                SettingsActionKind::DiskCleanup => {
                    crate::i18n::t(lang, "settings_disk_cleanup_failed_windows").to_string()
                }
                SettingsActionKind::TempFiles => {
                    crate::i18n::t(lang, "settings_temp_files_failed_windows").to_string()
                }
            },
        }));
    }
}

#[cfg(windows)]
fn run_powershell_logged(cmd: &str, tx: mpsc::Sender<SettingsActionEvent>) -> bool {
    use std::process::{Command, Stdio};
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let script = format!(
        "$ErrorActionPreference = 'Continue'; & {{ {cmd} }} 2>&1 | ForEach-Object {{ $_.ToString() }}"
    );
    let mut child = match Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            let _ = tx.send(SettingsActionEvent::Log(format!("Failed to start: {e}")));
            return false;
        }
    };
    let mut handles = Vec::new();
    if let Some(stdout) = child.stdout.take() {
        let tx_out = tx.clone();
        handles.push(std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines().flatten() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    let _ = tx_out.send(SettingsActionEvent::Log(trimmed.to_string()));
                }
            }
        }));
    }
    if let Some(stderr) = child.stderr.take() {
        let tx_err = tx.clone();
        handles.push(std::thread::spawn(move || {
            for line in BufReader::new(stderr).lines().flatten() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    let _ = tx_err.send(SettingsActionEvent::Log(trimmed.to_string()));
                }
            }
        }));
    }
    let status = child.wait();
    for handle in handles {
        let _ = handle.join();
    }
    match status {
        Ok(status) => status.success(),
        Err(e) => {
            let _ = tx.send(SettingsActionEvent::Log(format!("Failed to wait for command: {e}")));
            false
        }
    }
}
