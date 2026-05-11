use super::WinchiselApp;
use crate::app_definitions::{AppCategory, AppItem, get_all_apps};
use eframe::egui;
use std::collections::HashSet;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::mpsc::Receiver;

#[derive(Clone)]
pub(crate) struct DebloaterState {
    pub(crate) debloater_query: String,
    pub(crate) debloater_items: Vec<AppItem>,
    pub(crate) debloater_selected: Vec<bool>,
    pub(crate) debloater_installed: Vec<bool>,
    pub(crate) debloater_tab: usize,
    pub(crate) debloater_view_mode: usize,
    pub(crate) debloater_loading: bool,
    pub(crate) debloater_filter_cache_query: String,
    pub(crate) debloater_filter_cache_tab: usize,
    pub(crate) debloater_filter_cache_view_mode: usize,
    pub(crate) debloater_filter_cache_all: Vec<usize>,
}

pub(crate) struct DebloaterLoadResult {
    pub(crate) installed: Vec<bool>,
}

pub(crate) struct DebloaterLoadWorker {
    pub(crate) rx: Receiver<DebloaterLoadResult>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum DebloaterAction {
    Install,
    Remove,
}

pub(crate) struct DebloaterInstalledState {
    pub(crate) appx: HashSet<String>,
    pub(crate) capabilities: HashSet<String>,
    pub(crate) features: HashSet<String>,
}

impl WinchiselApp {
    pub(crate) fn ensure_debloater_selection(&mut self) {
        if self.state.debloater.debloater_selected.len()
            != self.state.debloater.debloater_items.len()
        {
            self.state.debloater.debloater_selected =
                vec![false; self.state.debloater.debloater_items.len()];
        }
        if self.state.debloater.debloater_installed.len()
            != self.state.debloater.debloater_items.len()
        {
            self.state.debloater.debloater_installed =
                vec![false; self.state.debloater.debloater_items.len()];
        }
    }

    pub(crate) fn spawn_debloater_worker(
        items: Vec<AppItem>,
    ) -> (Option<DebloaterLoadWorker>, bool) {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let installed_state = Self::get_debloater_installed_state();
            let installed = items
                .iter()
                .map(|item| Self::is_app_installed(item, &installed_state))
                .collect();
            let _ = tx.send(DebloaterLoadResult { installed });
        });
        (Some(DebloaterLoadWorker { rx }), true)
    }

    pub(crate) fn start_debloater_load(&mut self) {
        if self.debloater_cache_ready
            || self.debloater_load_worker.is_some()
            || self.state.debloater.debloater_loading
        {
            return;
        }

        let (worker, loading) =
            Self::spawn_debloater_worker(self.state.debloater.debloater_items.clone());
        self.debloater_load_worker = worker;
        self.state.debloater.debloater_loading = loading;
    }

    pub(crate) fn selected_package_items(&self) -> Vec<AppItem> {
        self.state
            .debloater
            .debloater_items
            .iter()
            .zip(self.state.debloater.debloater_selected.iter())
            .filter(|(_, selected)| **selected)
            .map(|(item, _)| item.clone())
            .collect()
    }

    pub(crate) fn selected_debloater_count(&self) -> usize {
        self.state
            .debloater
            .debloater_selected
            .iter()
            .filter(|checked| **checked)
            .count()
    }

    fn selected_package_count(&self) -> usize {
        self.selected_debloater_count()
    }

    fn category_label(category: &AppCategory) -> &'static str {
        match category {
            AppCategory::WindowsApps => "debloater_tab_0",
            AppCategory::Capabilities => "debloater_tab_1",
            AppCategory::OptionalFeatures => "debloater_tab_2",
        }
    }

    fn filtered_debloater_items(&self) -> Vec<usize> {
        self.state.debloater.debloater_filter_cache_all.clone()
    }

    pub(crate) fn rebuild_debloater_filter_cache(&mut self) {
        let query = self.state.debloater.debloater_query.trim().to_lowercase();
        let tab = self.state.debloater.debloater_tab;
        let view_mode = self.state.debloater.debloater_view_mode;
        if self.state.debloater.debloater_filter_cache_query == query
            && self.state.debloater.debloater_filter_cache_tab == tab
            && self.state.debloater.debloater_filter_cache_view_mode == view_mode
            && !self.state.debloater.debloater_filter_cache_all.is_empty()
        {
            return;
        }

        self.state.debloater.debloater_filter_cache_query = query.clone();
        self.state.debloater.debloater_filter_cache_tab = tab;
        self.state.debloater.debloater_filter_cache_view_mode = view_mode;
        self.state.debloater.debloater_filter_cache_all.clear();

        for (idx, item) in self.state.debloater.debloater_items.iter().enumerate() {
            let search_match = query.is_empty()
                || item.name_lc.contains(&query)
                || item.description_lc.contains(&query)
                || item.group_lc.contains(&query)
                || item.package_name_lc.contains(&query)
                || item.package_names_lc.iter().any(|p| p.contains(&query));
            let category_match = if query.is_empty() {
                match tab {
                    0 => item.category == AppCategory::WindowsApps,
                    1 => item.category == AppCategory::Capabilities,
                    2 => item.category == AppCategory::OptionalFeatures,
                    _ => true,
                }
            } else {
                true
            };
            let view_match = match view_mode {
                1 => self.state.debloater.debloater_installed[idx],
                2 => !self.state.debloater.debloater_installed[idx],
                _ => true,
            };
            if category_match && search_match && view_match {
                self.state.debloater.debloater_filter_cache_all.push(idx);
            }
        }
    }

    pub(crate) fn run_debloater_action(&mut self, install: bool) {
        let items = self.selected_package_items();
        if items.is_empty() {
            self.state.update_status = self.tr("debloater_nothing_selected").to_string();
            return;
        }

        let mut ok = 0usize;
        let mut fail = 0usize;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        for item in items {
            let cmd = if install {
                match item.category {
                    AppCategory::WindowsApps => Some(format!(
                        r#"$pkgs = @('{}'); foreach ($pkg in $pkgs) {{ Get-AppxPackage -AllUsers "*$pkg*" -ErrorAction SilentlyContinue | ForEach-Object {{ Add-AppxPackage -DisableDevelopmentMode -Register "$($_.InstallLocation)\AppxManifest.xml" -ErrorAction SilentlyContinue }} }}"#,
                        item.package_names
                            .iter()
                            .chain(std::iter::once(&item.package_name))
                            .map(|s| Self::ps_escape(s))
                            .collect::<Vec<_>>()
                            .join("', '")
                    )),
                    AppCategory::Capabilities => Some(format!(
                        "Add-WindowsCapability -Online -Name '{}'",
                        Self::ps_escape(&item.package_name)
                    )),
                    AppCategory::OptionalFeatures => Some(format!(
                        "Enable-WindowsOptionalFeature -Online -FeatureName '{}' -NoRestart",
                        Self::ps_escape(&item.package_name)
                    )),
                }
            } else {
                match item.category {
                    AppCategory::WindowsApps => Some(format!(
                        r#"$pkgs = @('{}'); foreach ($pkg in $pkgs) {{ Get-AppxPackage -Name $pkg -AllUsers | Remove-AppxPackage }}"#,
                        item.package_names
                            .iter()
                            .chain(std::iter::once(&item.package_name))
                            .map(|s| Self::ps_escape(s))
                            .collect::<Vec<_>>()
                            .join("', '")
                    )),
                    AppCategory::Capabilities => Some(format!(
                        "Remove-WindowsCapability -Online -Name '{}'",
                        Self::ps_escape(&item.package_name)
                    )),
                    AppCategory::OptionalFeatures => Some(format!(
                        "Disable-WindowsOptionalFeature -Online -FeatureName '{}' -NoRestart",
                        Self::ps_escape(&item.package_name)
                    )),
                }
            };

            let Some(cmd) = cmd else {
                fail += 1;
                continue;
            };

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
            #[cfg(windows)]
            {
                command.creation_flags(CREATE_NO_WINDOW);
            }
            match command.output() {
                Ok(output) if output.status.success() => ok += 1,
                _ => fail += 1,
            }
        }

        self.state.update_status = if install {
            self.tr("debloater_result_install")
                .replacen("{}", &ok.to_string(), 1)
                .replacen("{}", &fail.to_string(), 1)
        } else {
            self.tr("debloater_result_remove")
                .replacen("{}", &ok.to_string(), 1)
                .replacen("{}", &fail.to_string(), 1)
        };
        self.state.debloater.debloater_selected =
            vec![false; self.state.debloater.debloater_items.len()];
        self.debloater_cache_ready = false;
        self.state.debloater.debloater_filter_cache_query.clear();
        self.state.debloater.debloater_filter_cache_tab = usize::MAX;
        self.state.debloater.debloater_filter_cache_all.clear();
        self.start_debloater_load();
    }

    pub(crate) fn render_debloater_tab(&mut self, ui: &mut egui::Ui) {
        // Repaint wird bereits im Haupt-Loop (app.rs) gehandhabt,
        // wenn debloater_load_worker.is_some()
        let title = self.tr("debloater_title").to_string();
        let subtitle = self.tr("debloater_subtitle").to_string();
        let refresh = self.tr("debloater_refresh").to_string();
        let install_selected = self.tr("debloater_install_selected").to_string();
        let remove_selected = self.tr("debloater_remove_selected").to_string();
        let search = self.tr("debloater_search").to_string();
        let loading = self.tr("debloater_loading").to_string();
        let scanning = self.tr("debloater_scanning").to_string();
        let none = self.tr("debloater_none").to_string();
        let selected_label = self.tr("debloater_nothing_selected").to_string();
        let all_items = self.tr("debloater_all_items").to_string();
        let installed_only = self.tr("debloater_installed_only").to_string();
        let not_installed_only = self.tr("debloater_not_installed_only").to_string();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading(title);
                    ui.label(subtitle);
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        for (idx, label) in [
                            self.tr("debloater_tab_0"),
                            self.tr("debloater_tab_1"),
                            self.tr("debloater_tab_2"),
                        ]
                        .iter()
                        .enumerate()
                        .rev()
                        {
                            let selected = self.state.debloater.debloater_tab == idx;
                            let fill = if selected {
                                egui::Color32::from_rgb(42, 62, 94)
                            } else {
                                egui::Color32::from_rgb(20, 20, 23)
                            };
                            let stroke = if selected {
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(88, 126, 180))
                            } else {
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(34, 34, 38))
                            };
                            if ui
                                .add_sized(
                                    [156.0, 34.0],
                                    egui::Button::new(*label)
                                        .fill(fill)
                                        .stroke(stroke)
                                        .corner_radius(8.0),
                                )
                                .clicked()
                            {
                                self.state.debloater.debloater_tab = idx;
                                self.rebuild_debloater_filter_cache();
                            }
                        }
                    });
                });
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                if ui
                    .add_sized([96.0, 34.0], egui::Button::new(refresh))
                    .clicked()
                {
                    self.state.debloater.debloater_items =
                        get_all_apps(self.state.settings.language);
                    self.state.debloater.debloater_selected =
                        vec![false; self.state.debloater.debloater_items.len()];
                    self.state.debloater.debloater_installed =
                        vec![false; self.state.debloater.debloater_items.len()];
                    self.state.debloater.debloater_filter_cache_query.clear();
                    self.state.debloater.debloater_filter_cache_tab = usize::MAX;
                    self.state.debloater.debloater_filter_cache_all.clear();
                    self.debloater_cache_ready = false;
                    self.start_debloater_load();
                }
                ui.add_space(8.0);
                let previous_view_mode = self.state.debloater.debloater_view_mode;
                egui::ComboBox::from_id_salt("debloater_view_mode")
                    .width(190.0)
                    .selected_text(match self.state.debloater.debloater_view_mode {
                        0 => all_items.as_str(),
                        1 => installed_only.as_str(),
                        2 => not_installed_only.as_str(),
                        _ => all_items.as_str(),
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.state.debloater.debloater_view_mode,
                            0,
                            all_items.as_str(),
                        );
                        ui.selectable_value(
                            &mut self.state.debloater.debloater_view_mode,
                            1,
                            installed_only.as_str(),
                        );
                        ui.selectable_value(
                            &mut self.state.debloater.debloater_view_mode,
                            2,
                            not_installed_only.as_str(),
                        );
                    });
                if self.state.debloater.debloater_view_mode != previous_view_mode {
                    self.state.debloater.debloater_filter_cache_query.clear();
                    self.state.debloater.debloater_filter_cache_tab = usize::MAX;
                    self.state.debloater.debloater_filter_cache_view_mode = usize::MAX;
                }
                ui.add_space(12.0);
                let selected_count = self.selected_package_count();
                ui.label(format!("{} {}", selected_count, selected_label));
                ui.add_space(12.0);
                if ui
                    .add_enabled(
                        selected_count > 0,
                        egui::Button::new(install_selected)
                            .min_size(egui::vec2(132.0, 34.0))
                            .fill(egui::Color32::from_rgb(35, 88, 55))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(72, 145, 92))),
                    )
                    .clicked()
                {
                    self.pending_debloater_action = Some(DebloaterAction::Install);
                }
                if ui
                    .add_enabled(
                        selected_count > 0,
                        egui::Button::new(remove_selected)
                            .min_size(egui::vec2(132.0, 34.0))
                            .fill(egui::Color32::from_rgb(100, 42, 42))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(168, 72, 72))),
                    )
                    .clicked()
                {
                    self.pending_debloater_action = Some(DebloaterAction::Remove);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_sized(
                        [280.0, 30.0],
                        egui::TextEdit::singleline(&mut self.state.debloater.debloater_query)
                            .hint_text(search),
                    );
                });
            });

            ui.add_space(12.0);
            if self.state.debloater.debloater_loading {
                ui.add_space(56.0);
                ui.vertical_centered(|ui| {
                    ui.add(egui::Spinner::new().size(28.0));
                    ui.add_space(10.0);
                    ui.strong(loading);
                    ui.label(scanning);
                });
                ui.add_space(56.0);
            } else {
                if self.state.debloater.debloater_view_mode > 2 {
                    self.state.debloater.debloater_view_mode = 0;
                }
                self.rebuild_debloater_filter_cache();
                let visible_items = self.filtered_debloater_items();
                if visible_items.is_empty() {
                    ui.label(none);
                } else {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for idx in visible_items {
                                let item = &self.state.debloater.debloater_items[idx];
                                let meta = self.tr("debloater_meta").to_string();
                                let installed_text = self.tr("debloater_installed").to_string();
                                let not_installed_text =
                                    self.tr("debloater_not_installed").to_string();
                                let cannot_reinstall_text =
                                    self.tr("debloater_cannot_reinstall").to_string();
                                let selected = &mut self.state.debloater.debloater_selected[idx];
                                let row_width = ui.available_width();
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
                                        .stroke(egui::Stroke::new(
                                            1.0,
                                            egui::Color32::from_rgb(35, 35, 39),
                                        ))
                                };
                                row_frame
                                    .corner_radius(10.0)
                                    .inner_margin(egui::Margin::symmetric(10, 8))
                                    .show(ui, |ui| {
                                        ui.horizontal_top(|ui| {
                                            let checkbox_size = 22.0;
                                            let status_width = 150.0;
                                            let text_width = (row_width
                                                - 24.0
                                                - checkbox_size
                                                - status_width
                                                - 24.0)
                                                .max(220.0);

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
                                                    ui.label(&item.name).on_hover_text(
                                                        meta.replacen("{}", &item.package_name, 1)
                                                            .replacen(
                                                                "{}",
                                                                Self::category_label(
                                                                    &item.category,
                                                                ),
                                                                1,
                                                            )
                                                            .replacen("{}", &item.group, 1),
                                                    );
                                                    ui.add_space(2.0);
                                                    ui.label(&item.description);
                                                },
                                            );

                                            ui.allocate_ui_with_layout(
                                                egui::vec2(status_width, 28.0),
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    ui.set_width(status_width);
                                                    ui.set_height(28.0);
                                                    ui.vertical(|ui| {
                                                        ui.set_width(status_width);
                                                        ui.with_layout(
                                                            egui::Layout::right_to_left(
                                                                egui::Align::Center,
                                                            ),
                                                            |ui| {
                                                                if self
                                                                    .state
                                                                    .debloater
                                                                    .debloater_installed[idx]
                                                                {
                                                                    ui.colored_label(
                                                                        egui::Color32::from_rgb(
                                                                            96, 181, 103,
                                                                        ),
                                                                        &installed_text,
                                                                    );
                                                                } else {
                                                                    ui.colored_label(
                                                                        egui::Color32::from_rgb(
                                                                            130, 130, 130,
                                                                        ),
                                                                        &not_installed_text,
                                                                    );
                                                                }
                                                            },
                                                        );
                                                        if !item.can_reinstall {
                                                            ui.with_layout(
                                                                egui::Layout::right_to_left(
                                                                    egui::Align::Center,
                                                                ),
                                                                |ui| {
                                                                    ui.colored_label(
                                                                        egui::Color32::from_rgb(
                                                                            210, 80, 80,
                                                                        ),
                                                                        &cannot_reinstall_text,
                                                                    );
                                                                },
                                                            );
                                                        }
                                                    });
                                                },
                                            );
                                        });
                                    });
                                ui.add_space(8.0);
                            }
                        });
                }
            }
        });
    }

    pub(crate) fn poll_debloater_load(&mut self) {
        let Some(worker) = self.debloater_load_worker.as_ref() else {
            return;
        };
        match worker.rx.try_recv() {
            Ok(result) => {
                self.state.debloater.debloater_installed = result.installed;
                self.state.debloater.debloater_loading = false;
                self.debloater_cache_ready = true;
                self.debloater_load_worker = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.state.debloater.debloater_loading = false;
                self.debloater_load_worker = None;
            }
        }
    }

    pub(crate) fn show_debloater_dialog(&mut self, ctx: &egui::Context) {
        let Some(action) = self.pending_debloater_action else {
            return;
        };

        egui::Window::new(match action {
            DebloaterAction::Install => self.tr("debloater_confirm_install_title"),
            DebloaterAction::Remove => self.tr("debloater_confirm_remove_title"),
        })
        .collapsible(false)
        .resizable(false)
        .default_width(420.0)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            let items = self.selected_package_items();
            ui.label(match action {
                DebloaterAction::Install => self.tr("debloater_confirm_install_desc"),
                DebloaterAction::Remove => self.tr("debloater_confirm_remove_desc"),
            });
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
                let ok_label = match action {
                    DebloaterAction::Install => self.tr("debloater_confirm_install_btn"),
                    DebloaterAction::Remove => self.tr("debloater_confirm_remove_btn"),
                };
                if ui.button(ok_label).clicked() {
                    self.pending_debloater_action = None;
                    self.run_debloater_action(matches!(action, DebloaterAction::Install));
                }
                if ui.button(self.tr("debloater_cancel")).clicked() {
                    self.pending_debloater_action = None;
                }
            });
        });
    }

    fn get_debloater_installed_state() -> DebloaterInstalledState {
        let mut appx = HashSet::new();
        let mut capabilities = HashSet::new();
        let mut features = HashSet::new();

        let appx_handle = std::thread::spawn(|| {
            Self::ps_lines("Get-AppxPackage | Select-Object -ExpandProperty Name")
        });
        let caps_handle = std::thread::spawn(|| {
            Self::ps_lines(
                "Get-WindowsCapability -Online | Where-Object State -eq Installed | Select-Object -ExpandProperty Name",
            )
        });
        let features_handle = std::thread::spawn(|| {
            Self::ps_lines(
                "Get-WindowsOptionalFeature -Online | Where-Object State -eq Enabled | Select-Object -ExpandProperty FeatureName",
            )
        });

        if let Ok(lines) = appx_handle.join() {
            for line in lines {
                appx.insert(line.to_lowercase());
            }
        }
        if let Ok(lines) = caps_handle.join() {
            for line in lines {
                capabilities.insert(line.split("~~~~").next().unwrap_or("").to_lowercase());
            }
        }
        if let Ok(lines) = features_handle.join() {
            for line in lines {
                features.insert(line.to_lowercase());
            }
        }

        DebloaterInstalledState {
            appx,
            capabilities,
            features,
        }
    }

    fn is_app_installed(item: &AppItem, installed: &DebloaterInstalledState) -> bool {
        match item.category {
            AppCategory::WindowsApps => {
                let package_names = if item.package_names_lc.is_empty() {
                    std::slice::from_ref(&item.package_name_lc)
                } else {
                    item.package_names_lc.as_slice()
                };
                package_names.iter().any(|name| {
                    installed.appx.iter().any(|installed_name| {
                        installed_name.contains(name) || name.contains(installed_name)
                    })
                })
            }
            AppCategory::Capabilities => installed.capabilities.iter().any(|installed_name| {
                installed_name.contains(&item.package_name_lc)
                    || item.package_name_lc.contains(installed_name)
            }),
            AppCategory::OptionalFeatures => installed.features.iter().any(|installed_name| {
                installed_name.contains(&item.package_name_lc)
                    || item.package_name_lc.contains(installed_name)
            }),
        }
    }
}
