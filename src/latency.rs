use std::collections::HashMap;
use crate::app_definitions::current_language;
use crate::i18n::t;
use winreg::RegKey;
use winreg::enums::*;

// ─── Color type constants (match PS script palette) ──────────────────────────
pub const COL_NORMAL: i32 = 0; // #e2e2e2 — white/text
pub const COL_MINT: i32 = 1; // #00ff87 — CHIP 0 / best
pub const COL_ORANGE: i32 = 2; // #ffb347 — CHIP 1 / ok
pub const COL_CORAL: i32 = 3; // #ff6b6b — CHIP 2+ / slow
pub const COL_SKY: i32 = 4; // #87ceeb — section headers
pub const COL_DIM: i32 = 5; // #6c6c6c — dimmed info
pub const COL_BORDER: i32 = 6; // #4a4a4a — separators

#[derive(Clone)]
pub struct StyledLine {
    pub text: String,
    pub color: i32,
    pub bold: bool,
}

impl StyledLine {
    pub fn new(text: impl Into<String>, color: i32, bold: bool) -> Self {
        Self {
            text: text.into(),
            color,
            bold,
        }
    }
    pub fn dim(text: impl Into<String>) -> Self {
        Self::new(text, COL_DIM, false)
    }
    pub fn border(text: impl Into<String>) -> Self {
        Self::new(text, COL_BORDER, false)
    }
    pub fn empty() -> Self {
        Self::new("", COL_NORMAL, false)
    }
}

pub struct LatencyAnalysis {
    pub styled_lines: Vec<StyledLine>,
}

// ─── Device database ─────────────────────────────────────────────────────────
// (vid, did, chip_level, name, platform, usb_version)
type DbEntry = (
    &'static str,
    &'static str,
    i32,
    &'static str,
    &'static str,
    &'static str,
);

const INTEL_DB: &[DbEntry] = &[
    (
        "8086",
        "8a13",
        0,
        "Ice Lake Thunderbolt 3 USB",
        "Ice Lake (10th Gen)",
        "USB 3.2/TB3",
    ),
    (
        "8086",
        "9a13",
        0,
        "Tiger Lake-LP Thunderbolt 4 USB",
        "Tiger Lake (11th Gen)",
        "USB4/TB4",
    ),
    (
        "8086",
        "9a17",
        0,
        "Tiger Lake-H Thunderbolt 4 USB",
        "Tiger Lake-H (11th Gen)",
        "USB4/TB4",
    ),
    (
        "8086",
        "461e",
        0,
        "Alder Lake-P Thunderbolt 4 USB",
        "Alder Lake (12th Gen)",
        "USB4/TB4",
    ),
    (
        "8086",
        "464e",
        0,
        "Alder Lake-N USB 3.2 xHCI",
        "Alder Lake-N",
        "USB 3.2",
    ),
    (
        "8086",
        "a71e",
        0,
        "Raptor Lake-P Thunderbolt 4 USB",
        "Raptor Lake (13th Gen)",
        "USB4/TB4",
    ),
    (
        "8086",
        "7ec0",
        0,
        "Meteor Lake-P Thunderbolt 4 USB",
        "Meteor Lake (Core Ultra)",
        "USB4/TB4",
    ),
    (
        "8086",
        "a831",
        0,
        "Lunar Lake-M Thunderbolt 4 USB",
        "Lunar Lake",
        "USB4/TB4",
    ),
    (
        "8086",
        "5782",
        0,
        "JHL9580 Thunderbolt 5 USB",
        "Barlow Ridge 80G",
        "USB4/TB5",
    ),
    (
        "8086",
        "5785",
        0,
        "JHL9540 Thunderbolt 4 USB",
        "Barlow Ridge 40G",
        "USB4/TB4",
    ),
    (
        "8086",
        "1138",
        0,
        "Thunderbolt 4 USB [Maple Ridge 4C]",
        "Maple Ridge 4C",
        "USB4/TB4",
    ),
    (
        "8086",
        "1135",
        0,
        "Thunderbolt 4 USB [Maple Ridge 2C]",
        "Maple Ridge 2C",
        "USB4/TB4",
    ),
    (
        "8086",
        "0b27",
        0,
        "Thunderbolt 4 USB [Goshen Ridge]",
        "Goshen Ridge",
        "USB4/TB4",
    ),
    (
        "8086",
        "15e9",
        0,
        "JHL7540 Thunderbolt 3 USB [Titan Ridge 2C]",
        "Titan Ridge 2C",
        "USB 3.1/TB3",
    ),
    (
        "8086",
        "15ec",
        0,
        "JHL7540 Thunderbolt 3 USB [Titan Ridge 4C]",
        "Titan Ridge 4C",
        "USB 3.1/TB3",
    ),
    (
        "8086",
        "15f0",
        0,
        "JHL7440 Thunderbolt 3 USB [Titan Ridge DD]",
        "Titan Ridge DD",
        "USB 3.1/TB3",
    ),
    (
        "8086",
        "15b5",
        0,
        "DSL6340 USB 3.1 [Alpine Ridge 2C]",
        "Alpine Ridge 2C",
        "USB 3.1/TB3",
    ),
    (
        "8086",
        "15b6",
        0,
        "DSL6540 USB 3.1 [Alpine Ridge 4C]",
        "Alpine Ridge 4C",
        "USB 3.1/TB3",
    ),
    (
        "8086",
        "15c1",
        0,
        "JHL6240 Thunderbolt 3 [Alpine Ridge LP]",
        "Alpine Ridge LP",
        "USB 3.1/TB3",
    ),
    (
        "8086",
        "15d4",
        0,
        "JHL6540 Thunderbolt 3 [Alpine Ridge 4C C-step]",
        "Alpine Ridge 4C C-step",
        "USB 3.1/TB3",
    ),
    (
        "8086",
        "15db",
        0,
        "JHL6340 Thunderbolt 3 [Alpine Ridge 2C C-step]",
        "Alpine Ridge 2C C-step",
        "USB 3.1/TB3",
    ),
    (
        "8086",
        "7f6e",
        1,
        "800 Series PCH USB 3.1 xHCI",
        "800 Series PCH",
        "USB 3.1",
    ),
    (
        "8086",
        "7a60",
        1,
        "Raptor Lake USB 3.2 Gen 2x2 xHCI",
        "700 Series PCH",
        "USB 3.2 Gen 2x2",
    ),
    (
        "8086",
        "7a61",
        1,
        "Raptor Lake USB 3.2 Gen 1 xHCI",
        "700 Series PCH",
        "USB 3.2 Gen 1",
    ),
    (
        "8086",
        "7ae0",
        1,
        "Alder Lake-S PCH USB 3.2 Gen 2x2",
        "600 Series PCH",
        "USB 3.2 Gen 2x2",
    ),
    (
        "8086",
        "51ed",
        1,
        "Alder Lake PCH USB 3.2 xHCI",
        "600 Series PCH",
        "USB 3.2",
    ),
    (
        "8086",
        "54ed",
        1,
        "Alder Lake-N PCH USB 3.2 Gen 2 xHCI",
        "Alder Lake-N PCH",
        "USB 3.2 Gen 2",
    ),
    (
        "8086",
        "7e7d",
        1,
        "Meteor Lake-P USB 3.2 Gen 2 xHCI",
        "Meteor Lake PCH",
        "USB 3.2 Gen 2",
    ),
    (
        "8086",
        "777d",
        1,
        "Arrow Lake USB 3.2 xHCI",
        "Arrow Lake",
        "USB 3.2",
    ),
    (
        "8086",
        "a87d",
        1,
        "Lunar Lake-M USB 3.2 Gen 2 xHCI",
        "Lunar Lake PCH",
        "USB 3.2 Gen 2",
    ),
    (
        "8086",
        "a0ed",
        1,
        "Tiger Lake-LP USB 3.2 Gen 2 xHCI",
        "500 Series PCH",
        "USB 3.2 Gen 2",
    ),
    (
        "8086",
        "43ed",
        1,
        "Tiger Lake-H USB 3.2 Gen 2 xHCI",
        "500 Series PCH-H",
        "USB 3.2 Gen 2",
    ),
    (
        "8086",
        "a3af",
        1,
        "Comet Lake PCH-V USB",
        "400 Series PCH",
        "USB 3.1",
    ),
    (
        "8086",
        "02ed",
        1,
        "Comet Lake PCH-LP USB 3.1 xHCI",
        "400 Series PCH-LP",
        "USB 3.1",
    ),
    (
        "8086",
        "06ed",
        1,
        "Comet Lake USB 3.1 xHCI",
        "400 Series PCH",
        "USB 3.1",
    ),
    (
        "8086",
        "a36d",
        1,
        "Cannon Lake PCH USB 3.1 xHCI",
        "300 Series PCH",
        "USB 3.1",
    ),
    (
        "8086",
        "9ded",
        1,
        "Cannon Point-LP USB 3.1 xHCI",
        "300 Series PCH-LP",
        "USB 3.1",
    ),
    (
        "8086",
        "a2af",
        1,
        "200 Series/Z370 USB 3.0 xHCI",
        "200 Series PCH",
        "USB 3.0",
    ),
    (
        "8086",
        "a12f",
        1,
        "100 Series/C230 USB 3.0 xHCI",
        "100 Series PCH",
        "USB 3.0",
    ),
    (
        "8086",
        "9d2f",
        1,
        "Sunrise Point-LP USB 3.0 xHCI",
        "100 Series PCH-LP",
        "USB 3.0",
    ),
    (
        "8086",
        "8cb1",
        1,
        "9 Series Chipset USB xHCI",
        "9 Series PCH",
        "USB 3.0",
    ),
    (
        "8086",
        "9cb1",
        1,
        "Wildcat Point-LP USB xHCI",
        "9 Series PCH-LP",
        "USB 3.0",
    ),
    (
        "8086",
        "8c31",
        1,
        "8 Series/C220 USB xHCI",
        "8 Series PCH",
        "USB 3.0",
    ),
    (
        "8086",
        "9c31",
        1,
        "8 Series USB xHCI HC",
        "8 Series PCH-LP",
        "USB 3.0",
    ),
    (
        "8086",
        "1e31",
        1,
        "7 Series/C210 USB xHCI",
        "7 Series PCH",
        "USB 3.0",
    ),
    (
        "8086",
        "8d31",
        1,
        "C610/X99 USB xHCI",
        "X99/C610 HEDT",
        "USB 3.0",
    ),
    (
        "8086",
        "a1af",
        1,
        "C620 Series USB 3.0 xHCI",
        "C620 Server",
        "USB 3.0",
    ),
];

const AMD_DB: &[DbEntry] = &[
    (
        "1022",
        "15b6",
        0,
        "Raphael/Granite Ridge USB 3.1 xHCI",
        "Ryzen 7000/9000 Desktop (AM5)",
        "USB 3.1",
    ),
    (
        "1022",
        "15b7",
        0,
        "Raphael/Granite Ridge USB 3.1 xHCI",
        "Ryzen 7000/9000 Desktop (AM5)",
        "USB 3.1",
    ),
    (
        "1022",
        "15b8",
        0,
        "Raphael/Granite Ridge USB 2.0 xHCI",
        "Ryzen 7000/9000 Desktop (AM5)",
        "USB 2.0",
    ),
    (
        "1022",
        "1587",
        0,
        "Strix Halo USB 3.1 xHCI",
        "Strix Halo (Zen 5)",
        "USB 3.1",
    ),
    (
        "1022",
        "1588",
        0,
        "Strix Halo USB 3.1 xHCI",
        "Strix Halo (Zen 5)",
        "USB 3.1",
    ),
    (
        "1022",
        "1589",
        0,
        "Strix Halo USB 3.1 xHCI",
        "Strix Halo (Zen 5)",
        "USB 3.1",
    ),
    (
        "1022",
        "158b",
        0,
        "Strix Halo USB 3.1 xHCI",
        "Strix Halo (Zen 5)",
        "USB 3.1",
    ),
    (
        "1022",
        "158d",
        0,
        "Strix Halo USB4 Host Router",
        "Strix Halo (Zen 5)",
        "USB4",
    ),
    (
        "1022",
        "158e",
        0,
        "Strix Halo USB4 Host Router",
        "Strix Halo (Zen 5)",
        "USB4",
    ),
    (
        "1022",
        "161a",
        0,
        "Rembrandt USB4 xHCI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4",
    ),
    (
        "1022",
        "161b",
        0,
        "Rembrandt USB4 xHCI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4",
    ),
    (
        "1022",
        "161c",
        0,
        "Rembrandt USB4 xHCI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4",
    ),
    (
        "1022",
        "161d",
        0,
        "Rembrandt USB4 xHCI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4",
    ),
    (
        "1022",
        "161e",
        0,
        "Rembrandt USB4 xHCI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4",
    ),
    (
        "1022",
        "161f",
        0,
        "Rembrandt USB4 xHCI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4",
    ),
    (
        "1022",
        "15d6",
        0,
        "Rembrandt USB4 xHCI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4",
    ),
    (
        "1022",
        "15d7",
        0,
        "Rembrandt USB4 xHCI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4",
    ),
    (
        "1022",
        "162e",
        0,
        "Rembrandt USB4/Thunderbolt NHI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4/TB",
    ),
    (
        "1022",
        "162f",
        0,
        "Rembrandt USB4/Thunderbolt NHI",
        "Ryzen 6000 Mobile (Zen 3+)",
        "USB4/TB",
    ),
    (
        "1022",
        "15c4",
        0,
        "Phoenix USB4/Thunderbolt NHI",
        "Ryzen 7040 Mobile (Zen 4)",
        "USB4/TB",
    ),
    (
        "1022",
        "15c5",
        0,
        "Phoenix USB4/Thunderbolt NHI",
        "Ryzen 7040 Mobile (Zen 4)",
        "USB4/TB",
    ),
    (
        "1022",
        "1668",
        0,
        "Pink Sardine USB4/Thunderbolt NHI",
        "Pink Sardine",
        "USB4/TB",
    ),
    (
        "1022",
        "1669",
        0,
        "Pink Sardine USB4/Thunderbolt NHI",
        "Pink Sardine",
        "USB4/TB",
    ),
    (
        "1022",
        "1639",
        0,
        "Renoir/Cezanne USB 3.1",
        "Ryzen 4000/5000 APU (Zen 2/3)",
        "USB 3.1",
    ),
    (
        "1022",
        "15e0",
        0,
        "Raven USB 3.1",
        "Ryzen 2000 APU (Zen)",
        "USB 3.1",
    ),
    (
        "1022",
        "15e1",
        0,
        "Raven USB 3.1",
        "Ryzen 2000 APU (Zen)",
        "USB 3.1",
    ),
    (
        "1022",
        "15e5",
        0,
        "Raven2 USB 3.1",
        "Ryzen 3000 APU (Zen+)",
        "USB 3.1",
    ),
    (
        "1022",
        "149c",
        0,
        "Matisse USB 3.0 Host Controller",
        "Ryzen 3000/5000 Desktop (Zen 2/3)",
        "USB 3.0",
    ),
    (
        "1022",
        "148c",
        0,
        "Starship USB 3.0 Host Controller",
        "EPYC Rome / Threadripper 3rd Gen",
        "USB 3.0",
    ),
    (
        "1022",
        "145f",
        0,
        "Zeppelin USB 3.0 xHCI",
        "Ryzen 1000 (Zen)",
        "USB 3.0",
    ),
    (
        "1022",
        "145c",
        0,
        "Family 17h USB 3.0 Host Controller",
        "Ryzen 1000 (Zen)",
        "USB 3.0",
    ),
    (
        "1022",
        "162c",
        0,
        "VanGogh USB2",
        "Steam Deck (Van Gogh)",
        "USB 2.0",
    ),
    (
        "1022",
        "163a",
        0,
        "VanGogh USB0",
        "Steam Deck (Van Gogh)",
        "USB 3.1",
    ),
    (
        "1022",
        "163b",
        0,
        "VanGogh USB1",
        "Steam Deck (Van Gogh)",
        "USB 3.1",
    ),
    (
        "1022",
        "15d4",
        0,
        "FireFlight USB 3.1",
        "FireFlight",
        "USB 3.1",
    ),
    (
        "1022",
        "15d5",
        0,
        "FireFlight USB 3.1",
        "FireFlight",
        "USB 3.1",
    ),
    (
        "1022",
        "13ed",
        0,
        "Ariel USB 3.1 Type-C",
        "Ariel",
        "USB 3.1 Gen 2",
    ),
    (
        "1022",
        "13ee",
        0,
        "Ariel USB 3.1 Type-A",
        "Ariel",
        "USB 3.1 Gen 2",
    ),
    (
        "1022",
        "1557",
        0,
        "Turin USB 3.1 xHCI",
        "EPYC Turin",
        "USB 3.1",
    ),
    (
        "1022",
        "43fc",
        1,
        "800 Series Chipset USB 3.x xHCI",
        "X870/B850 (AM5)",
        "USB 3.2",
    ),
    (
        "1022",
        "43fd",
        1,
        "800 Series Chipset USB 3.x xHCI",
        "X870/B850 (AM5)",
        "USB 3.2",
    ),
    (
        "1022",
        "43f7",
        1,
        "600 Series Chipset USB 3.2",
        "X670/B650 (AM5)",
        "USB 3.2",
    ),
    (
        "1022",
        "43ee",
        1,
        "500 Series Chipset USB 3.1 xHCI",
        "X570/B550 (AM4)",
        "USB 3.1",
    ),
    (
        "1022",
        "43ec",
        1,
        "A520 Series Chipset USB 3.1 xHCI",
        "A520 (AM4)",
        "USB 3.1",
    ),
    (
        "1022",
        "43d5",
        1,
        "400 Series Chipset USB 3.1 xHCI",
        "X470/B450 (AM4)",
        "USB 3.1",
    ),
    (
        "1022",
        "43b9",
        1,
        "X370 Series Chipset USB 3.1 xHCI",
        "X370 (AM4)",
        "USB 3.1",
    ),
    (
        "1022",
        "43ba",
        1,
        "X399 Series Chipset USB 3.1 xHCI",
        "X399 (Threadripper)",
        "USB 3.1",
    ),
    (
        "1022",
        "43bb",
        1,
        "300 Series Chipset USB 3.1 xHCI",
        "B350 (AM4)",
        "USB 3.1",
    ),
    (
        "1022",
        "43bc",
        1,
        "A320 USB 3.1 xHCI",
        "A320 (AM4)",
        "USB 3.1",
    ),
    ("1022", "7814", 1, "FCH USB xHCI", "Legacy FCH", "USB 3.0"),
    ("1022", "7812", 1, "FCH USB xHCI", "Legacy FCH", "USB 3.0"),
];

const THIRD_PARTY_DB: &[(&str, &str, &str, &str, &str)] = &[
    (
        "1b21",
        "1042",
        "ASM1042 SuperSpeed USB 3.0",
        "ASMedia",
        "USB 3.0",
    ),
    ("1b21", "1142", "ASM1042A USB 3.0", "ASMedia", "USB 3.0"),
    (
        "1b21",
        "1242",
        "ASM1142 USB 3.1 Gen 2",
        "ASMedia",
        "USB 3.1 Gen 2",
    ),
    (
        "1b21",
        "1343",
        "ASM1143 USB 3.1 Gen 2",
        "ASMedia",
        "USB 3.1 Gen 2",
    ),
    (
        "1b21",
        "2142",
        "ASM2142/ASM3142 USB 3.1",
        "ASMedia",
        "USB 3.1 Gen 2",
    ),
    (
        "1b21",
        "3042",
        "ASM3042 USB 3.2 Gen 1",
        "ASMedia",
        "USB 3.2 Gen 1",
    ),
    (
        "1b21",
        "3142",
        "ASM3142 USB 3.2 Gen 2",
        "ASMedia",
        "USB 3.2 Gen 2",
    ),
    (
        "1b21",
        "3242",
        "ASM3242 USB 3.2 Gen 2x2",
        "ASMedia",
        "USB 3.2 Gen 2x2",
    ),
    (
        "1b21",
        "2425",
        "ASM4242 USB4/TB3 Host Router",
        "ASMedia",
        "USB4/TB3",
    ),
    ("1b21", "2426", "ASM4242 USB 3.2 xHCI", "ASMedia", "USB 3.2"),
    ("1106", "3483", "VL805/806 xHCI USB 3.0", "VIA", "USB 3.0"),
    ("1106", "3432", "VL800/801 xHCI USB 3.0", "VIA", "USB 3.0"),
    ("1b73", "1000", "FL1000G USB 3.0", "Fresco Logic", "USB 3.0"),
    ("1b73", "1009", "FL1009 USB 3.0", "Fresco Logic", "USB 3.0"),
    ("1b73", "1100", "FL1100 USB 3.0", "Fresco Logic", "USB 3.0"),
    ("1b73", "1400", "FL1400 USB 3.0", "Fresco Logic", "USB 3.0"),
    ("1b6f", "7023", "EJ168 USB 3.0", "Etron", "USB 3.0"),
    ("1b6f", "7052", "EJ188/EJ198 USB 3.0", "Etron", "USB 3.0"),
    ("1912", "0014", "uPD720201 USB 3.0", "Renesas", "USB 3.0"),
    ("1912", "0015", "uPD720202 USB 3.0", "Renesas", "USB 3.0"),
];

// ─── Structs ─────────────────────────────────────────────────────────────────

struct ControllerInfo {
    vid: String,
    did: String,
    chip_level: i32,
    name: String,
    platform: String,
    usb: String,
    instance_id: String,
    msi_status: String,
    selective_suspend: Option<bool>,
    bus_prefix: String, // "4&abc&0" — used to match ROOT_HUBs
}

struct DeviceEntry {
    name: String,
    vid: String,
    pid: String,
    chip_count: i32, // controller.chip_level + hub_count
    hub_count: i32,
    ctrl_idx: usize,
}

struct PnpInputDevice {
    name: String,
    vid: String,
    pid: String,
    controller_instance_id: String,
    hub_count: i32,
}

fn lookup_controller(
    vid: &str,
    did: &str,
    instance_id: String,
    bus_prefix: String,
) -> ControllerInfo {
    let v = vid.to_lowercase();
    let d = did.to_lowercase();

    for &(dv, dd, chip, name, platform, usb) in INTEL_DB {
        if v == dv && d == dd {
            return ControllerInfo {
                vid: vid.to_uppercase(),
                did: did.to_uppercase(),
                chip_level: chip,
                name: name.into(),
                platform: platform.into(),
                usb: usb.into(),
                instance_id,
                msi_status: "Unknown".into(),
                selective_suspend: None,
                bus_prefix,
            };
        }
    }
    for &(dv, dd, chip, name, platform, usb) in AMD_DB {
        if v == dv && d == dd {
            return ControllerInfo {
                vid: vid.to_uppercase(),
                did: did.to_uppercase(),
                chip_level: chip,
                name: name.into(),
                platform: platform.into(),
                usb: usb.into(),
                instance_id,
                msi_status: "Unknown".into(),
                selective_suspend: None,
                bus_prefix,
            };
        }
    }
    for &(dv, dd, name, vendor, usb) in THIRD_PARTY_DB {
        if v == dv && d == dd {
            return ControllerInfo {
                vid: vid.to_uppercase(),
                did: did.to_uppercase(),
                chip_level: 1,
                name: name.into(),
                platform: format!("PCIe Add-in ({})", vendor),
                usb: usb.into(),
                instance_id,
                msi_status: "Unknown".into(),
                selective_suspend: None,
                bus_prefix,
            };
        }
    }

    let (chip_level, name, platform) = if v == "8086" {
        (
            1,
            "Intel USB Controller".into(),
            format!("Unknown PCH (DID:{})", did.to_uppercase()),
        )
    } else if v == "1022" {
        (
            1,
            "AMD USB Controller".into(),
            format!("Unknown Chipset (DID:{})", did.to_uppercase()),
        )
    } else {
        let vendor = match v.as_str() {
            "1b21" => "ASMedia",
            "1106" => "VIA",
            "1b73" => "Fresco Logic",
            "1912" => "Renesas",
            "1b6f" => "Etron",
            "104c" => "Texas Instruments",
            _ => "Unknown",
        };
        (
            1,
            format!("{} Controller", vendor),
            format!(
                "PCIe Add-in (VID:{} DID:{})",
                vid.to_uppercase(),
                did.to_uppercase()
            ),
        )
    };

    ControllerInfo {
        vid: vid.to_uppercase(),
        did: did.to_uppercase(),
        chip_level,
        name,
        platform,
        usb: "USB 3.x".into(),
        instance_id,
        msi_status: "Unknown".into(),
        selective_suspend: None,
        bus_prefix,
    }
}

// ─── Registry helpers ─────────────────────────────────────────────────────────

fn extract_hex(s: &str, prefix: &str) -> Option<String> {
    let start = s.find(prefix)? + prefix.len();
    let end = (start + 4).min(s.len());
    Some(s[start..end].to_lowercase())
}

// "4&abc&0&00f0" → "4&abc&0"  (strips last &segment)
fn strip_last_segment(s: &str) -> Option<&str> {
    let pos = s.rfind('&')?;
    Some(&s[..pos])
}

const USB_HOST_CLASS_GUID: &str = "{36fc9e60-c465-11cf-8056-444553540000}";

// ─── PCI USB-controller scan ──────────────────────────────────────────────────

fn enum_usb_controllers(hklm: &RegKey) -> Vec<ControllerInfo> {
    let mut list = Vec::new();

    let pci = match hklm.open_subkey("SYSTEM\\CurrentControlSet\\Enum\\PCI") {
        Ok(k) => k,
        Err(_) => return list,
    };

    for dev_name in pci.enum_keys().flatten() {
        let vid = match extract_hex(&dev_name, "VEN_") {
            Some(v) => v,
            None => continue,
        };
        let did = match extract_hex(&dev_name, "DEV_") {
            Some(d) => d,
            None => continue,
        };
        let dev_key = match pci.open_subkey(&dev_name) {
            Ok(k) => k,
            Err(_) => continue,
        };

        for inst_name in dev_key.enum_keys().flatten() {
            let inst = match dev_key.open_subkey(&inst_name) {
                Ok(k) => k,
                Err(_) => continue,
            };
            let guid: String = match inst.get_value("ClassGUID") {
                Ok(v) => v,
                Err(_) => continue,
            };
            if guid.to_lowercase() != USB_HOST_CLASS_GUID {
                continue;
            }

            // Bus-prefix: everything before the last &segment of the instance key
            let bp = strip_last_segment(&inst_name)
                .unwrap_or(&inst_name)
                .to_string();
            let instance_id = format!("PCI\\{}\\{}", dev_name, inst_name);
            list.push(lookup_controller(&vid, &did, instance_id, bp));
        }
    }
    list
}

fn enrich_controller_status(hklm: &RegKey, controllers: &mut [ControllerInfo]) {
    for ctrl in controllers {
        let msi_path = format!(
            "SYSTEM\\CurrentControlSet\\Enum\\{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties",
            ctrl.instance_id
        );
        ctrl.msi_status = match hklm.open_subkey(&msi_path) {
            Ok(key) => match key.get_value::<u32, _>("MSISupported") {
                Ok(1) => "MSI".into(),
                Ok(_) => "Line-Based".into(),
                Err(_) => "Unknown".into(),
            },
            Err(_) => "Unknown".into(),
        };

        let power_path = format!(
            "SYSTEM\\CurrentControlSet\\Enum\\{}\\Device Parameters",
            ctrl.instance_id
        );
        ctrl.selective_suspend = hklm
            .open_subkey(&power_path)
            .ok()
            .and_then(|key| key.get_value::<u32, _>("SelectiveSuspendEnabled").ok())
            .map(|v| v == 1);
    }
}

fn get_system_usb_suspend() -> Option<bool> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let out = std::process::Command::new("powercfg")
        .args(["/query", "SCHEME_CURRENT", "SUB_USB", "USBSELECTIVESUSPEND"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(hex) = trimmed.strip_prefix("Current AC Power Setting Index:") {
            let value = hex.trim().trim_start_matches("0x");
            return u32::from_str_radix(value, 16).ok().map(|v| v == 1);
        }
    }
    None
}

// ─── USB device-tree scan ─────────────────────────────────────────────────────
//
// Parent-chain walk algorithm:
//   Each USB instance suffix encodes parent: "6&abc&0&1" → parent has ParentIdPrefix="6&abc&0"
//   Walk up until hitting ROOT_HUB* → match its bus-prefix to PCI controller
//   Count non-root intermediate hubs along the way

struct UsbNode {
    dev_key_name: String,     // "ROOT_HUB30" | "VID_046D&PID_C52B" | …
    inst_suffix: String,      // "4&abc&0&0"
    parent_id_prefix: String, // value of ParentIdPrefix registry entry
    friendly_name: String,
    compat_ids: Vec<String>,
}

fn scan_usb_tree(hklm: &RegKey) -> Vec<UsbNode> {
    let mut nodes = Vec::new();

    let usb = match hklm.open_subkey("SYSTEM\\CurrentControlSet\\Enum\\USB") {
        Ok(k) => k,
        Err(_) => return nodes,
    };

    for dev_name in usb.enum_keys().flatten() {
        let dev_key = match usb.open_subkey(&dev_name) {
            Ok(k) => k,
            Err(_) => continue,
        };
        for inst_suffix in dev_key.enum_keys().flatten() {
            let inst = match dev_key.open_subkey(&inst_suffix) {
                Ok(k) => k,
                Err(_) => continue,
            };
            let pip: String = inst.get_value("ParentIdPrefix").unwrap_or_default();
            let fname = inst
                .get_value::<String, _>("FriendlyName")
                .or_else(|_| inst.get_value::<String, _>("DeviceDesc"))
                .unwrap_or_else(|_| dev_name.clone());
            let compat: Vec<String> = inst.get_value("CompatibleIDs").unwrap_or_default();
            nodes.push(UsbNode {
                dev_key_name: dev_name.clone(),
                inst_suffix,
                parent_id_prefix: pip,
                friendly_name: fname,
                compat_ids: compat,
            });
        }
    }
    nodes
}

// Returns (controller_idx, hub_count) by walking the PnP parent chain.
//
// Two lookup maps are used at each step:
//   pip_map:  ParentIdPrefix → node  (normal parent lookup via PnP prefix)
//   inst_map: inst_suffix    → node  (handles MI-interface → composite-device step)
fn trace_chain(
    inst_suffix: &str,
    pip_map: &HashMap<String, usize>,
    inst_map: &HashMap<String, usize>,
    nodes: &[UsbNode],
    ctrl_bus_map: &HashMap<String, usize>,
) -> Option<(usize, i32)> {
    let mut current = inst_suffix.to_string();
    let mut hub_count = 0i32;
    let mut steps = 0usize;

    while steps < 20 {
        steps += 1;
        let stripped = strip_last_segment(&current)?.to_string();

        // Try ParentIdPrefix lookup first
        if let Some(&pidx) = pip_map.get(&stripped) {
            let parent = &nodes[pidx];
            if parent
                .dev_key_name
                .to_ascii_uppercase()
                .contains("ROOT_HUB")
            {
                let rh_bus = strip_last_segment(&parent.inst_suffix)?;
                let ctrl_idx = *ctrl_bus_map.get(rh_bus)?;
                return Some((ctrl_idx, hub_count));
            }
            // External hub in chain
            hub_count += 1;
            current = parent.inst_suffix.clone();
            continue;
        }

        // Fallback: match by inst_suffix (handles MI-interface → composite USB device)
        if let Some(&pidx) = inst_map.get(&stripped) {
            // Don't count composite device as a hub
            current = nodes[pidx].inst_suffix.clone();
            continue;
        }

        break;
    }
    None
}

fn get_pnp_input_devices() -> Vec<PnpInputDevice> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let script = r#"
$inputDevices = @()
$usbDevices = Get-PnpDevice -Status OK -ErrorAction SilentlyContinue | Where-Object { $_.InstanceId -match '^USB\\' }
foreach ($dev in $usbDevices) {
    try {
        $compatIds = (Get-PnpDeviceProperty -InstanceId $dev.InstanceId -KeyName 'DEVPKEY_Device_CompatibleIds' -ErrorAction SilentlyContinue).Data
        if ($compatIds -match 'Class_03') { $inputDevices += $dev }
    } catch {}
}
$xboxDevices = Get-PnpDevice -Class 'XboxComposite','XnaComposite','XUSBClass' -Status OK -ErrorAction SilentlyContinue
if ($xboxDevices) { $inputDevices += $xboxDevices }
$inputDevices = $inputDevices | Sort-Object InstanceId -Unique

foreach ($device in $inputDevices) {
    $instanceId = $device.InstanceId
    $usbParent = $instanceId
    if ($instanceId -match '^HID\\') {
        try {
            $usbParent = (Get-PnpDeviceProperty -InstanceId $instanceId -KeyName 'DEVPKEY_Device_Parent' -ErrorAction Stop).Data
        } catch { continue }
    }

    $currentId = $usbParent
    $hubCount = 0
    $controllerId = ''
    $count = 0
    while ($currentId -and $count -lt 15) {
        $count++
        $dev = Get-PnpDevice -InstanceId $currentId -ErrorAction SilentlyContinue
        if ($currentId -match 'ROOT_HUB') {
            try {
                $controllerId = (Get-PnpDeviceProperty -InstanceId $currentId -KeyName 'DEVPKEY_Device_Parent' -ErrorAction Stop).Data
            } catch {}
            break
        }
        if ($dev -and $dev.FriendlyName -match 'Hub' -and $dev.FriendlyName -notmatch 'Root') {
            $hubCount++
        }
        try {
            $currentId = (Get-PnpDeviceProperty -InstanceId $currentId -KeyName 'DEVPKEY_Device_Parent' -ErrorAction Stop).Data
        } catch { break }
    }
    if (-not $controllerId) { continue }

    $devVid = '????'
    $devPid = '????'
    if ($usbParent -match 'VID_([0-9A-Fa-f]{4})') { $devVid = $Matches[1].ToUpper() }
    if ($usbParent -match 'PID_([0-9A-Fa-f]{4})') { $devPid = $Matches[1].ToUpper() }

    $devName = $device.FriendlyName
    try {
        $busDesc = (Get-PnpDeviceProperty -InstanceId $usbParent -KeyName 'DEVPKEY_Device_BusReportedDeviceDesc' -ErrorAction SilentlyContinue).Data
        if ($busDesc -and $busDesc.Trim()) { $devName = $busDesc.Trim() }
    } catch {}

    "$($devName -replace "`t",' ')`t$devVid`t$devPid`t$controllerId`t$hubCount"
}
"#;

    let out = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let stdout = match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => return Vec::new(),
    };

    stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            let name = parts.next()?.trim().to_string();
            let vid = parts.next()?.trim().to_string();
            let pid = parts.next()?.trim().to_string();
            let controller_instance_id = parts.next()?.trim().to_string();
            let hub_count = parts.next()?.trim().parse::<i32>().ok()?;
            if name.is_empty() || controller_instance_id.is_empty() {
                return None;
            }
            Some(PnpInputDevice {
                name,
                vid,
                pid,
                controller_instance_id,
                hub_count,
            })
        })
        .collect()
}

// ─── Color helpers ────────────────────────────────────────────────────────────

fn chip_color(level: i32) -> i32 {
    match level {
        0 => COL_MINT,
        1 => COL_ORANGE,
        _ => COL_CORAL,
    }
}

fn chip_label(level: i32) -> &'static str {
    match level {
        0 => "CHIP 0 — INSIDE CPU",
        1 => "CHIP 1 — CHIPSET",
        _ => "CHIP 2+ — HUB",
    }
}

// ─── Public entry point ───────────────────────────────────────────────────────

#[allow(dead_code)]
pub fn analyze_usb_latency() -> Result<LatencyAnalysis, String> {
    analyze_usb_latency_with_progress(|_, _| {})
}

pub fn analyze_usb_latency_with_progress<F>(mut progress: F) -> Result<LatencyAnalysis, String>
where
    F: FnMut(i32, &str),
{
    let lang = current_language();
    progress(5, "Checking power settings...");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // 1. PCI USB controllers
    progress(15, "Scanning USB controllers...");
    let mut controllers = enum_usb_controllers(&hklm);
    progress(25, "Checking MSI and suspend settings...");
    enrich_controller_status(&hklm, &mut controllers);
    controllers.sort_by_key(|c| c.chip_level);
    let system_usb_suspend = get_system_usb_suspend();

    let ctrl_bus_map: HashMap<String, usize> = controllers
        .iter()
        .enumerate()
        .map(|(i, c)| (c.bus_prefix.clone(), i))
        .collect();

    // 2. USB device tree
    progress(35, t(lang, "latency_progress_usb_registry_tree"));
    let nodes = scan_usb_tree(&hklm);

    // pip_map: ParentIdPrefix value → node index
    let mut pip_map: HashMap<String, usize> = HashMap::new();
    // inst_map: inst_suffix → node index (for MI-interface → composite lookup)
    let mut inst_map: HashMap<String, usize> = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        if !n.parent_id_prefix.is_empty() {
            pip_map.insert(n.parent_id_prefix.clone(), i);
        }
        inst_map.insert(n.inst_suffix.clone(), i);
    }

    // 3. Trace HID devices to their controllers
    let mut seen_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut devices: Vec<DeviceEntry> = Vec::new();
    let mut ctrl_devs: Vec<Vec<usize>> = vec![Vec::new(); controllers.len()];

    progress(55, "Finding input devices...");
    for dev in get_pnp_input_devices() {
        let dev_key = format!("{}:{}", dev.vid, dev.pid);
        if !seen_keys.insert(dev_key) {
            continue;
        }

        let ctrl_idx = match controllers.iter().position(|c| {
            c.instance_id
                .eq_ignore_ascii_case(&dev.controller_instance_id)
        }) {
            Some(idx) => idx,
            None => continue,
        };

        let chip_count = controllers[ctrl_idx].chip_level + dev.hub_count;
        let di = devices.len();
        devices.push(DeviceEntry {
            name: dev.name,
            vid: dev.vid,
            pid: dev.pid,
            chip_count,
            hub_count: dev.hub_count,
            ctrl_idx,
        });
        ctrl_devs[ctrl_idx].push(di);
    }

    // Fallback for environments where PnP cmdlets are unavailable.
    progress(82, "Verifying fallback USB tree...");
    for node in &nodes {
        let is_hid = node
            .compat_ids
            .iter()
            .any(|id| id.to_ascii_lowercase().contains("class_03"));
        if !is_hid {
            continue;
        }

        // Dedup by base key (strip &MI_XX so all interfaces of one device count as one)
        let base_key = match node.dev_key_name.find("&MI_") {
            Some(pos) => node.dev_key_name[..pos].to_string(),
            None => node.dev_key_name.clone(),
        };
        if !seen_keys.insert(base_key.clone()) {
            continue;
        }

        // For MI-interface devices, start tracing from one level up (the composite inst)
        let trace_inst = if node.dev_key_name.contains("&MI_") {
            match strip_last_segment(&node.inst_suffix) {
                Some(s) => s.to_string(),
                None => node.inst_suffix.clone(),
            }
        } else {
            node.inst_suffix.clone()
        };

        let (ctrl_idx, hub_count) =
            match trace_chain(&trace_inst, &pip_map, &inst_map, &nodes, &ctrl_bus_map) {
                Some(r) => r,
                None => continue,
            };

        // Prefer composite parent's FriendlyName over the generic MI-interface name
        let name = if node.dev_key_name.contains("&MI_") {
            let composite_inst = strip_last_segment(&node.inst_suffix).unwrap_or("");
            nodes
                .iter()
                .find(|n| {
                    !n.dev_key_name.contains("&MI_")
                        && n.dev_key_name.starts_with(&base_key)
                        && n.inst_suffix == composite_inst
                })
                .map(|n| n.friendly_name.clone())
                .unwrap_or_else(|| node.friendly_name.clone())
        } else {
            node.friendly_name.clone()
        };

        let chip_count = controllers[ctrl_idx].chip_level + hub_count;
        let di = devices.len();
        devices.push(DeviceEntry {
            name,
            vid: "????".into(),
            pid: "????".into(),
            chip_count,
            hub_count,
            ctrl_idx,
        });
        ctrl_devs[ctrl_idx].push(di);
    }

    // ── Build styled output ──────────────────────────────────────────────────
    progress(95, "Building report...");
    let mut lines: Vec<StyledLine> = vec![
        StyledLine::empty(),
        StyledLine::new("  USB LATENCY ANALYZER", COL_SKY, true),
        StyledLine::border(
            "  =====================================================================",
        ),
        StyledLine::empty(),
        StyledLine::dim("  Count chips between your device and CPU. More chips = more latency."),
        StyledLine::empty(),
        StyledLine::new("  0 CHIPS  device ——— [CPU]", COL_MINT, true),
        StyledLine::new("  1 CHIP   device —[CHIPSET]— [CPU]", COL_ORANGE, true),
        StyledLine::new("  2 CHIPS  device —[HUB]—[CHIPSET]— [CPU]", COL_CORAL, true),
        StyledLine::empty(),
        StyledLine::border("  ============================================================="),
        StyledLine::empty(),
    ];

    let has_chip0 = controllers.iter().any(|c| c.chip_level == 0);
    if !has_chip0 && !controllers.is_empty() {
        lines.push(StyledLine::new(
            "  ! This system has no direct CPU USB",
            COL_ORANGE,
            false,
        ));
        lines.push(StyledLine::dim("    1 chip is your best option here"));
        lines.push(StyledLine::empty());
    }

    if controllers.is_empty() {
        lines.push(StyledLine::dim(
            "  No USB controllers detected. Try running as Administrator.",
        ));
        lines.push(StyledLine::empty());
    } else {
        // Tree: group devices by chip_count
        for level in [0i32, 1, 2] {
            let group: Vec<&DeviceEntry> = devices
                .iter()
                .filter(|d| {
                    if level == 2 {
                        d.chip_count >= 2
                    } else {
                        d.chip_count == level
                    }
                })
                .collect();
            if group.is_empty() {
                continue;
            }

            let label = match level {
                0 => "  0 chips — direct to CPU",
                1 => "  1 chip — through chipset",
                _ => "  2+ chips — through hub",
            };
            lines.push(StyledLine::new(label, chip_color(level), false));

            let n = group.len();
            for (i, dev) in group.iter().enumerate() {
                let branch = if i + 1 == n { "  '— " } else { "  |— " };
                let hub_info = if dev.hub_count > 0 {
                    format!(" (+{} hub)", dev.hub_count)
                } else {
                    String::new()
                };
                lines.push(StyledLine::new(
                    format!("{}{}{}", branch, dev.name, hub_info),
                    chip_color(level),
                    false,
                ));
            }
            lines.push(StyledLine::empty());
        }

        if devices.is_empty() {
            lines.push(StyledLine::dim("  No USB input devices detected"));
            lines.push(StyledLine::empty());
        }
    }

    lines.push(StyledLine::border(
        "  =============================================================",
    ));
    lines.push(StyledLine::empty());
    lines.push(StyledLine::dim(
        "  Unplug and replug to test different ports",
    ));
    lines.push(StyledLine::empty());

    // ── CONTROLLERS section ───────────────────────────────────────────────────
    lines.push(StyledLine::new("  CONTROLLERS", COL_NORMAL, true));
    lines.push(StyledLine::border(
        "  ---------------------------------------------------------------------",
    ));

    for (ci, ctrl) in controllers.iter().enumerate() {
        let col = chip_color(ctrl.chip_level);
        lines.push(StyledLine::empty());
        lines.push(StyledLine::new(
            format!("  {}", chip_label(ctrl.chip_level)),
            col,
            true,
        ));
        lines.push(StyledLine::new(
            format!("      {}", ctrl.name),
            COL_NORMAL,
            false,
        ));
        lines.push(StyledLine::dim(format!(
            "      VID:{} DID:{} | {} | {}",
            ctrl.vid, ctrl.did, ctrl.platform, ctrl.usb
        )));

        match ctrl.msi_status.as_str() {
            "MSI" => lines.push(StyledLine::new(
                "      IRQ: MSI (low latency interrupts)",
                COL_MINT,
                false,
            )),
            "Line-Based" => lines.push(StyledLine::new(
                "      IRQ: Line-Based (higher latency)",
                COL_CORAL,
                false,
            )),
            _ => lines.push(StyledLine::dim("      IRQ: Unknown")),
        }

        if ctrl.selective_suspend == Some(true) {
            lines.push(StyledLine::new(
                "      ! Selective Suspend ENABLED (causes latency spikes)",
                COL_ORANGE,
                false,
            ));
        }

        if !ctrl_devs[ci].is_empty() {
            lines.push(StyledLine::dim("      Devices:"));
            let n = ctrl_devs[ci].len();
            for (i, &di) in ctrl_devs[ci].iter().enumerate() {
                let branch = if i + 1 == n {
                    "        '— "
                } else {
                    "        |— "
                };
                let hub_info = if devices[di].hub_count > 0 {
                    format!(" (+{} hub)", devices[di].hub_count)
                } else {
                    String::new()
                };
                lines.push(StyledLine::new(
                    format!("{}{}{}", branch, devices[di].name, hub_info),
                    COL_NORMAL,
                    false,
                ));
            }
        }
    }

    lines.push(StyledLine::empty());

    // ── INPUT DEVICES section ─────────────────────────────────────────────────
    lines.push(StyledLine::new("  INPUT DEVICES", COL_NORMAL, true));
    lines.push(StyledLine::border(
        "  ---------------------------------------------------------------------",
    ));

    if devices.is_empty() {
        lines.push(StyledLine::empty());
        lines.push(StyledLine::dim(
            "  (No HID input devices detected. Plug in a keyboard/mouse.)",
        ));
    } else {
        let mut sorted: Vec<&DeviceEntry> = devices.iter().collect();
        sorted.sort_by_key(|d| d.chip_count);

        for dev in sorted {
            let col = chip_color(dev.chip_count);
            let ctrl = &controllers[dev.ctrl_idx];
            lines.push(StyledLine::empty());
            lines.push(StyledLine::new(
                format!("  {}", dev.name),
                COL_NORMAL,
                false,
            ));
            lines.push(StyledLine::dim(format!(
                "      VID:{} PID:{}",
                dev.vid, dev.pid
            )));
            lines.push(StyledLine::new(
                format!("      {}", chip_label(dev.chip_count)),
                col,
                false,
            ));
            lines.push(StyledLine::dim(format!(
                "      via {} ({})",
                ctrl.name, ctrl.platform
            )));
        }
    }

    lines.push(StyledLine::empty());

    let has_line_based = controllers.iter().any(|c| c.msi_status == "Line-Based");
    let has_controller_suspend = controllers
        .iter()
        .any(|c| c.selective_suspend == Some(true));
    let has_system_suspend = system_usb_suspend == Some(true);

    if has_line_based || has_controller_suspend || has_system_suspend {
        lines.push(StyledLine::new(
            "  OPTIMIZATIONS AVAILABLE",
            COL_NORMAL,
            true,
        ));
        lines.push(StyledLine::border(
            "  ---------------------------------------------------------------------",
        ));
        lines.push(StyledLine::empty());

        if has_system_suspend {
            lines.push(StyledLine::new(
                "  ! Disable USB Selective Suspend in current power plan",
                COL_ORANGE,
                false,
            ));
        }

        for ctrl in &controllers {
            if ctrl.msi_status == "Line-Based" {
                lines.push(StyledLine::new(
                    format!("  ! Enable MSI interrupts on {}", ctrl.name),
                    COL_CORAL,
                    false,
                ));
            }
            if ctrl.selective_suspend == Some(true) {
                lines.push(StyledLine::new(
                    format!("  ! Disable Selective Suspend on {}", ctrl.name),
                    COL_ORANGE,
                    false,
                ));
            }
        }
        lines.push(StyledLine::empty());
    }

    lines.push(StyledLine::border(
        "  =====================================================================",
    ));
    lines.push(StyledLine::empty());

    progress(100, "Ready");
    Ok(LatencyAnalysis {
        styled_lines: lines,
    })
}
