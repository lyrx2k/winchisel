use super::{HomeState, WinchiselApp};
use crate::{Language, i18n};
struct HomeStaticSnapshot {
    computer_name: String,
    os_version: String,
    kernel_version: String,
    bios_version: String,
    bios_date: String,
    cpu_brand: String,
    cpu_cores: String,
    gpu_name: String,
}

impl WinchiselApp {
    pub(crate) fn build_home_state(lang: Language) -> HomeState {
        use sysinfo::System;
        let mut system = System::new();
        system.refresh_memory();
        system.refresh_cpu_usage();
        let snapshot = {
            let cpu_brand = system
                .cpus()
                .first()
                .map(|cpu| cpu.brand().to_string())
                .unwrap_or_else(|| i18n::t(lang, "home_unknown_cpu").to_string());
            let cpu_cores = System::physical_core_count()
                .or_else(|| Some(system.cpus().len()))
                .unwrap_or(0);
            let (gpu_name, _, _) = Self::read_gpu_info(lang);
            let (_system_model, _system_manufacturer, bios_version, bios_date) =
                Self::read_system_info(lang);
            HomeStaticSnapshot {
                computer_name: System::host_name()
                    .unwrap_or_else(|| i18n::t(lang, "home_unknown_pc").to_string()),
                os_version: System::os_version()
                    .unwrap_or_else(|| i18n::t(lang, "home_unknown_os").to_string()),
                kernel_version: System::kernel_version()
                    .unwrap_or_else(|| i18n::t(lang, "home_unknown_kernel").to_string()),
                bios_version,
                bios_date,
                cpu_brand,
                cpu_cores: i18n::t(lang, "home_cores_suffix").replacen(
                    "{}",
                    &cpu_cores.to_string(),
                    1,
                ),
                gpu_name: if gpu_name.is_empty() {
                    i18n::t(lang, "home_unknown_gpu").to_string()
                } else {
                    gpu_name
                },
            }
        };
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
        let cpu_usage = format!("{:.0}%", system.global_cpu_usage());
        let uptime_secs = System::uptime();

        HomeState {
            computer_name: snapshot.computer_name,
            os_version: snapshot.os_version,
            kernel_version: snapshot.kernel_version,
            bios_version: snapshot.bios_version,
            bios_date: snapshot.bios_date,
            cpu_brand: snapshot.cpu_brand,
            cpu_cores: snapshot.cpu_cores,
            gpu_name: snapshot.gpu_name,
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
            cpu_usage,
            uptime: format!(
                "{}d {:02}h {:02}m",
                uptime_secs / 86_400,
                (uptime_secs % 86_400) / 3_600,
                (uptime_secs % 3_600) / 60
            ),
        }
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
}
