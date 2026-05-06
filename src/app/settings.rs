use super::WinchiselApp;
use eframe::egui;
use iconflow::Pack;

impl WinchiselApp {
    pub(crate) fn render_settings_tab(&mut self, ui: &mut egui::Ui) {
        Self::page_shell(
                ui,
                "Settings",
                "Keep the app behavior aligned with your workflow.",
                |ui| {
                    Self::card_frame().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "settings-2",
                            16.0,
                            egui::Color32::from_rgb(160, 124, 220),
                        ));
                        ui.colored_label(egui::Color32::from_rgb(160, 124, 220), "Application");
                    });
                    ui.label("These settings are saved automatically.");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "refresh-cw",
                            14.0,
                            egui::Color32::from_rgb(226, 226, 226),
                        ));
                        ui.checkbox(
                            &mut self.state.settings.check_updates_on_startup,
                            "Check updates on startup",
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label(Self::icon_text(
                            Pack::Lucide,
                            "monitor",
                            14.0,
                            egui::Color32::from_rgb(226, 226, 226),
                        ));
                        ui.checkbox(&mut self.state.settings.show_console, "Show console");
                    });
                    ui.add_space(8.0);
                    if ui.button("Open Logs").clicked() {
                        self.show_log_window = true;
                    }
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
                        ui.colored_label(egui::Color32::from_rgb(160, 124, 220), "System Protection");
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
                                ui.label("System Restore Point");
                            });
                            ui.label("Create a rollback point before major system changes");
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Create Restore Point").clicked() {
                                self.start_restore_point();
                            }
                        });
                    });
                });
            },
        );
    }
}
