use super::WinchiselApp;
use eframe::egui;
use iconflow::Pack;

impl WinchiselApp {
    pub(crate) fn render_settings_tab(&mut self, ui: &mut egui::Ui) {
        Self::page_shell(
            ui,
            self.tr("settings_title"),
            self.tr("settings_subtitle"),
            |ui| {
                let check_updates_startup = self.tr("check_updates_startup").to_string();
                let show_console = self.tr("show_console").to_string();
                Self::card_frame().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(Self::icon_text(
                                    Pack::Lucide,
                                    "settings-2",
                                    16.0,
                                    egui::Color32::from_rgb(160, 124, 220),
                                ));
                                ui.colored_label(
                                    egui::Color32::from_rgb(160, 124, 220),
                                    self.tr("settings_application"),
                                );
                            });
                            ui.label(self.tr("settings_saved_auto"));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui
                                .add_sized([116.0, 34.0], egui::Button::new(self.tr("open_logs")))
                                .clicked()
                            {
                                self.show_log_window = true;
                            }
                        });
                    });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "languages",
                            14.0,
                            egui::Color32::from_rgb(226, 226, 226),
                        ));
                        ui.label(self.tr("language"));
                        let current_language = self.state.settings.language;
                        egui::ComboBox::from_id_salt("language_combo")
                            .selected_text(match current_language {
                                crate::Language::English => "English",
                                crate::Language::German => "Deutsch",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::English,
                                    "English",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::German,
                                    "Deutsch",
                                );
                            });
                    });
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "refresh-cw",
                            14.0,
                            egui::Color32::from_rgb(226, 226, 226),
                        ));
                        ui.label(check_updates_startup);
                        Self::native_toggle_switch(
                            ui,
                            &mut self.state.settings.check_updates_on_startup,
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "monitor",
                            14.0,
                            egui::Color32::from_rgb(226, 226, 226),
                        ));
                        ui.label(show_console);
                        Self::native_toggle_switch(ui, &mut self.state.settings.show_console);
                    });
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "shield-check",
                            16.0,
                            egui::Color32::from_rgb(160, 124, 220),
                        ));
                        ui.colored_label(
                            egui::Color32::from_rgb(160, 124, 220),
                            self.tr("system_protection"),
                        );
                    });
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(Self::icon_text(
                                    Pack::Lucide,
                                    "archive-restore",
                                    14.0,
                                    egui::Color32::from_rgb(226, 226, 226),
                                ));
                                ui.label(self.tr("restore_point"));
                            });
                            ui.label(self.tr("restore_point_desc"));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let restore_button = egui::Button::new(self.tr("create_restore_point"))
                                .fill(egui::Color32::from_rgb(35, 54, 80))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    egui::Color32::from_rgb(10, 210, 254),
                                ));
                            if ui.add_sized([168.0, 34.0], restore_button).clicked() {
                                self.start_restore_point();
                            }
                        });
                    });
                });
            },
        );
    }
}
