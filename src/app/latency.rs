use super::WinchiselApp;
use crate::latency as latency_core;
use eframe::egui;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::time::Instant;

#[derive(Clone)]
pub(crate) struct LatencyState {
    pub(crate) latency_loading: bool,
    pub(crate) latency_completion_pending: bool,
    pub(crate) latency_progress: i32,
    pub(crate) latency_progress_display: f32,
    pub(crate) latency_status: String,
    pub(crate) latency_lines: Vec<latency_core::StyledLine>,
    pub(crate) latency_pending_lines: Vec<latency_core::StyledLine>,
    pub(crate) latency_tick_next_at: Option<Instant>,
    pub(crate) latency_completion_ready_at: Option<Instant>,
}

pub(crate) struct LatencyLoadWorker {
    pub(crate) rx: Receiver<LatencyWorkerMessage>,
}

pub(crate) enum LatencyWorkerMessage {
    Progress(i32, String),
    Done(Vec<latency_core::StyledLine>),
}

impl WinchiselApp {
    pub(crate) fn render_latency_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Latency");
                ui.label("Analyze USB latency and device topology.");
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let button_text = if self.state.latency.latency_loading {
                    "Analyzing..."
                } else {
                    "Analyze USB Latency"
                };
                let button = egui::Button::new(button_text)
                    .fill(egui::Color32::from_rgb(35, 88, 55))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(72, 145, 92)));
                if ui
                    .add_enabled_ui(self.latency_load_worker.is_none(), |ui| {
                        ui.add_sized([180.0, 34.0], button)
                    })
                    .inner
                    .clicked()
                {
                    self.state.latency.latency_loading = true;
                    self.state.latency.latency_completion_pending = false;
                    self.state.latency.latency_progress = 0;
                    self.state.latency.latency_progress_display = 0.0;
                    self.state.latency.latency_status = "Starting analysis...".to_string();
                    self.state.latency.latency_lines = vec![
                        latency_core::StyledLine {
                            text: String::new(),
                            color: latency_core::COL_NORMAL,
                            bold: false,
                        },
                        latency_core::StyledLine {
                            text: "  Analyzing USB topology...".to_string(),
                            color: latency_core::COL_SKY,
                            bold: true,
                        },
                        latency_core::StyledLine {
                            text:
                                "  Scanning PnP devices, controller chain, MSI and power settings."
                                    .to_string(),
                            color: latency_core::COL_DIM,
                            bold: false,
                        },
                    ];
                    self.sync_latency_loading_line();
                    self.state.latency.latency_pending_lines.clear();
                    self.state.latency.latency_tick_next_at =
                        Some(std::time::Instant::now() + Duration::from_millis(250));
                    self.state.latency.latency_completion_ready_at = None;
                    self.latency_load_worker = Some(Self::spawn_latency_worker());
                }
            });
        });

        ui.add_space(14.0);

        if self.state.latency.latency_loading || self.latency_load_worker.is_some() {
            ui.ctx().request_repaint_after(Duration::from_millis(16));
        }
        if (self.state.latency.latency_loading || self.state.latency.latency_completion_pending)
            && self.update_latency_progress_display()
        {
            ui.ctx().request_repaint_after(Duration::from_millis(16));
        }

        if self.state.latency.latency_lines.is_empty() && !self.state.latency.latency_loading {
            Self::card_frame().show(ui, |ui| {
                ui.label("Click 'Analyze USB Latency' to begin analysis.");
            });
        } else {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    Self::card_frame().show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.spacing_mut().item_spacing.y = 2.0;
                        if self.state.latency.latency_loading {
                            ui.add(
                                egui::ProgressBar::new(self.state.latency.latency_progress_display)
                                    .show_percentage()
                                    .animate(true),
                            );
                            ui.add_space(10.0);
                        }
                        for line in &self.state.latency.latency_lines {
                            let mut text = egui::RichText::new(&line.text)
                                .monospace()
                                .color(Self::latency_line_color(line.color));
                            if line.bold {
                                text = text.strong();
                            }
                            ui.label(text);
                        }
                    });
                });
        }
    }

    pub(crate) fn spawn_latency_worker() -> LatencyLoadWorker {
        let (tx, rx) = std::sync::mpsc::channel::<LatencyWorkerMessage>();
        std::thread::spawn(move || {
            let result = latency_core::analyze_usb_latency_with_progress(|pct, msg| {
                let _ = tx.send(LatencyWorkerMessage::Progress(pct, msg.to_string()));
            });
            let lines = match result {
                Ok(analysis) => analysis.styled_lines,
                Err(err) => vec![
                    latency_core::StyledLine::empty(),
                    latency_core::StyledLine::new(
                        "  ERROR - USB LATENCY ANALYSIS FAILED",
                        latency_core::COL_CORAL,
                        true,
                    ),
                    latency_core::StyledLine::border(
                        "  ---------------------------------------------------------------------",
                    ),
                    latency_core::StyledLine::empty(),
                    latency_core::StyledLine::new(
                        format!("  {}", err),
                        latency_core::COL_NORMAL,
                        false,
                    ),
                    latency_core::StyledLine::empty(),
                    latency_core::StyledLine::dim("  Make sure you are running as Administrator."),
                ],
            };
            let _ = tx.send(LatencyWorkerMessage::Done(lines));
        });

        LatencyLoadWorker { rx }
    }

    pub(crate) fn poll_latency_worker(&mut self) {
        let Some(worker) = self.latency_load_worker.as_mut() else {
            return;
        };
        let mut clear_worker = false;
        let mut needs_latency_line_sync = false;
        while let Ok(message) = worker.rx.try_recv() {
            match message {
                LatencyWorkerMessage::Progress(pct, msg) => {
                    let pct = pct.clamp(0, 100);
                    self.state.latency.latency_loading = true;
                    if pct > self.state.latency.latency_progress {
                        self.state.latency.latency_progress = pct;
                    }
                    self.state.latency.latency_status = msg;
                    self.state.latency.latency_tick_next_at =
                        Some(Instant::now() + Duration::from_millis(250));
                    needs_latency_line_sync = true;
                }
                LatencyWorkerMessage::Done(lines) => {
                    self.state.latency.latency_completion_pending = true;
                    self.state.latency.latency_completion_ready_at =
                        Some(Instant::now() + Duration::from_millis(900));
                    self.state.latency.latency_pending_lines = lines;
                    needs_latency_line_sync = true;
                    clear_worker = true;
                }
            }
        }
        if needs_latency_line_sync {
            self.sync_latency_loading_line();
        }
        if clear_worker {
            self.latency_load_worker = None;
        }
    }

    fn latency_line_color(color: i32) -> egui::Color32 {
        match color {
            latency_core::COL_MINT => egui::Color32::from_rgb(0, 255, 135),
            latency_core::COL_ORANGE => egui::Color32::from_rgb(255, 179, 71),
            latency_core::COL_CORAL => egui::Color32::from_rgb(255, 107, 107),
            latency_core::COL_SKY => egui::Color32::from_rgb(135, 206, 235),
            latency_core::COL_DIM => egui::Color32::from_rgb(108, 108, 108),
            latency_core::COL_BORDER => egui::Color32::from_rgb(74, 74, 74),
            _ => egui::Color32::from_rgb(226, 226, 226),
        }
    }

    fn latency_status_for_progress(progress: i32) -> &'static str {
        if progress < 25 {
            "Checking power settings..."
        } else if progress < 45 {
            "Scanning USB controllers..."
        } else if progress < 65 {
            "Finding input devices..."
        } else if progress < 82 {
            "Tracing devices to root hubs..."
        } else if progress < 95 {
            "Verifying topology and power hints..."
        } else {
            "Building report..."
        }
    }

    pub(crate) fn sync_latency_loading_line(&mut self) {
        if self.state.latency.latency_loading && self.state.latency.latency_lines.len() > 1 {
            self.state.latency.latency_lines[1].text =
                format!("  {}", self.state.latency.latency_status);
        }
    }

    pub(crate) fn update_latency_progress_display(&mut self) -> bool {
        let now = Instant::now();
        let mut changed = false;

        if (self.state.latency.latency_loading || self.state.latency.latency_completion_pending)
            && self.state.latency.latency_progress < 98
        {
            let should_tick = self
                .state
                .latency
                .latency_tick_next_at
                .is_none_or(|next_at| now >= next_at);

            if should_tick {
                let current = self.state.latency.latency_progress;
                let next = if current < 35 {
                    current + 3
                } else if current < 75 {
                    current + 2
                } else {
                    current + 1
                }
                .min(98);
                self.state.latency.latency_progress = next;
                self.state.latency.latency_status =
                    Self::latency_status_for_progress(next).to_string();
                self.sync_latency_loading_line();
                self.state.latency.latency_tick_next_at = Some(now + Duration::from_millis(250));
                changed = true;
            }
        }

        let display = self.state.latency.latency_progress_display;
        let target = (self.state.latency.latency_progress as f32 / 100.0).clamp(0.0, 1.0);

        if (display - target).abs() > 0.001 {
            let step = 0.03;
            self.state.latency.latency_progress_display = if display < target {
                (display + step).min(target)
            } else {
                (display - step).max(target)
            };
            changed = true;
        } else {
            self.state.latency.latency_progress_display = target;
        }

        if self.state.latency.latency_completion_pending {
            let completion_ready = self
                .state
                .latency
                .latency_completion_ready_at
                .is_some_and(|ready_at| now >= ready_at);

            if completion_ready && (self.state.latency.latency_progress_display - 1.0).abs() < 0.001
            {
                self.state.latency.latency_loading = false;
                self.state.latency.latency_completion_pending = false;
                self.state.latency.latency_completion_ready_at = None;
                self.state.latency.latency_tick_next_at = None;
                self.state.latency.latency_status = "Ready".to_string();
                self.state.latency.latency_lines =
                    std::mem::take(&mut self.state.latency.latency_pending_lines);
                changed = true;
            }
        }

        changed
    }
}
