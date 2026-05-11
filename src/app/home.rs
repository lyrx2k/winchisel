use super::{HomeState, WinchiselApp};
use eframe::egui;
use iconflow::Pack;

impl WinchiselApp {
    pub(crate) fn render_home_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);

            // ===== HARDWARE SECTION =====
            ui.horizontal(|ui| {
                ui.heading(self.tr("home_section_hardware"));
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("━━")
                        .color(egui::Color32::from_rgb(10, 210, 254))
                        .size(14.0),
                );
            });
            ui.add_space(10.0);
            self.render_hardware_cards(ui, &self.state.home);
            ui.add_space(20.0);

            // ===== SYSTEM SECTION =====
            ui.horizontal(|ui| {
                ui.heading(self.tr("home_section_system"));
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("━━")
                        .color(egui::Color32::from_rgb(96, 181, 103))
                        .size(14.0),
                );
            });
            ui.add_space(10.0);
            self.render_system_cards(ui, &self.state.home);
        });
    }

    fn render_hardware_cards(&self, ui: &mut egui::Ui, home: &HomeState) {
        let full = ui.available_width();
        let gap = 12.0;
        let col_w = ((full - gap) / 2.0).floor();
        egui::Grid::new("home_hardware_grid")
            .num_columns(2)
            .striped(false)
            .min_col_width(col_w)
            .max_col_width(col_w)
            .spacing(egui::vec2(gap, gap))
            .show(ui, |ui| {
                let cpu_sub = if home.cpu_speed.is_empty() {
                    home.cpu_cores.clone()
                } else {
                    format!("{} · {}", home.cpu_cores, home.cpu_speed)
                };
                self.hardware_card(
                    ui,
                    "cpu",
                    self.tr("home_processor"),
                    &home.cpu_brand,
                    Some(&cpu_sub),
                    None,
                    Some(&home.cpu_usage),
                    (home.cpu_usage_percent / 100.0).clamp(0.0, 1.0),
                    egui::Color32::from_rgb(10, 210, 254),
                );
                self.hardware_card(
                    ui,
                    "image",
                    self.tr("home_graphics"),
                    &home.gpu_name,
                    Some(&home.gpu_vram),
                    Some(&home.gpu_driver_version),
                    None,
                    0.0,
                    egui::Color32::from_rgb(10, 210, 254),
                );
                ui.end_row();
                let mem_pct = (home.memory_used_gb / home.memory_total_gb.max(1.0) * 100.0) as i32;
                let mem_value = format!("{} · {}", home.memory_total, home.memory_used);
                self.hardware_card(
                    ui,
                    "memory-stick",
                    self.tr("home_memory"),
                    &mem_value,
                    Some(&home.ram_details),
                    None,
                    Some(&format!("{mem_pct}%")),
                    (home.memory_used_gb / home.memory_total_gb.max(1.0)).clamp(0.0, 1.0) as f32,
                    egui::Color32::from_rgb(10, 210, 254),
                );
                let stor_pct = (home.storage_used_gb / home.storage_total_gb.max(1.0) * 100.0) as i32;
                self.hardware_card(
                    ui,
                    "hard-drive",
                    self.tr("home_storage"),
                    &home.storage_total,
                    Some(&home.storage_used),
                    None,
                    Some(&format!("{stor_pct}%")),
                    (home.storage_used_gb / home.storage_total_gb.max(1.0)).clamp(0.0, 1.0) as f32,
                    egui::Color32::from_rgb(10, 210, 254),
                );
                ui.end_row();
            });
    }

    fn render_system_cards(&self, ui: &mut egui::Ui, home: &HomeState) {
        let gap = 12.0;
        let full = ui.available_width();
        let card_w = ((full - gap * 3.0) / 4.0).floor().max(120.0);
        let cards = [
            (
                "layout-dashboard",
                self.tr("home_windows"),
                home.os_version.as_str(),
                Some(format!("Build {}", home.windows_build)),
                Some(format!("{} — {}", home.computer_name, home.kernel_version)),
                egui::Color32::from_rgb(96, 181, 103),
            ),
            (
                "monitor",
                self.tr("home_motherboard"),
                home.motherboard.as_str(),
                {
                    let bios_info = if home.bios_date.is_empty() {
                        home.bios_version.clone()
                    } else {
                        format!("{} — {}", home.bios_version, home.bios_date)
                    };
                    Some(bios_info)
                },
                None,
                egui::Color32::from_rgb(96, 181, 103),
            ),
            (
                "scan",
                self.tr("home_display"),
                home.display_info.as_str(),
                None,
                None,
                egui::Color32::from_rgb(10, 210, 254),
            ),
            (
                "clock-3",
                self.tr("home_uptime"),
                home.uptime.as_str(),
                None,
                None,
                egui::Color32::from_rgb(226, 196, 84),
            ),
        ];
        ui.with_layout(
            egui::Layout::left_to_right(egui::Align::Min)
                .with_main_justify(true)
                .with_cross_align(egui::Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing.x = gap;
                for (icon, label, value, subtitle, extra, accent) in cards {
                    ui.vertical(|ui| {
                        ui.set_min_width(card_w);
                        self.system_card(ui, icon, label, value, subtitle.as_deref(), extra.as_deref(), accent);
                    });
                }
            },
        );
    }

    fn hardware_card(
        &self,
        ui: &mut egui::Ui,
        icon_name: &str,
        label: &str,
        value: &str,
        subtitle: Option<&str>,
        extra: Option<&str>,
        usage: Option<&str>,
        progress: f32,
        accent: egui::Color32,
    ) {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(38, 38, 40))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 48, 52)))
            .corner_radius(10.0)
            .inner_margin(egui::Margin::symmetric(14, 12))
            .show(ui, |ui| {
                ui.set_min_height(86.0);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        let icon_text = if let Ok(icon) = iconflow::try_icon(
                            Pack::Lucide,
                            icon_name,
                            iconflow::Style::Regular,
                            iconflow::Size::Regular,
                        ) {
                            let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
                            egui::RichText::new(glyph.to_string())
                                .size(16.0)
                                .color(accent)
                        } else {
                            egui::RichText::new("").size(16.0)
                        };
                        ui.label(icon_text);
                        ui.label(
                            egui::RichText::new(label)
                                .strong()
                                .size(12.0)
                                .color(egui::Color32::from_gray(180)),
                        );
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if let Some(u) = usage {
                                    ui.label(
                                        egui::RichText::new(u)
                                            .size(12.0)
                                            .strong()
                                            .color(accent),
                                    );
                                }
                            },
                        );
                    });
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(value)
                            .size(13.0)
                            .color(egui::Color32::from_rgb(232, 232, 232)),
                    );
                    if let Some(sub) = subtitle {
                        ui.label(
                            egui::RichText::new(sub)
                                .size(10.0)
                                .color(egui::Color32::from_gray(140)),
                        );
                    }
                    if let Some(ex) = extra {
                        ui.label(
                            egui::RichText::new(ex)
                                .size(10.0)
                                .color(egui::Color32::from_gray(120)),
                        );
                    }
                    if progress > 0.001 {
                        ui.add_space(8.0);
                        let bar_color = if progress > 0.9 {
                            egui::Color32::from_rgb(226, 80, 80)
                        } else if progress > 0.75 {
                            egui::Color32::from_rgb(226, 196, 84)
                        } else {
                            accent
                        };
                        ui.add(
                            egui::ProgressBar::new(progress)
                                .desired_height(6.0)
                                .corner_radius(3)
                                .fill(bar_color)
                                .text(""),
                        );
                    }
                });
            });
    }

    fn system_card(
        &self,
        ui: &mut egui::Ui,
        icon_name: &str,
        label: &str,
        value: &str,
        subtitle: Option<&str>,
        extra: Option<&str>,
        accent: egui::Color32,
    ) {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(38, 38, 40))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 48, 52)))
            .corner_radius(10.0)
            .inner_margin(egui::Margin::symmetric(14, 12))
            .show(ui, |ui| {
                ui.set_min_height(60.0);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        let icon_text = if let Ok(icon) = iconflow::try_icon(
                            Pack::Lucide,
                            icon_name,
                            iconflow::Style::Regular,
                            iconflow::Size::Regular,
                        ) {
                            let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
                            egui::RichText::new(glyph.to_string())
                                .size(16.0)
                                .color(accent)
                        } else {
                            egui::RichText::new("").size(16.0)
                        };
                        ui.label(icon_text);
                        ui.label(
                            egui::RichText::new(label)
                                .strong()
                                .size(12.0)
                                .color(egui::Color32::from_gray(180)),
                        );
                    });
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(value)
                            .size(13.0)
                            .color(egui::Color32::from_rgb(232, 232, 232)),
                    );
                    if let Some(sub) = subtitle {
                        ui.label(
                            egui::RichText::new(sub)
                                .size(10.0)
                                .color(egui::Color32::from_gray(140)),
                        );
                    }
                    if let Some(ex) = extra {
                        ui.label(
                            egui::RichText::new(ex)
                                .size(10.0)
                                .color(egui::Color32::from_gray(120)),
                        );
                    }
                });
            });
    }
}
