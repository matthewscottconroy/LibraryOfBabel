//! Persistence layer — load/save `UserProfile` as JSON.
//!
//! Profiles are stored in `{workspace_root}/data/profiles/{safe_name}.json`.
//! Writes are atomic: data is written to a `.tmp` file and then renamed so a
//! crash mid-write cannot leave a partially-written profile on disk.

use std::path::{Path, PathBuf};

use crate::profile::UserProfile;

// ── Path helpers ──────────────────────────────────────────────────────────────

/// Convert any name to a filesystem-safe identifier.
fn safe_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

/// Absolute path for a profile file given the profiles directory and user name.
fn profile_path(profiles_dir: &Path, name: &str) -> PathBuf {
    profiles_dir.join(format!("{}.json", safe_name(name)))
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Return the names of all saved profiles (sorted alphabetically).
///
/// `profiles_dir` is typically `{workspace_root}/data/profiles/`.
pub fn list_profiles(profiles_dir: &Path) -> Vec<String> {
    let mut names = Vec::new();
    let entries = match std::fs::read_dir(profiles_dir) {
        Ok(e) => e,
        Err(_) => return names,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&data) {
                if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
                    names.push(name.to_string());
                }
            }
        }
    }
    names.sort();
    names
}

/// Return rich metadata for each profile (name, total_answered, last_seen,
/// due_count), sorted by name.
pub fn list_profiles_with_summary(profiles_dir: &Path) -> Vec<ProfileSummary> {
    let entries = match std::fs::read_dir(profiles_dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let today = chrono::Local::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();

    let mut summaries: Vec<ProfileSummary> = entries
        .flatten()
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                == Some("json")
        })
        .filter_map(|e| {
            let data = std::fs::read_to_string(e.path()).ok()?;
            let v: serde_json::Value = serde_json::from_str(&data).ok()?;
            let name = v.get("name")?.as_str()?.to_string();
            let last_seen = v
                .get("last_seen")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            let mastery = v.get("mastery").and_then(|m| m.as_object())?;
            let total_answered: u64 = mastery
                .values()
                .filter_map(|rec| rec.get("total_seen")?.as_u64())
                .sum();
            let due_count = mastery
                .values()
                .filter(|rec| {
                    rec.get("next_review")
                        .and_then(|d| d.as_str())
                        .map_or(false, |d| !d.is_empty() && today.as_str() >= d)
                })
                .count() as u32;
            Some(ProfileSummary {
                name,
                total_answered: total_answered as u32,
                last_seen,
                due_count,
            })
        })
        .collect();

    summaries.sort_by(|a, b| a.name.cmp(&b.name));
    summaries
}

/// Load a profile by name from `profiles_dir`.
///
/// Falls back to a case-insensitive search across all files if the canonical
/// path doesn't exist.  Returns `None` if not found or corrupt.
pub fn load_profile(profiles_dir: &Path, name: &str) -> Option<UserProfile> {
    let path = profile_path(profiles_dir, name);
    if path.exists() {
        return try_load_profile(&path);
    }
    // Fallback: scan all files for a name match
    for entry in std::fs::read_dir(profiles_dir).ok()?.flatten() {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        if let Ok(data) = std::fs::read_to_string(&p) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&data) {
                if v.get("name")
                    .and_then(|n| n.as_str())
                    .map_or(false, |n| n.to_lowercase() == name.to_lowercase())
                {
                    return try_load_profile(&p);
                }
            }
        }
    }
    None
}

/// Persist a `UserProfile` to disk using an atomic write.
pub fn save_profile(profiles_dir: &Path, profile: &UserProfile) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(profiles_dir)?;
    let path = profile_path(profiles_dir, &profile.name);
    let tmp = path.with_extension("tmp");
    let json = serde_json::to_string_pretty(profile)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(&tmp, &json)?;
    std::fs::rename(&tmp, &path)?;
    Ok(())
}

/// Create and immediately save a fresh profile with `name`.
pub fn create_profile(profiles_dir: &Path, name: &str) -> Result<UserProfile, std::io::Error> {
    let profile = UserProfile::new(name);
    save_profile(profiles_dir, &profile)?;
    Ok(profile)
}

/// Load an existing profile or create one if none exists.
pub fn load_or_create(profiles_dir: &Path, name: &str) -> Result<UserProfile, std::io::Error> {
    if let Some(p) = load_profile(profiles_dir, name) {
        return Ok(p);
    }
    create_profile(profiles_dir, name)
}

/// Delete a profile by name.  Returns `true` if the file existed.
pub fn delete_profile(profiles_dir: &Path, name: &str) -> bool {
    let path = profile_path(profiles_dir, name);
    if path.exists() {
        std::fs::remove_file(&path).is_ok()
    } else {
        false
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Attempt to deserialise a `UserProfile` from `path`.
/// On corrupt JSON: backs up the file and returns `None`.
fn try_load_profile(path: &Path) -> Option<UserProfile> {
    let data = std::fs::read_to_string(path).ok()?;
    match serde_json::from_str::<UserProfile>(&data) {
        Ok(p) => Some(p),
        Err(_) => {
            // Rename to .backup.json so the user can inspect it
            let backup = path.with_extension("backup.json");
            let _ = std::fs::rename(path, &backup);
            eprintln!(
                "Warning: profile at '{}' was corrupt and has been backed up to '{}'.",
                path.display(),
                backup.display()
            );
            None
        }
    }
}

// ── Summary type ──────────────────────────────────────────────────────────────

/// Brief metadata returned by `list_profiles_with_summary`.
#[derive(Debug, Clone)]
pub struct ProfileSummary {
    pub name: String,
    pub total_answered: u32,
    pub last_seen: String,
    pub due_count: u32,
}
