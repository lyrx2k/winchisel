pub struct ProfileRule {
    pub id: &'static str,
    pub rec_t: i8,
    pub def_t: i8,
    pub rec_s: i16,
    pub def_s: i16,
}

pub const PROFILE_RULES: &[ProfileRule] = &[
    ProfileRule {
        id: "gaming-game-mode",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-performance-explorer-mouse-precision",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-performance-mouse-hover-time",
        rec_t: -1,
        def_t: -1,
        rec_s: 0,
        def_s: 5,
    },
    ProfileRule {
        id: "gaming-performance-autostart-delay",
        rec_t: 0,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-background-apps",
        rec_t: -1,
        def_t: -1,
        rec_s: 2,
        def_s: 0,
    },
    ProfileRule {
        id: "gaming-storage-sense",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-performance-explorer-search",
        rec_t: 0,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-performance-search-webview2",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-performance-wallpaper-compression",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-performance-explorer-menu-show-delay",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-explorer-alt-tab-filter",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-win32-priority",
        rec_t: -1,
        def_t: -1,
        rec_s: 0,
        def_s: 0,
    },
    ProfileRule {
        id: "gaming-system-responsiveness",
        rec_t: 1,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-cpu-priority",
        rec_t: 1,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-scheduling-category",
        rec_t: 1,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-performance-svchost-split-threshold",
        rec_t: -1,
        def_t: -1,
        rec_s: 0,
        def_s: 0,
    },
    ProfileRule {
        id: "gaming-gpu-priority",
        rec_t: 1,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-gpu-scheduling",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-directx-flip-model",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-directx-vrr-optimizations",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-directx-auto-hdr",
        rec_t: 0,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-nvidia-sharpening",
        rec_t: 1,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-fullscreen-optimizations",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-performance-desktop-composition",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-auto-color-management",
        rec_t: 0,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-disable-mpo",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-disable-mpo-min-fps",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-network-throttling",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-nagle-algorithm",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-dns-server",
        rec_t: -1,
        def_t: -1,
        rec_s: 0,
        def_s: 0,
    },
    ProfileRule {
        id: "gaming-virtualization-based-security",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-memory-integrity",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-xbox-game-dvr",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-game-bar-controller",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-game-bar-tips",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-performance-background-services",
        rec_t: 1,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-sysmain-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 0,
        def_s: 2,
    },
    ProfileRule {
        id: "gaming-performance-prefetch",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-windows-search-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 2,
    },
    ProfileRule {
        id: "gaming-print-spooler-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 2,
    },
    ProfileRule {
        id: "gaming-telemetry-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 2,
    },
    ProfileRule {
        id: "gaming-connected-devices-platform-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 2,
    },
    ProfileRule {
        id: "gaming-compatibility-assistant-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 2,
    },
    ProfileRule {
        id: "gaming-error-reporting-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-geolocation-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-retail-demo-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-insider-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-phone-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-wallet-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-smart-card-services",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-maps-broker-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 2,
    },
    ProfileRule {
        id: "gaming-fax-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 0,
        def_s: 0,
    },
    ProfileRule {
        id: "gaming-wmp-network-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 0,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-mixed-reality-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 0,
    },
    ProfileRule {
        id: "gaming-mobile-hotspot-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-sms-router-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-parental-controls-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-payments-nfc-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-spot-verifier-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-remote-access-manager",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-remote-access-auto",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-remote-desktop-services",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-remote-desktop-configuration",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-remote-desktop-port-redirector",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-xbox-auth-manager",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-xbox-game-save",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-xbox-networking",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-biometric-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-touch-keyboard-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 0,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-sensor-monitoring-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-sensor-data-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 1,
        def_s: 1,
    },
    ProfileRule {
        id: "gaming-ai-fabric-service",
        rec_t: -1,
        def_t: -1,
        rec_s: 0,
        def_s: 2,
    },
    ProfileRule {
        id: "gaming-task-compatibility-appraiser",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-program-data-updater",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-ceip-consolidator",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-usb-ceip",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-disk-diagnostic",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-feedback-dmclient",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-feedback-dmclient-download",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-error-reporting-queue",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-sqm",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-mare-backup",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-startup-app",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-maps-update",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-autochk-proxy",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-family-safety",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-power-efficiency",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-windows-ai",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-task-office-actions-server",
        rec_t: -1,
        def_t: -1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "visual-effects-mode",
        rec_t: -1,
        def_t: -1,
        rec_s: 3,
        def_s: 0,
    },
    ProfileRule {
        id: "ui-effects",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "window-animation",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "taskbar-animations",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "enable-peek",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "menu-animation",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "fade-tooltip",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "fade-menu-items",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "taskbar-thumbnails",
        rec_t: 0,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "mouse-shadow",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "window-shadows",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "show-thumbnails",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "translucent-selection",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "drag-full-windows",
        rec_t: 1,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "combo-box-animation",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "font-smoothing",
        rec_t: 1,
        def_t: 0,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "smooth-scroll-listboxes",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "drop-shadows",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "gaming-narrator-hotkey",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "accessibility-stickykeys-hotkey",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "accessibility-filterkeys-hotkey",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "accessibility-togglekeys-hotkey",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "accessibility-mousekeys-hotkey",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
    ProfileRule {
        id: "accessibility-highcontrast-hotkey",
        rec_t: 0,
        def_t: 1,
        rec_s: -1,
        def_s: -1,
    },
];

pub struct RawRegRule {
    pub id: &'static str,
    pub root: u8,
    pub path: &'static str,
    pub name: &'static str,
    pub kind: u8,
    pub byte_index: i32,
    pub bit_mask: u8,
    pub enabled: &'static [&'static str],
    pub disabled: &'static [&'static str],
}

pub const RAW_REG_RULES: &[RawRegRule] = &[
    RawRegRule {
        id: "gaming-game-mode",
        root: 0,
        path: r"Software\Microsoft\GameBar",
        name: "AutoGameModeEnabled",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1", "__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-performance-explorer-mouse-precision",
        root: 0,
        path: r"Control Panel\Mouse",
        name: "MouseSpeed",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-performance-autostart-delay",
        root: 0,
        path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
        name: "StartupDelayInMSec",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["10000"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-storage-sense",
        root: 0,
        path: r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
        name: "AllowStorageSenseGlobal",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1", "__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-storage-sense",
        root: 1,
        path: r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
        name: "AllowStorageSenseGlobal",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1", "__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-performance-explorer-search",
        root: 0,
        path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Search\Preferences",
        name: "WholeFileSystem",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-performance-search-webview2",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\FeatureManagement\Overrides\8\1694661260",
        name: "EnabledState",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["2"],
        disabled: &["1"],
    },
    RawRegRule {
        id: "gaming-performance-search-webview2",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\FeatureManagement\Overrides\8\1694661260",
        name: "EnabledStateOptions",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-performance-search-webview2",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\FeatureManagement\Overrides\8\1694661260",
        name: "Variant",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-performance-search-webview2",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\FeatureManagement\Overrides\8\1694661260",
        name: "VariantPayload",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-performance-search-webview2",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\FeatureManagement\Overrides\8\1694661260",
        name: "VariantPayloadKind",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-performance-wallpaper-compression",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "JPEGImportQuality",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["0", "__MISSING__"],
        disabled: &["100"],
    },
    RawRegRule {
        id: "gaming-performance-explorer-menu-show-delay",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "MenuShowDelay",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["400"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-explorer-alt-tab-filter",
        root: 0,
        path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        name: "MultiTaskingAltTabFilter",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["3"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-system-responsiveness",
        root: 1,
        path: r"Software\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        name: "SystemResponsiveness",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["10"],
        disabled: &["20"],
    },
    RawRegRule {
        id: "gaming-cpu-priority",
        root: 1,
        path: r"Software\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
        name: "Priority",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["6"],
        disabled: &["2"],
    },
    RawRegRule {
        id: "gaming-scheduling-category",
        root: 1,
        path: r"Software\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
        name: "Scheduling Category",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["High"],
        disabled: &["Medium"],
    },
    RawRegRule {
        id: "gaming-gpu-priority",
        root: 1,
        path: r"Software\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
        name: "GPU Priority",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["8"],
        disabled: &["2"],
    },
    RawRegRule {
        id: "gaming-gpu-scheduling",
        root: 1,
        path: r"System\CurrentControlSet\Control\GraphicsDrivers",
        name: "HwSchMode",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["2", "__MISSING__"],
        disabled: &["1"],
    },
    RawRegRule {
        id: "gaming-directx-flip-model",
        root: 0,
        path: r"Software\Microsoft\DirectX\UserGpuPreferences",
        name: "DirectXUserGlobalSettings",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-directx-vrr-optimizations",
        root: 0,
        path: r"Software\Microsoft\DirectX\UserGpuPreferences",
        name: "DirectXUserGlobalSettings",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-directx-auto-hdr",
        root: 0,
        path: r"Software\Microsoft\DirectX\UserGpuPreferences",
        name: "DirectXUserGlobalSettings",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-nvidia-sharpening",
        root: 1,
        path: r"Software\NVIDIA Corporation\Global\FTS",
        name: "EnableGR535",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["0"],
        disabled: &["1"],
    },
    RawRegRule {
        id: "gaming-fullscreen-optimizations",
        root: 0,
        path: r"System\GameConfigStore",
        name: "GameDVR_FSEBehaviorMode",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["0"],
        disabled: &["2"],
    },
    RawRegRule {
        id: "gaming-performance-desktop-composition",
        root: 0,
        path: r"Software\Microsoft\Windows\DWM",
        name: "CompositionPolicy",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-auto-color-management",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\MonitorDataStore",
        name: "AutoColorManagementEnabled",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-disable-mpo",
        root: 1,
        path: r"SOFTWARE\Microsoft\Windows\Dwm",
        name: "OverlayTestMode",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["5"],
    },
    RawRegRule {
        id: "gaming-disable-mpo",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        name: "DisableOverlays",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["1"],
    },
    RawRegRule {
        id: "gaming-disable-mpo-min-fps",
        root: 1,
        path: r"SOFTWARE\Microsoft\Windows\Dwm",
        name: "OverlayMinFPS",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-network-throttling",
        root: 1,
        path: r"Software\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        name: "NetworkThrottlingIndex",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["10"],
        disabled: &["-1"],
    },
    RawRegRule {
        id: "gaming-nagle-algorithm",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces",
        name: "TcpAckFrequency",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["1"],
    },
    RawRegRule {
        id: "gaming-nagle-algorithm",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces",
        name: "TCPNoDelay",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["1"],
    },
    RawRegRule {
        id: "gaming-virtualization-based-security",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\DeviceGuard",
        name: "EnableVirtualizationBasedSecurity",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-virtualization-based-security",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\DeviceGuard",
        name: "RequirePlatformSecurityFeatures",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-virtualization-based-security",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\DeviceGuard",
        name: "Locked",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-memory-integrity",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
        name: "Enabled",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-memory-integrity",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
        name: "Locked",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-memory-integrity",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
        name: "WasEnabledBy",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["2"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-xbox-game-dvr",
        root: 0,
        path: r"System\GameConfigStore",
        name: "GameDVR_Enabled",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-xbox-game-dvr",
        root: 0,
        path: r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
        name: "AppCaptureEnabled",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-xbox-game-dvr",
        root: 1,
        path: r"SOFTWARE\Policies\Microsoft\Windows\GameDVR",
        name: "AllowGameDVR",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-game-bar-controller",
        root: 0,
        path: r"Software\Microsoft\GameBar",
        name: "UseNexusForGameBarEnabled",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-game-bar-tips",
        root: 0,
        path: r"Software\Microsoft\GameBar",
        name: "ShowStartupPanel",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-performance-background-services",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control",
        name: "ServicesPipeTimeout",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["30000"],
        disabled: &["60000"],
    },
    RawRegRule {
        id: "gaming-performance-prefetch",
        root: 1,
        path: r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
        name: "EnablePrefetcher",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["3"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "ui-effects",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "UserPreferencesMask",
        kind: 2,
        byte_index: 4,
        bit_mask: 2,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "window-animation",
        root: 0,
        path: r"Control Panel\Desktop\WindowMetrics",
        name: "MinAnimate",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "taskbar-animations",
        root: 0,
        path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        name: "TaskbarAnimations",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "enable-peek",
        root: 0,
        path: r"Software\Microsoft\Windows\DWM",
        name: "EnableAeroPeek",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "menu-animation",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "UserPreferencesMask",
        kind: 2,
        byte_index: 0,
        bit_mask: 2,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "fade-tooltip",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "UserPreferencesMask",
        kind: 2,
        byte_index: 1,
        bit_mask: 8,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "fade-menu-items",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "UserPreferencesMask",
        kind: 2,
        byte_index: 1,
        bit_mask: 4,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "taskbar-thumbnails",
        root: 0,
        path: r"Software\Microsoft\Windows\DWM",
        name: "AlwaysHibernateThumbnails",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "mouse-shadow",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "UserPreferencesMask",
        kind: 2,
        byte_index: 1,
        bit_mask: 32,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "window-shadows",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "UserPreferencesMask",
        kind: 2,
        byte_index: 2,
        bit_mask: 4,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "show-thumbnails",
        root: 0,
        path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        name: "IconsOnly",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["0"],
        disabled: &["1"],
    },
    RawRegRule {
        id: "translucent-selection",
        root: 0,
        path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        name: "ListviewAlphaSelect",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "drag-full-windows",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "DragFullWindows",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "combo-box-animation",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "UserPreferencesMask",
        kind: 2,
        byte_index: 0,
        bit_mask: 4,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "font-smoothing",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "FontSmoothing",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["2"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "smooth-scroll-listboxes",
        root: 0,
        path: r"Control Panel\Desktop",
        name: "UserPreferencesMask",
        kind: 2,
        byte_index: 0,
        bit_mask: 8,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "drop-shadows",
        root: 0,
        path: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        name: "ListviewShadow",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["1"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "gaming-narrator-hotkey",
        root: 0,
        path: r"Software\Microsoft\Narrator\NoRoam",
        name: "WinEnterLaunchEnabled",
        kind: 0,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["__MISSING__"],
        disabled: &["0"],
    },
    RawRegRule {
        id: "accessibility-stickykeys-hotkey",
        root: 0,
        path: r"Control Panel\Accessibility\StickyKeys",
        name: "Flags",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["510"],
        disabled: &["2"],
    },
    RawRegRule {
        id: "accessibility-filterkeys-hotkey",
        root: 0,
        path: r"Control Panel\Accessibility\Keyboard Response",
        name: "Flags",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["126"],
        disabled: &["2"],
    },
    RawRegRule {
        id: "accessibility-togglekeys-hotkey",
        root: 0,
        path: r"Control Panel\Accessibility\ToggleKeys",
        name: "Flags",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["62"],
        disabled: &["34"],
    },
    RawRegRule {
        id: "accessibility-mousekeys-hotkey",
        root: 0,
        path: r"Control Panel\Accessibility\MouseKeys",
        name: "Flags",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["126"],
        disabled: &["130"],
    },
    RawRegRule {
        id: "accessibility-highcontrast-hotkey",
        root: 0,
        path: r"Control Panel\Accessibility\HighContrast",
        name: "Flags",
        kind: 1,
        byte_index: -1,
        bit_mask: 0,
        enabled: &["126"],
        disabled: &["4194"],
    },
];

use crate::GamingTweakRow;
use crate::{Language, i18n::t};
use std::collections::{HashMap, HashSet};
use std::os::windows::process::CommandExt;
use std::sync::{Arc, LazyLock, Mutex, OnceLock};
use winreg::HKEY;
use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE};

pub struct RawCatalogItemNative {
    pub id: &'static str,
    pub name: &'static str,
    pub group: i32,
    pub input: i32,
    pub options: &'static [&'static str],
}

pub const PERFORMANCE_CATALOG: &[RawCatalogItemNative] = &[
    RawCatalogItemNative {
        id: "gaming-game-mode",
        name: "Game Mode",
        group: 0,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-performance-explorer-mouse-precision",
        name: "Enhance Pointer Precision",
        group: 0,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-performance-mouse-hover-time",
        name: "Mouse Hover Time",
        group: 0,
        input: 1,
        options: &[
            "1ms (Instant)",
            "10ms (Very Fast)",
            "50ms (Fast)",
            "100ms (Moderate)",
            "200ms",
            "400ms (Default)",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-performance-autostart-delay",
        name: "Startup Delay for Apps",
        group: 0,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-background-apps",
        name: "Let Apps Run in Background",
        group: 0,
        input: 1,
        options: &["User in Control (Default)", "Force Allow", "Force Deny"],
    },
    RawCatalogItemNative {
        id: "gaming-storage-sense",
        name: "Storage Sense",
        group: 0,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-performance-explorer-search",
        name: "Search Entire File System",
        group: 0,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-performance-search-webview2",
        name: "WebView2 in Windows Search",
        group: 0,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-performance-wallpaper-compression",
        name: "Allow Desktop Wallpaper Compression",
        group: 0,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-performance-explorer-menu-show-delay",
        name: "Enable Menu Show Delay",
        group: 0,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-explorer-alt-tab-filter",
        name: "Alt+Tab Filter",
        group: 0,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-win32-priority",
        name: "Adjust processor for best performance of",
        group: 1,
        input: 1,
        options: &["Programs", "Background Services"],
    },
    RawCatalogItemNative {
        id: "gaming-system-responsiveness",
        name: "System Responsiveness for Games",
        group: 1,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-cpu-priority",
        name: "CPU Priority for Gaming",
        group: 1,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-scheduling-category",
        name: "High Scheduling Category for Gaming",
        group: 1,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-performance-svchost-split-threshold",
        name: "Svchost Split Threshold",
        group: 1,
        input: 1,
        options: &[
            "Default", "4 GB", "6 GB", "8 GB", "12 GB", "16 GB", "24 GB", "32 GB", "64 GB",
            "128 GB", "Custom",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-gpu-priority",
        name: "GPU Priority for Gaming",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-gpu-scheduling",
        name: "Hardware-Accelerated GPU Scheduling",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-directx-flip-model",
        name: "Optimizations for windowed games",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-directx-vrr-optimizations",
        name: "Variable Refresh Rate",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-directx-auto-hdr",
        name: "Auto HDR",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-nvidia-sharpening",
        name: "Legacy NVIDIA Sharpening",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-fullscreen-optimizations",
        name: "Fullscreen Optimizations",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-performance-desktop-composition",
        name: "Desktop Composition Effects",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-auto-color-management",
        name: "Automatically manage color for apps",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-disable-mpo",
        name: "Multi-Plane Overlay (MPO)",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-disable-mpo-min-fps",
        name: "MPO Minimum Frame Rate Requirement",
        group: 2,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-network-throttling",
        name: "Network Throttling",
        group: 3,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-nagle-algorithm",
        name: "Nagle's Algorithm",
        group: 3,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-dns-server",
        name: "DNS Server",
        group: 3,
        input: 1,
        options: &[
            "Setting_gaming-dns-server_Option_0",
            "Setting_gaming-dns-server_Option_1",
            "Setting_gaming-dns-server_Option_2",
            "Setting_gaming-dns-server_Option_3",
            "Setting_gaming-dns-server_Option_4",
            "Setting_gaming-dns-server_Option_5",
            "Setting_gaming-dns-server_Option_6",
            "Custom (User Defined)",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-virtualization-based-security",
        name: "Virtualization Based Security (VBS)",
        group: 4,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-memory-integrity",
        name: "Memory Integrity (HVCI)",
        group: 4,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-xbox-game-dvr",
        name: "Xbox Game DVR",
        group: 5,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-game-bar-controller",
        name: "Game Bar Controller Access",
        group: 5,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-game-bar-tips",
        name: "Game Bar Tips and Hints",
        group: 5,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-performance-background-services",
        name: "Optimize Background Services",
        group: 6,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-sysmain-service",
        name: "SysMain Service (Superfetch)",
        group: 6,
        input: 1,
        options: &[
            "Disabled (Recommended for SSD)",
            "Manual",
            "Automatic (Recommended for HDD)",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-performance-prefetch",
        name: "Prefetch Feature",
        group: 6,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-windows-search-service",
        name: "Windows Search Indexing Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-print-spooler-service",
        name: "Print Spooler Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-telemetry-service",
        name: "Connected User Experiences and Telemetry Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-connected-devices-platform-service",
        name: "Connected Devices Platform Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-compatibility-assistant-service",
        name: "Program Compatibility Assistant Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-error-reporting-service",
        name: "Windows Error Reporting Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-geolocation-service",
        name: "Geolocation Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-retail-demo-service",
        name: "Retail Demo Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-insider-service",
        name: "Windows Insider Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-phone-service",
        name: "Phone Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-wallet-service",
        name: "Wallet Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-smart-card-services",
        name: "Smart Card Services",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-maps-broker-service",
        name: "Downloaded Maps Manager",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-fax-service",
        name: "Fax Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_DisabledRecommended",
            "ServiceOption_Manual",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-wmp-network-service",
        name: "Windows Media Player Network Sharing Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_DisabledRecommended",
            "ServiceOption_Manual",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-mixed-reality-service",
        name: "Windows Mixed Reality OpenXR Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-mobile-hotspot-service",
        name: "Windows Mobile Hotspot Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-sms-router-service",
        name: "Microsoft Windows SMS Router Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-parental-controls-service",
        name: "Parental Controls Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-payments-nfc-service",
        name: "Payments and NFC/SE Manager",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-spot-verifier-service",
        name: "Spot Verifier Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-remote-access-manager",
        name: "Remote Access Connection Manager",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-remote-access-auto",
        name: "Remote Access Auto Connection Manager",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-remote-desktop-services",
        name: "Remote Desktop Services",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-remote-desktop-configuration",
        name: "Remote Desktop Configuration",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-remote-desktop-port-redirector",
        name: "Remote Desktop Services UserMode Port Redirector",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-xbox-auth-manager",
        name: "Xbox Live Auth Manager",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-xbox-game-save",
        name: "Xbox Live Game Save",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-xbox-networking",
        name: "Xbox Live Networking Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-biometric-service",
        name: "Windows Biometric Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-touch-keyboard-service",
        name: "Touch Keyboard and Handwriting Panel Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_DisabledRecommended",
            "ServiceOption_Manual",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-sensor-monitoring-service",
        name: "Sensor Monitoring Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-sensor-data-service",
        name: "Sensor Data Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_Disabled",
            "ServiceOption_ManualRecommended",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "gaming-ai-fabric-service",
        name: "Windows AI Fabric Service",
        group: 6,
        input: 1,
        options: &[
            "ServiceOption_DisabledRecommended",
            "ServiceOption_Manual",
            "ServiceOption_Automatic",
        ],
    },
    RawCatalogItemNative {
        id: "CompatibilityAppraiserTask",
        name: "Microsoft Compatibility Appraiser Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "ProgramDataUpdaterTask",
        name: "Program Data Updater Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "CEIPConsolidatorTask",
        name: "Customer Experience Improvement Program Consolidator",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "UsbCeipTask",
        name: "USB CEIP Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "DiskDiagnosticTask",
        name: "Disk Diagnostic Data Collector Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "FeedbackDmClientTask",
        name: "Feedback DmClient Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "FeedbackDmClientDownloadTask",
        name: "Feedback DmClient Scenario Download Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "ErrorReportingQueueTask",
        name: "Windows Error Reporting Queue Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "SqmTask",
        name: "Software Quality Metrics Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "MareBackupTask",
        name: "MAR (Malicious Software Removal) Backup Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "StartupAppTask",
        name: "Startup App Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "MapsUpdateTask",
        name: "Maps Update Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "AutochkProxyTask",
        name: "AutoChk Proxy Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "FamilySafetyTask",
        name: "Family Safety Monitor Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "PowerEfficiencyTask",
        name: "Power Efficiency Diagnostics Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "WindowsAIRecallConfig",
        name: "Windows AI Recall Configuration Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "WindowsAIRecallPipeline",
        name: "Windows AI Recall Pipeline Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "OfficeActionsServer",
        name: "Office Actions Server Task",
        group: 7,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "visual-effects-mode",
        name: "Visual Effects",
        group: 8,
        input: 1,
        options: &[
            "Let Windows choose what's best for my computer",
            "Adjust for best appearance",
            "Adjust for best performance",
            "Custom",
        ],
    },
    RawCatalogItemNative {
        id: "ui-effects",
        name: "Animate controls and elements inside windows",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "window-animation",
        name: "Animate windows when minimizing and maximizing",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "taskbar-animations",
        name: "Animations in the taskbar",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "enable-peek",
        name: "Enable Peek",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "menu-animation",
        name: "Fade or slide menus into view",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "fade-tooltip",
        name: "Fade or slide ToolTips into view",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "fade-menu-items",
        name: "Fade out menu items after clicking",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "taskbar-thumbnails",
        name: "Save taskbar thumbnail previews",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "mouse-shadow",
        name: "Show shadows under mouse pointer",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "window-shadows",
        name: "Show shadows under windows",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "show-thumbnails",
        name: "Show thumbnails instead of icons",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "translucent-selection",
        name: "Show translucent selection rectangle",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "drag-full-windows",
        name: "Show window contents while dragging",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "combo-box-animation",
        name: "Slide open combo boxes",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "font-smoothing",
        name: "Smooth edges of screen fonts",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "smooth-scroll-listboxes",
        name: "Smooth-scroll list boxes",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "drop-shadows",
        name: "Use drop shadows for icon labels on the desktop",
        group: 8,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "gaming-narrator-hotkey",
        name: "Narrator Win+Ctrl+Enter Hotkey",
        group: 9,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "accessibility-stickykeys-hotkey",
        name: "StickyKeys Hotkey (ShiftÃ—5)",
        group: 9,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "accessibility-filterkeys-hotkey",
        name: "FilterKeys Hotkey (Right Shift 8s)",
        group: 9,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "accessibility-togglekeys-hotkey",
        name: "ToggleKeys Hotkey (Num Lock 5s)",
        group: 9,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "accessibility-mousekeys-hotkey",
        name: "MouseKeys Hotkey (Alt+Shift+NumLock)",
        group: 9,
        input: 0,
        options: &[],
    },
    RawCatalogItemNative {
        id: "accessibility-highcontrast-hotkey",
        name: "High Contrast Hotkey (Alt+Shift+PrtScn)",
        group: 9,
        input: 0,
        options: &[],
    },
];

static STATE_CACHE: LazyLock<Mutex<HashMap<i32, (bool, i32)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static SCHEDULED_TASK_CACHE: OnceLock<Arc<HashMap<String, bool>>> = OnceLock::new();
static CATALOG_EN: OnceLock<Vec<CatalogItem>> = OnceLock::new();
static CATALOG_DE: OnceLock<Vec<CatalogItem>> = OnceLock::new();
static CATALOG_BY_ID_EN: OnceLock<HashMap<i32, &'static CatalogItem>> = OnceLock::new();
static CATALOG_BY_ID_DE: OnceLock<HashMap<i32, &'static CatalogItem>> = OnceLock::new();

#[derive(Debug, Clone)]
struct CatalogItem {
    num_id: i32,
    key: &'static str,
    category: i32,
    name: String,
    name_lc: String,
    desc_lc: String,
    input_type: i32,
    options: Vec<String>,
    is_new: bool,
}
static PERF_EXPANDED: LazyLock<Mutex<HashMap<&'static str, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy)]
enum DependencyRule {
    RequiresEnabled {
        dependent: &'static str,
        required: &'static str,
    },
    RequiresSelection {
        dependent: &'static str,
        required: &'static str,
        required_index: i32,
    },
}

const DEPENDENCY_RULES: &[DependencyRule] = &[
    DependencyRule::RequiresEnabled {
        dependent: "gaming-memory-integrity",
        required: "gaming-virtualization-based-security",
    },
    DependencyRule::RequiresEnabled {
        dependent: "gaming-performance-prefetch",
        required: "gaming-sysmain-service",
    },
    DependencyRule::RequiresEnabled {
        dependent: "gaming-disable-mpo-min-fps",
        required: "gaming-disable-mpo",
    },
    DependencyRule::RequiresSelection {
        dependent: "ui-effects",
        required: "visual-effects-mode",
        required_index: 3, // Custom
    },
    DependencyRule::RequiresSelection {
        dependent: "window-animation",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "taskbar-animations",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "enable-peek",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "menu-animation",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "fade-tooltip",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "fade-menu-items",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "taskbar-thumbnails",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "mouse-shadow",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "window-shadows",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "show-thumbnails",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "translucent-selection",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "drag-full-windows",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "combo-box-animation",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "font-smoothing",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "smooth-scroll-listboxes",
        required: "visual-effects-mode",
        required_index: 3,
    },
    DependencyRule::RequiresSelection {
        dependent: "drop-shadows",
        required: "visual-effects-mode",
        required_index: 3,
    },
];

fn catalog(lang: Language) -> &'static [CatalogItem] {
    let cache = match lang {
        Language::English => &CATALOG_EN,
        Language::German => &CATALOG_DE,
        Language::French => &CATALOG_EN,
        Language::Spanish => &CATALOG_EN,
        Language::Turkish => &CATALOG_EN,
        Language::Greek => &CATALOG_EN,
        Language::Dutch => &CATALOG_EN,
        Language::Portuguese => &CATALOG_EN,
        Language::Italian => &CATALOG_EN,
        Language::Polish => &CATALOG_EN,
        Language::Russian => &CATALOG_EN,
        Language::Japanese => &CATALOG_EN,
        Language::ChineseSimplified => &CATALOG_EN,
    };
    cache.get_or_init(|| {
        PERFORMANCE_CATALOG
            .iter()
            .enumerate()
            .map(|(i, r)| CatalogItem {
                num_id: 2000 + i as i32,
                key: r.id,
                category: r.group,
                name: r.name.to_string(),
                name_lc: r.name.to_ascii_lowercase(),
                desc_lc: format!(
                    "{} {}",
                    t(lang, r.id).to_ascii_lowercase(),
                    t(Language::English, r.id).to_ascii_lowercase()
                ),
                input_type: if r.input == 1 { 1 } else { 0 },
                options: if r.options.is_empty() {
                    vec!["Off".to_string(), "On".to_string()]
                } else {
                    r.options
                        .iter()
                        .map(|s| normalize_option_label(s))
                        .collect()
                },
                is_new: is_new_setting(r.id),
            })
            .collect()
    })
}

fn catalog_by_id(lang: Language) -> &'static HashMap<i32, &'static CatalogItem> {
    let cache = match lang {
        Language::English => &CATALOG_BY_ID_EN,
        Language::German => &CATALOG_BY_ID_DE,
        Language::French => &CATALOG_BY_ID_EN,
        Language::Spanish => &CATALOG_BY_ID_EN,
        Language::Turkish => &CATALOG_BY_ID_EN,
        Language::Greek => &CATALOG_BY_ID_EN,
        Language::Dutch => &CATALOG_BY_ID_EN,
        Language::Portuguese => &CATALOG_BY_ID_EN,
        Language::Italian => &CATALOG_BY_ID_EN,
        Language::Polish => &CATALOG_BY_ID_EN,
        Language::Russian => &CATALOG_BY_ID_EN,
        Language::Japanese => &CATALOG_BY_ID_EN,
        Language::ChineseSimplified => &CATALOG_BY_ID_EN,
    };
    cache.get_or_init(|| {
        let items = catalog(lang);
        items.iter().map(|item| (item.num_id, item)).collect()
    })
}

fn is_new_setting(id: &str) -> bool {
    matches!(
        id,
        "gaming-game-mode"
            | "gaming-ai-fabric-service"
            | "WindowsAIRecallConfig"
            | "OfficeActionsServer"
            | "gaming-performance-search-webview2"
            | "gaming-performance-mouse-hover-time"
            | "gaming-performance-svchost-split-threshold"
            | "gaming-directx-auto-hdr"
            | "gaming-directx-vrr-optimizations"
            | "gaming-disable-mpo"
            | "gaming-disable-mpo-min-fps"
            | "gaming-dns-server"
            | "gaming-connected-devices-platform-service"
    )
}

fn normalize_option_label(raw: &str) -> String {
    match raw {
        "ServiceOption_Disabled" => "Disabled".to_string(),
        "ServiceOption_DisabledRecommended" => "Disabled (Recommended)".to_string(),
        "ServiceOption_Manual" => "Manual".to_string(),
        "ServiceOption_ManualRecommended" => "Manual (Recommended)".to_string(),
        "ServiceOption_Automatic" => "Automatic".to_string(),
        "Setting_gaming-dns-server_Option_0" => "Automatic (System Default)".to_string(),
        "Setting_gaming-dns-server_Option_1" => "Cloudflare (1.1.1.1)".to_string(),
        "Setting_gaming-dns-server_Option_2" => "Cloudflare Malware (1.1.1.2)".to_string(),
        "Setting_gaming-dns-server_Option_3" => "Cloudflare Family (1.1.1.3)".to_string(),
        "Setting_gaming-dns-server_Option_4" => "Google (8.8.8.8)".to_string(),
        "Setting_gaming-dns-server_Option_5" => "Quad9 (9.9.9.9)".to_string(),
        "Setting_gaming-dns-server_Option_6" => "OpenDNS (208.67.222.222)".to_string(),
        _ => raw.to_string(),
    }
}

fn apply_bulk_profile(recommended: bool) {
    for item in catalog(Language::English) {
        apply_single_profile(item.num_id, recommended);
    }
}

fn apply_single_profile(id: i32, recommended: bool) {
    let Some(item) = item_by_num_id(id) else {
        return;
    };
    let rule = profile_rule(item.key);
    if item.input_type == 0 {
        let on = if recommended {
            rule.and_then(|r| {
                if r.rec_t >= 0 {
                    Some(r.rec_t == 1)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| detect_gaming_tweak(item.num_id).unwrap_or(false))
        } else {
            rule.and_then(|r| {
                if r.def_t >= 0 {
                    Some(r.def_t == 1)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| detect_gaming_tweak(item.num_id).unwrap_or(false))
        };
        apply_gaming_tweak(item.num_id, on);
    } else {
        let idx = if recommended {
            rule.and_then(|r| {
                if r.rec_s >= 0 {
                    Some(r.rec_s as i32)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| detect_selection_index(item).unwrap_or(0))
        } else {
            rule.and_then(|r| {
                if r.def_s >= 0 {
                    Some(r.def_s as i32)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| detect_selection_index(item).unwrap_or(0))
        };
        apply_gaming_selection_option(item.num_id, idx);
    }
}

fn profile_rule(id: &str) -> Option<&'static ProfileRule> {
    PROFILE_RULES.iter().find(|r| r.id == id)
}

fn item_by_num_id(id: i32) -> Option<&'static CatalogItem> {
    catalog_by_id(Language::English).get(&id).copied()
}

fn build_grouped_rows_filtered(query: &str, lang: Language) -> [Vec<GamingTweakRow>; 10] {
    let q = query.trim().to_ascii_lowercase();
    let cached = STATE_CACHE.lock().ok();
    build_grouped_rows_from(cached.as_deref(), true, Some(&q), lang)
}

fn build_grouped_rows_from(
    states: Option<&HashMap<i32, (bool, i32)>>,
    detect_missing: bool,
    query: Option<&str>,
    lang: Language,
) -> [Vec<GamingTweakRow>; 10] {
    let mut groups: [Vec<GamingTweakRow>; 10] = Default::default();
    let mut state_by_key: HashMap<&'static str, (bool, i32)> = HashMap::new();
    let query = query.filter(|q| !q.is_empty());
    let items = catalog(lang);
    let show_sysmain_warning = has_hdd_disk();

    for item in items {
        if let Some(q) = query
            && !item.name_lc.contains(q)
            && !item.desc_lc.contains(q)
        {
            continue;
        }
        let idx = item.category as usize;
        if idx >= groups.len() {
            continue;
        }
        let (enabled, mut selected) = state_for_row(item, states, detect_missing);
        if item.input_type == 0 {
            selected = if enabled { 1 } else { 0 };
        } else {
            let max = (item.options.len().saturating_sub(1)) as i32;
            selected = selected.clamp(0, max);
        }

        let (badge_recommended, badge_default, badge_custom) =
            profile_status_badges(item, enabled, selected);
        state_by_key.insert(item.key, (enabled, selected));
        let is_child = parent_for(item.key).is_some();
        let expanded = if let Ok(m) = PERF_EXPANDED.lock() {
            *m.get(item.key).unwrap_or(&true)
        } else {
            true
        };
        groups[idx].push(GamingTweakRow {
            tweak_id: item.num_id,
            category: item.category,
            key: item.key,
            name: item.name.clone(),
            description: if lang == Language::English {
                t(Language::English, item.key).to_string()
            } else {
                t(lang, item.key).to_string()
            },
            enabled,
            is_editable: true,
            is_child,
            is_parent: has_children(item.key),
            is_expanded: expanded,
            warning_text: warning_for(item.key, selected, show_sysmain_warning),
            input_type: item.input_type,
            options: item.options.clone(),
            selected_index: selected,
            recommended_label: profile_target_label(item, true, selected),
            default_label: profile_target_label(item, false, selected),
            badge_recommended,
            badge_default,
            badge_custom,
            is_new: item.is_new,
        });
    }
    apply_dependency_editability(&mut groups, &state_by_key);
    apply_parent_visibility(&mut groups);
    groups
}

fn parent_for(key: &str) -> Option<&'static str> {
    match key {
        "gaming-memory-integrity" => Some("gaming-virtualization-based-security"),
        "gaming-performance-prefetch" => Some("gaming-sysmain-service"),
        "gaming-disable-mpo-min-fps" => Some("gaming-disable-mpo"),
        _ => None,
    }
}

fn canonical_key(key: &str) -> &str {
    match key {
        "gaming-task-compatibility-appraiser" => "CompatibilityAppraiserTask",
        "gaming-task-program-data-updater" => "ProgramDataUpdaterTask",
        "gaming-task-ceip-consolidator" => "CEIPConsolidatorTask",
        "gaming-task-usb-ceip" => "UsbCeipTask",
        "gaming-task-disk-diagnostic" => "DiskDiagnosticTask",
        "gaming-task-feedback-dmclient" => "FeedbackDmClientTask",
        "gaming-task-feedback-dmclient-download" => "FeedbackDmClientDownloadTask",
        "gaming-task-error-reporting-queue" => "ErrorReportingQueueTask",
        "gaming-task-sqm" => "SqmTask",
        "gaming-task-mare-backup" => "MareBackupTask",
        "gaming-task-startup-app" => "StartupAppTask",
        "gaming-task-maps-update" => "MapsUpdateTask",
        "gaming-task-autochk-proxy" => "AutochkProxyTask",
        "gaming-task-family-safety" => "FamilySafetyTask",
        "gaming-task-power-efficiency" => "PowerEfficiencyTask",
        "gaming-task-office-actions-server" => "OfficeActionsServer",
        "gaming-task-windows-ai" => "WindowsAIRecallConfig",
        _ => key,
    }
}

fn has_children(key: &str) -> bool {
    matches!(
        key,
        "gaming-virtualization-based-security" | "gaming-sysmain-service" | "gaming-disable-mpo"
    )
}

fn apply_parent_visibility(groups: &mut [Vec<GamingTweakRow>; 10]) {
    let mut parent_state: HashMap<i32, bool> = HashMap::new();
    let mut parent_lookup: HashMap<&'static str, i32> = HashMap::new();
    for item in catalog(Language::English) {
        parent_lookup.insert(item.key, item.num_id);
    }
    for group in groups.iter() {
        for row in group {
            if row.is_parent {
                parent_state.insert(row.tweak_id, row.is_expanded);
            }
        }
    }
    for group in groups.iter_mut() {
        group.retain(|row| {
            if let Some(parent_key) = parent_for(row.key)
                && let Some(parent_id) = parent_lookup.get(parent_key)
            {
                return parent_state.get(parent_id).copied().unwrap_or(true);
            }
            true
        });
    }
}

fn warning_for(key: &str, selected: i32, show_sysmain_warning: bool) -> String {
    match key {
        "gaming-sysmain-service" if selected == 0 && show_sysmain_warning => "WARNING: Disabling SysMain on HDD systems can reduce responsiveness and slow app launches.".to_string(),
        "gaming-windows-search-service" if selected == 0 => "WARNING: Disabling Windows Search can break/slow Start, Explorer, and Outlook search.".to_string(),
        "gaming-disable-mpo-min-fps" if selected == 0 => "NOTE: This setting only matters when MPO is enabled.".to_string(),
        _ => String::new(),
    }
}

fn has_hdd_disk() -> bool {
    use sysinfo::{DiskKind, Disks};

    Disks::new_with_refreshed_list()
        .list()
        .iter()
        .any(|disk| matches!(disk.kind(), DiskKind::HDD))
}

fn apply_dependency_editability(
    groups: &mut [Vec<GamingTweakRow>; 10],
    state_by_key: &HashMap<&'static str, (bool, i32)>,
) {
    let mut blocked: HashSet<&'static str> = HashSet::new();
    for rule in DEPENDENCY_RULES {
        match *rule {
            DependencyRule::RequiresEnabled {
                dependent,
                required,
            } => {
                let req_enabled = state_by_key
                    .get(required)
                    .map(|(enabled, _)| *enabled)
                    .unwrap_or(false);
                if !req_enabled {
                    blocked.insert(dependent);
                }
            }
            DependencyRule::RequiresSelection {
                dependent,
                required,
                required_index,
            } => {
                let req_selected = state_by_key
                    .get(required)
                    .map(|(_, selected)| *selected)
                    .unwrap_or(-1);
                if req_selected != required_index {
                    blocked.insert(dependent);
                }
            }
        }
    }

    for group in groups.iter_mut() {
        for row in group.iter_mut() {
            row.is_editable = !blocked.contains(row.key);
        }
    }
}

fn state_for_row(
    item: &CatalogItem,
    states: Option<&HashMap<i32, (bool, i32)>>,
    detect_missing: bool,
) -> (bool, i32) {
    if let Some(state) = states.and_then(|m| m.get(&item.num_id).copied()) {
        return state;
    }
    if detect_missing {
        return detect_item_state(item);
    }
    default_profile_state(item)
}

fn default_profile_state(item: &CatalogItem) -> (bool, i32) {
    if item.input_type == 0 {
        let enabled = profile_target_toggle(item, false).unwrap_or(false);
        return (enabled, if enabled { 1 } else { 0 });
    }
    let max = (item.options.len().saturating_sub(1)) as i32;
    let selected = profile_target_selection(item, false)
        .unwrap_or(0)
        .clamp(0, max);
    (selected > 0, selected)
}

fn profile_target_toggle(item: &CatalogItem, recommended: bool) -> Option<bool> {
    let rule = profile_rule(item.key)?;
    if recommended {
        (rule.rec_t >= 0).then_some(rule.rec_t == 1)
    } else {
        (rule.def_t >= 0).then_some(rule.def_t == 1)
    }
}

fn profile_target_selection(item: &CatalogItem, recommended: bool) -> Option<i32> {
    let rule = profile_rule(item.key)?;
    if recommended {
        (rule.rec_s >= 0).then_some(rule.rec_s as i32)
    } else {
        (rule.def_s >= 0).then_some(rule.def_s as i32)
    }
}

fn profile_status_badges(item: &CatalogItem, enabled: bool, selected: i32) -> (bool, bool, bool) {
    if item.input_type == 0 {
        let rec = profile_target_toggle(item, true);
        let def = profile_target_toggle(item, false);
        let is_rec = rec == Some(enabled);
        let is_def = def == Some(enabled);
        let is_custom = !is_rec && !is_def;
        return (is_rec, is_def, is_custom);
    }

    let rec = profile_target_selection(item, true);
    let def = profile_target_selection(item, false);
    let is_rec = rec == Some(selected);
    let is_def = def == Some(selected);
    let is_custom = !is_rec && !is_def;
    (is_rec, is_def, is_custom)
}

fn profile_target_label(item: &CatalogItem, recommended: bool, current_selected: i32) -> String {
    let rule = profile_rule(item.key);
    if item.input_type == 0 {
        let target = if recommended {
            rule.and_then(|r| (r.rec_t >= 0).then_some(r.rec_t == 1))
        } else {
            rule.and_then(|r| (r.def_t >= 0).then_some(r.def_t == 1))
        };
        return match target {
            Some(true) => "Enabled".to_string(),
            Some(false) => "Disabled".to_string(),
            None => {
                if current_selected > 0 {
                    "Enabled".to_string()
                } else {
                    "Disabled".to_string()
                }
            }
        };
    }

    let idx = if recommended {
        rule.and_then(|r| (r.rec_s >= 0).then_some(r.rec_s as usize))
    } else {
        rule.and_then(|r| (r.def_s >= 0).then_some(r.def_s as usize))
    };
    if let Some(i) = idx
        && let Some(opt) = item.options.get(i)
    {
        return normalize_option_label(opt);
    }
    option_label_from_index(item, current_selected)
        .or_else(|| item.options.first().map(|s| normalize_option_label(s)))
        .unwrap_or_else(|| "Unknown".to_string())
}

fn option_label_from_index(item: &CatalogItem, index: i32) -> Option<String> {
    let idx = index.clamp(0, item.options.len().saturating_sub(1) as i32) as usize;
    item.options.get(idx).map(|s| normalize_option_label(s))
}

fn detect_all_states_parallel() -> HashMap<i32, (bool, i32)> {
    let scheduled_task_cache = SCHEDULED_TASK_CACHE
        .get_or_init(|| Arc::new(load_scheduled_task_states()))
        .clone();
    let items: Vec<&'static CatalogItem> = catalog(Language::English).iter().collect();
    let worker_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .clamp(2, 8)
        .min(items.len().max(1));
    let chunk_size = items.len().div_ceil(worker_count).max(1);
    let mut map = HashMap::new();
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in items.chunks(chunk_size) {
            let scheduled_task_cache = scheduled_task_cache.clone();
            handles.push(scope.spawn(move || {
                let mut partial = Vec::with_capacity(chunk.len());
                for item in chunk {
                    partial.push((
                        item.num_id,
                        detect_item_state_with_cache(item, &scheduled_task_cache),
                    ));
                }
                partial
            }));
        }

        for handle in handles {
            let partial = handle.join().unwrap_or_default();
            for (id, state) in partial {
                map.insert(id, state);
            }
        }
    });
    map
}

fn detect_item_state_with_cache(
    item: &CatalogItem,
    scheduled_task_cache: &HashMap<String, bool>,
) -> (bool, i32) {
    if item.input_type == 0 {
        let enabled =
            detect_gaming_tweak_with_task_cache_item(item, scheduled_task_cache).unwrap_or(false);
        return (enabled, if enabled { 1 } else { 0 });
    }
    let selected = detect_selection_index(item).unwrap_or(0);
    (selected > 0, selected)
}

fn detect_item_state(item: &CatalogItem) -> (bool, i32) {
    if item.input_type == 0 {
        let enabled = detect_gaming_tweak_item(item).unwrap_or(false);
        return (enabled, if enabled { 1 } else { 0 });
    }
    let selected = detect_selection_index(item).unwrap_or(0);
    (selected > 0, selected)
}

fn detect_gaming_tweak(id: i32) -> Option<bool> {
    let item = item_by_num_id(id)?;
    detect_gaming_tweak_item(item)
}

fn detect_gaming_tweak_item(item: &CatalogItem) -> Option<bool> {
    let key = canonical_key(item.key);
    if let Some(task_path) = scheduled_task_for_key(key) {
        return get_scheduled_task_enabled(task_path);
    }
    let result = match key {
        "gaming-game-mode" => dword_matches(
            HKEY_CURRENT_USER,
            r"Software\Microsoft\GameBar",
            "AutoGameModeEnabled",
            &[Some(1), None],
        ),
        "gaming-performance-explorer-mouse-precision" => string_matches(
            HKEY_CURRENT_USER,
            r"Control Panel\Mouse",
            "MouseSpeed",
            &[Some("1")],
        ),
        "gaming-performance-autostart-delay" => dword_matches(
            HKEY_CURRENT_USER,
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
            "StartupDelayInMSec",
            &[Some(10000)],
        ),
        "gaming-storage-sense" => {
            dword_matches(
                HKEY_CURRENT_USER,
                r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
                "AllowStorageSenseGlobal",
                &[Some(1), None],
            ) || dword_matches(
                HKEY_LOCAL_MACHINE,
                r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
                "AllowStorageSenseGlobal",
                &[Some(1), None],
            )
        }
        "gaming-performance-explorer-search" => dword_matches(
            HKEY_CURRENT_USER,
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Search\Preferences",
            "WholeFileSystem",
            &[Some(1)],
        ),
        "gaming-directx-flip-model" => {
            directx_user_global_settings_state("gaming-directx-flip-model").unwrap_or(false)
        }
        "gaming-directx-vrr-optimizations" => {
            directx_user_global_settings_state("gaming-directx-vrr-optimizations").unwrap_or(false)
        }
        "gaming-directx-auto-hdr" => {
            directx_user_global_settings_state("gaming-directx-auto-hdr").unwrap_or(false)
        }
        "gaming-performance-search-webview2" => dword_matches(
            HKEY_LOCAL_MACHINE,
            r"SYSTEM\CurrentControlSet\Control\FeatureManagement\Overrides\8\1694661260",
            "EnabledState",
            &[Some(2)],
        ),
        "gaming-performance-wallpaper-compression" => dword_matches(
            HKEY_CURRENT_USER,
            r"Control Panel\Desktop",
            "JPEGImportQuality",
            &[Some(0), None],
        ),
        "gaming-performance-explorer-menu-show-delay" => string_matches(
            HKEY_CURRENT_USER,
            r"Control Panel\Desktop",
            "MenuShowDelay",
            &[Some("400")],
        ),
        _ => detect_toggle_by_registry_rules(key)?,
    };
    Some(result)
}

fn detect_gaming_tweak_with_task_cache_item(
    item: &CatalogItem,
    scheduled_task_cache: &HashMap<String, bool>,
) -> Option<bool> {
    let key = canonical_key(item.key);
    if let Some(task_path) = scheduled_task_for_key(key) {
        return scheduled_task_cache.get(task_path).copied();
    }
    detect_gaming_tweak_item(item)
}

fn load_scheduled_task_states() -> HashMap<String, bool> {
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let paths: Vec<&str> = catalog(Language::English)
        .iter()
        .filter_map(|item| scheduled_task_for_key(canonical_key(item.key)))
        .collect();
    if paths.is_empty() {
        return HashMap::new();
    }

    let list = paths
        .iter()
        .map(|p| format!("'{}'", p.replace('\'', "''")))
        .collect::<Vec<_>>()
        .join(",");
    let cmd = format!(
        "$want=@({}); Get-ScheduledTask -ErrorAction SilentlyContinue | ForEach-Object {{ $full=('\\'+$_.TaskPath.Trim('\\')+'\\'+$_.TaskName).Replace('\\\\','\\'); if($want -contains $full) {{ if($_.Settings.Enabled) {{ \"$full=1\" }} else {{ \"$full=0\" }} }} }}",
        list
    );

    let out = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    let Ok(out) = out else {
        return HashMap::new();
    };

    let mut map = HashMap::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.trim().to_string(), v.trim() == "1");
        }
    }
    map
}

fn detect_toggle_by_registry_rules(setting_id: &str) -> Option<bool> {
    let rules: Vec<_> = RAW_REG_RULES
        .iter()
        .filter(|r| r.id == setting_id)
        .collect();
    if rules.is_empty() {
        return None;
    }
    for rule in rules {
        let current = match rule.kind {
            0 => read_dword(root_hkey(rule.root), rule.path, rule.name)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "__MISSING__".to_string()),
            1 => read_string(root_hkey(rule.root), rule.path, rule.name)
                .unwrap_or_else(|| "__MISSING__".to_string()),
            2 => read_binary_bit(
                root_hkey(rule.root),
                rule.path,
                rule.name,
                rule.byte_index,
                rule.bit_mask,
            )
            .map(|v| v.to_string())
            .unwrap_or_else(|| "__MISSING__".to_string()),
            _ => "__MISSING__".to_string(),
        };
        let mut matched = false;
        for ev in rule.enabled {
            if ev.eq_ignore_ascii_case(&current) {
                matched = true;
                break;
            }
        }
        if !matched {
            return Some(false);
        }
    }
    Some(true)
}

fn root_hkey(root: u8) -> HKEY {
    if root == 1 {
        HKEY_LOCAL_MACHINE
    } else {
        HKEY_CURRENT_USER
    }
}

fn read_binary_bit(
    root: HKEY,
    path: &str,
    name: &str,
    byte_index: i32,
    bit_mask: u8,
) -> Option<u8> {
    if byte_index < 0 {
        return None;
    }
    let key = open_key(root, path)?;
    let raw = key.get_raw_value(name).ok()?;
    let idx = byte_index as usize;
    if idx >= raw.bytes.len() {
        return None;
    }
    let b = raw.bytes[idx];
    Some(if b & bit_mask != 0 { 1 } else { 0 })
}

fn directx_user_global_settings_tokens() -> [(&'static str, &'static str); 3] {
    [
        ("gaming-directx-flip-model", "SwapEffectUpgradeEnablement"),
        ("gaming-directx-vrr-optimizations", "VRROptimizeEnable"),
        ("gaming-directx-auto-hdr", "AutoHDREnable"),
    ]
}

fn directx_user_global_settings_value() -> String {
    read_string(
        HKEY_CURRENT_USER,
        r"Software\Microsoft\DirectX\UserGpuPreferences",
        "DirectXUserGlobalSettings",
    )
    .unwrap_or_default()
}

fn directx_user_global_settings_state(key: &str) -> Option<bool> {
    let token = directx_user_global_settings_tokens()
        .into_iter()
        .find_map(|(id, token)| (id == key).then_some(token))?;
    let raw = directx_user_global_settings_value();
    let normalized = raw.trim();
    if normalized.is_empty() {
        return Some(false);
    }
    Some(normalized.contains(token))
}

fn set_directx_user_global_settings_flag(key: &str, enabled: bool) {
    let Some(token) = directx_user_global_settings_tokens()
        .into_iter()
        .find_map(|(id, token)| (id == key).then_some(token))
    else {
        return;
    };

    let raw = directx_user_global_settings_value();
    let mut flags: HashMap<String, String> = HashMap::new();
    for part in raw
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
    {
        if let Some((name, value)) = part.split_once('=') {
            flags.insert(name.trim().to_string(), value.trim().to_string());
        } else {
            flags.insert(part.to_string(), String::new());
        }
    }

    if enabled {
        flags.insert(token.to_string(), "1".to_string());
    } else {
        flags.remove(token);
    }

    let mut next = flags
        .into_iter()
        .map(|(name, value)| {
            if value.is_empty() {
                name
            } else {
                format!("{name}={value}")
            }
        })
        .collect::<Vec<_>>();
    next.sort_unstable();
    let value = next.join(";");
    write_string(
        HKEY_CURRENT_USER,
        r"Software\Microsoft\DirectX\UserGpuPreferences",
        "DirectXUserGlobalSettings",
        Some(&value),
    );
}

fn apply_gaming_tweak(id: i32, enabled: bool) {
    if let Some(item) = item_by_num_id(id) {
        let key = canonical_key(item.key);
        if let Some(task_path) = scheduled_task_for_key(key) {
            set_scheduled_task_enabled(task_path, enabled);
        }
        match key {
            "gaming-game-mode" => write_dword(
                HKEY_CURRENT_USER,
                r"Software\Microsoft\GameBar",
                "AutoGameModeEnabled",
                if enabled { Some(1) } else { Some(0) },
            ),
            "gaming-performance-explorer-mouse-precision" => write_string(
                HKEY_CURRENT_USER,
                r"Control Panel\Mouse",
                "MouseSpeed",
                if enabled { Some("1") } else { Some("0") },
            ),
            "gaming-performance-autostart-delay" => write_dword(
                HKEY_CURRENT_USER,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
                "StartupDelayInMSec",
                if enabled { Some(10000) } else { Some(0) },
            ),
            "gaming-storage-sense" => {
                let value = if enabled { Some(1) } else { Some(0) };
                write_dword(
                    HKEY_CURRENT_USER,
                    r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
                    "AllowStorageSenseGlobal",
                    value,
                );
                write_dword(
                    HKEY_LOCAL_MACHINE,
                    r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
                    "AllowStorageSenseGlobal",
                    value,
                );
            }
            "gaming-performance-explorer-search" => write_dword(
                HKEY_CURRENT_USER,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Search\Preferences",
                "WholeFileSystem",
                if enabled { Some(1) } else { Some(0) },
            ),
            "gaming-directx-flip-model"
            | "gaming-directx-vrr-optimizations"
            | "gaming-directx-auto-hdr" => set_directx_user_global_settings_flag(key, enabled),
            "gaming-performance-search-webview2" => {
                let path =
                    r"SYSTEM\CurrentControlSet\Control\FeatureManagement\Overrides\8\1694661260";
                if enabled {
                    write_dword(HKEY_LOCAL_MACHINE, path, "EnabledState", Some(2));
                    write_dword(HKEY_LOCAL_MACHINE, path, "EnabledStateOptions", None);
                    write_dword(HKEY_LOCAL_MACHINE, path, "Variant", None);
                    write_dword(HKEY_LOCAL_MACHINE, path, "VariantPayload", None);
                    write_dword(HKEY_LOCAL_MACHINE, path, "VariantPayloadKind", None);
                } else {
                    write_dword(HKEY_LOCAL_MACHINE, path, "EnabledState", Some(1));
                    write_dword(HKEY_LOCAL_MACHINE, path, "EnabledStateOptions", Some(0));
                    write_dword(HKEY_LOCAL_MACHINE, path, "Variant", Some(0));
                    write_dword(HKEY_LOCAL_MACHINE, path, "VariantPayload", Some(0));
                    write_dword(HKEY_LOCAL_MACHINE, path, "VariantPayloadKind", Some(0));
                }
            }
            "gaming-performance-wallpaper-compression" => write_dword(
                HKEY_CURRENT_USER,
                r"Control Panel\Desktop",
                "JPEGImportQuality",
                if enabled { Some(0) } else { Some(100) },
            ),
            "gaming-performance-explorer-menu-show-delay" => write_string(
                HKEY_CURRENT_USER,
                r"Control Panel\Desktop",
                "MenuShowDelay",
                if enabled { Some("400") } else { Some("0") },
            ),
            _ => apply_toggle_by_registry_rules(item.key, enabled),
        }
    }

    if let Ok(mut m) = STATE_CACHE.lock() {
        let selected = if enabled { 1 } else { 0 };
        m.insert(id, (enabled, selected));
    }
}

fn scheduled_task_for_key(key: &str) -> Option<&'static str> {
    match canonical_key(key) {
        "CompatibilityAppraiserTask" => {
            Some(r"\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser")
        }
        "ProgramDataUpdaterTask" => {
            Some(r"\Microsoft\Windows\Application Experience\ProgramDataUpdater")
        }
        "CEIPConsolidatorTask" => {
            Some(r"\Microsoft\Windows\Customer Experience Improvement Program\Consolidator")
        }
        "UsbCeipTask" => {
            Some(r"\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip")
        }
        "DiskDiagnosticTask" => {
            Some(r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector")
        }
        "FeedbackDmClientTask" => Some(r"\Microsoft\Windows\Feedback\Siuf\DmClient"),
        "FeedbackDmClientDownloadTask" => {
            Some(r"\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload")
        }
        "ErrorReportingQueueTask" => {
            Some(r"\Microsoft\Windows\Windows Error Reporting\QueueReporting")
        }
        "SqmTask" => Some(r"\Microsoft\Windows\PI\Sqm-Tasks"),
        "MareBackupTask" => Some(r"\Microsoft\Windows\Application Experience\MareBackup"),
        "StartupAppTask" => Some(r"\Microsoft\Windows\Application Experience\StartupAppTask"),
        "MapsUpdateTask" => Some(r"\Microsoft\Windows\Maps\MapsUpdateTask"),
        "AutochkProxyTask" => Some(r"\Microsoft\Windows\Autochk\Proxy"),
        "FamilySafetyTask" => Some(r"\Microsoft\Windows\Shell\FamilySafetyMonitor"),
        "PowerEfficiencyTask" => {
            Some(r"\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem")
        }
        "WindowsAIRecallConfig" => Some(r"\Microsoft\Windows\WindowsAI\RecallConfiguration"),
        "WindowsAIRecallPipeline" => Some(r"\Microsoft\Windows\WindowsAI\RecallPipeline"),
        "OfficeActionsServer" => Some(r"\Microsoft\Office\Office Actions Server"),
        _ => None,
    }
}

fn split_task_path(full: &str) -> Option<(String, String)> {
    let trimmed = full.trim_matches('\\');
    let mut parts: Vec<&str> = trimmed.split('\\').collect();
    if parts.is_empty() {
        return None;
    }
    let name = parts.pop()?.to_string();
    let path = if parts.is_empty() {
        "\\".to_string()
    } else {
        format!(r"\{}\{}", parts.join(r"\"), "")
    };
    Some((path, name))
}

fn get_scheduled_task_enabled(full_path: &str) -> Option<bool> {
    let (task_path, task_name) = split_task_path(full_path)?;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let cmd = format!(
        "$t=Get-ScheduledTask -TaskPath '{}' -TaskName '{}' -ErrorAction SilentlyContinue; if($null -eq $t){{''}} else {{ if($item.Settings.Enabled){{'1'}} else {{'0'}} }}",
        task_path.replace('\'', "''"),
        task_name.replace('\'', "''")
    );
    let out = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    match s.as_str() {
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}

fn set_scheduled_task_enabled(full_path: &str, enabled: bool) {
    let (task_path, task_name) = match split_task_path(full_path) {
        Some(v) => v,
        None => return,
    };
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let cmd = if enabled {
        format!(
            "Enable-ScheduledTask -TaskPath '{}' -TaskName '{}' -ErrorAction SilentlyContinue",
            task_path.replace('\'', "''"),
            task_name.replace('\'', "''")
        )
    } else {
        format!(
            "Disable-ScheduledTask -TaskPath '{}' -TaskName '{}' -ErrorAction SilentlyContinue",
            task_path.replace('\'', "''"),
            task_name.replace('\'', "''")
        )
    };
    let _ = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
}

fn apply_gaming_selection_option(id: i32, option_index: i32) {
    if let Some(item) = item_by_num_id(id) {
        match item.key {
            "gaming-performance-mouse-hover-time" => {
                if let Some(value) = item
                    .options
                    .get(option_index as usize)
                    .and_then(|s| s.split('m').next())
                    .and_then(|n| n.trim().parse::<u32>().ok())
                {
                    write_string(
                        HKEY_CURRENT_USER,
                        r"Control Panel\Mouse",
                        "MouseHoverTime",
                        Some(&value.to_string()),
                    );
                }
            }
            "gaming-background-apps" => {
                let val = match option_index {
                    1 => Some(1),
                    2 => Some(2),
                    _ => None,
                };
                write_dword(
                    HKEY_CURRENT_USER,
                    r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
                    "LetAppsRunInBackground",
                    val,
                );
                write_dword(
                    HKEY_LOCAL_MACHINE,
                    r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
                    "LetAppsRunInBackground",
                    val,
                );
            }
            "gaming-win32-priority" => {
                let val = if option_index == 0 { 38 } else { 24 };
                write_dword(
                    HKEY_LOCAL_MACHINE,
                    r"SYSTEM\CurrentControlSet\Control\PriorityControl",
                    "Win32PrioritySeparation",
                    Some(val),
                );
            }
            "gaming-performance-svchost-split-threshold" => {
                let values: [u32; 10] = [
                    380000, 327680, 491520, 655360, 983040, 1310720, 1966080, 2621440, 5242880,
                    10485760,
                ];
                if let Some(v) = values.get(option_index as usize) {
                    write_dword(
                        HKEY_LOCAL_MACHINE,
                        r"SYSTEM\CurrentControlSet\Control",
                        "SvcHostSplitThresholdInKB",
                        Some(*v),
                    );
                }
            }
            "visual-effects-mode" => {
                let v = match option_index {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    _ => 3,
                };
                write_dword(
                    HKEY_CURRENT_USER,
                    r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects",
                    "VisualFXSetting",
                    Some(v),
                );
            }
            "gaming-dns-server" => apply_dns_option(option_index),
            "gaming-sysmain-service" => {
                apply_service_start_by_option("SysMain", item, option_index)
            }
            "gaming-windows-search-service" => {
                apply_service_start_by_option("WSearch", item, option_index)
            }
            "gaming-print-spooler-service" => {
                apply_service_start_by_option("Spooler", item, option_index)
            }
            "gaming-telemetry-service" => {
                apply_service_start_by_option("DiagTrack", item, option_index)
            }
            "gaming-connected-devices-platform-service" => {
                apply_service_start_by_option("CDPSvc", item, option_index)
            }
            "gaming-compatibility-assistant-service" => {
                apply_service_start_by_option("PcaSvc", item, option_index)
            }
            "gaming-error-reporting-service" => {
                apply_service_start_by_option("WerSvc", item, option_index)
            }
            "gaming-geolocation-service" => {
                apply_service_start_by_option("lfsvc", item, option_index)
            }
            "gaming-retail-demo-service" => {
                apply_service_start_by_option("RetailDemo", item, option_index)
            }
            "gaming-insider-service" => apply_service_start_by_option("wisvc", item, option_index),
            "gaming-phone-service" => apply_service_start_by_option("PhoneSvc", item, option_index),
            "gaming-wallet-service" => {
                apply_service_start_by_option("WalletService", item, option_index)
            }
            "gaming-smart-card-services" => {
                apply_service_start_by_option("SCardSvr", item, option_index)
            }
            "gaming-maps-broker-service" => {
                apply_service_start_by_option("MapsBroker", item, option_index)
            }
            "gaming-fax-service" => apply_service_start_by_option("Fax", item, option_index),
            "gaming-wmp-network-service" => {
                apply_service_start_by_option("WMPNetworkSvc", item, option_index)
            }
            "gaming-mixed-reality-service" => {
                apply_service_start_by_option("MixedRealityOpenXRSvc", item, option_index)
            }
            "gaming-mobile-hotspot-service" => {
                apply_service_start_by_option("icssvc", item, option_index)
            }
            "gaming-sms-router-service" => {
                apply_service_start_by_option("SmsRouter", item, option_index)
            }
            "gaming-parental-controls-service" => {
                apply_service_start_by_option("WpcMonSvc", item, option_index)
            }
            "gaming-payments-nfc-service" => {
                apply_service_start_by_option("SEMgrSvc", item, option_index)
            }
            "gaming-spot-verifier-service" => {
                apply_service_start_by_option("svsvc", item, option_index)
            }
            "gaming-remote-access-manager" => {
                apply_service_start_by_option("RasMan", item, option_index)
            }
            "gaming-remote-access-auto" => {
                apply_service_start_by_option("RasAuto", item, option_index)
            }
            "gaming-remote-desktop-services" => {
                apply_service_start_by_option("TermService", item, option_index)
            }
            "gaming-remote-desktop-configuration" => {
                apply_service_start_by_option("SessionEnv", item, option_index)
            }
            "gaming-remote-desktop-port-redirector" => {
                apply_service_start_by_option("UmRdpService", item, option_index)
            }
            "gaming-xbox-auth-manager" => {
                apply_service_start_by_option("XblAuthManager", item, option_index)
            }
            "gaming-xbox-game-save" => {
                apply_service_start_by_option("XblGameSave", item, option_index)
            }
            "gaming-xbox-networking" => {
                apply_service_start_by_option("XboxNetApiSvc", item, option_index)
            }
            "gaming-biometric-service" => {
                apply_service_start_by_option("WbioSrvc", item, option_index)
            }
            "gaming-touch-keyboard-service" => {
                apply_service_start_by_option("TabletInputService", item, option_index)
            }
            "gaming-sensor-monitoring-service" => {
                apply_service_start_by_option("SensrSvc", item, option_index)
            }
            "gaming-sensor-data-service" => {
                apply_service_start_by_option("SensorDataService", item, option_index)
            }
            "gaming-ai-fabric-service" => {
                apply_service_start_by_option("AIFabricSvc", item, option_index)
            }
            _ => {}
        }
    }

    if let Ok(mut m) = STATE_CACHE.lock() {
        let enabled = option_index > 0;
        m.insert(id, (enabled, option_index));
    }
}

fn apply_toggle_by_registry_rules(setting_id: &str, enabled: bool) {
    let rules: Vec<_> = RAW_REG_RULES
        .iter()
        .filter(|r| r.id == setting_id)
        .collect();
    if rules.is_empty() {
        return;
    }
    for rule in rules {
        let source = if enabled { rule.enabled } else { rule.disabled };
        let Some(token) = source.first() else {
            continue;
        };
        apply_rule_value(rule, token);
    }
}

fn apply_rule_value(rule: &RawRegRule, token: &str) {
    let root = root_hkey(rule.root);
    if token == "__MISSING__" {
        match rule.kind {
            0 | 2 => write_dword(root, rule.path, rule.name, None),
            1 => write_string(root, rule.path, rule.name, None),
            _ => {}
        }
        return;
    }

    match rule.kind {
        0 => {
            if let Ok(v) = token.parse::<u32>() {
                write_dword(root, rule.path, rule.name, Some(v));
            }
        }
        1 => write_string(root, rule.path, rule.name, Some(token)),
        2 => {
            if let Ok(v) = token.parse::<u8>() {
                write_binary_bit(
                    root,
                    rule.path,
                    rule.name,
                    rule.byte_index,
                    rule.bit_mask,
                    v != 0,
                );
            }
        }
        _ => {}
    }
}

fn detect_selection_index(item: &CatalogItem) -> Option<i32> {
    match item.key {
        "gaming-performance-mouse-hover-time" => {
            let v = read_string(HKEY_CURRENT_USER, r"Control Panel\Mouse", "MouseHoverTime")?;
            let idx = match v.trim() {
                "1" => 0,
                "10" => 1,
                "50" => 2,
                "100" => 3,
                "200" => 4,
                _ => 5,
            };
            Some(idx)
        }
        "gaming-background-apps" => {
            let v = read_dword(
                HKEY_CURRENT_USER,
                r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
                "LetAppsRunInBackground",
            )
            .or_else(|| {
                read_dword(
                    HKEY_LOCAL_MACHINE,
                    r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
                    "LetAppsRunInBackground",
                )
            });
            Some(match v {
                Some(1) => 1,
                Some(2) => 2,
                _ => 0,
            })
        }
        "gaming-win32-priority" => {
            let v = read_dword(
                HKEY_LOCAL_MACHINE,
                r"SYSTEM\CurrentControlSet\Control\PriorityControl",
                "Win32PrioritySeparation",
            )
            .unwrap_or(38);
            Some(if v == 24 { 1 } else { 0 })
        }
        "gaming-performance-svchost-split-threshold" => {
            let v = read_dword(
                HKEY_LOCAL_MACHINE,
                r"SYSTEM\CurrentControlSet\Control",
                "SvcHostSplitThresholdInKB",
            )
            .unwrap_or(380000);
            let idx = match v {
                327680 => 1,
                491520 => 2,
                655360 => 3,
                983040 => 4,
                1310720 => 5,
                1966080 => 6,
                2621440 => 7,
                5242880 => 8,
                10485760 => 9,
                _ => 0,
            };
            Some(idx)
        }
        "visual-effects-mode" => {
            let v = read_dword(
                HKEY_CURRENT_USER,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects",
                "VisualFXSetting",
            )
            .unwrap_or(0);
            Some((v.min(3)) as i32)
        }
        "gaming-dns-server" => detect_dns_option_index(),
        _ => detect_service_selection_index(item),
    }
}

fn detect_service_selection_index(item: &CatalogItem) -> Option<i32> {
    let svc = service_name_for_key(item.key)?;
    let current = read_dword(
        HKEY_LOCAL_MACHINE,
        &format!(r"SYSTEM\CurrentControlSet\Services\{}", svc),
        "Start",
    )?;
    for (i, _) in item.options.iter().enumerate() {
        if let Some(expected) = service_start_from_option_index(item.key, i as i32)
            && current == expected
        {
            return Some(i as i32);
        }
    }
    Some(0)
}

fn apply_service_start_by_option(service_name: &str, item: &CatalogItem, option_index: i32) {
    if let Some(start) = service_start_from_option_index(item.key, option_index) {
        write_dword(
            HKEY_LOCAL_MACHINE,
            &format!(r"SYSTEM\CurrentControlSet\Services\{}", service_name),
            "Start",
            Some(start),
        );
    }
}

fn service_start_from_option_index(key: &str, option_index: i32) -> Option<u32> {
    match key {
        "gaming-sysmain-service"
        | "gaming-windows-search-service"
        | "gaming-print-spooler-service"
        | "gaming-telemetry-service"
        | "gaming-connected-devices-platform-service"
        | "gaming-compatibility-assistant-service"
        | "gaming-error-reporting-service"
        | "gaming-geolocation-service"
        | "gaming-retail-demo-service"
        | "gaming-insider-service"
        | "gaming-phone-service"
        | "gaming-wallet-service"
        | "gaming-smart-card-services"
        | "gaming-bitlocker-service"
        | "gaming-biometric-service"
        | "gaming-touch-keyboard-service" => match option_index {
            0 => Some(4),
            1 => Some(3),
            2 => Some(2),
            _ => None,
        },
        _ => None,
    }
}

fn service_name_for_key(key: &str) -> Option<&'static str> {
    match key {
        "gaming-sysmain-service" => Some("SysMain"),
        "gaming-windows-search-service" => Some("WSearch"),
        "gaming-print-spooler-service" => Some("Spooler"),
        "gaming-telemetry-service" => Some("DiagTrack"),
        "gaming-connected-devices-platform-service" => Some("CDPSvc"),
        "gaming-compatibility-assistant-service" => Some("PcaSvc"),
        "gaming-error-reporting-service" => Some("WerSvc"),
        "gaming-geolocation-service" => Some("lfsvc"),
        "gaming-retail-demo-service" => Some("RetailDemo"),
        "gaming-insider-service" => Some("wisvc"),
        "gaming-phone-service" => Some("PhoneSvc"),
        "gaming-wallet-service" => Some("WalletService"),
        "gaming-smart-card-services" => Some("SCardSvr"),
        "gaming-maps-broker-service" => Some("MapsBroker"),
        "gaming-fax-service" => Some("Fax"),
        "gaming-wmp-network-service" => Some("WMPNetworkSvc"),
        "gaming-mixed-reality-service" => Some("MixedRealityOpenXRSvc"),
        "gaming-mobile-hotspot-service" => Some("icssvc"),
        "gaming-sms-router-service" => Some("SmsRouter"),
        "gaming-parental-controls-service" => Some("WpcMonSvc"),
        "gaming-payments-nfc-service" => Some("SEMgrSvc"),
        "gaming-spot-verifier-service" => Some("svsvc"),
        "gaming-remote-access-manager" => Some("RasMan"),
        "gaming-remote-access-auto" => Some("RasAuto"),
        "gaming-remote-desktop-services" => Some("TermService"),
        "gaming-remote-desktop-configuration" => Some("SessionEnv"),
        "gaming-remote-desktop-port-redirector" => Some("UmRdpService"),
        "gaming-xbox-auth-manager" => Some("XblAuthManager"),
        "gaming-xbox-game-save" => Some("XblGameSave"),
        "gaming-xbox-networking" => Some("XboxNetApiSvc"),
        "gaming-biometric-service" => Some("WbioSrvc"),
        "gaming-touch-keyboard-service" => Some("TabletInputService"),
        "gaming-sensor-monitoring-service" => Some("SensrSvc"),
        "gaming-sensor-data-service" => Some("SensorDataService"),
        "gaming-ai-fabric-service" => Some("AIFabricSvc"),
        _ => None,
    }
}

fn detect_dns_option_index() -> Option<i32> {
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let cmd = "(Get-DnsClientServerAddress -AddressFamily IPv4 | Where-Object { $_.ServerAddresses -and $_.ServerAddresses.Count -gt 0 } | Select-Object -First 1 -ExpandProperty ServerAddresses) -join ','";
    let out = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().replace(' ', "");
    if s.is_empty() {
        return Some(0);
    }
    let idx = match s.as_str() {
        "1.1.1.1,1.0.0.1" => 1,
        "1.1.1.2,1.0.0.2" => 2,
        "1.1.1.3,1.0.0.3" => 3,
        "8.8.8.8,8.8.4.4" => 4,
        "9.9.9.9,149.112.112.112" => 5,
        "208.67.222.222,208.67.220.220" => 6,
        _ => 0,
    };
    Some(idx)
}

fn apply_dns_option(option_index: i32) {
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let cmd = match option_index {
        1 => {
            "Get-NetAdapter | ForEach-Object { Set-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex -ServerAddresses @('1.1.1.1','1.0.0.1') }"
        }
        2 => {
            "Get-NetAdapter | ForEach-Object { Set-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex -ServerAddresses @('1.1.1.2','1.0.0.2') }"
        }
        3 => {
            "Get-NetAdapter | ForEach-Object { Set-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex -ServerAddresses @('1.1.1.3','1.0.0.3') }"
        }
        4 => {
            "Get-NetAdapter | ForEach-Object { Set-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex -ServerAddresses @('8.8.8.8','8.8.4.4') }"
        }
        5 => {
            "Get-NetAdapter | ForEach-Object { Set-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex -ServerAddresses @('9.9.9.9','149.112.112.112') }"
        }
        6 => {
            "Get-NetAdapter | ForEach-Object { Set-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex -ServerAddresses @('208.67.222.222','208.67.220.220') }"
        }
        _ => {
            "Get-NetAdapter | ForEach-Object { Set-DnsClientServerAddress -InterfaceIndex $_.InterfaceIndex -ResetServerAddresses }"
        }
    };
    let _ = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
}

pub fn reload_gaming_tweaks_filtered(query: &str, lang: Language) -> [Vec<GamingTweakRow>; 10] {
    let has_cache = STATE_CACHE.lock().ok().is_some_and(|m| !m.is_empty());
    if !has_cache {
        let states = detect_all_states_parallel();
        if let Ok(mut m) = STATE_CACHE.lock() {
            *m = states;
        }
        return build_grouped_rows_filtered(query, lang);
    }
    let q = query.trim().to_ascii_lowercase();
    let cached = STATE_CACHE.lock().ok();
    build_grouped_rows_from(cached.as_deref(), false, Some(&q), lang)
}

pub fn preview_gaming_tweaks(query: &str, lang: Language) -> [Vec<GamingTweakRow>; 10] {
    let q = query.trim().to_ascii_lowercase();
    build_grouped_rows_from(None, false, Some(&q), lang)
}

pub fn toggle_gaming_tweak_state(id: i32, enabled: bool) {
    apply_gaming_tweak(id, enabled);
}

pub fn select_gaming_tweak_option_state(id: i32, option_index: i32) {
    apply_gaming_selection_option(id, option_index);
}

pub fn apply_gaming_recommended_state() {
    apply_bulk_profile(true);
}

pub fn apply_gaming_defaults_state() {
    apply_bulk_profile(false);
}

pub fn apply_gaming_tweak_recommended_state(id: i32) {
    apply_single_profile(id, true);
}

pub fn apply_gaming_tweak_default_state(id: i32) {
    apply_single_profile(id, false);
}

pub fn toggle_gaming_expand_state(id: i32) {
    if let Some(item) = item_by_num_id(id)
        && has_children(item.key)
        && let Ok(mut m) = PERF_EXPANDED.lock()
    {
        let current = *m.get(item.key).unwrap_or(&true);
        m.insert(item.key, !current);
    }
}

fn open_key(root: HKEY, path: &str) -> Option<RegKey> {
    let predef = RegKey::predef(root);
    predef
        .open_subkey_with_flags(path, KEY_READ | KEY_WRITE)
        .ok()
}

fn create_or_open_key(root: HKEY, path: &str) -> Option<RegKey> {
    let predef = RegKey::predef(root);
    predef.create_subkey(path).ok().map(|(key, _)| key)
}

fn read_dword(root: HKEY, path: &str, name: &str) -> Option<u32> {
    let key = open_key(root, path)?;
    key.get_value(name).ok()
}

fn read_string(root: HKEY, path: &str, name: &str) -> Option<String> {
    let key = open_key(root, path)?;
    key.get_value(name).ok()
}

fn write_dword(root: HKEY, path: &str, name: &str, value: Option<u32>) {
    if let Some(v) = value {
        if let Some(key) = create_or_open_key(root, path) {
            let _ = key.set_value(name, &v);
        }
    } else if let Some(key) = open_key(root, path) {
        let _ = key.delete_value(name);
    }
}

fn write_string(root: HKEY, path: &str, name: &str, value: Option<&str>) {
    if let Some(v) = value {
        if let Some(key) = create_or_open_key(root, path) {
            let _ = key.set_value(name, &v);
        }
    } else if let Some(key) = open_key(root, path) {
        let _ = key.delete_value(name);
    }
}

fn write_binary_bit(root: HKEY, path: &str, name: &str, byte_index: i32, bit_mask: u8, set: bool) {
    if byte_index < 0 {
        return;
    }
    let idx = byte_index as usize;
    let mut bytes: Vec<u8> = Vec::new();
    if let Some(key) = open_key(root, path)
        && let Ok(raw) = key.get_raw_value(name)
    {
        bytes = raw.bytes.to_vec();
    }
    if bytes.len() <= idx {
        bytes.resize(idx + 1, 0);
    }
    if set {
        bytes[idx] |= bit_mask;
    } else {
        bytes[idx] &= !bit_mask;
    }
    if let Some(key) = create_or_open_key(root, path) {
        let _ = key.set_raw_value(
            name,
            &winreg::RegValue {
                vtype: winreg::enums::RegType::REG_BINARY,
                bytes: bytes.into(),
            },
        );
    }
}

fn dword_matches(root: HKEY, path: &str, name: &str, values: &[Option<u32>]) -> bool {
    let current = read_dword(root, path, name);
    values.contains(&current)
}

fn string_matches(root: HKEY, path: &str, name: &str, values: &[Option<&str>]) -> bool {
    let current = read_string(root, path, name);
    values.iter().any(|v| match (v, &current) {
        (None, None) => true,
        (Some(a), Some(b)) => a.eq_ignore_ascii_case(b),
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_service_option_labels_match_expected_text() {
        assert_eq!(
            normalize_option_label("ServiceOption_DisabledRecommended"),
            "Disabled (Recommended)"
        );
        assert_eq!(
            normalize_option_label("ServiceOption_ManualRecommended"),
            "Manual (Recommended)"
        );
        assert_eq!(normalize_option_label("ServiceOption_Manual"), "Manual");
    }

    #[test]
    fn selection_label_falls_back_to_current_option_text() {
        let item = CatalogItem {
            num_id: 0,
            key: "custom-selection",
            name: String::new(),
            desc_lc: String::new(),
            category: 0,
            input_type: 1,
            options: vec![
                "User in Control (Default)".to_string(),
                "Force Allow".to_string(),
                "Force Deny".to_string(),
            ],
            is_new: false,
            name_lc: String::new(),
        };

        assert_eq!(
            option_label_from_index(&item, 2).as_deref(),
            Some("Force Deny")
        );
    }

    #[test]
    fn profile_target_label_uses_current_option_as_fallback() {
        let item = CatalogItem {
            num_id: 0,
            key: "gaming-background-apps",
            name: String::new(),
            desc_lc: String::new(),
            category: 0,
            input_type: 1,
            options: vec![
                "User in Control (Default)".to_string(),
                "Force Allow".to_string(),
                "Force Deny".to_string(),
            ],
            is_new: false,
            name_lc: String::new(),
        };

        assert_eq!(profile_target_label(&item, true, 2), "Force Deny");
        assert_eq!(
            profile_target_label(&item, false, 0),
            "User in Control (Default)"
        );
    }

    #[test]
    fn service_special_case_labels_match_expected_upstream_style() {
        assert_eq!(
            normalize_option_label("ServiceOption_DisabledRecommended"),
            "Disabled (Recommended)"
        );
        assert_eq!(
            normalize_option_label("ServiceOption_ManualRecommended"),
            "Manual (Recommended)"
        );
    }
}
