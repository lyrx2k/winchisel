use super::{UpdateCheckResult, UpdateDialog, WinchiselApp};
use crate::updater;
use eframe::egui;
use std::sync::mpsc;
use std::time::Duration;

impl WinchiselApp {
    pub(crate) fn start_update_check(&mut self, show_dialog: bool) {
        if self.update_check_rx.is_some() || self.state.update_check_loading {
            return;
        }
        self.state.update_check_loading = true;
        self.update_dialog_on_complete = show_dialog;
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let result = match updater::check_for_update() {
                Ok(Some(version)) => UpdateCheckResult::UpdateAvailable(version),
                Ok(None) => UpdateCheckResult::UpToDate,
                Err(err) => UpdateCheckResult::Error(err),
            };
            let _ = tx.send(result);
        });
        self.update_check_rx = Some(rx);
    }

    pub(crate) fn poll_update_check(&mut self) {
        let Some(rx) = self.update_check_rx.as_ref() else {
            return;
        };
        match rx.try_recv() {
            Ok(result) => {
                self.state.update_check_loading = false;
                self.update_check_rx = None;
                match result {
                    UpdateCheckResult::UpToDate => {
                        self.state.update_status = "Up to date".to_string();
                        self.toasts
                            .info("Update check: already up to date")
                            .duration(Duration::from_secs_f64(2.5));
                        if self.update_dialog_on_complete {
                            self.pending_update_dialog = Some(UpdateDialog::UpToDate);
                        }
                    }
                    UpdateCheckResult::UpdateAvailable(version) => {
                        self.state.update_status = format!("Update available: {}", version);
                        self.toasts
                            .warning(format!("Update available: {}", version))
                            .duration(Duration::from_secs_f64(3.5));
                        if self.update_dialog_on_complete {
                            self.pending_update_dialog = Some(UpdateDialog::UpdateAvailable {
                                latest_version: version,
                            });
                        }
                    }
                    UpdateCheckResult::Error(err) => {
                        self.state.update_status = format!("Update check failed: {}", err);
                        self.toasts
                            .error(format!("Update check failed: {}", err))
                            .duration(Duration::from_secs_f64(3.5));
                        if self.update_dialog_on_complete {
                            self.pending_update_dialog = Some(UpdateDialog::Error { message: err });
                        }
                    }
                }
                self.update_dialog_on_complete = false;
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.state.update_check_loading = false;
                self.update_check_rx = None;
                self.update_dialog_on_complete = false;
            }
        }
    }

    pub(crate) fn show_update_dialog(&mut self, ctx: &egui::Context) {
        let Some(dialog) = self.pending_update_dialog.clone() else {
            return;
        };
        let title = match &dialog {
            UpdateDialog::UpToDate => "No Update Found",
            UpdateDialog::UpdateAvailable { .. } => "Update Available",
            UpdateDialog::Error { .. } => "Update Check Failed",
        };
        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .default_width(420.0)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    match &dialog {
                        UpdateDialog::UpToDate => {
                            ui.label("You are already on the latest version.");
                        }
                        UpdateDialog::UpdateAvailable { latest_version } => {
                            ui.label(format!("A newer version is available: {}", latest_version));
                            ui.label("Download and restart the app to install it.");
                        }
                        UpdateDialog::Error { message } => {
                            ui.colored_label(
                                egui::Color32::from_rgb(210, 80, 80),
                                "Update check failed.",
                            );
                            ui.label(message);
                        }
                    }
                    ui.add_space(14.0);
                    ui.horizontal(|ui| {
                        if ui.button("Close").clicked() {
                            self.pending_update_dialog = None;
                        }
                        if matches!(dialog, UpdateDialog::UpdateAvailable { .. })
                            && ui.button("Download & Restart").clicked()
                        {
                            if let UpdateDialog::UpdateAvailable { latest_version } = dialog.clone()
                            {
                                self.pending_update_dialog = None;
                                if let Err(e) = updater::download_and_install(&latest_version) {
                                    self.pending_update_dialog = Some(UpdateDialog::Error {
                                        message: e,
                                    });
                                }
                            }
                        }
                        if matches!(dialog, UpdateDialog::UpdateAvailable { .. })
                            && ui.button("Check Again").clicked()
                        {
                            self.pending_update_dialog = None;
                            self.start_update_check(true);
                        }
                    });
                });
            });
    }
}
