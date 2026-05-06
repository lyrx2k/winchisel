use super::WinchiselApp;
use crate::download_definitions::{DownloadApp, DownloadCategory, get_all_downloads};
use eframe::egui;
use std::process::Command;

impl WinchiselApp {
    fn rebuild_downloads_filter_cache(&mut self) {
        let query = self.state.downloads.downloads_query.trim().to_lowercase();
        if self.state.downloads.downloads_filter_cache_query == query
            && !self.state.downloads.downloads_filter_cache_all.is_empty()
        {
            return;
        }

        self.state.downloads.downloads_filter_cache_query = query.clone();
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
            if search_match {
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

    fn filtered_download_items(&self) -> Vec<usize> {
        self.state.downloads.downloads_filter_cache_all.clone()
    }

    fn selected_download_count(&self) -> usize {
        self.state
            .downloads
            .downloads_selected
            .iter()
            .filter(|checked| **checked)
            .count()
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
        let selected = self
            .state
            .downloads
            .downloads_selected
            .get_mut(idx)
            .expect("download selection state in sync");
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
                            ui.label(&item.name).on_hover_text(format!(
                                "Winget IDs: {}\nCategory: {}\nWebsite: {}",
                                if item.winget_ids.is_empty() {
                                    "none".to_string()
                                } else {
                                    item.winget_ids.join(", ")
                                },
                                Self::category_label_download(&item.category),
                                item.website_url
                            ));
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
                                        .add_sized([72.0, 20.0], egui::Button::new("Website"))
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
                                        ("Installed", egui::Color32::from_rgb(96, 181, 103))
                                    } else {
                                        ("Not installed", egui::Color32::from_rgb(210, 80, 80))
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

    pub(crate) fn run_download_install(&mut self) {
        let items = self.selected_download_items();
        if items.is_empty() {
            self.state.update_status = "Nothing selected".to_string();
            return;
        }

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

        self.state.update_status = format!("Installed: {}  Failed: {}", ok, fail);
        self.downloads_cache_ready = false;
        self.start_downloads_load();
    }

    fn downloads_category_label(idx: usize) -> &'static str {
        match idx {
            0 => "Browsers",
            1 => "Document Viewers",
            2 => "Messaging, Email & Calendar",
            3 => "Online Storage & Backup",
            4 => "Multimedia",
            5 => "Imaging",
            6 => "Customization Utilities",
            7 => "Gaming",
            8 => "Compression",
            9 => "File & Disk Management",
            10 => "Remote Access",
            11 => "Optical Disc Tools",
            12 => "Other Utilities",
            13 => "Privacy & Security",
            14 => "Development Apps",
            _ => "Runtimes & Dependencies",
        }
    }

    pub(crate) fn render_downloads_tab(&mut self, ui: &mut egui::Ui) {
        self.start_downloads_load();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Apps & Downloads");
                    ui.label("Install popular software with one click.");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_sized(
                        [280.0, 30.0],
                        egui::TextEdit::singleline(&mut self.state.downloads.downloads_query)
                            .hint_text("Search apps & downloads..."),
                    );
                });
            });
            self.rebuild_downloads_filter_cache();

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                if ui
                    .add_sized([96.0, 34.0], egui::Button::new("Refresh"))
                    .clicked()
                {
                    self.state.downloads.downloads_items = get_all_downloads();
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
                ui.label(format!("{} selected", self.selected_download_count()));
                ui.add_space(12.0);
                let install_enabled = self
                    .selected_download_items()
                    .iter()
                    .any(|item| !item.winget_ids.is_empty());
                if ui
                    .add_enabled(
                        install_enabled,
                        egui::Button::new("Install Selected")
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
                    ui.strong("Loading apps & downloads...");
                });
                ui.add_space(40.0);
            }

            let visible_items = self.filtered_download_items();
            if visible_items.is_empty() {
                ui.label("No apps or downloads to display.");
            } else {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for idx in 0..16 {
                            let label = Self::downloads_category_label(idx);
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
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                ui.add_space(4.0);
                                ui.separator();
                                ui.add_space(10.0);
                                ui.label(
                                    egui::RichText::new(label)
                                        .strong()
                                        .size(13.0)
                                        .color(egui::Color32::from_rgb(149, 194, 255)),
                                );
                            });
                            ui.add_space(8.0);
                            for item_idx in category_items {
                                if visible_items.contains(&item_idx) {
                                    self.render_download_row(ui, item_idx);
                                    ui.add_space(8.0);
                                }
                            }
                            ui.add_space(14.0);
                        }
                    });
            }
        });
    }
}
