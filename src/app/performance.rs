use super::WinchiselApp;
use crate::{GamingTweakRow, i18n, performance};
use eframe::egui;
use iconflow::{Pack, Size, Style, try_icon};
use std::sync::mpsc::Receiver;

#[derive(Clone)]
pub(crate) struct PerformanceState {
    pub(crate) performance_query: String,
    pub(crate) performance_loaded: bool,
    pub(crate) performance_groups: [Vec<GamingTweakRow>; 10],
    pub(crate) performance_quick_action_index: usize,
}

pub(crate) struct PerformanceLoadResult {
    pub(crate) groups: [Vec<GamingTweakRow>; 10],
}

pub(crate) struct PerformanceLoadWorker {
    pub(crate) rx: Receiver<PerformanceLoadResult>,
}

impl WinchiselApp {
    pub(crate) fn performance_tick(app: &mut WinchiselApp, ui: &mut egui::Ui) {
        if app.state.active_tab == super::Tab::Performance
            && !app.state.performance.performance_loaded
            && app.performance_load_worker.is_none()
        {
            app.start_performance_load();
        }
        if app.performance_load_worker.is_some() {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(50));
        }
    }

    pub(crate) fn performance_sidebar_loading(app: &WinchiselApp) -> bool {
        app.performance_load_worker.is_some() || !app.state.performance.performance_loaded
    }

    pub(crate) fn spawn_performance_worker(
        query: &str,
        lang: crate::Language,
    ) -> (Option<PerformanceLoadWorker>, bool) {
        let query = query.trim().to_string();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let groups = performance::reload_gaming_tweaks_filtered(&query, lang);
            let _ = tx.send(PerformanceLoadResult { groups });
        });
        (Some(PerformanceLoadWorker { rx }), true)
    }

    pub(crate) fn start_performance_load(&mut self) {
        if self.performance_load_worker.is_some() {
            return;
        }
        self.state.performance.performance_groups = performance::preview_gaming_tweaks(
            &self.state.performance.performance_query,
            self.state.settings.language,
        );
        self.state.performance.performance_loaded = true;
        let (worker, loading) = Self::spawn_performance_worker(
            &self.state.performance.performance_query,
            self.state.settings.language,
        );
        self.performance_load_worker = worker;
        self.state.performance.performance_loaded = loading;
    }

    pub(crate) fn poll_performance_load(&mut self) {
        let Some(worker) = self.performance_load_worker.as_ref() else {
            return;
        };
        match worker.rx.try_recv() {
            Ok(result) => {
                self.state.performance.performance_groups = result.groups;
                self.state.performance.performance_loaded = true;
                self.performance_load_worker = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.state.performance.performance_loaded = false;
                self.performance_load_worker = None;
            }
        }
    }

    pub(crate) fn refresh_performance_groups(&mut self) {
        let query = self.state.performance.performance_query.trim();
        self.state.performance.performance_groups =
            performance::reload_gaming_tweaks_filtered(query, self.state.settings.language);
        self.state.performance.performance_loaded = true;
        self.state.performance.performance_quick_action_index = 0;
    }

    pub(crate) fn request_performance_action(&mut self, action_index: usize) {
        match action_index {
            1 => performance::apply_gaming_recommended_state(),
            2 => performance::apply_gaming_defaults_state(),
            _ => return,
        }
        self.refresh_performance_groups();
    }

    pub(crate) fn performance_group_label(
        &self,
        group_idx: usize,
        lang: crate::Language,
    ) -> &'static str {
        match group_idx {
            0 => i18n::t(lang, "performance_group_0"),
            1 => i18n::t(lang, "performance_group_1"),
            2 => i18n::t(lang, "performance_group_2"),
            3 => i18n::t(lang, "performance_group_3"),
            4 => i18n::t(lang, "performance_group_4"),
            5 => i18n::t(lang, "performance_group_5"),
            6 => i18n::t(lang, "performance_group_6"),
            7 => i18n::t(lang, "performance_group_7"),
            8 => i18n::t(lang, "performance_group_8"),
            9 => i18n::t(lang, "performance_group_9"),
            _ => self.tr("performance_title"),
        }
    }

    pub(crate) fn render_performance_row(&mut self, ui: &mut egui::Ui, row: &GamingTweakRow) {
        let editable = row.is_editable;
        let row_color = if editable {
            egui::Color32::from_rgb(23, 23, 25)
        } else {
            egui::Color32::from_rgb(31, 31, 31)
        };
        let text_color = if editable {
            egui::Color32::from_rgb(232, 232, 232)
        } else {
            egui::Color32::from_rgb(154, 154, 154)
        };
        let dim_color = if editable {
            egui::Color32::from_rgb(190, 190, 190)
        } else {
            egui::Color32::from_rgb(115, 115, 115)
        };

        egui::Frame::new()
            .fill(row_color)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(36, 36, 40)))
            .corner_radius(10.0)
            .inner_margin(egui::Margin::symmetric(10, 8))
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    let left_pad = if row.is_child { 34.0 } else { 18.0 };
                    let action_width = match (row.input_type, row.is_parent) {
                        (0, true) => 146.0,
                        (0, false) => 110.0,
                        (1, true) => 330.0,
                        (1, false) => 294.0,
                        _ => 110.0,
                    };
                    let text_width =
                        (ui.available_width() - left_pad - action_width - 12.0).max(220.0);

                    ui.add_space(left_pad);
                    ui.allocate_ui_with_layout(
                        egui::vec2(text_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(text_width);
                            ui.label(egui::RichText::new(&row.name).color(text_color).size(13.0));
                            ui.add_space(2.0);
                            ui.label(
                                egui::RichText::new(&row.description)
                                    .color(dim_color)
                                    .size(11.0),
                            );
                        },
                    );

                    ui.allocate_ui_with_layout(
                        egui::vec2(action_width, 32.0),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            if row.is_parent {
                                let (rect, response) = ui.allocate_exact_size(
                                    egui::vec2(28.0, 28.0),
                                    egui::Sense::click(),
                                );
                                let response = if editable {
                                    response
                                } else {
                                    response.on_hover_cursor(egui::CursorIcon::Default)
                                };
                                let visuals = if response.hovered() {
                                    ui.visuals().widgets.hovered
                                } else {
                                    ui.visuals().widgets.inactive
                                };
                                ui.painter().rect(
                                    rect,
                                    6.0,
                                    visuals.bg_fill,
                                    visuals.bg_stroke,
                                    egui::StrokeKind::Inside,
                                );
                                let icon = if row.is_expanded { "v" } else { ">" };
                                ui.painter().text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    icon,
                                    egui::FontId::proportional(12.0),
                                    visuals.text_color(),
                                );
                                if editable && response.clicked() {
                                    performance::toggle_gaming_expand_state(row.tweak_id);
                                    self.refresh_performance_groups();
                                }
                                ui.add_space(6.0);
                            }

                            if row.input_type == 0 {
                                let mut enabled = row.enabled;
                                let resp = ui
                                    .add_enabled_ui(editable, |ui| {
                                        Self::native_toggle_switch(ui, &mut enabled)
                                    })
                                    .inner;
                                if resp.changed() {
                                    performance::toggle_gaming_tweak_state(row.tweak_id, enabled);
                                    self.refresh_performance_groups();
                                }
                            } else {
                                let mut selected_index = row.selected_index.max(0) as usize;
                                egui::ComboBox::from_id_salt(row.tweak_id)
                                    .width(220.0)
                                    .selected_text(
                                        row.options
                                            .get(selected_index)
                                            .cloned()
                                            .unwrap_or_default(),
                                    )
                                    .show_ui(ui, |ui| {
                                        for (opt_idx, opt) in row.options.iter().enumerate() {
                                            ui.selectable_value(&mut selected_index, opt_idx, opt);
                                        }
                                    });
                                if selected_index != row.selected_index.max(0) as usize {
                                    performance::select_gaming_tweak_option_state(
                                        row.tweak_id,
                                        selected_index as i32,
                                    );
                                    self.refresh_performance_groups();
                                }
                            }

                            ui.add_space(6.0);
                            let default_icon = if let Ok(icon) =
                                try_icon(Pack::Lucide, "warehouse", Style::Regular, Size::Regular)
                            {
                                let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
                                glyph.to_string()
                            } else {
                                String::new()
                            };
                            if ui
                                .add_enabled(
                                    editable,
                                    egui::Button::new(default_icon)
                                        .min_size(egui::vec2(29.0, 29.0))
                                        .fill(egui::Color32::from_rgb(56, 56, 58))
                                        .stroke(egui::Stroke::new(
                                            1.0,
                                            egui::Color32::from_rgb(136, 136, 136),
                                        )),
                                )
                                .on_hover_text(format!(
                                    "{}{}",
                                    self.tr("performance_current_default"),
                                    row.default_label
                                ))
                                .clicked()
                            {
                                performance::apply_gaming_tweak_default_state(row.tweak_id);
                                self.refresh_performance_groups();
                            }
                            ui.add_space(6.0);
                            let rec_icon = if let Ok(icon) =
                                try_icon(Pack::Lucide, "star", Style::Regular, Size::Regular)
                            {
                                let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
                                glyph.to_string()
                            } else {
                                String::new()
                            };
                            if ui
                                .add_enabled(
                                    editable,
                                    egui::Button::new(rec_icon)
                                        .min_size(egui::vec2(29.0, 29.0))
                                        .fill(egui::Color32::from_rgb(35, 88, 55))
                                        .stroke(egui::Stroke::new(
                                            1.0,
                                            egui::Color32::from_rgb(72, 145, 92),
                                        )),
                                )
                                .on_hover_text(format!(
                                    "{}{}",
                                    self.tr("performance_current_recommended"),
                                    row.recommended_label
                                ))
                                .clicked()
                            {
                                performance::apply_gaming_tweak_recommended_state(row.tweak_id);
                                self.refresh_performance_groups();
                            }
                        },
                    );
                });
            });
    }

    pub(crate) fn render_performance_tab(&mut self, ui: &mut egui::Ui) {
        let perf_title = self.tr("performance_title").to_string();
        let perf_subtitle = self.tr("performance_subtitle").to_string();
        let perf_search = self.tr("performance_search").to_string();
        let perf_quick = self.tr("performance_quick").to_string();
        let perf_apply = self.tr("performance_apply_recommended").to_string();
        let perf_reset = self.tr("performance_reset_defaults").to_string();
        let perf_loading = self.tr("performance_loading").to_string();
        let perf_empty = self.tr("performance_empty").to_string();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading(perf_title);
                    ui.label(perf_subtitle);
                });
                ui.add_space(12.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let response = ui.add_sized(
                        [280.0, 30.0],
                        egui::TextEdit::singleline(&mut self.state.performance.performance_query)
                            .hint_text(perf_search),
                    );
                    if response.changed() {
                        self.refresh_performance_groups();
                    }
                    ui.add_space(10.0);
                    let quick_index = self.state.performance.performance_quick_action_index;
                    let selected_text = match quick_index {
                        1 => perf_apply.as_str(),
                        2 => perf_reset.as_str(),
                        _ => perf_quick.as_str(),
                    };
                    let mut chosen_index = quick_index;
                    ui.allocate_ui_with_layout(
                        egui::vec2(220.0, 30.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.scope(|ui| {
                                ui.style_mut().spacing.interact_size.y = 30.0;
                                egui::ComboBox::from_id_salt("performance_quick_actions")
                                    .width(220.0)
                                    .selected_text(selected_text)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut chosen_index,
                                            0,
                                            perf_quick.as_str(),
                                        );
                                        ui.selectable_value(
                                            &mut chosen_index,
                                            1,
                                            perf_apply.as_str(),
                                        );
                                        ui.selectable_value(
                                            &mut chosen_index,
                                            2,
                                            perf_reset.as_str(),
                                        );
                                    });
                            });
                        },
                    );
                    if chosen_index != quick_index {
                        self.state.performance.performance_quick_action_index = chosen_index;
                        if chosen_index > 0 {
                            self.request_performance_action(chosen_index);
                            self.state.performance.performance_quick_action_index = 0;
                        }
                    }
                });
            });

            if !self.state.performance.performance_loaded {
                ui.add_space(40.0);
                ui.vertical_centered(|ui| {
                    ui.add(egui::Spinner::new().size(28.0));
                    ui.add_space(10.0);
                    ui.strong(perf_loading);
                });
                ui.add_space(40.0);
            } else {
                ui.separator();
                ui.add_space(10.0);

                let groups = self.state.performance.performance_groups.clone();
                if groups.iter().all(|g| g.is_empty()) {
                    ui.label(perf_empty);
                } else {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (group_idx, rules) in groups.iter().enumerate() {
                                if rules.is_empty() {
                                    continue;
                                }
                                let header = egui::CollapsingHeader::new(
                                    egui::RichText::new(self.performance_group_label(
                                        group_idx,
                                        self.state.settings.language,
                                    ))
                                    .strong()
                                    .size(15.0)
                                    .color(egui::Color32::from_rgb(149, 194, 255)),
                                )
                                .id_salt(("performance_group", group_idx))
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.add_space(8.0);
                                    for row in rules.iter() {
                                        self.render_performance_row(ui, row);
                                        if group_idx == 6 && !row.warning_text.is_empty() {
                                            ui.add_space(6.0);
                                            Self::card_frame()
                                                .fill(egui::Color32::from_rgb(64, 54, 20))
                                                .stroke(egui::Stroke::new(
                                                    1.0,
                                                    egui::Color32::from_rgb(181, 149, 38),
                                                ))
                                                .corner_radius(6.0)
                                                .show(ui, |ui| {
                                                    ui.horizontal_wrapped(|ui| {
                                                        ui.colored_label(
                                                            egui::Color32::from_rgb(255, 216, 102),
                                                            "!",
                                                        );
                                                        ui.colored_label(
                                                            egui::Color32::from_rgb(255, 236, 179),
                                                            &row.warning_text,
                                                        );
                                                    });
                                                });
                                        }
                                        ui.add_space(8.0);
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
            }
        });
    }
}
