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
        if let Some(cooldown) = self.update_check_cooldown_until
            && std::time::Instant::now() < cooldown
        {
            self.toasts
                .warning(self.tr("update_check_cooldown"))
                .duration(Duration::from_secs_f64(2.5));
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
                        self.state.update_status = self.tr("update_checked").to_string();
                        self.toasts
                            .success(self.tr("update_checked"))
                            .duration(Duration::from_secs_f64(3.5));
                    }
                    UpdateCheckResult::UpdateAvailable(version) => {
                        self.state.update_status =
                            format!("{} {}", self.tr("update_available_prefix"), version);
                        self.toasts
                            .warning(format!(
                                "{} {}",
                                self.tr("update_available_prefix"),
                                version
                            ))
                            .duration(Duration::from_secs_f64(3.5));
                        if self.update_dialog_on_complete {
                            self.pending_update_dialog = Some(UpdateDialog::UpdateAvailable {
                                latest_version: version,
                            });
                        }
                    }
                    UpdateCheckResult::Error(err) => {
                        let localized_prefix = match err.as_str() {
                            "update_error_check_updates" => self.tr("update_error_check_updates"),
                            "update_error_read_response" => self.tr("update_error_read_response"),
                            "update_error_parse_json" => self.tr("update_error_parse_json"),
                            "update_error_no_tag_name" => self.tr("update_error_no_tag_name"),
                            _ => self.tr("update_failed_prefix"),
                        };
                        let message = format!("{} {}", localized_prefix, err);
                        self.state.update_status = message.clone();
                        self.toasts
                            .error(message)
                            .duration(Duration::from_secs_f64(3.5));
                        if self.update_dialog_on_complete {
                            self.pending_update_dialog = Some(UpdateDialog::Error { message: err });
                        }
                    }
                }
                self.update_check_cooldown_until =
                    Some(std::time::Instant::now() + Duration::from_secs(30));
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
        // MSI download/install in progress → show progress dialog instead
        if self.msi_download_worker.is_some() || self.state.msi_download_installing {
            let title = if self.state.msi_download_installing {
                self.tr("update_installing_title")
            } else {
                self.tr("update_downloading_title")
            };
            egui::Window::new(title)
                .collapsible(false)
                .resizable(false)
                .default_width(360.0)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        if self.state.msi_download_installing {
                            ui.label(self.tr("update_installing_desc"));
                            ui.add_space(12.0);
                            ui.horizontal(|ui| {
                                ui.add(egui::Spinner::new().size(22.0));
                                ui.label(self.tr("update_installing_wait"));
                            });
                        } else {
                            let pct = (self.state.msi_download_progress * 100.0).clamp(0.0, 100.0);
                            ui.label(format!(
                                "{} {:.0}%",
                                self.tr("update_downloading_desc"),
                                pct
                            ));
                            ui.add_space(8.0);
                            ui.add(
                                egui::ProgressBar::new(self.state.msi_download_progress)
                                    .desired_width(320.0),
                            );
                        }
                    });
                });
            return;
        }

        let Some(dialog) = self.pending_update_dialog.clone() else {
            return;
        };
        let title = match &dialog {
            UpdateDialog::UpdateAvailable { .. } => self.tr("update_available_title"),
            UpdateDialog::Error { .. } => self.tr("update_failed_title"),
        };
        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .default_width(420.0)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    match &dialog {
                        UpdateDialog::UpdateAvailable { latest_version } => {
                            ui.label(format!(
                                "{} {}",
                                self.tr("update_newer_version_prefix"),
                                latest_version
                            ));
                            ui.label(self.tr("update_download_restart"));
                            if self.state.is_msi_install {
                                ui.add_space(4.0);
                                ui.colored_label(
                                    egui::Color32::from_rgb(226, 196, 84),
                                    self.tr("update_msi_auto_hint"),
                                );
                            }
                        }
                        UpdateDialog::Error { message } => {
                            ui.colored_label(
                                egui::Color32::from_rgb(210, 80, 80),
                                self.tr("update_failed"),
                            );
                            let display_message = match message.as_str() {
                                "update_error_check_updates" => {
                                    self.tr("update_error_check_updates")
                                }
                                "update_error_read_response" => {
                                    self.tr("update_error_read_response")
                                }
                                "update_error_parse_json" => self.tr("update_error_parse_json"),
                                "update_error_no_tag_name" => self.tr("update_error_no_tag_name"),
                                "update_error_download" => self.tr("update_error_download"),
                                "update_error_create_temp_file" => {
                                    self.tr("update_error_create_temp_file")
                                }
                                "update_error_write_update_file" => {
                                    self.tr("update_error_write_update_file")
                                }
                                msg if msg.starts_with("update_error_download_too_small") => {
                                    self.tr("update_error_download_too_small")
                                }
                                "update_error_resolve_current_exe" => {
                                    self.tr("update_error_resolve_current_exe")
                                }
                                "update_error_write_update_script" => {
                                    self.tr("update_error_write_update_script")
                                }
                                "update_error_launch_updater" => {
                                    self.tr("update_error_launch_updater")
                                }
                                "update_error_no_body" => self.tr("update_error_no_body"),
                                "update_error_hash_not_found" => {
                                    self.tr("update_error_hash_not_found")
                                }
                                "update_error_hash_compute" => self.tr("update_error_hash_compute"),
                                "update_error_hash_mismatch" => {
                                    self.tr("update_error_hash_mismatch")
                                }
                                _ => message.as_str(),
                            };
                            ui.label(display_message);
                        }
                    }
                    ui.add_space(14.0);
                    ui.horizontal(|ui| {
                        if ui.button(self.tr("update_close")).clicked() {
                            self.pending_update_dialog = None;
                        }
                        if matches!(dialog, UpdateDialog::UpdateAvailable { .. }) {
                            if self.state.is_msi_install {
                                if ui.button(self.tr("update_msi_install_btn")).clicked()
                                    && let UpdateDialog::UpdateAvailable { latest_version } = dialog.clone()
                                {
                                    self.pending_update_dialog = None;
                                    let (tx, rx) = mpsc::channel::<crate::updater::MsiDownloadEvent>();
                                    std::thread::spawn(move || {
                                        let result = updater::download_msi(&latest_version, tx.clone());
                                        let _ = tx.send(crate::updater::MsiDownloadEvent::Done(result));
                                    });
                                    self.msi_download_worker = Some(super::MsiDownloadWorker { rx });
                                }
                            } else {
                                if ui.button(self.tr("update_download_restart_btn")).clicked()
                                    && let UpdateDialog::UpdateAvailable { latest_version } = dialog.clone()
                                {
                                    self.pending_update_dialog = None;
                                    if let Err(e) = updater::download_and_install(&latest_version) {
                                        self.pending_update_dialog =
                                            Some(UpdateDialog::Error { message: e });
                                    }
                                }
                            }
                            if ui.button(self.tr("update_check_again")).clicked() {
                                self.pending_update_dialog = None;
                                self.start_update_check(true);
                            }
                        }
                    });
                });
            });
    }
}
