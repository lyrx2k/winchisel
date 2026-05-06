use super::{HomeState, WinchiselApp};
impl WinchiselApp {
    pub(crate) fn build_home_state() -> HomeState {
        use sysinfo::{ProcessesToUpdate, System};

        let mut system = System::new();
        system.refresh_memory();
        system.refresh_cpu_usage();
        let _ = system.refresh_processes(ProcessesToUpdate::All, true);

        let cpu_brand = system
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
        let cpu_cores = System::physical_core_count()
            .or_else(|| Some(system.cpus().len()))
            .unwrap_or(0);
        let (gpu_name, _, _) = Self::read_gpu_info();
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
        let (_system_model, _system_manufacturer, bios_version, bios_date) = Self::read_system_info();
        let cpu_usage = format!("{:.0}%", system.global_cpu_usage());
        let uptime_secs = System::uptime();

        HomeState {
            computer_name: System::host_name().unwrap_or_else(|| "Unknown PC".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown OS".to_string()),
            kernel_version: System::kernel_version()
                .unwrap_or_else(|| "Unknown kernel".to_string()),
            bios_version,
            bios_date,
            cpu_brand,
            cpu_cores: format!("{} cores", cpu_cores),
            gpu_name,
            memory_total: format!("{total_memory_gb:.1} GB total"),
            memory_used: format!("{used_memory_gb:.1} GB used"),
            storage_total: format!("{total_disk_gb:.2} TB total"),
            storage_used: format!("{used_disk_gb:.2} TB used"),
            cpu_usage,
            uptime: format!(
                "{}d {:02}h {:02}m",
                uptime_secs / 86_400,
                (uptime_secs % 86_400) / 3_600,
                (uptime_secs % 3_600) / 60
            ),
        }
    }

    fn read_gpu_info() -> (String, Option<String>, String) {
        use windows::Win32::Graphics::Dxgi::{
            CreateDXGIFactory1, DXGI_ADAPTER_FLAG_SOFTWARE, IDXGIFactory1,
        };
        unsafe {
            let factory: IDXGIFactory1 = match CreateDXGIFactory1() {
                Ok(factory) => factory,
                Err(_) => return ("Unknown GPU".to_string(), None, "Unknown VRAM".to_string()),
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
                        "Unknown GPU".to_string()
                    } else {
                        name
                    },
                    None,
                    format!("{memory_gb:.0} GB VRAM"),
                );
            }
        }
        ("Unknown GPU".to_string(), None, "Unknown VRAM".to_string())
    }

    fn read_system_info() -> (String, String, String, String) {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
        let bios_key = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\BIOS");
        let model = bios_key
            .as_ref()
            .ok()
            .and_then(|k| k.get_value::<String, _>("SystemProductName").ok())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "Unknown Model".to_string());
        let manufacturer = bios_key
            .as_ref()
            .ok()
            .and_then(|k| k.get_value::<String, _>("SystemManufacturer").ok())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "Unknown Vendor".to_string());
        let bios_version = bios_key
            .as_ref()
            .ok()
            .and_then(|k| k.get_value::<String, _>("BIOSVersion").ok())
            .unwrap_or_else(|| "Unknown BIOS".to_string());
        let bios_date = bios_key
            .as_ref()
            .ok()
            .and_then(|k| k.get_value::<String, _>("BIOSReleaseDate").ok())
            .unwrap_or_else(|| "Unknown Date".to_string());
        (model, manufacturer, bios_version, bios_date)
    }

}
