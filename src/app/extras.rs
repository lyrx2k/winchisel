use super::WinchiselApp;
use eframe::egui;
use std::sync::mpsc;

#[derive(Clone, Copy)]
enum ExtrasKind {
    Brave,
    Edge,
    Widgets,
    TimerResolution,
    Ipv6,
    Teredo,
    Ps7,
    Hpet,
    ModernStandby,
}

impl WinchiselApp {
    pub(crate) fn render_extras_tab(&mut self, ui: &mut egui::Ui) {
        Self::page_shell(
            ui,
            self.tr("extras_title"),
            self.tr("extras_subtitle"),
            |ui| {
                if let Some(worker) = self.extras_teredo_worker.as_ref() {
                    match worker.rx.try_recv() {
                        Ok(result) => {
                            self.extras_teredo_worker = None;
                            if let Err(err) = result {
                                self.toasts
                                    .error(err)
                                    .duration(std::time::Duration::from_secs_f64(3.5));
                            } else {
                                self.state.extras.teredo_disabled_enabled =
                                    Self::teredo_disabled_enabled();
                                self.state.extras.teredo_disabled_loaded = true;
                            }
                        }
                        Err(mpsc::TryRecvError::Empty) => {}
                        Err(mpsc::TryRecvError::Disconnected) => {
                            self.extras_teredo_worker = None;
                            self.state.extras.teredo_disabled_loaded = true;
                        }
                    }
                }
                if let Some(worker) = self.extras_hpet_worker.as_ref() {
                    match worker.rx.try_recv() {
                        Ok(result) => {
                            self.extras_hpet_worker = None;
                            if let Err(err) = result {
                                self.toasts
                                    .error(err)
                                    .duration(std::time::Duration::from_secs_f64(3.5));
                            } else {
                                self.state.extras.hpet_preferred_enabled =
                                    Self::hpet_preferred_enabled();
                                self.state.extras.hpet_preferred_loaded = true;
                            }
                        }
                        Err(mpsc::TryRecvError::Empty) => {}
                        Err(mpsc::TryRecvError::Disconnected) => {
                            self.extras_hpet_worker = None;
                            self.state.extras.hpet_preferred_loaded = true;
                        }
                    }
                }
                if let Some(worker) = self.extras_power_plan_worker.as_ref() {
                    match worker.rx.try_recv() {
                        Ok(result) => {
                            self.extras_power_plan_worker = None;
                            if let Err(err) = result {
                                self.toasts
                                    .error(err)
                                    .duration(std::time::Duration::from_secs_f64(3.5));
                            } else {
                                self.toasts
                                    .success(self.tr("extras_power_plan_applied"))
                                    .duration(std::time::Duration::from_secs_f64(3.5));
                                self.state.extras.winchisel_power_plan_enabled =
                                    Self::winchisel_power_plan_active();
                                self.state.extras.winchisel_power_plan_loaded = true;
                            }
                        }
                        Err(mpsc::TryRecvError::Empty) => {}
                        Err(mpsc::TryRecvError::Disconnected) => {
                            self.extras_power_plan_worker = None;
                            self.state.extras.winchisel_power_plan_loaded = true;
                        }
                    }
                }
                if !self.state.extras.brave_debloat_loaded
                    || !self.state.extras.edge_debloat_loaded
                    || !self.state.extras.widgets_removed_loaded
                    || !self.state.extras.ctfmon_blocked_loaded
                    || !self.state.extras.timer_resolution_loaded
                    || !self.state.extras.ipv6_preferred_loaded
                    || !self.state.extras.teredo_disabled_loaded
                    || !self.state.extras.powershell7_telemetry_loaded
                    || !self.state.extras.hpet_preferred_loaded
                    || !self.state.extras.modern_standby_disabled_loaded
                    || !self.state.extras.winchisel_power_plan_loaded
                {
                    ui.add_space(40.0);
                    ui.vertical_centered(|ui| {
                        ui.add(egui::Spinner::new().size(28.0));
                        ui.add_space(10.0);
                        ui.label(self.tr("extras_loading"));
                    });
                    return;
                }

                self.extras_power_plan_section(ui);
                ui.add_space(8.0);
                self.extras_policy_row(ui, ExtrasKind::ModernStandby);
                ui.add_space(8.0);
                self.extras_policy_row(ui, ExtrasKind::Brave);
                ui.add_space(8.0);
                self.extras_policy_row(ui, ExtrasKind::Edge);
                ui.add_space(8.0);
                self.extras_policy_row(ui, ExtrasKind::Widgets);
                ui.add_space(8.0);
                self.extras_ctfmon_section(ui);
                ui.add_space(8.0);
                self.extras_policy_row(ui, ExtrasKind::TimerResolution);
                ui.add_space(8.0);
                self.extras_policy_row(ui, ExtrasKind::Ipv6);
                ui.add_space(8.0);
                self.extras_policy_row(ui, ExtrasKind::Teredo);
                ui.add_space(8.0);
                self.extras_policy_row(ui, ExtrasKind::Ps7);
                ui.add_space(8.0);
                self.extras_policy_row(ui, ExtrasKind::Hpet);
            },
        );
    }

    fn extras_policy_row(&mut self, ui: &mut egui::Ui, kind: ExtrasKind) {
        let (label, desc, tooltip, enabled) = match kind {
            ExtrasKind::Brave => (
                self.tr("extras_brave_label"),
                self.tr("extras_brave_desc"),
                self.brave_debloat_tooltip(),
                self.state.extras.brave_debloat_enabled,
            ),
            ExtrasKind::Edge => (
                self.tr("extras_edge_label"),
                self.tr("extras_edge_desc"),
                self.edge_debloat_tooltip(),
                self.state.extras.edge_debloat_enabled,
            ),
            ExtrasKind::Widgets => (
                self.tr("extras_widgets_label"),
                self.tr("extras_widgets_desc"),
                self.widgets_remove_tooltip(),
                self.state.extras.widgets_removed_enabled,
            ),
            ExtrasKind::TimerResolution => (
                self.tr("extras_timer_resolution_label"),
                self.tr("extras_timer_resolution_desc"),
                self.timer_resolution_tooltip(),
                self.state.extras.timer_resolution_enabled,
            ),
            ExtrasKind::Ipv6 => (
                self.tr("extras_ipv6_label"),
                self.tr("extras_ipv6_desc"),
                self.ipv6_preferred_tooltip(),
                self.state.extras.ipv6_preferred_enabled,
            ),
            ExtrasKind::Teredo => (
                self.tr("extras_teredo_label"),
                self.tr("extras_teredo_desc"),
                self.teredo_disabled_tooltip(),
                self.state.extras.teredo_disabled_enabled,
            ),
            ExtrasKind::Ps7 => (
                self.tr("extras_ps7_label"),
                self.tr("extras_ps7_desc"),
                self.powershell7_telemetry_tooltip(),
                self.state.extras.powershell7_telemetry_enabled,
            ),
            ExtrasKind::Hpet => (
                self.tr("extras_hpet_label"),
                self.tr("extras_hpet_desc"),
                self.hpet_tooltip(),
                self.state.extras.hpet_preferred_enabled,
            ),
            ExtrasKind::ModernStandby => (
                self.tr("extras_modern_standby_label"),
                self.tr("extras_modern_standby_desc"),
                self.modern_standby_tooltip(),
                self.state.extras.modern_standby_disabled_enabled,
            ),
        };
        let pending = matches!(kind, ExtrasKind::Teredo) && self.extras_teredo_worker.is_some()
            || matches!(kind, ExtrasKind::Hpet) && self.extras_hpet_worker.is_some();

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(20, 20, 23))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 35, 39)))
            .corner_radius(10.0)
            .inner_margin(egui::Margin::symmetric(10, 8))
            .show(ui, |ui| {
                let available = ui.available_width();
                let switch_slot = 72.0;
                let left_width = (available - switch_slot - 12.0).max(220.0);

                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(left_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(left_width);
                            ui.horizontal(|ui| {
                                ui.label(label);
                                let info = ui.add(
                                    egui::Label::new(
                                        egui::RichText::new("?")
                                            .strong()
                                            .color(egui::Color32::from_rgb(149, 194, 255)),
                                    )
                                    .sense(egui::Sense::hover()),
                                );
                                info.on_hover_text(tooltip.clone());
                            });
                            ui.add_space(2.0);
                            ui.label(desc);
                        },
                    );

                    ui.allocate_ui_with_layout(
                        egui::vec2(switch_slot, 28.0),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.set_width(switch_slot);
                            if pending {
                                ui.add(egui::Spinner::new().size(18.0));
                                return;
                            }
                            let mut local_enabled = enabled;
                            let resp = Self::native_toggle_switch(ui, &mut local_enabled);
                            if resp.changed() {
                                let result = match kind {
                                    ExtrasKind::Brave => Self::apply_brave_debloat(local_enabled),
                                    ExtrasKind::Edge => Self::apply_edge_debloat(local_enabled),
                                    ExtrasKind::Widgets => {
                                        Self::apply_widgets_removed(local_enabled)
                                    }
                                    ExtrasKind::TimerResolution => {
                                        Self::apply_timer_resolution(local_enabled)
                                    }
                                    ExtrasKind::Ipv6 => Self::apply_ipv6_preferred(local_enabled),
                                    ExtrasKind::Teredo => {
                                        self.state.extras.teredo_disabled_loaded = false;
                                        let (tx, rx) = mpsc::channel();
                                        self.extras_teredo_worker =
                                            Some(super::ExtrasBoolWorker { rx });
                                        let desired = local_enabled;
                                        std::thread::spawn(move || {
                                            let _ = tx.send(Self::apply_teredo_disabled(desired));
                                        });
                                        return;
                                    }
                                    ExtrasKind::Ps7 => {
                                        Self::apply_powershell7_telemetry(local_enabled)
                                    }
                                    ExtrasKind::Hpet => {
                                        self.state.extras.hpet_preferred_loaded = false;
                                        let (tx, rx) = mpsc::channel();
                                        self.extras_hpet_worker =
                                            Some(super::ExtrasBoolWorker { rx });
                                        let desired = local_enabled;
                                        std::thread::spawn(move || {
                                            let _ = tx.send(Self::apply_hpet_preferred(desired));
                                        });
                                        return;
                                    }
                                    ExtrasKind::ModernStandby => {
                                        Self::apply_modern_standby_disabled(local_enabled)
                                    }
                                };
                                if let Err(err) = result {
                                    self.toasts
                                        .error(err)
                                        .duration(std::time::Duration::from_secs_f64(3.5));
                                    return;
                                }
                                match kind {
                                    ExtrasKind::Brave => {
                                        self.state.extras.brave_debloat_enabled = local_enabled
                                    }
                                    ExtrasKind::Edge => {
                                        self.state.extras.edge_debloat_enabled = local_enabled
                                    }
                                    ExtrasKind::Widgets => {
                                        self.state.extras.widgets_removed_enabled = local_enabled
                                    }
                                    ExtrasKind::TimerResolution => {
                                        self.state.extras.timer_resolution_enabled = local_enabled
                                    }
                                    ExtrasKind::Ipv6 => {
                                        self.state.extras.ipv6_preferred_enabled = local_enabled
                                    }
                                    ExtrasKind::Teredo => {
                                        self.state.extras.teredo_disabled_enabled = local_enabled
                                    }
                                    ExtrasKind::Ps7 => {
                                        self.state.extras.powershell7_telemetry_enabled =
                                            local_enabled
                                    }
                                    ExtrasKind::Hpet => {
                                        self.state.extras.hpet_preferred_enabled = local_enabled;
                                    }
                                }
                            }
                        },
                    );
                });
            });
    }

    fn extras_ctfmon_section(&mut self, ui: &mut egui::Ui) {
        let enabled = self.state.extras.ctfmon_blocked_enabled;
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(20, 20, 23))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 35, 39)))
            .corner_radius(10.0)
            .inner_margin(egui::Margin::symmetric(10, 8))
            .show(ui, |ui| {
                let available = ui.available_width();
                let action_width = 170.0;
                let text_width = (available - action_width - 12.0).max(220.0);
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(text_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(text_width);
                            ui.horizontal(|ui| {
                                ui.label(self.tr("extras_ctfmon_label"));
                                let info = ui.add(
                                    egui::Label::new(
                                        egui::RichText::new("?")
                                            .strong()
                                            .color(egui::Color32::from_rgb(149, 194, 255)),
                                    )
                                    .sense(egui::Sense::hover()),
                                );
                                info.on_hover_text(self.ctfmon_tooltip());
                            });
                            ui.add_space(2.0);
                            ui.label(self.tr("extras_ctfmon_desc"));
                        },
                    );

                    ui.allocate_ui_with_layout(
                        egui::vec2(action_width, 32.0),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.set_width(action_width);
                            if ui
                                .add(
                                    egui::Button::new(
                                        if self.state.extras.ctfmon_service_dll_open {
                                            "v"
                                        } else {
                                            ">"
                                        },
                                    )
                                    .frame(false)
                                    .min_size(egui::vec2(28.0, 28.0)),
                                )
                                .clicked()
                            {
                                self.state.extras.ctfmon_service_dll_open =
                                    !self.state.extras.ctfmon_service_dll_open;
                            }
                            let mut local_enabled = enabled;
                            let resp = Self::native_toggle_switch(ui, &mut local_enabled);
                            if resp.changed() {
                                if let Err(err) = Self::apply_ctfmon_blocked(local_enabled) {
                                    self.toasts
                                        .error(err)
                                        .duration(std::time::Duration::from_secs_f64(3.5));
                                    return;
                                }
                                self.state.extras.ctfmon_blocked_enabled = local_enabled;
                            }
                        },
                    );
                });
            });

        if self.state.extras.ctfmon_service_dll_open {
            ui.add_space(4.0);
            self.extras_ctfmon_child_row(ui);
        }
    }

    fn brave_debloat_tooltip(&self) -> String {
        [
            "BraveRewardsDisabled = 1",
            "BraveWalletDisabled = 1",
            "BraveVPNDisabled = 1",
            "BraveAIChatEnabled = 0",
            "BraveStatsPingEnabled = 0",
        ]
        .join("\n")
    }

    fn edge_debloat_tooltip(&self) -> String {
        [
            "CreateDesktopShortcutDefault = 0",
            "PersonalizationReportingEnabled = 0",
            "ExtensionInstallBlocklist\\1 = ofefcgjbeghpigppfmkologfjadafddi",
            "ShowRecommendationsEnabled = 0",
            "HideFirstRunExperience = 1",
            "UserFeedbackAllowed = 0",
            "ConfigureDoNotTrack = 1",
            "AlternateErrorPagesEnabled = 0",
            "EdgeCollectionsEnabled = 0",
            "EdgeShoppingAssistantEnabled = 0",
            "MicrosoftEdgeInsiderPromotionEnabled = 0",
            "ShowMicrosoftRewards = 0",
            "WebWidgetAllowed = 0",
            "DiagnosticData = 0",
            "EdgeAssetDeliveryServiceEnabled = 0",
            "WalletDonationEnabled = 0",
            "DefaultBrowserSettingsCampaignEnabled = 0",
        ]
        .join("\n")
    }

    fn widgets_remove_tooltip(&self) -> String {
        [
            "Get-AppxPackage Microsoft.WidgetsPlatformRuntime -AllUsers | Remove-AppxPackage -AllUsers",
            "Get-AppxPackage MicrosoftWindows.Client.WebExperience -AllUsers | Remove-AppxPackage -AllUsers",
            "Add-AppxPackage -Register C:\\Program Files\\WindowsApps\\Microsoft.WidgetsPlatformRuntime*\\AppxManifest.xml -DisableDevelopmentMode",
            "Add-AppxPackage -Register C:\\Program Files\\WindowsApps\\MicrosoftWindows.Client.WebExperience*\\AppxManifest.xml -DisableDevelopmentMode",
        ]
        .join("\n")
    }

    fn ctfmon_tooltip(&self) -> String {
        [
            r"HKLM\Software\Microsoft\Input\InputServiceEnabled = 0/1",
            r"HKLM\Software\Microsoft\Input\InputServiceEnabledForCCI = 0/1",
        ]
        .join("\n")
    }

    fn timer_resolution_tooltip(&self) -> String {
        [
            r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\kernel\GlobalTimerResolutionRequests = 1",
            "Off removes the value again.",
            "Requires a reboot to take effect.",
        ]
        .join("\n")
    }

    fn extras_ctfmon_child_row(&mut self, ui: &mut egui::Ui) {
        let enabled = self.state.extras.ctfmon_service_dll_enabled;
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(20, 20, 23))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 35, 39)))
            .corner_radius(10.0)
            .inner_margin(egui::Margin::symmetric(10, 8))
            .show(ui, |ui| {
                let available = ui.available_width();
                let left_pad = 34.0;
                let switch_slot = 72.0;
                let left_width = (available - left_pad - switch_slot - 12.0).max(220.0);
                ui.horizontal_top(|ui| {
                    ui.add_space(left_pad);
                    ui.allocate_ui_with_layout(
                        egui::vec2(left_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(left_width);
                            ui.label(self.tr("extras_ctfmon_details_label"));
                            ui.add_space(2.0);
                            ui.label(self.tr("extras_ctfmon_details_desc"));
                        },
                    );
                    ui.allocate_ui_with_layout(
                        egui::vec2(switch_slot, 28.0),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.set_width(switch_slot);
                            let mut local_enabled = enabled;
                            let resp = Self::native_toggle_switch(ui, &mut local_enabled);
                            if resp.changed() {
                                if let Err(err) = Self::apply_ctfmon_service_dll(local_enabled) {
                                    self.toasts
                                        .error(err)
                                        .duration(std::time::Duration::from_secs_f64(3.5));
                                    return;
                                }
                                self.state.extras.ctfmon_service_dll_enabled = local_enabled;
                            }
                        },
                    );
                });
            });
    }

    fn ipv6_preferred_tooltip(&self) -> String {
        ["SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters\\DisabledComponents = 32"]
            .join("\n")
    }

    fn teredo_disabled_tooltip(&self) -> String {
        [
            "SYSTEM\\CurrentControlSet\\Services\\Tcpip6\\Parameters\\DisabledComponents = 1",
            "netsh interface teredo set state disabled",
            "netsh interface teredo set state default",
        ]
        .join("\n")
    }

    fn powershell7_telemetry_tooltip(&self) -> String {
        [
            "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment\\POWERSHELL_TELEMETRY_OPTOUT = 1",
            "[Environment]::SetEnvironmentVariable('POWERSHELL_TELEMETRY_OPTOUT', '1', 'Machine')",
            "[Environment]::SetEnvironmentVariable('POWERSHELL_TELEMETRY_OPTOUT', '', 'Machine')",
        ].join("\n")
    }

    fn hpet_tooltip(&self) -> String {
        [
            "bcdedit /set useplatformclock false",
            "bcdedit /set useplatformclock true",
            "HPET off can help some AMD CPUs and FACEIT anti-cheat.",
        ]
        .join("\n")
    }

    fn extras_power_plan_section(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(20, 20, 23))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(35, 35, 39)))
            .corner_radius(10.0)
            .inner_margin(egui::Margin::symmetric(10, 8))
            .show(ui, |ui| {
                let available = ui.available_width();
                let action_width = 118.0;
                let text_width = (available - action_width - 12.0).max(220.0);
                ui.horizontal(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(text_width, 34.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.vertical(|ui| {
                                ui.set_width(text_width);
                                ui.label(self.tr("extras_power_plan_label"));
                                ui.add_space(2.0);
                                ui.label(self.tr("extras_power_plan_desc"));
                            });
                        },
                    );

                    ui.allocate_ui_with_layout(
                        egui::vec2(action_width, 34.0),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.set_width(action_width);
                            let active = self.state.extras.winchisel_power_plan_enabled;
                            let pending = self.extras_power_plan_worker.is_some();

                            if pending {
                                ui.add(egui::Spinner::new().size(18.0));
                                return;
                            }

                            let (btn_fill, btn_stroke) = if active {
                                (
                                    egui::Color32::from_rgb(34, 139, 34),
                                    egui::Color32::from_rgb(50, 205, 50),
                                )
                            } else {
                                (
                                    egui::Color32::from_rgb(35, 54, 80),
                                    egui::Color32::from_rgb(10, 210, 254),
                                )
                            };

                            if ui
                                .add_sized(
                                    [118.0, 34.0],
                                    egui::Button::new(if active {
                                        self.tr("extras_power_plan_active")
                                    } else {
                                        self.tr("extras_power_plan_apply")
                                    })
                                    .fill(btn_fill)
                                    .stroke(egui::Stroke::new(1.0, btn_stroke)),
                                )
                                .clicked()
                                && !active
                            {
                                self.state.extras.winchisel_power_plan_loaded = false;
                                let (tx, rx) = mpsc::channel();
                                self.extras_power_plan_worker =
                                    Some(super::ExtrasBoolWorker { rx });
                                std::thread::spawn(move || {
                                    let _ = tx.send(Self::apply_winchisel_power_plan());
                                });
                            }
                        },
                    );
                });
            });
    }
}
