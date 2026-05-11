use super::WinchiselApp;
use eframe::egui;
use eframe::egui::{FontData, FontDefinitions, FontFamily, FontId, RichText};
use iconflow::{Pack, Size, Style, fonts, try_icon};
use std::sync::Arc;

impl WinchiselApp {
    pub(crate) fn tree_header_hover(ui: &mut egui::Ui, response: &egui::Response) {
        let response = response.clone();
        let hovered = response.hovered();
        let rect = response.rect;
        let _ = response
            .clone()
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        if hovered {
            let rect = rect.expand(2.0);
            let fill = egui::Color32::from_rgba_unmultiplied(20, 54, 66, 96);
            let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(10, 210, 254));
            ui.painter()
                .rect(rect, 6.0, fill, stroke, egui::StrokeKind::Inside);
        }
    }

    pub(crate) fn native_toggle_switch(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }
        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
        });
        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter().rect(
                rect,
                radius,
                visuals.bg_fill,
                visuals.bg_stroke,
                egui::StrokeKind::Inside,
            );
            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }
        response
    }

    pub(crate) fn tab_button(
        ui: &mut egui::Ui,
        current: &mut super::Tab,
        tab: super::Tab,
        label: &str,
        loading: bool,
    ) {
        let selected = *current == tab;
        let desired_size = egui::vec2(ui.available_width(), 42.0);
        let (rect, _) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
        let response = ui.interact(
            rect,
            ui.id().with(("sidebar_tab_button", tab as i32)),
            egui::Sense::click(),
        );
        let hovered = response.hovered();
        let fill = if selected {
            egui::Color32::from_rgb(10, 210, 254)
        } else if hovered {
            egui::Color32::from_rgb(20, 54, 66)
        } else {
            egui::Color32::from_rgb(31, 31, 36)
        };
        let stroke = if selected || hovered {
            egui::Stroke::new(1.0, egui::Color32::from_rgb(10, 210, 254))
        } else {
            egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 48, 54))
        };
        ui.painter()
            .rect(rect, 9.0, fill, stroke, egui::StrokeKind::Inside);
        if loading {
            let spinner_pos = egui::pos2(rect.right() - 18.0, rect.center().y);
            let time = ui.input(|i| i.time) as f32;
            let points = 24;
            let radius = 6.0;
            let start = time * 4.5;
            let sweep = std::f32::consts::TAU * 0.75;
            let mut arc_points = Vec::with_capacity(points + 1);
            for i in 0..=points {
                let t = i as f32 / points as f32;
                let angle = start + sweep * t;
                arc_points.push(egui::pos2(
                    spinner_pos.x + angle.cos() * radius,
                    spinner_pos.y + angle.sin() * radius,
                ));
            }
            ui.painter().add(egui::Shape::line(
                arc_points,
                egui::Stroke::new(1.8, egui::Color32::from_rgb(226, 226, 226)),
            ));
        }
        let label_x = 14.0;
        let text_pos = egui::pos2(rect.left() + label_x, rect.center().y);
        let text_color = if selected {
            egui::Color32::BLACK
        } else {
            ui.visuals().widgets.active.text_color()
        };
        ui.painter().text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            label,
            egui::TextStyle::Button.resolve(ui.style()),
            text_color,
        );
        if response.clicked() {
            *current = tab;
        }
    }

    pub(crate) fn card_frame() -> egui::Frame {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(23, 23, 25))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(36, 36, 40)))
            .corner_radius(14.0)
            .inner_margin(egui::Margin::same(16))
    }

    pub(crate) fn page_shell(
        ui: &mut egui::Ui,
        title: &str,
        subtitle: &str,
        body: impl FnOnce(&mut egui::Ui),
    ) {
        ui.vertical(|ui| {
            ui.heading(title);
            ui.label(subtitle);
            ui.add_space(14.0);
            body(ui);
        });
    }

    pub(crate) fn show_toast_layer(&mut self, ui: &mut egui::Ui) {
        self.toasts.show(ui.ctx());
    }

    pub(crate) fn show_log_window(&mut self, ctx: &egui::Context) {
        if !self.show_log_window {
            return;
        }
        egui::Window::new(self.tr("open_logs"))
            .open(&mut self.show_log_window)
            .default_size([860.0, 520.0])
            .show(ctx, |ui| {
                egui_logger::logger_ui().show(ui);
            });
    }

    pub(crate) fn install_icon_fonts(ctx: &egui::Context) {
        let mut definitions = FontDefinitions::default();
        let fallback_fonts: Vec<String> = definitions.font_data.keys().cloned().collect();
        for font in fonts() {
            definitions.font_data.insert(
                font.family.to_string(),
                Arc::new(FontData::from_static(font.bytes)),
            );
            let family = definitions
                .families
                .entry(FontFamily::Proportional)
                .or_default();
            family.insert(0, font.family.to_string());
            for fallback in &fallback_fonts {
                if fallback != font.family {
                    family.push(fallback.clone());
                }
            }
        }
        ctx.set_fonts(definitions);
    }

    pub(crate) fn icon_text(pack: Pack, name: &str, size: f32, color: egui::Color32) -> RichText {
        if let Ok(icon) = try_icon(pack, name, Style::Regular, Size::Regular) {
            let glyph = char::from_u32(icon.codepoint).unwrap_or('?');
            let font_id = FontId::new(size, FontFamily::Proportional);
            RichText::new(glyph.to_string()).font(font_id).color(color)
        } else {
            RichText::new("?").size(size).color(color)
        }
    }
}
