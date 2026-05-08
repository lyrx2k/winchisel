use super::{RepairDialog, RepairEvent, RepairLoadWorker, RepairResult, WinchiselApp};
use eframe::egui;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::time::Duration;

impl WinchiselApp {
    pub(crate) fn start_system_repair(&mut self) {
        if self.repair_load_worker.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        let lang = self.state.settings.language;
        std::thread::spawn(move || {
            run_system_repair_worker(lang, tx);
        });
        self.repair_dialog = Some(RepairDialog::Progress {
            stage: self.tr("repair_running").to_string(),
            log: Vec::new(),
        });
        self.repair_load_worker = Some(RepairLoadWorker { rx });
    }

    pub(crate) fn poll_system_repair(&mut self) {
        let Some(worker) = self.repair_load_worker.as_ref() else {
            return;
        };
        loop {
            match worker.rx.try_recv() {
                Ok(event) => match event {
                    RepairEvent::Stage(stage) => {
                        if let Some(RepairDialog::Progress { stage: current, .. }) =
                            self.repair_dialog.as_mut()
                        {
                            *current = stage;
                        }
                    }
                    RepairEvent::Log(line) => {
                        if let Some(RepairDialog::Progress { log, .. }) = self.repair_dialog.as_mut() {
                            log.push(line);
                            if log.len() > 12 {
                                let excess = log.len() - 12;
                                log.drain(0..excess);
                            }
                        }
                    }
                    RepairEvent::Finished(result) => {
                        self.repair_load_worker = None;
                        self.repair_dialog = Some(RepairDialog::Result {
                            title: if result.success {
                                self.tr("repair_window").to_string()
                            } else {
                                self.tr("repair_failed").to_string()
                            },
                            message: result.message,
                        });
                        if result.success {
                            self.toasts
                                .success(self.tr("repair_success"))
                                .duration(Duration::from_secs_f64(3.5));
                        } else {
                            self.toasts
                                .error(self.tr("repair_failed"))
                                .duration(Duration::from_secs_f64(3.5));
                        }
                        break;
                    }
                },
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.repair_load_worker = None;
                    self.repair_dialog = Some(RepairDialog::Result {
                        title: self.tr("repair_failed").to_string(),
                        message: self.tr("repair_failed_run").to_string(),
                    });
                    self.toasts
                        .error(self.tr("repair_failed"))
                        .duration(Duration::from_secs_f64(3.5));
                    break;
                }
            }
        }
    }

    pub(crate) fn show_system_repair_dialog(&mut self, ctx: &egui::Context) {
        let Some(dialog) = self.repair_dialog.clone() else {
            return;
        };
        match dialog {
            RepairDialog::Progress { stage, log } => {
                egui::Window::new(self.tr("repair_window"))
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
            RepairDialog::Result { title, message } => {
                egui::Window::new(title)
                    .collapsible(false)
                    .resizable(false)
                    .default_width(460.0)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.label(message);
                        ui.add_space(12.0);
                        if ui.button(self.tr("update_close")).clicked() {
                            self.repair_dialog = None;
                        }
                    });
            }
        }
    }
}

fn run_system_repair_worker(lang: crate::Language, tx: mpsc::Sender<RepairEvent>) {
    #[cfg(windows)]
    {
        let _ = tx.send(RepairEvent::Stage(
            crate::i18n::t(lang, "repair_stage_dism").to_string(),
        ));
        let dism_ok = run_shell_command(
            "DISM /Online /Cleanup-Image /RestoreHealth",
            tx.clone(),
        );
        if !dism_ok {
            let _ = tx.send(RepairEvent::Finished(RepairResult {
                success: false,
                message: crate::i18n::t(lang, "repair_failed").to_string(),
            }));
            return;
        }
        let _ = tx.send(RepairEvent::Stage(
            crate::i18n::t(lang, "repair_stage_sfc").to_string(),
        ));
        let sfc_ok = run_shell_command("sfc /scannow", tx.clone());
        let result = if sfc_ok {
            RepairResult {
                success: true,
                message: crate::i18n::t(lang, "repair_success").to_string(),
            }
        } else {
            RepairResult {
                success: false,
                message: crate::i18n::t(lang, "repair_failed").to_string(),
            }
        };
        let _ = tx.send(RepairEvent::Finished(result));
    }
    #[cfg(not(windows))]
    {
        let _ = tx.send(RepairEvent::Finished(RepairResult {
            success: false,
            message: crate::i18n::t(lang, "repair_failed_windows").to_string(),
        }));
    }
}

#[cfg(windows)]
fn run_shell_command(cmd: &str, tx: mpsc::Sender<RepairEvent>) -> bool {
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
            let _ = tx.send(RepairEvent::Log(format!("Failed to start: {e}")));
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
                    let _ = tx_out.send(RepairEvent::Log(trimmed.to_string()));
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
                    let _ = tx_err.send(RepairEvent::Log(trimmed.to_string()));
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
            let _ = tx.send(RepairEvent::Log(format!("Failed to wait for command: {e}")));
            false
        }
    }
}
