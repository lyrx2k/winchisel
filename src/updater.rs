const REPO: &str = "lyrx2k/Winchisel";

pub fn check_for_update() -> Result<Option<String>, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);
    let mut response = ureq::get(&url)
        .header("User-Agent", "Winchisel-Updater")
        .call()
        .map_err(|e| format!("Failed to check updates: {}", e))?;
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("Failed to read response: {}", e))?;
    let json: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    let latest_tag = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .ok_or("No tag_name in response".to_string())?;
    let current = format!("v{}", env!("CARGO_PKG_VERSION"));
    if is_newer(&current, latest_tag) {
        Ok(Some(latest_tag.to_string()))
    } else {
        Ok(None)
    }
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
