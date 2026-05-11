use super::{HomeState, WinchiselApp};
use crate::{Language, i18n};

impl WinchiselApp {
    pub(crate) fn build_home_state(system: &mut sysinfo::System, lang: Language) -> HomeState {
        use sysinfo::System;
        system.refresh_memory();
        system.refresh_cpu_usage();

        let cpu_brand = system
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| i18n::t(lang, "home_unknown_cpu").to_string());
        let cpu_cores = System::physical_core_count()
            .or_else(|| Some(system.cpus().len()))
            .unwrap_or(0);
        let cpu_speed = Self::read_cpu_speed(system);
        let (gpu_name, _, gpu_vram) = Self::read_gpu_info(lang);
        let gpu_driver_version = Self::read_gpu_driver_version(&gpu_name);
        let (system_model, system_manufacturer, bios_version, bios_date) =
            Self::read_system_info(lang);
        let windows_build = Self::read_windows_build(lang);
        let motherboard = if system_manufacturer.is_empty() && system_model.is_empty() {
            i18n::t(lang, "home_unknown_model").to_string()
        } else if system_manufacturer.is_empty() {
            system_model
        } else if system_model.is_empty() {
            system_manufacturer
        } else {
            format!("{} {}", system_manufacturer, system_model)
        };
        let ram_details = Self::read_ram_details();
        let display_info = Self::read_display_info();

        let total_memory_gb = system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_memory_gb = system.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let mut total_disk_gb = 0.0f64;
        let mut used_disk_gb = 0.0f64;
        for disk in sysinfo::Disks::new_with_refreshed_list().list() {
            let total = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let used = (disk.total_space().saturating_sub(disk.available_space())) as f64
                / 1024.0
                / 1024.0
                / 1024.0;
            total_disk_gb += total;
            used_disk_gb += used;
        }
        let cpu_usage_percent = system.global_cpu_usage();
        let cpu_usage = format!("{:.0}%", cpu_usage_percent);
        let uptime_secs = System::uptime();

        HomeState {
            computer_name: System::host_name()
                .unwrap_or_else(|| i18n::t(lang, "home_unknown_pc").to_string()),
            os_version: System::os_version()
                .unwrap_or_else(|| i18n::t(lang, "home_unknown_os").to_string()),
            kernel_version: System::kernel_version()
                .unwrap_or_else(|| i18n::t(lang, "home_unknown_kernel").to_string()),
            windows_build,
            bios_version,
            bios_date,
            cpu_brand,
            cpu_cores: i18n::t(lang, "home_cores_suffix").replacen(
                "{}",
                &cpu_cores.to_string(),
                1,
            ),
            cpu_speed,
            gpu_name: if gpu_name.is_empty() {
                i18n::t(lang, "home_unknown_gpu").to_string()
            } else {
                gpu_name
            },
            gpu_vram,
            gpu_driver_version,
            motherboard,
            memory_total: i18n::t(lang, "home_gb_total").replacen(
                "{:.1}",
                &format!("{total_memory_gb:.1}"),
                1,
            ),
            memory_used: i18n::t(lang, "home_gb_used").replacen(
                "{:.1}",
                &format!("{used_memory_gb:.1}"),
                1,
            ),
            memory_total_gb: total_memory_gb,
            memory_used_gb: used_memory_gb,
            storage_total: i18n::t(lang, "home_tb_total").replacen(
                "{:.2}",
                &format!("{total_disk_gb:.2}"),
                1,
            ),
            storage_used: i18n::t(lang, "home_tb_used").replacen(
                "{:.2}",
                &format!("{used_disk_gb:.2}"),
                1,
            ),
            storage_total_gb: total_disk_gb,
            storage_used_gb: used_disk_gb,
            cpu_usage,
            cpu_usage_percent,
            ram_details,
            display_info,
            uptime: format!(
                "{}d {:02}h {:02}m",
                uptime_secs / 86_400,
                (uptime_secs % 86_400) / 3_600,
                (uptime_secs % 3_600) / 60
            ),
        }
    }

    fn read_cpu_speed(system: &sysinfo::System) -> String {
        system
            .cpus()
            .first()
            .map(|cpu| {
                let mhz = cpu.frequency();
                if mhz >= 1000 {
                    format!("{:.2} GHz", mhz as f64 / 1000.0)
                } else if mhz > 0 {
                    format!("{} MHz", mhz)
                } else {
                    String::new()
                }
            })
            .unwrap_or_default()
    }

    fn read_ram_details() -> String {
        use std::sync::OnceLock;
        static CACHE: OnceLock<String> = OnceLock::new();
        CACHE
            .get_or_init(|| {
                let cmd = r#"
$sticks = Get-CimInstance Win32_PhysicalMemory -ErrorAction SilentlyContinue
if (-not $sticks) { exit }
$speed = ($sticks | Select-Object -First 1).Speed
$typeCode = ($sticks | Select-Object -First 1).SMBIOSMemoryType
$type = switch ($typeCode) { 26 {'DDR4'} 34 {'DDR5'} default {"DDR"}}
$count = $sticks.Count
$each = [math]::Round(($sticks | Select-Object -First 1).Capacity / 1GB, 0)
Write-Output "$count x $each GB $speed MHz $type"
"#;
                Self::ps_lines(cmd)
                    .into_iter()
                    .next()
                    .unwrap_or_default()
            })
            .clone()
    }

    fn read_display_info() -> String {
        let cmd = r#"
$dm = Get-CimInstance Win32_VideoController -ErrorAction SilentlyContinue | Where-Object { $_.CurrentHorizontalResolution -gt 0 } | Select-Object -First 1
if ($dm) {
    "$($dm.CurrentHorizontalResolution)x$($dm.CurrentVerticalResolution) @ $($dm.CurrentRefreshRate)Hz"
}
"#;
        Self::ps_lines(cmd)
            .into_iter()
            .next()
            .unwrap_or_default()
    }

    fn read_gpu_info(lang: Language) -> (String, Option<String>, String) {
        use windows::Win32::Graphics::Dxgi::{
            CreateDXGIFactory1, DXGI_ADAPTER_FLAG_SOFTWARE, IDXGIFactory1,
        };
        unsafe {
            let factory: IDXGIFactory1 = match CreateDXGIFactory1() {
                Ok(factory) => factory,
                Err(_) => {
                    return (
                        i18n::t(lang, "home_unknown_gpu").to_string(),
                        None,
                        i18n::t(lang, "home_unknown_vram").to_string(),
                    );
                }
            };
            for idx in 0..16 {
                let adapter = match factory.EnumAdapters1(idx) {
                    Ok(adapter) => adapter,
                    Err(_) => break,
                };
                let desc = match adapter.GetDesc1() {
                    Ok(desc) => desc,
                    Err(_) => continue,
                };
                if (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32) != 0 {
                    continue;
                }
                let name = String::from_utf16_lossy(&desc.Description)
                    .trim_matches(char::from(0))
                    .trim()
                    .to_string();
                let memory_gb = desc.DedicatedVideoMemory as f64 / 1024.0 / 1024.0 / 1024.0;
                return (
                    if name.is_empty() {
                        i18n::t(lang, "home_unknown_gpu").to_string()
                    } else {
                        name
                    },
                    None,
                    format!("{memory_gb:.0} GB VRAM"),
                );
            }
        }
        (
            i18n::t(lang, "home_unknown_gpu").to_string(),
            None,
            i18n::t(lang, "home_unknown_vram").to_string(),
        )
    }

    fn read_system_info(lang: Language) -> (String, String, String, String) {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
        let bios_key = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\BIOS");
        let model = bios_key
            .as_ref()
            .ok()
            .and_then(|k| k.get_value::<String, _>("SystemProductName").ok())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| i18n::t(lang, "home_unknown_model").to_string());
        let manufacturer = bios_key
            .as_ref()
            .ok()
            .and_then(|k| k.get_value::<String, _>("SystemManufacturer").ok())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| i18n::t(lang, "home_unknown_vendor").to_string());
        let bios_version = bios_key
            .as_ref()
            .ok()
            .and_then(|k| k.get_value::<String, _>("BIOSVersion").ok())
            .unwrap_or_else(|| i18n::t(lang, "home_unknown_bios").to_string());
        let bios_date = bios_key
            .as_ref()
            .ok()
            .and_then(|k| k.get_value::<String, _>("BIOSReleaseDate").ok())
            .unwrap_or_else(|| i18n::t(lang, "home_unknown_date").to_string());
        (model, manufacturer, bios_version, bios_date)
    }

    fn read_windows_build(_lang: Language) -> String {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
        hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
            .ok()
            .and_then(|k| k.get_value::<String, _>("CurrentBuildNumber").ok())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    fn read_gpu_driver_version(gpu_name: &str) -> String {
        use winreg::enums::*;
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let Ok(class_key) = hklm.open_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}",
            KEY_READ,
        ) else {
            return String::new();
        };
        let gpu_name_norm = gpu_name.to_lowercase();
        let mut fallback = String::new();
        for sub_name in class_key.enum_keys().flatten() {
            if !sub_name.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }
            let Ok(sub_key) = class_key.open_subkey_with_flags(&sub_name, KEY_READ) else {
                continue;
            };
            let Ok(driver_desc) = sub_key.get_value::<String, _>("DriverDesc") else {
                continue;
            };
            let driver_version: String = sub_key
                .get_value::<String, _>("DriverVersion")
                .unwrap_or_default();
            let driver_date: String = sub_key
                .get_value::<String, _>("DriverDate")
                .unwrap_or_default();
            if driver_version.is_empty() {
                continue;
            }
            let info = if driver_date.is_empty() {
                driver_version.clone()
            } else {
                format!("{} — {}", driver_version, driver_date)
            };
            if fallback.is_empty() {
                fallback = info.clone();
            }
            let desc_norm = driver_desc.to_lowercase();
            if desc_norm == gpu_name_norm
                || desc_norm.contains(&gpu_name_norm)
                || gpu_name_norm.contains(&desc_norm)
            {
                return info;
            }
        }
        fallback
    }
}
