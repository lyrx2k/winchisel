use super::{RestorePointDialog, RestorePointLoadWorker, RestorePointResult, WinchiselApp};
use eframe::egui;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::sync::mpsc;
use std::time::Duration;

impl WinchiselApp {
    pub(crate) fn start_restore_point(&mut self) {
        if self.restore_point_load_worker.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let result = create_restore_point_worker();
            let _ = tx.send(result);
        });
        self.restore_point_dialog = Some(RestorePointDialog::Progress);
        self.restore_point_load_worker = Some(RestorePointLoadWorker { rx });
    }

    pub(crate) fn poll_restore_point(&mut self) {
        let Some(worker) = self.restore_point_load_worker.as_ref() else {
            return;
        };
        match worker.rx.try_recv() {
            Ok(result) => {
                self.restore_point_load_worker = None;
                self.restore_point_dialog = Some(RestorePointDialog::Result {
                    title: if result.success {
                        "Restore Point".to_string()
                    } else {
                        "Restore Point Error".to_string()
                    },
                    message: result.message,
                });
                if result.success {
                    self.toasts
                        .success("Restore point created")
                        .duration(Duration::from_secs_f64(3.5));
                } else {
                    self.toasts
                        .error("Restore point failed")
                        .duration(Duration::from_secs_f64(3.5));
                }
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.restore_point_load_worker = None;
                self.restore_point_dialog = Some(RestorePointDialog::Result {
                    title: "Restore Point Error".to_string(),
                    message: "Failed to create restore point.".to_string(),
                });
                self.toasts
                    .error("Restore point failed")
                    .duration(Duration::from_secs_f64(3.5));
            }
        }
    }

    pub(crate) fn show_restore_point_dialog(&mut self, ctx: &egui::Context) {
        let Some(dialog) = self.restore_point_dialog.clone() else {
            return;
        };
        match dialog {
            RestorePointDialog::Progress => {
                egui::Window::new("Create Restore Point")
                    .collapsible(false)
                    .resizable(false)
                    .default_width(420.0)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add(egui::Spinner::new().size(28.0));
                            ui.add_space(10.0);
                            ui.label("Creating restore point. This can take a moment...");
                        });
                    });
            }
            RestorePointDialog::Result { title, message } => {
                egui::Window::new(title)
                    .collapsible(false)
                    .resizable(false)
                    .default_width(460.0)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.label(message);
                        ui.add_space(12.0);
                        if ui.button("Close").clicked() {
                            self.restore_point_dialog = None;
                        }
                    });
            }
        }
    }
}

fn create_restore_point_worker() -> RestorePointResult {
    #[cfg(windows)]
    {
        use std::process::Stdio;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let ps = r#"
$ErrorActionPreference = 'Stop'
Enable-ComputerRestore -Drive 'C:\' | Out-Null
Checkpoint-Computer -Description 'Winchisel Restore Point' -RestorePointType 'MODIFY_SETTINGS' | Out-Null
"#;
        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", ps])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();
        match output {
            Ok(out) if out.status.success() => RestorePointResult {
                success: true,
                message: "Restore point created successfully.".to_string(),
            },
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let detail = if stderr.trim().is_empty() {
                    "Failed to create restore point.".to_string()
                } else {
                    stderr.trim().to_string()
                };
                RestorePointResult {
                    success: false,
                    message: detail,
                }
            }
            Err(e) => RestorePointResult {
                success: false,
                message: format!("Failed to run PowerShell.\n\n{}", e),
            },
        }
    }
    #[cfg(not(windows))]
    {
        RestorePointResult {
            success: false,
            message: "Restore points are only supported on Windows.".to_string(),
        }
    }
}
