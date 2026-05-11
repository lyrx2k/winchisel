use super::WinchiselApp;
use crate::{GamingTweakRow, app_definitions::current_language, i18n::t};
use eframe::egui;
// iconflow nicht mehr direkt benötigt (Icons werden über IconCache gecacht)
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{LazyLock, Mutex};
use winreg::{RegKey, enums::*};

static PRIVACY_EXPANDED: LazyLock<Mutex<HashMap<&'static str, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone)]
pub(crate) struct PrivacySecurityState {
    pub(crate) privacy_query: String,
    pub(crate) privacy_quick_action_index: usize,
    pub(crate) loaded: bool,
    pub(crate) groups: [Vec<GamingTweakRow>; 13],
}

pub(crate) struct PrivacySecurityLoadResult {
    pub(crate) groups: [Vec<GamingTweakRow>; 13],
}

pub(crate) struct PrivacySecurityLoadWorker {
    pub(crate) rx: Receiver<PrivacySecurityLoadResult>,
}

#[derive(Clone)]
struct RegistryTarget {
    root: &'static str,
    path: &'static str,
    value_name: &'static str,
    kind: RegistryKind,
}

#[derive(Clone, PartialEq)]
enum RegistryKind {
    Dword,
    String,
}

#[derive(Clone, PartialEq)]
enum ValueState {
    Dword(u32),
    String(&'static str),
    Missing,
}

#[derive(Clone)]
struct PrivacyOption {
    label: &'static str,
    values: Vec<ValueState>,
    is_default: bool,
    is_recommended: bool,
}

#[derive(Clone)]
struct PrivacyRowMeta {
    recommended: String,
    default: String,
    badge_recommended: bool,
    badge_default: bool,
    badge_custom: bool,
}

impl WinchiselApp {
    pub(crate) fn privacy_tick(app: &mut WinchiselApp, _ui: &mut egui::Ui) {
        if app.state.active_tab == super::Tab::SecurityPrivacy
            && !app.state.privacy_security.loaded
            && app.privacy_load_worker.is_none()
        {
            app.start_privacy_security_load();
        }
        // Repaint wird bereits im Haupt-Loop (app.rs) gehandhabt,
        // wenn privacy_load_worker.is_some()
    }

    pub(crate) fn privacy_sidebar_loading(app: &WinchiselApp) -> bool {
        app.privacy_load_worker.is_some()
    }

    pub(crate) fn spawn_privacy_security_worker(
        lang: crate::Language,
    ) -> (Option<PrivacySecurityLoadWorker>, bool) {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let groups = Self::build_privacy_security_groups(lang);
            let _ = tx.send(PrivacySecurityLoadResult { groups });
        });
        (Some(PrivacySecurityLoadWorker { rx }), true)
    }

    pub(crate) fn start_privacy_security_load(&mut self) {
        if self.privacy_load_worker.is_some() {
            return;
        }
        self.state.privacy_security.loaded = true;
        let (worker, loading) = Self::spawn_privacy_security_worker(self.state.settings.language);
        self.privacy_load_worker = worker;
        self.state.privacy_security.loaded = loading;
    }

    pub(crate) fn poll_privacy_security_load(&mut self) {
        let Some(worker) = self.privacy_load_worker.as_ref() else {
            return;
        };
        match worker.rx.try_recv() {
            Ok(result) => {
                self.state.privacy_security.groups = result.groups;
                self.state.privacy_security.loaded = true;
                self.privacy_load_worker = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.state.privacy_security.loaded = false;
                self.privacy_load_worker = None;
            }
        }
    }

    pub(crate) fn build_privacy_security_groups(
        _lang: crate::Language,
    ) -> [Vec<GamingTweakRow>; 13] {
        {
            let mut groups: [Vec<GamingTweakRow>; 13] = Default::default();

            groups[0] = vec![
                Self::combo_row(
                    3000,
                    "security-uac-level",
                    t(_lang, "privacy_uac_title"),
                    t(_lang, "privacy_uac_desc"),
                    vec![
                        RegistryTarget {
                            root: "HKLM",
                            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                            value_name: "ConsentPromptBehaviorAdmin",
                            kind: RegistryKind::Dword,
                        },
                        RegistryTarget {
                            root: "HKLM",
                            path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                            value_name: "PromptOnSecureDesktop",
                            kind: RegistryKind::Dword,
                        },
                    ],
                    vec![
                        PrivacyOption {
                            label: t(_lang, "privacy_uac_opt_0"),
                            values: vec![ValueState::Dword(1), ValueState::Dword(1)],
                            is_default: false,
                            is_recommended: false,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_uac_opt_1"),
                            values: vec![ValueState::Dword(2), ValueState::Dword(1)],
                            is_default: false,
                            is_recommended: false,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_uac_opt_2"),
                            values: vec![ValueState::Dword(5), ValueState::Dword(1)],
                            is_default: true,
                            is_recommended: false,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_uac_opt_3"),
                            values: vec![ValueState::Dword(5), ValueState::Dword(0)],
                            is_default: false,
                            is_recommended: false,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_uac_opt_4"),
                            values: vec![ValueState::Dword(0), ValueState::Dword(0)],
                            is_default: false,
                            is_recommended: true,
                        },
                    ],
                ),
                Self::toggle_row(
                    3001,
                    "security-workplace-join-messages",
                    t(_lang, "privacy_workplace_join_title"),
                    t(_lang, "privacy_workplace_join_desc"),
                    vec![
                        RegistryTarget {
                            root: "HKLM",
                            path: r"SOFTWARE\Policies\Microsoft\Windows\WorkplaceJoin",
                            value_name: "BlockAADWorkplaceJoin",
                            kind: RegistryKind::Dword,
                        },
                        RegistryTarget {
                            root: "HKCU",
                            path: r"SOFTWARE\Policies\Microsoft\Windows\WorkplaceJoin",
                            value_name: "BlockAADWorkplaceJoin",
                            kind: RegistryKind::Dword,
                        },
                    ],
                    vec![ValueState::Missing],
                    vec![ValueState::Dword(1)],
                ),
                Self::toggle_row(
                    3002,
                    "security-bitlocker-auto-encryption",
                    t(_lang, "privacy_bitlocker_title"),
                    t(_lang, "privacy_bitlocker_desc"),
                    vec![RegistryTarget {
                        root: "HKLM",
                        path: r"SYSTEM\CurrentControlSet\Control\BitLocker",
                        value_name: "PreventDeviceEncryption",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(0)],
                    vec![ValueState::Dword(1)],
                ),
                Self::toggle_row(
                    3003,
                    "security-wifi-sense",
                    t(_lang, "privacy_wifi_sense_title"),
                    t(_lang, "privacy_wifi_sense_desc"),
                    vec![
                        RegistryTarget {
                            root: "HKLM",
                            path: r"Software\Microsoft\PolicyManager\default\WiFi\AllowWiFiHotSpotReporting",
                            value_name: "Value",
                            kind: RegistryKind::Dword,
                        },
                        RegistryTarget {
                            root: "HKLM",
                            path: r"Software\Microsoft\PolicyManager\default\WiFi\AllowAutoConnectToWiFiSenseHotspots",
                            value_name: "Value",
                            kind: RegistryKind::Dword,
                        },
                    ],
                    vec![ValueState::Dword(1), ValueState::Dword(1)],
                    vec![ValueState::Dword(0), ValueState::Dword(0)],
                ),
                Self::toggle_row(
                    3004,
                    "security-automatic-maintenance",
                    t(_lang, "privacy_automatic_maintenance_title"),
                    t(_lang, "privacy_automatic_maintenance_desc"),
                    vec![RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance",
                        value_name: "MaintenanceDisabled",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(0)],
                    vec![ValueState::Dword(1)],
                ),
                Self::toggle_row(
                    3005,
                    "security-error-reporting",
                    t(_lang, "privacy_error_reporting_title"),
                    t(_lang, "privacy_error_reporting_desc"),
                    vec![
                        RegistryTarget {
                            root: "HKLM",
                            path: r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                            value_name: "Disabled",
                            kind: RegistryKind::Dword,
                        },
                        RegistryTarget {
                            root: "HKCU",
                            path: r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                            value_name: "Disabled",
                            kind: RegistryKind::Dword,
                        },
                    ],
                    vec![ValueState::Dword(0), ValueState::Dword(0)],
                    vec![ValueState::Dword(1), ValueState::Dword(1)],
                ),
                Self::toggle_row(
                    3006,
                    "security-remote-assistance",
                    t(_lang, "privacy_remote_assistance_title"),
                    t(_lang, "privacy_remote_assistance_desc"),
                    vec![RegistryTarget {
                        root: "HKLM",
                        path: r"SYSTEM\CurrentControlSet\Control\Remote Assistance",
                        value_name: "fAllowToGetHelp",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(1)],
                    vec![ValueState::Dword(0)],
                ),
                Self::combo_row(
                    3007,
                    "security-smart-app-control",
                    t(_lang, "privacy_smart_app_control_title"),
                    t(_lang, "privacy_smart_app_control_desc"),
                    vec![RegistryTarget {
                        root: "HKLM",
                        path: r"SYSTEM\CurrentControlSet\Control\CI\Policy",
                        value_name: "VerifiedAndReputablePolicyState",
                        kind: RegistryKind::Dword,
                    }],
                    vec![
                        PrivacyOption {
                            label: t(_lang, "privacy_smart_app_control_opt_0"),
                            values: vec![ValueState::Dword(0)],
                            is_default: false,
                            is_recommended: true,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_smart_app_control_opt_1"),
                            values: vec![ValueState::Dword(1)],
                            is_default: false,
                            is_recommended: false,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_smart_app_control_opt_2"),
                            values: vec![ValueState::Dword(2)],
                            is_default: true,
                            is_recommended: false,
                        },
                    ],
                ),
                Self::toggle_row(
                    3008,
                    "security-developer-mode",
                    t(_lang, "privacy_developer_title"),
                    t(_lang, "privacy_developer_desc"),
                    vec![RegistryTarget {
                        root: "HKLM",
                        path: r"Software\Microsoft\Windows\CurrentVersion\AppModelUnlock",
                        value_name: "AllowDevelopmentWithoutDevLicense",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(1)],
                    vec![ValueState::Dword(0)],
                ),
                Self::combo_row(
                    3009,
                    "security-powershell-execution-policy",
                    t(_lang, "privacy_powershell_title"),
                    t(_lang, "privacy_powershell_desc"),
                    vec![
                        RegistryTarget {
                            root: "HKCU",
                            path: r"Software\Microsoft\PowerShell\1\ShellIds\Microsoft.PowerShell",
                            value_name: "ExecutionPolicy",
                            kind: RegistryKind::String,
                        },
                        RegistryTarget {
                            root: "HKLM",
                            path: r"Software\Microsoft\PowerShell\1\ShellIds\Microsoft.PowerShell",
                            value_name: "ExecutionPolicy",
                            kind: RegistryKind::String,
                        },
                    ],
                    vec![
                        PrivacyOption {
                            label: t(_lang, "privacy_powershell_opt_0"),
                            values: vec![
                                ValueState::String("Restricted"),
                                ValueState::String("Restricted"),
                            ],
                            is_default: true,
                            is_recommended: false,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_powershell_opt_1"),
                            values: vec![
                                ValueState::String("AllSigned"),
                                ValueState::String("AllSigned"),
                            ],
                            is_default: false,
                            is_recommended: false,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_powershell_opt_2"),
                            values: vec![
                                ValueState::String("RemoteSigned"),
                                ValueState::String("RemoteSigned"),
                            ],
                            is_default: false,
                            is_recommended: true,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_powershell_opt_3"),
                            values: vec![
                                ValueState::String("Unrestricted"),
                                ValueState::String("Unrestricted"),
                            ],
                            is_default: false,
                            is_recommended: false,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_powershell_opt_4"),
                            values: vec![
                                ValueState::String("Bypass"),
                                ValueState::String("Bypass"),
                            ],
                            is_default: false,
                            is_recommended: false,
                        },
                    ],
                ),
            ];

            groups[1] = vec![
                Self::combo_row(
                    3010,
                    "privacy-ads-promotional-master",
                    t(_lang, "privacy_ads_title"),
                    t(_lang, "privacy_ads_desc"),
                    vec![RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Winhance\Settings",
                        value_name: "AdsPromotionalContentMode",
                        kind: RegistryKind::Dword,
                    }],
                    vec![
                        PrivacyOption {
                            label: t(_lang, "privacy_ads_opt_0"),
                            values: vec![ValueState::Dword(0)],
                            is_default: false,
                            is_recommended: false,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_ads_opt_1"),
                            values: vec![ValueState::Dword(1)],
                            is_default: false,
                            is_recommended: true,
                        },
                        PrivacyOption {
                            label: t(_lang, "privacy_ads_opt_2"),
                            values: vec![ValueState::Dword(2)],
                            is_default: true,
                            is_recommended: false,
                        },
                    ],
                ),
                Self::toggle_row(
                    3011,
                    "privacy-content-delivery-allowed",
                    t(_lang, "privacy_content_delivery_title"),
                    t(_lang, "privacy_content_delivery_desc"),
                    vec![RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "ContentDeliveryAllowed",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(1)],
                    vec![ValueState::Dword(0)],
                ),
                Self::toggle_row(
                    3012,
                    "privacy-subscribed-content",
                    t(_lang, "privacy_subscribed_content_title"),
                    t(_lang, "privacy_subscribed_content_desc"),
                    vec![RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "SubscribedContentEnabled",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Missing],
                    vec![ValueState::Dword(0)],
                ),
                Self::toggle_row(
                    3013,
                    "privacy-feature-management",
                    t(_lang, "privacy_feature_management_title"),
                    t(_lang, "privacy_feature_management_desc"),
                    vec![RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "FeatureManagementEnabled",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(1)],
                    vec![ValueState::Dword(0)],
                ),
                Self::toggle_row(
                    3014,
                    "privacy-soft-landing",
                    t(_lang, "privacy_soft_landing_title"),
                    t(_lang, "privacy_soft_landing_desc"),
                    vec![RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "SoftLandingEnabled",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(1)],
                    vec![ValueState::Dword(0)],
                ),
                Self::toggle_row(
                    3015,
                    "privacy-oem-preinstalled-apps",
                    t(_lang, "privacy_oem_preinstalled_title"),
                    t(_lang, "privacy_oem_preinstalled_desc"),
                    vec![RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "OemPreInstalledAppsEnabled",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(1)],
                    vec![ValueState::Dword(0)],
                ),
                Self::toggle_row(
                    3016,
                    "privacy-preinstalled-apps",
                    t(_lang, "privacy_preinstalled_title"),
                    t(_lang, "privacy_preinstalled_desc"),
                    vec![RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "PreInstalledAppsEnabled",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(1)],
                    vec![ValueState::Dword(0)],
                ),
                Self::toggle_row(
                    3017,
                    "privacy-preinstalled-apps-ever",
                    t(_lang, "privacy_preinstalled_ever_title"),
                    t(_lang, "privacy_preinstalled_ever_desc"),
                    vec![RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "PreInstalledAppsEverEnabled",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Missing],
                    vec![ValueState::Dword(0)],
                ),
                Self::toggle_row(
                    3018,
                    "privacy-silent-installed-apps",
                    t(_lang, "privacy_silent_installed_title"),
                    t(_lang, "privacy_silent_installed_desc"),
                    vec![RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "SilentInstalledAppsEnabled",
                        kind: RegistryKind::Dword,
                    }],
                    vec![ValueState::Dword(1)],
                    vec![ValueState::Dword(0)],
                ),
            ];

            groups[2].push(Self::toggle_row(
                3034,
                "privacy-lock-screen",
                t(_lang, "privacy_lock_screen_title"),
                t(_lang, "privacy_lock_screen_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
                    value_name: "DisableLockWorkstation",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0)],
                vec![ValueState::Dword(1)],
            ));
            groups[2].push(Self::toggle_row(
                3035,
                "privacy-rotating-lock-screen",
                t(_lang, "privacy_rotating_lock_title"),
                t(_lang, "privacy_rotating_lock_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "RotatingLockScreenEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1)],
                vec![ValueState::Dword(0)],
            ));
            groups[2].push(Self::toggle_row(
                3036,
                "privacy-lock-screen-overlay",
                t(_lang, "privacy_lock_screen_overlay_title"),
                t(_lang, "privacy_lock_screen_overlay_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "RotatingLockScreenOverlayEnabled",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "SubscribedContent-338387Enabled",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![ValueState::Dword(1), ValueState::Dword(1)],
                vec![ValueState::Dword(0), ValueState::Dword(0)],
            ));
            groups[3].push(Self::toggle_row(3037, "privacy-advertising-id", t(_lang, "privacy_advertising_id_title"), t(_lang, "privacy_advertising_id_desc"), vec![RegistryTarget { root: "HKCU", path: r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo", value_name: "Enabled", kind: RegistryKind::Dword }, RegistryTarget { root: "HKCU", path: r"Software\Microsoft\Windows\CurrentVersion\CPSS\Store\AdvertisingInfo", value_name: "Value", kind: RegistryKind::Dword }, RegistryTarget { root: "HKCU", path: r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo", value_name: "DisabledByGroupPolicy", kind: RegistryKind::Dword }, RegistryTarget { root: "HKLM", path: r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo", value_name: "DisabledByGroupPolicy", kind: RegistryKind::Dword }], vec![ValueState::Dword(1), ValueState::Dword(1), ValueState::Missing, ValueState::Missing], vec![ValueState::Dword(0), ValueState::Dword(0), ValueState::Dword(1), ValueState::Dword(1)]));
            groups[3].push(Self::toggle_row(
                3038,
                "privacy-language-list",
                t(_lang, "privacy_language_list_title"),
                t(_lang, "privacy_language_list_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Control Panel\International\User Profile",
                    value_name: "HttpAcceptLanguageOptOut",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[3].push(Self::toggle_row(
                3039,
                "privacy-app-launch-tracking",
                t(_lang, "privacy_app_launch_title"),
                t(_lang, "privacy_app_launch_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "Start_TrackProgs",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[3].push(Self::toggle_row(
                3040,
                "privacy-settings-content",
                t(_lang, "privacy_settings_content_title"),
                t(_lang, "privacy_settings_content_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "SubscribedContent-338393Enabled",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "SubscribedContent-353694Enabled",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                        value_name: "SubscribedContent-353696Enabled",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![
                    ValueState::Missing,
                    ValueState::Missing,
                    ValueState::Missing,
                ],
                vec![
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                ],
            ));
            groups[3].push(Self::toggle_row(3041, "privacy-settings-notifications", t(_lang, "privacy_settings_notifications_title"), t(_lang, "privacy_settings_notifications_desc"), vec![RegistryTarget { root: "HKCU", path: r"Software\Microsoft\Windows\CurrentVersion\SystemSettings\AccountNotifications", value_name: "EnableAccountNotifications", kind: RegistryKind::Dword }], vec![ValueState::Missing], vec![ValueState::Dword(0)]));
            groups[4].push(Self::toggle_row(
                3042,
                "privacy-speech-recognition",
                t(_lang, "privacy_speech_title"),
                t(_lang, "privacy_speech_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy",
                        value_name: "HasAccepted",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Policies\Microsoft\InputPersonalization",
                        value_name: "AllowInputPersonalization",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Policies\Microsoft\InputPersonalization",
                        value_name: "AllowInputPersonalization",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![
                    ValueState::Dword(1),
                    ValueState::Dword(1),
                    ValueState::Dword(1),
                ],
                vec![
                    ValueState::Dword(0),
                    ValueState::Missing,
                    ValueState::Missing,
                ],
            ));
            groups[4].push(Self::toggle_row(
                3043,
                "privacy-narrator-online-services",
                t(_lang, "privacy_narrator_online_title"),
                t(_lang, "privacy_narrator_online_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Narrator\NoRoam",
                    value_name: "OnlineServicesEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[4].push(Self::toggle_row(
                3044,
                "privacy-narrator-scripting",
                t(_lang, "privacy_narrator_scripting_title"),
                t(_lang, "privacy_narrator_scripting_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Narrator\NoRoam",
                    value_name: "ScriptingEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[5].push(Self::toggle_row(3045, "privacy-inking-typing-dictionary", t(_lang, "privacy_inking_title"), t(_lang, "privacy_inking_desc"), vec![RegistryTarget { root: "HKCU", path: r"Software\Microsoft\Windows\CurrentVersion\CPSS\Store\InkingAndTypingPersonalization", value_name: "Value", kind: RegistryKind::Dword }, RegistryTarget { root: "HKCU", path: r"Software\Microsoft\Personalization\Settings", value_name: "AcceptedPrivacyPolicy", kind: RegistryKind::Dword }, RegistryTarget { root: "HKCU", path: r"Software\Microsoft\InputPersonalization", value_name: "RestrictImplicitTextCollection", kind: RegistryKind::Dword }, RegistryTarget { root: "HKCU", path: r"Software\Microsoft\InputPersonalization\TrainedDataStore", value_name: "HarvestContacts", kind: RegistryKind::Dword }], vec![ValueState::Dword(1), ValueState::Dword(1), ValueState::Dword(0), ValueState::Dword(1)], vec![ValueState::Dword(0), ValueState::Dword(0), ValueState::Dword(1), ValueState::Dword(0)]));
            let mut diagnostics_row = Self::toggle_row(
                3046,
                "privacy-diagnostics",
                t(_lang, "privacy_diagnostics_title"),
                t(_lang, "privacy_diagnostics_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack",
                        value_name: "ShowedToastAtLevel",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                        value_name: "AllowTelemetry",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                        value_name: "AllowTelemetry",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                        value_name: "AllowTelemetry",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                        value_name: "MaxTelemetryAllowed",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                        value_name: "MaxTelemetryAllowed",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                        value_name: "AllowTelemetry",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                        value_name: "AITEnable",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                        value_name: "AITEnable",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![
                    ValueState::Dword(3),
                    ValueState::Dword(3),
                    ValueState::Dword(3),
                    ValueState::Dword(3),
                    ValueState::Dword(3),
                    ValueState::Dword(3),
                    ValueState::Dword(3),
                    ValueState::Missing,
                    ValueState::Missing,
                ],
                vec![
                    ValueState::Dword(1),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                ],
            );
            diagnostics_row.default_label = "3".to_string();
            diagnostics_row.recommended_label = "1".to_string();
            groups[6].push(diagnostics_row);
            groups[6].push(Self::toggle_row(3047, "privacy-improve-inking-typing", t(_lang, "privacy_improve_inking_title"), t(_lang, "privacy_improve_inking_desc"), vec![RegistryTarget { root: "HKCU", path: r"Software\Microsoft\Input\TIPC", value_name: "Enabled", kind: RegistryKind::Dword }, RegistryTarget { root: "HKCU", path: r"Software\Microsoft\Windows\CurrentVersion\CPSS\Store\ImproveInkingAndTyping", value_name: "Value", kind: RegistryKind::Dword }], vec![ValueState::Dword(1), ValueState::Dword(1)], vec![ValueState::Dword(0), ValueState::Dword(0)]));
            groups[6].push(Self::toggle_row(
                3048,
                "privacy-tailored-experiences",
                t(_lang, "privacy_tailored_experiences_title"),
                t(_lang, "privacy_tailored_experiences_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Windows\CurrentVersion\Privacy",
                        value_name: "TailoredExperiencesWithDiagnosticDataEnabled",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Policies\Microsoft\Windows\CloudContent",
                        value_name: "DisableTailoredExperiencesWithDiagnosticData",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKLM",
                        path: r"Software\Policies\Microsoft\Windows\CloudContent",
                        value_name: "DisableTailoredExperiencesWithDiagnosticData",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![
                    ValueState::Dword(1),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                ],
                vec![
                    ValueState::Dword(0),
                    ValueState::Missing,
                    ValueState::Missing,
                ],
            ));
            groups[6].push(Self::toggle_row(
                3049,
                "privacy-feedback-frequency",
                t(_lang, "privacy_feedback_title"),
                t(_lang, "privacy_feedback_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                        value_name: "DoNotShowFeedbackNotifications",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                        value_name: "DoNotShowFeedbackNotifications",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Microsoft\Siuf\Rules",
                        value_name: "NumberOfSIUFInPeriod",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![
                    ValueState::Missing,
                    ValueState::Missing,
                    ValueState::Missing,
                ],
                vec![
                    ValueState::Dword(1),
                    ValueState::Dword(1),
                    ValueState::Dword(0),
                ],
            ));
            groups[7].push(Self::toggle_row(
                3050,
                "privacy-activity-history",
                t(_lang, "privacy_activity_history_title"),
                t(_lang, "privacy_activity_history_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\System",
                        value_name: "PublishUserActivities",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\System",
                        value_name: "PublishUserActivities",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![ValueState::Missing, ValueState::Missing],
                vec![ValueState::Dword(0), ValueState::Dword(0)],
            ));
            groups[7].push(Self::toggle_row(
                3051,
                "privacy-timeline-suggestions",
                t(_lang, "privacy_timeline_title"),
                t(_lang, "privacy_timeline_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "SubscribedContent-353698Enabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[8].push(Self::toggle_row(
                3052,
                "privacy-search-history",
                t(_lang, "privacy_search_history_title"),
                t(_lang, "privacy_search_history_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                    value_name: "IsDeviceSearchHistoryEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[8].push(Self::toggle_row(
                3053,
                "privacy-search-highlights",
                t(_lang, "privacy_search_highlights_title"),
                t(_lang, "privacy_search_highlights_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                    value_name: "IsDynamicSearchBoxEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[8].push(Self::toggle_row(
                3054,
                "privacy-search-msa-cloud",
                t(_lang, "privacy_search_msa_title"),
                t(_lang, "privacy_search_msa_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                    value_name: "IsMSACloudSearchEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[8].push(Self::toggle_row(
                3055,
                "privacy-search-aad-cloud",
                t(_lang, "privacy_search_aad_title"),
                t(_lang, "privacy_search_aad_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                    value_name: "IsAADCloudSearchEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[8].push(Self::toggle_row(
                3056,
                "privacy-allow-cortana",
                t(_lang, "privacy_cortana_title"),
                t(_lang, "privacy_cortana_desc"),
                vec![
                    RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                        value_name: "AllowCortana",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                        value_name: "AllowCortana",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![ValueState::Missing, ValueState::Missing],
                vec![ValueState::Dword(0), ValueState::Dword(0)],
            ));
            groups[9].push(Self::toggle_row(3057, "privacy-location-services", t(_lang, "privacy_location_title"), t(_lang, "privacy_location_desc"), vec![RegistryTarget { root: "HKLM", path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location", value_name: "Value", kind: RegistryKind::String }, RegistryTarget { root: "HKCU", path: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors", value_name: "DisableLocation", kind: RegistryKind::Dword }, RegistryTarget { root: "HKLM", path: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors", value_name: "DisableLocation", kind: RegistryKind::Dword }], vec![ValueState::String("Allow"), ValueState::Missing, ValueState::Dword(0), ValueState::Dword(0)], vec![ValueState::String("Deny"), ValueState::Missing, ValueState::Missing]));
            groups[9].push(Self::toggle_row(3058, "privacy-camera-access", t(_lang, "privacy_camera_title"), t(_lang, "privacy_camera_desc"), vec![RegistryTarget { root: "HKLM", path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam", value_name: "Value", kind: RegistryKind::String }], vec![ValueState::String("Allow"), ValueState::Missing], vec![ValueState::String("Deny")]));
            groups[9].push(Self::toggle_row(3059, "privacy-microphone-access", t(_lang, "privacy_microphone_title"), t(_lang, "privacy_microphone_desc"), vec![RegistryTarget { root: "HKLM", path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone", value_name: "Value", kind: RegistryKind::String }], vec![ValueState::String("Allow"), ValueState::Missing], vec![ValueState::String("Deny")]));
            groups[9].push(Self::toggle_row(3060, "privacy-account-info-access", t(_lang, "privacy_account_info_title"), t(_lang, "privacy_account_info_desc"), vec![RegistryTarget { root: "HKLM", path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation", value_name: "Value", kind: RegistryKind::String }], vec![ValueState::String("Allow"), ValueState::Missing], vec![ValueState::String("Deny")]));
            groups[9].push(Self::toggle_row(3061, "privacy-app-diagnostic-access", t(_lang, "privacy_app_diagnostic_title"), t(_lang, "privacy_app_diagnostic_desc"), vec![RegistryTarget { root: "HKLM", path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics", value_name: "Value", kind: RegistryKind::String }], vec![ValueState::String("Allow"), ValueState::Missing], vec![ValueState::String("Deny")]));
            groups[9].push(Self::toggle_row(
                3062,
                "privacy-onedrive-auto-backup",
                t(_lang, "privacy_onedrive_backup_title"),
                t(_lang, "privacy_onedrive_backup_desc"),
                vec![
                    RegistryTarget {
                        root: "HKLM",
                        path: r"SOFTWARE\Policies\Microsoft\OneDrive",
                        value_name: "KFMBlockOptIn",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"SOFTWARE\Policies\Microsoft\OneDrive",
                        value_name: "KFMBlockOptIn",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![ValueState::Missing, ValueState::Missing],
                vec![ValueState::Dword(1), ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3063,
                "privacy-turn-off-copilot",
                t(_lang, "privacy_copilot_title"),
                t(_lang, "privacy_copilot_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Policies\Microsoft\Windows\WindowsCopilot",
                        value_name: "TurnOffWindowsCopilot",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKLM",
                        path: r"Software\Policies\Microsoft\Windows\WindowsCopilot",
                        value_name: "TurnOffWindowsCopilot",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![
                    ValueState::Dword(0),
                    ValueState::Missing,
                    ValueState::Dword(0),
                    ValueState::Missing,
                ],
                vec![ValueState::Dword(1), ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3064,
                "privacy-disable-ai-data-analysis",
                t(_lang, "privacy_ai_data_title"),
                t(_lang, "privacy_ai_data_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableAIDataAnalysis",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3065,
                "privacy-block-recall-enablement",
                t(_lang, "privacy_recall_enable_title"),
                t(_lang, "privacy_recall_enable_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "AllowRecallEnablement",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[10].push(Self::toggle_row(
                3066,
                "privacy-disable-recall-snapshots",
                t(_lang, "privacy_recall_snapshots_title"),
                t(_lang, "privacy_recall_snapshots_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "TurnOffSavingSnapshots",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3067,
                "privacy-disable-click-to-do",
                t(_lang, "privacy_click_to_do_title"),
                t(_lang, "privacy_click_to_do_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableClickToDo",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3068,
                "privacy-disable-settings-agent",
                t(_lang, "privacy_settings_agent_title"),
                t(_lang, "privacy_settings_agent_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableSettingsAgent",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3069,
                "privacy-disable-agent-connectors",
                t(_lang, "privacy_agent_connectors_title"),
                t(_lang, "privacy_agent_connectors_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableAgentConnectors",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3070,
                "privacy-disable-agent-workspaces",
                t(_lang, "privacy_agent_workspaces_title"),
                t(_lang, "privacy_agent_workspaces_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableAgentWorkspaces",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3071,
                "privacy-disable-remote-agent-connectors",
                t(_lang, "privacy_remote_agent_connectors_title"),
                t(_lang, "privacy_remote_agent_connectors_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableRemoteAgentConnectors",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3072,
                "privacy-disable-copilot-hardware-key",
                t(_lang, "privacy_copilot_key_title"),
                t(_lang, "privacy_copilot_key_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CopilotKey",
                    value_name: "SetCopilotHardwareKey",
                    kind: RegistryKind::String,
                }],
                vec![ValueState::Missing],
                vec![ValueState::String("")],
            ));
            groups[10].push(Self::toggle_row(
                3073,
                "privacy-disable-copilot-runtime",
                t(_lang, "privacy_copilot_runtime_title"),
                t(_lang, "privacy_copilot_runtime_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "AllowCopilotRuntime",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[10].push(Self::toggle_row(
                3074,
                "privacy-copilot-unavailable",
                t(_lang, "privacy_copilot_available_title"),
                t(_lang, "privacy_copilot_available_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\Shell\Copilot",
                    value_name: "IsCopilotAvailable",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[10].push(Self::toggle_row(
                3075,
                "privacy-disable-bing-chat",
                t(_lang, "privacy_bing_chat_title"),
                t(_lang, "privacy_bing_chat_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\Shell\Copilot\BingChat",
                    value_name: "IsUserEligible",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[10].push(Self::toggle_row(3076, "privacy-deny-generative-ai-access", t(_lang, "privacy_generative_ai_title"), t(_lang, "privacy_generative_ai_desc"), vec![RegistryTarget { root: "HKLM", path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\generativeAI", value_name: "Value", kind: RegistryKind::String }, RegistryTarget { root: "HKLM", path: r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy", value_name: "LetAppsAccessGenerativeAI", kind: RegistryKind::Dword }], vec![ValueState::String("Allow"), ValueState::Missing, ValueState::Dword(0), ValueState::Missing], vec![ValueState::String("Deny"), ValueState::Dword(2)]));
            groups[10].push(Self::toggle_row(3077, "privacy-deny-system-ai-models", t(_lang, "privacy_system_ai_title"), t(_lang, "privacy_system_ai_desc"), vec![RegistryTarget { root: "HKLM", path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\systemAIModels", value_name: "Value", kind: RegistryKind::String }, RegistryTarget { root: "HKLM", path: r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy", value_name: "LetAppsAccessSystemAIModels", kind: RegistryKind::Dword }, RegistryTarget { root: "HKLM", path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\systemAIModels", value_name: "RecordUsageData", kind: RegistryKind::Dword }], vec![ValueState::String("Allow"), ValueState::Missing, ValueState::Dword(0), ValueState::Missing, ValueState::Missing], vec![ValueState::String("Deny"), ValueState::Dword(2), ValueState::Dword(0)]));
            groups[10].push(Self::toggle_row(3078, "privacy-deny-copilot-microphone", t(_lang, "privacy_copilot_microphone_title"), t(_lang, "privacy_copilot_microphone_desc"), vec![RegistryTarget { root: "HKCU", path: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone\Microsoft.Copilot_8wekyb3d8bbwe", value_name: "Value", kind: RegistryKind::String }, RegistryTarget { root: "HKCU", path: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone\Microsoft.MicrosoftOfficeHub_8wekyb3d8bbwe", value_name: "Value", kind: RegistryKind::String }], vec![ValueState::String("Allow"), ValueState::Missing, ValueState::String("Allow"), ValueState::Missing], vec![ValueState::String("Deny"), ValueState::String("Deny")]));
            groups[10].push(Self::toggle_row(
                3079,
                "privacy-disable-paint-ai-image-creator",
                t(_lang, "privacy_paint_image_creator_title"),
                t(_lang, "privacy_paint_image_creator_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Paint",
                    value_name: "DisableImageCreator",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3080,
                "privacy-disable-paint-ai-cocreator",
                t(_lang, "privacy_paint_cocreator_title"),
                t(_lang, "privacy_paint_cocreator_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Paint",
                    value_name: "DisableCocreator",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3081,
                "privacy-disable-paint-generative-fill",
                t(_lang, "privacy_paint_fill_title"),
                t(_lang, "privacy_paint_fill_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Paint",
                    value_name: "DisableGenerativeFill",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3082,
                "privacy-disable-paint-generative-erase",
                t(_lang, "privacy_paint_erase_title"),
                t(_lang, "privacy_paint_erase_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Paint",
                    value_name: "DisableGenerativeErase",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3083,
                "privacy-disable-paint-remove-background",
                t(_lang, "privacy_paint_background_title"),
                t(_lang, "privacy_paint_background_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Paint",
                    value_name: "DisableRemoveBackground",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[10].push(Self::toggle_row(
                3084,
                "privacy-disable-input-insights",
                t(_lang, "privacy_input_insights_title"),
                t(_lang, "privacy_input_insights_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\input\Settings",
                    value_name: "InsightsEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1)],
                vec![ValueState::Dword(0)],
            ));
            groups[10].push(Self::toggle_row(
                3085,
                "privacy-disable-copilot-nudges",
                t(_lang, "privacy_copilot_nudges_title"),
                t(_lang, "privacy_copilot_nudges_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "ShowCopilotNudges",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[10].push(Self::toggle_row(
                3086,
                "privacy-disable-consumer-ai-content",
                t(_lang, "privacy_consumer_ai_title"),
                t(_lang, "privacy_consumer_ai_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                    value_name: "DisableConsumerAccountStateContent",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[11].push(Self::toggle_row(
                3087,
                "privacy-edge-copilot-cdp-page-context",
                t(_lang, "privacy_edge_cdp_title"),
                t(_lang, "privacy_edge_cdp_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "CopilotCDPPageContext",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[11].push(Self::toggle_row(
                3088,
                "privacy-edge-copilot-page-context",
                t(_lang, "privacy_edge_page_title"),
                t(_lang, "privacy_edge_page_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "CopilotPageContext",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[11].push(Self::toggle_row(
                3089,
                "privacy-edge-copilot-sidebar",
                t(_lang, "privacy_edge_sidebar_title"),
                t(_lang, "privacy_edge_sidebar_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "HubsSidebarEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[11].push(Self::toggle_row(
                3090,
                "privacy-edge-entra-copilot",
                t(_lang, "privacy_edge_entra_title"),
                t(_lang, "privacy_edge_entra_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "EdgeEntraCopilotPageContext",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[11].push(Self::toggle_row(
                3091,
                "privacy-edge-m365-copilot-icon",
                t(_lang, "privacy_edge_m365_icon_title"),
                t(_lang, "privacy_edge_m365_icon_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "Microsoft365CopilotChatIconEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[11].push(Self::toggle_row(
                3092,
                "privacy-edge-ai-history-search",
                t(_lang, "privacy_edge_history_title"),
                t(_lang, "privacy_edge_history_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "EdgeHistoryAISearchEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[11].push(Self::toggle_row(
                3093,
                "privacy-edge-inline-compose",
                t(_lang, "privacy_edge_inline_title"),
                t(_lang, "privacy_edge_inline_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "ComposeInlineEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[11].push(Self::toggle_row(
                3094,
                "privacy-edge-local-ai-model",
                t(_lang, "privacy_edge_local_model_title"),
                t(_lang, "privacy_edge_local_model_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "GenAILocalFoundationalModelSettings",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(1)],
            ));
            groups[11].push(Self::toggle_row(
                3095,
                "privacy-edge-builtin-ai-apis",
                t(_lang, "privacy_edge_builtin_title"),
                t(_lang, "privacy_edge_builtin_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "BuiltInAIAPIsEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[11].push(Self::toggle_row(
                3096,
                "privacy-edge-ai-themes",
                t(_lang, "privacy_edge_themes_title"),
                t(_lang, "privacy_edge_themes_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "AIGenThemesEnabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[11].push(Self::toggle_row(
                3097,
                "privacy-edge-devtools-ai",
                t(_lang, "privacy_edge_devtools_title"),
                t(_lang, "privacy_edge_devtools_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "DevToolsGenAiSettings",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(0), ValueState::Missing],
                vec![ValueState::Dword(2)],
            ));
            groups[11].push(Self::toggle_row(
                3098,
                "privacy-edge-share-history-copilot",
                t(_lang, "privacy_edge_share_history_title"),
                t(_lang, "privacy_edge_share_history_desc"),
                vec![RegistryTarget {
                    root: "HKLM",
                    path: r"SOFTWARE\Policies\Microsoft\Edge",
                    value_name: "ShareBrowsingHistoryWithCopilotSearchAllowed",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[12].push(Self::toggle_row(
                3099,
                "privacy-office-ai-training",
                t(_lang, "privacy_office_training_title"),
                t(_lang, "privacy_office_training_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Policies\Microsoft\office\16.0\common\ai\training",
                    value_name: "optionalconnectedexperiencesenabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[12].push(Self::toggle_row(
                3100,
                "privacy-office-connected-services",
                t(_lang, "privacy_office_connected_title"),
                t(_lang, "privacy_office_connected_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Policies\Microsoft\office\16.0\common\privacy",
                        value_name: "controllerconnectedservicesenabled",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Policies\Microsoft\office\16.0\common\privacy",
                        value_name: "usercontentdisabled",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![
                    ValueState::Dword(0),
                    ValueState::Missing,
                    ValueState::Dword(0),
                    ValueState::Missing,
                ],
                vec![ValueState::Dword(2), ValueState::Dword(2)],
            ));
            groups[12].push(Self::toggle_row(
                3101,
                "privacy-word-copilot",
                t(_lang, "privacy_word_copilot_title"),
                t(_lang, "privacy_word_copilot_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Office\16.0\Word\Options",
                    value_name: "EnableCopilot",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[12].push(Self::toggle_row(
                3102,
                "privacy-excel-copilot",
                t(_lang, "privacy_excel_copilot_title"),
                t(_lang, "privacy_excel_copilot_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Microsoft\Office\16.0\Excel\Options",
                    value_name: "EnableCopilot",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups[12].push(Self::toggle_row(
                3103,
                "privacy-onenote-copilot",
                t(_lang, "privacy_onenote_copilot_title"),
                t(_lang, "privacy_onenote_copilot_desc"),
                vec![
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Office\16.0\OneNote\Options\Other",
                        value_name: "EnableCopilot",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Office\16.0\OneNote\Options\Other",
                        value_name: "EnableCopilotNotebooks",
                        kind: RegistryKind::Dword,
                    },
                    RegistryTarget {
                        root: "HKCU",
                        path: r"Software\Microsoft\Office\16.0\OneNote\Options\Other",
                        value_name: "EnableCopilotSkittle",
                        kind: RegistryKind::Dword,
                    },
                ],
                vec![
                    ValueState::Missing,
                    ValueState::Missing,
                    ValueState::Missing,
                ],
                vec![
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                    ValueState::Dword(0),
                ],
            ));
            groups[12].push(Self::toggle_row(
                3104,
                "privacy-office-content-safety-ai",
                t(_lang, "privacy_office_safety_title"),
                t(_lang, "privacy_office_safety_desc"),
                vec![RegistryTarget {
                    root: "HKCU",
                    path: r"Software\Policies\Microsoft\office\16.0\common\ai",
                    value_name: "contentsafetyserviceenabled",
                    kind: RegistryKind::Dword,
                }],
                vec![ValueState::Dword(1), ValueState::Missing],
                vec![ValueState::Dword(0)],
            ));
            groups
        }
    }

    fn privacy_state_label(values: &[ValueState]) -> String {
        let lang = current_language();
        let Some(value) = values
            .iter()
            .find(|value| !matches!(value, ValueState::Missing))
        else {
            return t(lang, "privacy_option_off").to_string();
        };
        match value {
            ValueState::Dword(0) => t(lang, "privacy_option_off").to_string(),
            ValueState::Dword(1) => t(lang, "privacy_option_on").to_string(),
            ValueState::Dword(other) => other.to_string(),
            ValueState::String(s) => s.to_string(),
            ValueState::Missing => t(lang, "privacy_option_off").to_string(),
        }
    }

    fn toggle_meta(
        targets: &[RegistryTarget],
        recommended_values: &[ValueState],
        default_values: &[ValueState],
        current_enabled: bool,
    ) -> PrivacyRowMeta {
        let is_recommended = Self::matches_registry(targets, recommended_values);
        let is_default = Self::matches_registry(targets, default_values);
        let recommended = Self::privacy_state_label(recommended_values);
        let default = Self::privacy_state_label(default_values);
        let badge_recommended = is_recommended;
        let badge_default = is_default;
        let badge_custom = !is_recommended && !is_default && current_enabled;
        PrivacyRowMeta {
            recommended,
            default,
            badge_recommended,
            badge_default,
            badge_custom,
        }
    }

    fn toggle_row(
        tweak_id: i32,
        key: &'static str,
        name: &'static str,
        description: &'static str,
        targets: Vec<RegistryTarget>,
        enabled_values: Vec<ValueState>,
        disabled_values: Vec<ValueState>,
    ) -> GamingTweakRow {
        let enabled = Self::matches_registry(&targets, &enabled_values);
        let meta = Self::toggle_meta(&targets, &enabled_values, &disabled_values, enabled);
        let row_enabled = enabled;
        GamingTweakRow {
            tweak_id,
            category: 0,
            key,
            name: name.to_string(),
            description: description.to_string(),
            enabled: row_enabled,
            is_editable: true,
            is_child: false,
            is_parent: false,
            is_expanded: false,
            warning_text: String::new(),
            input_type: 0,
            options: vec![
                t(current_language(), "privacy_option_on").to_string(),
                t(current_language(), "privacy_option_off").to_string(),
            ],
            selected_index: if row_enabled { 1 } else { 0 },
            recommended_label: meta.recommended,
            default_label: meta.default,
            badge_recommended: meta.badge_recommended,
            badge_default: meta.badge_default,
            badge_custom: meta.badge_custom,
            is_new: false,
        }
    }

    fn combo_row(
        tweak_id: i32,
        key: &'static str,
        name: &'static str,
        description: &'static str,
        targets: Vec<RegistryTarget>,
        options: Vec<PrivacyOption>,
    ) -> GamingTweakRow {
        let mut selected_index = 0usize;
        for (idx, opt) in options.iter().enumerate() {
            if Self::matches_registry(&targets, &opt.values) {
                selected_index = idx;
                break;
            }
        }
        let default_index = options
            .iter()
            .position(|opt| opt.is_default)
            .unwrap_or(selected_index);
        let recommended_index = options
            .iter()
            .position(|opt| opt.is_recommended)
            .unwrap_or(default_index);
        let meta = PrivacyRowMeta {
            recommended: options
                .get(recommended_index)
                .map(|o| o.label.to_string())
                .unwrap_or_default(),
            default: options
                .get(default_index)
                .map(|o| o.label.to_string())
                .unwrap_or_default(),
            badge_recommended: selected_index == recommended_index,
            badge_default: selected_index == default_index,
            badge_custom: selected_index != default_index && selected_index != recommended_index,
        };
        GamingTweakRow {
            tweak_id,
            category: 0,
            key,
            name: name.to_string(),
            description: description.to_string(),
            enabled: false,
            is_editable: true,
            is_child: false,
            is_parent: false,
            is_expanded: false,
            warning_text: String::new(),
            input_type: 1,
            options: options.iter().map(|o| o.label.to_string()).collect(),
            selected_index: selected_index as i32,
            recommended_label: meta.recommended,
            default_label: meta.default,
            badge_recommended: meta.badge_recommended,
            badge_default: meta.badge_default,
            badge_custom: meta.badge_custom,
            is_new: false,
        }
    }

    fn matches_registry(targets: &[RegistryTarget], expected: &[ValueState]) -> bool {
        let mut seen = false;
        for (idx, target) in targets.iter().enumerate() {
            let Some(exp) = expected.get(idx) else {
                continue;
            };
            match Self::read_privacy_registry(target) {
                Ok(current) => {
                    seen = true;
                    if current != *exp {
                        return false;
                    }
                }
                Err(_) => {
                    if *exp != ValueState::Missing {
                        continue;
                    }
                    seen = true;
                }
            }
        }
        seen
    }

    fn privacy_has_children(key: &str) -> bool {
        matches!(key, "privacy-lock-screen")
    }

    fn privacy_toggle_expand_state(key: &'static str) {
        if Self::privacy_has_children(key)
            && let Ok(mut m) = PRIVACY_EXPANDED.lock()
        {
            let current = *m.get(key).unwrap_or(&true);
            m.insert(key, !current);
        }
    }

    fn privacy_expand_state(key: &'static str) -> bool {
        if let Ok(m) = PRIVACY_EXPANDED.lock() {
            *m.get(key).unwrap_or(&true)
        } else {
            true
        }
    }

    fn sync_privacy_expand_state(groups: &mut [Vec<GamingTweakRow>; 13]) {
        for group in groups.iter_mut() {
            for row in group.iter_mut() {
                if Self::privacy_has_children(row.key) {
                    row.is_parent = true;
                    row.is_expanded = Self::privacy_expand_state(row.key);
                }
            }
        }
    }

    fn read_privacy_registry(target: &RegistryTarget) -> std::io::Result<ValueState> {
        let root = match target.root {
            "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
            _ => RegKey::predef(HKEY_CURRENT_USER),
        };
        let key = root.open_subkey_with_flags(target.path, KEY_READ)?;
        Ok(match target.kind {
            RegistryKind::Dword => ValueState::Dword(key.get_value::<u32, _>(target.value_name)?),
            RegistryKind::String => {
                let s: String = key.get_value(target.value_name)?;
                ValueState::String(Box::leak(s.into_boxed_str()))
            }
        })
    }

    pub(crate) fn render_privacy_security_tab(&mut self, ui: &mut egui::Ui) {
        Self::sync_privacy_expand_state(&mut self.state.privacy_security.groups);
        let privacy_search = self.tr("privacy_security_search").to_string();
        let privacy_quick = self.tr("privacy_security_quick").to_string();
        let privacy_apply = self.tr("privacy_security_apply_recommended").to_string();
        let privacy_reset = self.tr("privacy_security_reset_defaults").to_string();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading(self.tr("privacy_security_title"));
                    ui.label(self.tr("privacy_security_subtitle"));
                });
                ui.add_space(12.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let response = ui.add_sized(
                        [280.0, 30.0],
                        egui::TextEdit::singleline(&mut self.state.privacy_security.privacy_query)
                            .hint_text(privacy_search),
                    );
                    if response.changed() {
                        ui.ctx().request_repaint();
                    }
                    ui.add_space(10.0);
                    let quick_index = self.state.privacy_security.privacy_quick_action_index;
                    let selected_text = match quick_index {
                        1 => privacy_apply.as_str(),
                        2 => privacy_reset.as_str(),
                        _ => privacy_quick.as_str(),
                    };
                    let mut chosen_index = quick_index;
                    ui.allocate_ui_with_layout(
                        egui::vec2(220.0, 30.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.scope(|ui| {
                                ui.style_mut().spacing.interact_size.y = 30.0;
                                egui::ComboBox::from_id_salt("privacy_quick_actions")
                                    .width(220.0)
                                    .selected_text(selected_text)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut chosen_index,
                                            0,
                                            privacy_quick.as_str(),
                                        );
                                        ui.selectable_value(
                                            &mut chosen_index,
                                            1,
                                            privacy_apply.as_str(),
                                        );
                                        ui.selectable_value(
                                            &mut chosen_index,
                                            2,
                                            privacy_reset.as_str(),
                                        );
                                    });
                            });
                        },
                    );
                    if chosen_index != quick_index {
                        self.state.privacy_security.privacy_quick_action_index = chosen_index;
                        if chosen_index > 0 {
                            self.apply_privacy_quick_action(chosen_index);
                            self.state.privacy_security.privacy_quick_action_index = 0;
                        }
                    }
                });
            });
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);

            let privacy_current_default = self.tr("privacy_security_current_default").to_string();
            let privacy_current_recommended =
                self.tr("privacy_security_current_recommended").to_string();

            if !self.state.privacy_security.loaded {
                ui.add_space(40.0);
                ui.vertical_centered(|ui| {
                    ui.add(egui::Spinner::new().size(28.0));
                    ui.add_space(10.0);
                    ui.label(self.tr("privacy_security_loading"));
                });
                return;
            }

            let lang = self.state.settings.language;
            let warehouse_icon = self.icon_cache.warehouse;
            let star_icon = self.icon_cache.star;
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let query = self
                        .state
                        .privacy_security
                        .privacy_query
                        .trim()
                        .to_lowercase();
                    for (group_idx, group) in
                        self.state.privacy_security.groups.iter_mut().enumerate()
                    {
                        if group.is_empty() {
                            continue;
                        }
                        if !query.is_empty()
                            && !group
                                .iter()
                                .any(|row| Self::privacy_row_matches_query(row, &query))
                        {
                            continue;
                        }
                        let header = egui::CollapsingHeader::new(
                            egui::RichText::new(crate::i18n::t(
                                lang,
                                Self::privacy_group_label(group_idx),
                            ))
                            .strong()
                            .size(15.0)
                            .color(egui::Color32::from_rgb(149, 194, 255)),
                        )
                        .id_salt(("privacy_security_group", group_idx))
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.add_space(8.0);
                            if group_idx == 2 && group.len() >= 3 {
                                let (first, rest) = group.split_at_mut(1);
                                let parent_row = &mut first[0];
                                parent_row.is_parent = true;
                                parent_row.is_child = false;
                                parent_row.is_expanded = Self::privacy_expand_state(parent_row.key);
                                Self::render_privacy_row(
                                    ui,
                                    parent_row,
                                    &privacy_current_default,
                                    &privacy_current_recommended,
                                    warehouse_icon,
                                    star_icon,
                                );
                                ui.add_space(8.0);
                                if parent_row.is_expanded {
                                    for row in rest.iter_mut() {
                                        row.is_child = true;
                                        row.is_parent = false;
                                        Self::render_privacy_row(
                                            ui,
                                            row,
                                            &privacy_current_default,
                                            &privacy_current_recommended,
                                            warehouse_icon,
                                            star_icon,
                                        );
                                        ui.add_space(8.0);
                                    }
                                }
                            } else {
                                for row in group.iter_mut() {
                                    row.is_parent = Self::privacy_has_children(row.key);
                                    if row.is_parent {
                                        row.is_expanded = Self::privacy_expand_state(row.key);
                                    }
                                    Self::render_privacy_row(
                                        ui,
                                        row,
                                        &privacy_current_default,
                                        &privacy_current_recommended,
                                        warehouse_icon,
                                        star_icon,
                                    );
                                    ui.add_space(8.0);
                                }
                            }
                            ui.add_space(8.0);
                        });
                        Self::tree_header_hover(ui, &header.header_response);
                    }
                });
        });
    }

    fn privacy_toggle_enabled_from_label(label: &str) -> bool {
        matches!(label, "On" | "Allow")
    }

    fn apply_privacy_quick_action(&mut self, action_index: usize) {
        for group in &mut self.state.privacy_security.groups {
            for row in group.iter_mut() {
                if row.input_type == 0 {
                    let target_label = match action_index {
                        1 => row.recommended_label.as_str(),
                        2 => row.default_label.as_str(),
                        _ => continue,
                    };
                    row.enabled = Self::privacy_toggle_enabled_from_label(target_label);
                } else {
                    let target_label = match action_index {
                        1 => row.recommended_label.as_str(),
                        2 => row.default_label.as_str(),
                        _ => continue,
                    };
                    if let Some(idx) = row.options.iter().position(|opt| opt == target_label) {
                        row.selected_index = idx as i32;
                    }
                }
            }
        }
    }

    fn privacy_row_matches_query(row: &GamingTweakRow, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let haystack = [
            row.key,
            row.name.as_str(),
            row.description.as_str(),
            row.default_label.as_str(),
            row.recommended_label.as_str(),
            row.warning_text.as_str(),
        ]
        .join(" ")
        .to_lowercase();
        haystack.contains(query)
            || row
                .options
                .iter()
                .any(|opt| opt.to_lowercase().contains(query))
    }

    fn privacy_group_label(idx: usize) -> &'static str {
        match idx {
            0 => "privacy_security_group_0",
            1 => "privacy_security_group_1",
            2 => "privacy_security_group_2",
            3 => "privacy_security_group_3",
            4 => "privacy_security_group_4",
            5 => "privacy_security_group_5",
            6 => "privacy_security_group_6",
            7 => "privacy_security_group_7",
            8 => "privacy_security_group_8",
            9 => "privacy_security_group_9",
            10 => "privacy_security_group_10",
            11 => "privacy_security_group_11",
            12 => "privacy_security_group_12",
            _ => "privacy_security_title",
        }
    }

    fn render_privacy_row(
        ui: &mut egui::Ui,
        row: &mut GamingTweakRow,
        current_default_prefix: &str,
        current_recommended_prefix: &str,
        warehouse_icon: Option<char>,
        star_icon: Option<char>,
    ) {
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
                                    Self::privacy_toggle_expand_state(row.key);
                                    row.is_expanded = Self::privacy_expand_state(row.key);
                                    ui.ctx().request_repaint();
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
                                    row.enabled = enabled;
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
                                    row.selected_index = selected_index as i32;
                                }
                            }
                            ui.add_space(6.0);
                            let default_icon = warehouse_icon
                                .map_or_else(String::new, |c| c.to_string());
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
                                    current_default_prefix, row.default_label
                                ))
                                .clicked()
                            {
                                if row.input_type == 0 {
                                    row.enabled =
                                        Self::privacy_toggle_enabled_from_label(&row.default_label);
                                } else {
                                    let default_idx = row
                                        .options
                                        .iter()
                                        .position(|opt| opt == &row.default_label)
                                        .unwrap_or(0);
                                    row.selected_index = default_idx as i32;
                                }
                            }
                            ui.add_space(6.0);
                            let rec_icon = star_icon
                                .map_or_else(String::new, |c| c.to_string());
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
                                    current_recommended_prefix, row.recommended_label
                                ))
                                .clicked()
                            {
                                if row.input_type == 0 {
                                    row.enabled = Self::privacy_toggle_enabled_from_label(
                                        &row.recommended_label,
                                    );
                                } else if !row.options.is_empty() {
                                    let rec_idx = row
                                        .options
                                        .iter()
                                        .position(|opt| opt == &row.recommended_label)
                                        .unwrap_or_else(|| row.options.len().saturating_sub(1));
                                    row.selected_index = rec_idx as i32;
                                }
                            }
                        },
                    );
                });
            });
    }
}
