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
                        let previous_language = self.state.settings.language;
                        egui::ComboBox::from_id_salt("language_combo")
                            .selected_text(match self.state.settings.language {
                                crate::Language::German => "Deutsch",
                                crate::Language::English => "English",
                                crate::Language::French => "Français",
                                crate::Language::Spanish => "Español",
                                crate::Language::Turkish => "Türkçe",
                                crate::Language::Greek => "Ελληνικά",
                                crate::Language::Dutch => "Nederlands",
                                crate::Language::Portuguese => "Português",
                                crate::Language::Italian => "Italiano",
                                crate::Language::Polish => "Polski",
                                crate::Language::Russian => "Русский",
                                crate::Language::Japanese => "日本語",
                                crate::Language::ChineseSimplified => "简体中文",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::German,
                                    "Deutsch",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::English,
                                    "English",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::French,
                                    "Français",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::Spanish,
                                    "Español",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::Turkish,
                                    "Türkçe",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::Greek,
                                    "Ελληνικά",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::Dutch,
                                    "Nederlands",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::Portuguese,
                                    "Português",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::Italian,
                                    "Italiano",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::Polish,
                                    "Polski",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::Russian,
                                    "Русский",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::Japanese,
                                    "日本語",
                                );
                                ui.selectable_value(
                                    &mut self.state.settings.language,
                                    crate::Language::ChineseSimplified,
                                    "简体中文",
                                );
                            });
                        if self.state.settings.language != previous_language {
                            self.settings_save_snapshot = self.state.settings.clone();
                            self.settings_save_due_at = None;
                            crate::save_app_settings(&self.state.settings);
                            self.apply_language_change();
                            self.state.update_status = self.tr("settings_saved").to_string();
                        }
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
