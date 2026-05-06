use super::{HomeState, WinchiselApp};
use eframe::egui;
use iconflow::Pack;

impl WinchiselApp {
    pub(crate) fn render_home_tab(&mut self, ui: &mut egui::Ui) {
        let home = self.state.home.clone();
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
                        ("monitor", "System", home.computer_name.clone(), Some("System Product Name".to_string()), egui::Color32::from_rgb(124, 171, 238), "system"),
                        ("cpu", "Processor", home.cpu_brand.clone(), Some(home.cpu_cores.clone()), egui::Color32::from_rgb(236, 184, 87), "cpu"),
                        ("image", "Graphics", home.gpu_name.clone(), None, egui::Color32::from_rgb(234, 149, 92), "gpu"),
                        ("memory-stick", "Memory", home.memory_total.clone(), Some(home.memory_used.clone()), egui::Color32::from_rgb(186, 129, 235), "memory"),
                        ("hard-drive", "Storage", home.storage_total.clone(), Some(home.storage_used.clone()), egui::Color32::from_rgb(112, 169, 236), "stats"),
                        ("windows", "Windows", home.os_version.clone(), Some(home.kernel_version.clone()), egui::Color32::from_rgb(145, 188, 110), "windows"),
                        ("clock-3", "Uptime", home.uptime.clone(), Some(self.state.update_status.clone()), egui::Color32::from_rgb(132, 198, 255), "stats"),
                        ("gauge", "Performance", home.cpu_usage.clone(), Some("-".to_string()), egui::Color32::from_rgb(132, 198, 255), "perf"),
                    ];

                    for (idx, (icon_name, label, value, subtitle, color, kind)) in cards.into_iter().enumerate() {
                        let resp = Self::home_metric_card(
                            ui,
                            Self::icon_text(Pack::Lucide, icon_name, 16.0, color),
                            label,
                            &value,
                            subtitle.as_deref(),
                            idx == 0,
                        );
                        match kind {
                            "system" => resp.on_hover_ui(|ui| Self::home_info_popup(ui, &home)),
                            "cpu" => resp.on_hover_ui(|ui| {
                                Self::detail_kv_rows(
                                    ui,
                                    &[
                                        ("CPU", home.cpu_usage.clone()),
                                        ("CPU Model", home.cpu_brand.clone()),
                                        ("Cores", home.cpu_cores.clone()),
                                    ],
                                );
                            }),
                            "gpu" => resp.on_hover_ui(|ui| {
                                Self::detail_kv_rows(
                                    ui,
                                    &[("GPU", home.gpu_name.clone())],
                                );
                            }),
                            "memory" => resp.on_hover_ui(|ui| {
                                Self::detail_kv_rows(
                                    ui,
                                    &[
                                        ("Memory Total", home.memory_total.clone()),
                                        ("Memory Used", home.memory_used.clone()),
                                    ],
                                );
                            }),
                            "windows" => resp.on_hover_ui(|ui| {
                                Self::detail_kv_rows(
                                    ui,
                                    &[
                                        ("Version", home.os_version.clone()),
                                        ("Kernel", home.kernel_version.clone()),
                                    ],
                                );
                            }),
                            _ => resp.on_hover_ui(|ui| Self::home_info_popup(ui, &home)),
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
            egui::Stroke::new(1.0, egui::Color32::from_rgb(92, 126, 180))
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

    fn detail_kv_rows(ui: &mut egui::Ui, rows: &[(&str, String)]) {
        ui.vertical(|ui| {
            for (key, value) in rows {
                Self::home_kv(ui, key, value);
            }
        });
    }

    fn home_info_popup(ui: &mut egui::Ui, state: &HomeState) {
        Self::detail_kv_rows(
            ui,
            &[
                ("Name", state.computer_name.clone()),
                ("BIOS Version", state.bios_version.clone()),
                ("BIOS Date", state.bios_date.clone()),
                ("CPU", state.cpu_usage.clone()),
                ("GPU", state.gpu_name.clone()),
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


