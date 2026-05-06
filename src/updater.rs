use std::fs;

const REPO: &str = "lyrx2k/Winchisel";

pub fn check_for_update() -> Result<Option<String>, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);
    let mut response = ureq::get(&url)
        .header("User-Agent", "Winchisel-Updater")
        .call()
        .map_err(|_| "update_error_check_updates".to_string())?;
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|_| "update_error_read_response".to_string())?;
    let json: serde_json::Value =
        serde_json::from_str(&body).map_err(|_| "update_error_parse_json".to_string())?;
    let latest_tag = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .ok_or("update_error_no_tag_name".to_string())?;
    let current = format!("v{}", env!("CARGO_PKG_VERSION"));
    if is_newer(&current, latest_tag) {
        Ok(Some(latest_tag.to_string()))
    } else {
        Ok(None)
    }
}

pub fn download_and_install(tag: &str) -> Result<(), String> {
    let url = format!(
        "https://github.com/{}/releases/download/{}/Winchisel.exe",
        REPO, tag
    );

    let mut response = ureq::get(&url)
        .header("User-Agent", "Winchisel-Updater")
        .call()
        .map_err(|_| "update_error_download".to_string())?;

    let temp_dir = std::env::temp_dir();
    let update_exe = temp_dir.join("Winchisel_update.exe");

    let mut file =
        fs::File::create(&update_exe).map_err(|_| "update_error_create_temp_file".to_string())?;

    let bytes = std::io::copy(&mut response.body_mut().as_reader(), &mut file)
        .map_err(|_| "update_error_write_update_file".to_string())?;
    drop(file);

    if bytes < 100_000 {
        return Err(format!("update_error_download_too_small:{}", bytes));
    }

    let current_exe = std::env::current_exe()
        .map_err(|_| "update_error_resolve_current_exe".to_string())?;

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let bat = format!(
            "@echo off\r\nping -n 3 127.0.0.1 > nul\r\nmove /y \"{}\" \"{}\"\r\nstart \"\" \"{}\"\r\ndel \"%~f0\"\r\n",
            update_exe.display(),
            current_exe.display(),
            current_exe.display()
        );

        let bat_path = temp_dir.join("Winchisel_update.bat");
        fs::write(&bat_path, bat.as_bytes())
            .map_err(|_| "update_error_write_update_script".to_string())?;

        std::process::Command::new("cmd")
            .args(["/c", bat_path.to_string_lossy().as_ref()])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|_| "update_error_launch_updater".to_string())?;
    }

    std::process::exit(0);
}

fn is_newer(current: &str, latest: &str) -> bool {
    let parse = |v: &str| -> (u32, u32, u32) {
        let v = v.trim_start_matches('v');
        let parts: Vec<u32> = v.split('.').filter_map(|p| p.parse().ok()).collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    };
    parse(latest) > parse(current)
}
