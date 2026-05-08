use super::{DownloadInstallResult, DownloadInstallWorker, WinchiselApp, DOWNLOAD_SCAN_CACHE};
use crate::download_definitions::{DownloadApp, DownloadCategory, get_all_downloads};
use eframe::egui;
use std::collections::HashSet;
use std::process::Command;
use std::sync::mpsc;

impl WinchiselApp {
    fn rebuild_downloads_filter_cache(&mut self) {
        let query = self.state.downloads.downloads_query.trim().to_lowercase();
        let view_mode = self.state.downloads.downloads_view_mode;
        if self.state.downloads.downloads_filter_cache_query == query
            && self.state.downloads.downloads_filter_cache_view_mode == view_mode
            && !self.state.downloads.downloads_filter_cache_all.is_empty()
        {
            return;
        }

        self.state.downloads.downloads_filter_cache_query = query.clone();
        self.state.downloads.downloads_filter_cache_view_mode = view_mode;
        self.state.downloads.downloads_filter_cache_all.clear();
        for bucket in &mut self.state.downloads.downloads_filter_cache_categories {
            bucket.clear();
        }

        for (idx, item) in self.state.downloads.downloads_items.iter().enumerate() {
            let search_match = query.is_empty()
                || item.name_lc.contains(&query)
                || item.description_lc.contains(&query)
                || item.website_url_lc.contains(&query)
                || item.winget_ids_lc.iter().any(|id| id.contains(&query));
            let installed = self
                .state
                .downloads
                .downloads_installed
                .get(idx)
                .copied()
                .unwrap_or(false);
            let view_match = match view_mode {
                1 => installed,
                2 => !installed,
                _ => true,
            };
            if search_match && view_match {
                self.state.downloads.downloads_filter_cache_all.push(idx);
                let category_idx = match item.category {
                    DownloadCategory::Browsers => 0,
                    DownloadCategory::DocumentViewers => 1,
                    DownloadCategory::MessagingEmailCalendar => 2,
                    DownloadCategory::OnlineStorageBackup => 3,
                    DownloadCategory::Multimedia => 4,
                    DownloadCategory::Imaging => 5,
                    DownloadCategory::CustomizationUtilities => 6,
                    DownloadCategory::Gaming => 7,
                    DownloadCategory::Compression => 8,
                    DownloadCategory::FileDiskManagement => 9,
                    DownloadCategory::RemoteAccess => 10,
                    DownloadCategory::OpticalDiscTools => 11,
                    DownloadCategory::OtherUtilities => 12,
                    DownloadCategory::PrivacySecurity => 13,
                    DownloadCategory::DevelopmentApps => 14,
                    DownloadCategory::RuntimesDependencies => 15,
                };
                self.state.downloads.downloads_filter_cache_categories[category_idx].push(idx);
            }
        }
    }

    fn filtered_download_items(&self) -> &[usize] {
        &self.state.downloads.downloads_filter_cache_all
    }

    fn has_installable_selected_download(&self) -> bool {
        self.state
            .downloads
            .downloads_items
            .iter()
            .zip(self.state.downloads.downloads_selected.iter())
            .any(|(item, selected)| *selected && !item.winget_ids.is_empty())
    }

    pub(crate) fn selected_download_items(&self) -> Vec<DownloadApp> {
        self.state
            .downloads
            .downloads_items
            .iter()
            .zip(self.state.downloads.downloads_selected.iter())
            .filter(|(_, selected)| **selected)
            .map(|(item, _)| item.clone())
            .collect()
    }

    fn render_download_row(&mut self, ui: &mut egui::Ui, idx: usize) {
        let item = &self.state.downloads.downloads_items[idx];
        if idx >= self.state.downloads.downloads_selected.len() {
            return;
        }
        let meta = self.tr("download_meta").to_string();
        let website = self.tr("download_website").to_string();
        let installed_label = self.tr("download_installed").to_string();
        let not_installed_label = self.tr("download_not_installed").to_string();
        let category_label = self.category_label_download(&item.category);
        let selected = &mut self.state.downloads.downloads_selected[idx];
        let row_frame = if *selected {
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(27, 32, 44))
                .stroke(egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgb(88, 126, 180),
                ))
        } else {
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(20, 20, 23))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 35, 39)))
        };

        row_frame
            .corner_radius(10.0)
            .inner_margin(egui::Margin::symmetric(10, 10))
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    let checkbox_size = 22.0;
                    let action_width = 212.0;
                    let action_height = 38.0;
                    let text_width =
                        (ui.available_width() - checkbox_size - action_width - 28.0).max(220.0);

                    ui.add_sized(
                        [checkbox_size, checkbox_size],
                        egui::Checkbox::new(selected, ""),
                    );
                    ui.add_space(10.0);
                    ui.allocate_ui_with_layout(
                        egui::vec2(text_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(text_width);
                            let winget_ids = if item.winget_ids.is_empty() {
                                "none".to_string()
                            } else {
                                item.winget_ids.join(", ")
                            };
                            ui.label(&item.name).on_hover_text(
                                meta.replacen("{}", &winget_ids, 1)
                                    .replacen("{}", &category_label, 1)
                                    .replacen("{}", &item.website_url, 1),
                            );
                            ui.add_space(2.0);
                            ui.label(&item.description);
                        },
                    );

                    ui.allocate_ui_with_layout(
                        egui::vec2(action_width, action_height),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.set_width(action_width);
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .add_sized([72.0, 20.0], egui::Button::new(website))
                                        .clicked()
                                    {
                                        let _ = Command::new("cmd")
                                            .args(["/C", "start", "", &item.website_url])
                                            .spawn();
                                    }
                                    ui.add_space(12.0);
                                    let installed = self
                                        .state
                                        .downloads
                                        .downloads_installed
                                        .get(idx)
                                        .copied()
                                        .unwrap_or(false);
                                    let (text, color) = if installed {
                                        (installed_label, egui::Color32::from_rgb(96, 181, 103))
                                    } else {
                                        (not_installed_label, egui::Color32::from_rgb(210, 80, 80))
                                    };
                                    ui.add_sized(
                                        [100.0, 14.0],
                                        egui::Label::new(
                                            egui::RichText::new(text).size(10.0).color(color),
                                        ),
                                    );
                                },
                            );
                        },
                    );
                });
            });
    }

    pub(crate) fn start_download_install(&mut self) {
        if self.downloads_install_worker.is_some() {
            return;
        }
        let items = self.selected_download_items();
        if items.is_empty() {
            self.state.update_status = self.tr("download_nothing_selected").to_string();
            return;
        }

        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let mut ok = 0usize;
            let mut fail = 0usize;

            for item in items {
                let Some(winget_id) = item.winget_ids.first() else {
                    fail += 1;
                    continue;
                };

                let cmd = format!(
                    "winget install --id \"{}\" --exact --accept-package-agreements --accept-source-agreements",
                    winget_id
                );
                let mut command = Command::new("powershell.exe");
                command.args([
                    "-NoLogo",
                    "-NoProfile",
                    "-NonInteractive",
                    "-WindowStyle",
                    "Hidden",
                    "-Command",
                    &cmd,
                ]);
                match command.output() {
                    Ok(output) if output.status.success() => ok += 1,
                    _ => fail += 1,
                }
            }

            let _ = tx.send(DownloadInstallResult { ok, fail });
        });
        self.downloads_install_worker = Some(DownloadInstallWorker { rx });
        self.state.update_status = self.tr("download_installing").to_string();
    }

    pub(crate) fn poll_download_install(&mut self) {
        let Some(worker) = self.downloads_install_worker.as_ref() else {
            return;
        };
        match worker.rx.try_recv() {
            Ok(result) => {
                self.state.update_status = self
                    .tr("download_result")
                    .replacen("{}", &result.ok.to_string(), 1)
                    .replacen("{}", &result.fail.to_string(), 1);
                self.downloads_cache_ready = false;
                self.downloads_install_worker = None;
                if let Ok(mut cache) = DOWNLOAD_SCAN_CACHE.lock() {
                    *cache = None;
                }
                self.start_downloads_load();
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.state.update_status = self.tr("download_install_job_failed").to_string();
                self.downloads_install_worker = None;
            }
        }
    }

    fn downloads_category_label(&self, idx: usize) -> &'static str {
        match idx {
            0 => self.tr("download_category_0"),
            1 => self.tr("download_category_1"),
            2 => self.tr("download_category_2"),
            3 => self.tr("download_category_3"),
            4 => self.tr("download_category_4"),
            5 => self.tr("download_category_5"),
            6 => self.tr("download_category_6"),
            7 => self.tr("download_category_7"),
            8 => self.tr("download_category_8"),
            9 => self.tr("download_category_9"),
            10 => self.tr("download_category_10"),
            11 => self.tr("download_category_11"),
            12 => self.tr("download_category_12"),
            13 => self.tr("download_category_13"),
            14 => self.tr("download_category_14"),
            _ => self.tr("download_category_15"),
        }
    }

    pub(crate) fn render_downloads_tab(&mut self, ui: &mut egui::Ui) {
        self.start_downloads_load();
        let title = self.tr("download_title").to_string();
        let subtitle = self.tr("download_subtitle").to_string();
        let search = self.tr("download_search").to_string();
        let refresh = self.tr("download_refresh").to_string();
        let install_selected = self.tr("download_install_selected").to_string();
        let loading = self.tr("download_loading").to_string();
        let none = self.tr("download_none").to_string();
        let selected_label = self.tr("downloads_selected").to_string();
        let all_items = self.tr("debloater_all_items").to_string();
        let installed_only = self.tr("debloater_installed_only").to_string();
        let not_installed_only = self.tr("debloater_not_installed_only").to_string();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading(title);
                    ui.label(subtitle);
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_sized(
                        [280.0, 30.0],
                        egui::TextEdit::singleline(&mut self.state.downloads.downloads_query)
                            .hint_text(search),
                    );
                });
            });
            self.rebuild_downloads_filter_cache();

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                if ui
                    .add_sized([96.0, 34.0], egui::Button::new(refresh))
                    .clicked()
                {
                    self.state.downloads.downloads_items =
                        get_all_downloads(self.state.settings.language);
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
                    self.start_downloads_load();
                }
                ui.add_space(8.0);
                ui.label(selected_label);
                ui.add_space(12.0);
                let previous_view_mode = self.state.downloads.downloads_view_mode;
                egui::ComboBox::from_id_salt("downloads_view_mode")
                    .width(180.0)
                    .selected_text(match self.state.downloads.downloads_view_mode {
                        1 => installed_only.clone(),
                        2 => not_installed_only.clone(),
                        _ => all_items.clone(),
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.state.downloads.downloads_view_mode,
                            0,
                            &all_items,
                        );
                        ui.selectable_value(
                            &mut self.state.downloads.downloads_view_mode,
                            1,
                            &installed_only,
                        );
                        ui.selectable_value(
                            &mut self.state.downloads.downloads_view_mode,
                            2,
                            &not_installed_only,
                        );
                    });
                if self.state.downloads.downloads_view_mode != previous_view_mode {
                    self.state.downloads.downloads_filter_cache_query.clear();
                    self.state.downloads.downloads_filter_cache_view_mode = usize::MAX;
                    self.state.downloads.downloads_filter_cache_all.clear();
                }
                ui.add_space(12.0);
                let install_enabled = self.has_installable_selected_download();
                if ui
                    .add_enabled(
                        install_enabled,
                        egui::Button::new(install_selected)
                            .min_size(egui::vec2(132.0, 34.0))
                            .fill(egui::Color32::from_rgb(35, 88, 55))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(72, 145, 92))),
                    )
                    .clicked()
                {
                    self.pending_download_action = Some(super::DownloadAction::Install);
                }
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);

            if self.state.downloads.downloads_loading {
                ui.add_space(40.0);
                ui.vertical_centered(|ui| {
                    ui.add(egui::Spinner::new().size(28.0));
                    ui.add_space(10.0);
                    ui.strong(loading);
                });
                ui.add_space(40.0);
            }

            let visible_items: HashSet<usize> =
                self.filtered_download_items().iter().copied().collect();
            if visible_items.is_empty() {
                ui.label(none);
            } else {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for idx in 0..16 {
                            let label = self.downloads_category_label(idx);
                            let category_items = self
                                .state
                                .downloads
                                .downloads_filter_cache_categories
                                .get(idx)
                                .cloned()
                                .unwrap_or_default();
                            if category_items.is_empty() {
                                continue;
                            }
                            let header = egui::CollapsingHeader::new(
                                egui::RichText::new(label)
                                    .strong()
                                    .size(15.0)
                                    .color(egui::Color32::from_rgb(149, 194, 255)),
                            )
                            .id_salt(("downloads_category", idx))
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.add_space(10.0);
                                for item_idx in category_items {
                                    if visible_items.contains(&item_idx) {
                                        self.render_download_row(ui, item_idx);
                                        ui.add_space(8.0);
                                    }
                                }
                                ui.add_space(8.0);
                            });
                            Self::tree_header_hover(
                                ui,
                                &header.header_response,
                            );
                        }
                    });
            }
        });
    }
}
