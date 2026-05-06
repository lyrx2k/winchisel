use super::{HomeState, WinchiselApp};
use eframe::egui;
use iconflow::Pack;

impl WinchiselApp {
    pub(crate) fn render_home_tab(&mut self, ui: &mut egui::Ui) {
        let home = &self.state.home;
        let system = self.tr("home_system");
        let processor = self.tr("home_processor");
        let graphics = self.tr("home_graphics");
        let memory = self.tr("home_memory");
        let storage = self.tr("home_storage");
        let windows = self.tr("home_windows");
        let uptime = self.tr("home_uptime");
        let performance = self.tr("home_performance");
        let product_name = self.tr("home_product_name");
        let cpu = self.tr("home_cpu");
        let cpu_model = self.tr("home_cpu_model");
        let cores = self.tr("home_cores");
        let gpu = self.tr("home_gpu");
        let memory_total = self.tr("home_memory_total");
        let memory_used = self.tr("home_memory_used");
        let version = self.tr("home_version");
        let kernel = self.tr("home_kernel");
        ui.vertical(|ui| {
            let gap = 10.0;
            let full = ui.available_width();
            let top_col_w = ((full - gap) / 2.0).floor();
            egui::Grid::new("home_top_grid")
                .num_columns(2)
                .striped(false)
                .min_col_width(top_col_w)
                .max_col_width(top_col_w)
                .spacing(egui::vec2(gap, gap))
                .show(ui, |ui| {
                    let cards = [
                        (
                            "monitor",
                            system,
                            home.computer_name.as_str(),
                            Some(product_name),
                            egui::Color32::from_rgb(10, 210, 254),
                            "system",
                        ),
                        (
                            "cpu",
                            processor,
                            home.cpu_brand.as_str(),
                            Some(home.cpu_cores.as_str()),
                            egui::Color32::from_rgb(10, 210, 254),
                            "cpu",
                        ),
                        (
                            "image",
                            graphics,
                            home.gpu_name.as_str(),
                            None,
                            egui::Color32::from_rgb(10, 210, 254),
                            "gpu",
                        ),
                        (
                            "memory-stick",
                            memory,
                            home.memory_total.as_str(),
                            Some(home.memory_used.as_str()),
                            egui::Color32::from_rgb(10, 210, 254),
                            "memory",
                        ),
                        (
                            "hard-drive",
                            storage,
                            home.storage_total.as_str(),
                            Some(home.storage_used.as_str()),
                            egui::Color32::from_rgb(10, 210, 254),
                            "stats",
                        ),
                        (
                            "windows",
                            windows,
                            home.os_version.as_str(),
                            Some(home.kernel_version.as_str()),
                            egui::Color32::from_rgb(10, 210, 254),
                            "windows",
                        ),
                        (
                            "clock-3",
                            uptime,
                            home.uptime.as_str(),
                            Some(self.state.update_status.as_str()),
                            egui::Color32::from_rgb(10, 210, 254),
                            "stats",
                        ),
                        (
                            "gauge",
                            performance,
                            home.cpu_usage.as_str(),
                            Some("-"),
                            egui::Color32::from_rgb(10, 210, 254),
                            "perf",
                        ),
                    ];

                    for (idx, (icon_name, label, value, subtitle, color, kind)) in
                        cards.into_iter().enumerate()
                    {
                        let resp = Self::home_metric_card(
                            ui,
                            Self::icon_text(Pack::Lucide, icon_name, 16.0, color),
                            label,
                            value,
                            subtitle,
                            idx == 0,
                        );
                        match kind {
                            "system" => resp.on_hover_ui(|ui| {
                                Self::home_info_popup(ui, home, self.state.settings.language)
                            }),
                            "cpu" => resp.on_hover_ui(|ui| {
                                Self::detail_kv_rows(
                                    ui,
                                    &[
                                        (cpu, home.cpu_usage.as_str()),
                                        (cpu_model, home.cpu_brand.as_str()),
                                        (cores, home.cpu_cores.as_str()),
                                    ],
                                );
                            }),
                            "gpu" => resp.on_hover_ui(|ui| {
                                Self::detail_kv_rows(ui, &[(gpu, home.gpu_name.as_str())]);
                            }),
                            "memory" => resp.on_hover_ui(|ui| {
                                Self::detail_kv_rows(
                                    ui,
                                    &[
                                        (memory_total, home.memory_total.as_str()),
                                        (memory_used, home.memory_used.as_str()),
                                    ],
                                );
                            }),
                            "windows" => resp.on_hover_ui(|ui| {
                                Self::detail_kv_rows(
                                    ui,
                                    &[
                                        (version, home.os_version.as_str()),
                                        (kernel, home.kernel_version.as_str()),
                                    ],
                                );
                            }),
                            _ => resp.on_hover_ui(|ui| {
                                Self::home_info_popup(ui, home, self.state.settings.language)
                            }),
                        };
                        if idx % 2 == 1 {
                            ui.end_row();
                        }
                    }
                });
        });
    }

    fn home_card_frame(selected: bool) -> egui::Frame {
        let fill = if selected {
            egui::Color32::from_rgb(33, 40, 52)
        } else {
            egui::Color32::from_rgb(38, 38, 40)
        };
        let stroke = if selected {
            egui::Stroke::new(1.0, egui::Color32::from_rgb(10, 210, 254))
        } else {
            egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 48, 52))
        };
        egui::Frame::new()
            .fill(fill)
            .stroke(stroke)
            .corner_radius(10.0)
            .inner_margin(egui::Margin::symmetric(14, 12))
    }

    fn home_metric_card(
        ui: &mut egui::Ui,
        icon: egui::RichText,
        label: &str,
        value: &str,
        subtitle: Option<&str>,
        selected: bool,
    ) -> egui::Response {
        Self::home_card_frame(selected)
            .show(ui, |ui| {
                ui.set_min_height(60.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    ui.label(icon);
                    ui.label(egui::RichText::new(label).strong().size(13.0));
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(value)
                            .size(14.0)
                            .color(egui::Color32::from_rgb(232, 232, 232)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(subtitle) = subtitle {
                            ui.label(
                                egui::RichText::new(subtitle)
                                    .size(10.0)
                                    .color(egui::Color32::from_gray(170)),
                            );
                        }
                    });
                });
            })
            .response
    }

    fn detail_kv_rows(ui: &mut egui::Ui, rows: &[(&str, &str)]) {
        ui.vertical(|ui| {
            for (key, value) in rows {
                Self::home_kv(ui, key, value);
            }
        });
    }

    fn home_info_popup(ui: &mut egui::Ui, state: &HomeState, lang: crate::Language) {
        Self::detail_kv_rows(
            ui,
            &[
                (
                    crate::i18n::t(lang, "home_name"),
                    state.computer_name.as_str(),
                ),
                (
                    crate::i18n::t(lang, "home_bios_version"),
                    state.bios_version.as_str(),
                ),
                (
                    crate::i18n::t(lang, "home_bios_date"),
                    state.bios_date.as_str(),
                ),
                (crate::i18n::t(lang, "home_cpu"), state.cpu_usage.as_str()),
                (crate::i18n::t(lang, "home_gpu"), state.gpu_name.as_str()),
            ],
        );
    }

    fn home_kv(ui: &mut egui::Ui, key: &str, value: &str) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(key).color(egui::Color32::from_gray(170)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(value);
            });
        });
    }
}
