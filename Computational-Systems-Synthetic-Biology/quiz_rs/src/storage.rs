use std::fs;
use std::path::PathBuf;

use crate::config::progress_dir;
use crate::models::UserProfile;

// ── Internal helpers ──────────────────────────────────────────────────────────

fn profile_path(name: &str) -> PathBuf {
    let safe: String = name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c.to_ascii_lowercase() } else { '_' })
        .collect();
    progress_dir().join(format!("{}.json", safe))
}

fn ensure_dirs() {
    let d = progress_dir();
    if !d.exists() {
        let _ = fs::create_dir_all(&d);
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn list_profiles() -> Vec<String> {
    ensure_dirs();
    let dir = progress_dir();
    let Ok(entries) = fs::read_dir(&dir) else { return Vec::new(); };
    let mut names: Vec<String> = entries
        .filter_map(|e| {
            let e = e.ok()?;
            let path = e.path();
            if path.extension()?.to_str()? != "json" { return None; }
            let data = fs::read_to_string(&path).ok()?;
            let v: serde_json::Value = serde_json::from_str(&data).ok()?;
            v["name"].as_str().map(|s| s.to_string())
        })
        .collect();
    names.sort();
    names
}

pub fn load_profile(name: &str) -> Option<UserProfile> {
    ensure_dirs();
    let path = profile_path(name);
    if path.exists() {
        match fs::read_to_string(&path).ok().and_then(|s| serde_json::from_str(&s).ok()) {
            Some(p) => return Some(p),
            None => {
                // corrupted — rename to backup
                let backup = path.with_extension("backup.json");
                let _ = fs::rename(&path, &backup);
                eprintln!(
                    "\n  Warning: profile '{}' was corrupted. Backup saved as {}.\n",
                    name, backup.display()
                );
                return None;
            }
        }
    }
    // fallback: scan all profiles
    let dir = progress_dir();
    let Ok(entries) = fs::read_dir(&dir) else { return None; };
    for entry in entries.filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) != Some("json") { continue; }
        if let Ok(data) = fs::read_to_string(&p) {
            if let Ok(profile) = serde_json::from_str::<UserProfile>(&data) {
                if profile.name.to_lowercase() == name.to_lowercase() {
                    return Some(profile);
                }
            }
        }
    }
    None
}

pub fn save_profile(profile: &UserProfile) {
    ensure_dirs();
    let path = profile_path(&profile.name);
    let tmp  = path.with_extension("tmp");
    let data = serde_json::to_string_pretty(profile).expect("serialize profile");
    fs::write(&tmp, data).expect("write profile tmp");
    fs::rename(&tmp, &path).expect("rename profile");
}

pub fn create_profile(name: &str) -> UserProfile {
    let profile = UserProfile::new(name.to_string());
    save_profile(&profile);
    profile
}

pub fn load_or_create(name: &str) -> UserProfile {
    load_profile(name).unwrap_or_else(|| create_profile(name))
}
