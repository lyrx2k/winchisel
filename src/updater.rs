use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};

#[derive(Debug)]
pub enum MsiDownloadEvent {
    Progress(u64, u64),
    Done(Result<std::path::PathBuf, String>),
}

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
    // 1. Fetch release info to get SHA256 from body
    let api_url = format!(
        "https://api.github.com/repos/{}/releases/tags/{}",
        REPO, tag
    );
    let mut api_response = ureq::get(&api_url)
        .header("User-Agent", "Winchisel-Updater")
        .call()
        .map_err(|_| "update_error_check_updates".to_string())?;

    let api_body = api_response
        .body_mut()
        .read_to_string()
        .map_err(|_| "update_error_read_response".to_string())?;

    let json: serde_json::Value =
        serde_json::from_str(&api_body).map_err(|_| "update_error_parse_json".to_string())?;

    let release_body = json
        .get("body")
        .and_then(|v| v.as_str())
        .ok_or("update_error_no_body".to_string())?;

    let expected_hash =
        extract_sha256_from_body(release_body).ok_or("update_error_hash_not_found".to_string())?;

    // 2. Download EXE
    let exe_url = format!(
        "https://github.com/{}/releases/download/{}/Winchisel.exe",
        REPO, tag
    );
    let mut response = ureq::get(&exe_url)
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
        let _ = fs::remove_file(&update_exe);
        return Err(format!("update_error_download_too_small:{}", bytes));
    }

    // 3. Verify SHA256
    let actual_hash =
        compute_sha256(&update_exe).map_err(|_| "update_error_hash_compute".to_string())?;

    if actual_hash != expected_hash {
        let _ = fs::remove_file(&update_exe);
        return Err("update_error_hash_mismatch".to_string());
    }

    let current_exe =
        std::env::current_exe().map_err(|_| "update_error_resolve_current_exe".to_string())?;

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

pub fn download_msi(
    tag: &str,
    event_tx: std::sync::mpsc::Sender<MsiDownloadEvent>,
) -> Result<std::path::PathBuf, String> {
    let msi_url = format!(
        "https://github.com/{}/releases/download/{}/Winchisel_Installer.msi",
        REPO, tag
    );
    let mut response = ureq::get(&msi_url)
        .header("User-Agent", "Winchisel-Updater")
        .call()
        .map_err(|_| "update_error_download".to_string())?;

    let total_size = response
        .headers()
        .get("Content-Length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let temp_dir = std::env::temp_dir();
    let update_msi = temp_dir.join("Winchisel_Installer.msi");

    let mut file =
        fs::File::create(&update_msi).map_err(|_| "update_error_create_temp_file".to_string())?;

    let mut reader = response.body_mut().as_reader();
    let mut buffer = [0u8; 8192];
    let mut downloaded = 0u64;

    loop {
        let n = reader
            .read(&mut buffer)
            .map_err(|_| "update_error_download".to_string())?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])
            .map_err(|_| "update_error_write_update_file".to_string())?;
        downloaded += n as u64;
        let _ = event_tx.send(MsiDownloadEvent::Progress(downloaded, total_size));
    }
    drop(file);

    if downloaded < 100_000 {
        let _ = fs::remove_file(&update_msi);
        return Err(format!("update_error_download_too_small:{}", downloaded));
    }

    Ok(update_msi)
}

pub fn install_msi(msi_path: &std::path::Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let bat = format!(
            "@echo off\r\nping -n 3 127.0.0.1 > nul\r\nmsiexec /i \"{}\" /qn\r\ndel \"{}\"\r\ndel \"%~f0\"\r\n",
            msi_path.display(),
            msi_path.display()
        );

        let bat_path = std::env::temp_dir().join("Winchisel_update.bat");
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

fn extract_sha256_from_body(body: &str) -> Option<String> {
    body.lines()
        .find(|line| line.trim_start().to_uppercase().starts_with("SHA256:"))
        .and_then(|line| line.split(':').nth(1).map(|s| s.trim().to_lowercase()))
}

fn compute_sha256(path: &std::path::Path) -> Result<String, std::io::Error> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect())
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
