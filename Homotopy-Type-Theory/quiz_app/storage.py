"""
Persistence layer — load/save UserProfile as JSON.
Also provides profile listing and creation helpers.
"""
from __future__ import annotations

import json
import uuid
from datetime import date
from pathlib import Path
from typing import Optional

from .config import PROGRESS_DIR
from .models import UserProfile


# ── Internal helpers ──────────────────────────────────────────────────────────

def _profile_path(name: str) -> Path:
    safe = "".join(c if c.isalnum() or c in "-_" else "_" for c in name.lower())
    return PROGRESS_DIR / f"{safe}.json"


# ── Public API ────────────────────────────────────────────────────────────────

def list_profiles() -> list[str]:
    """Return the names of all saved profiles (sorted)."""
    names = []
    for p in PROGRESS_DIR.glob("*.json"):
        try:
            data = json.loads(p.read_text())
            names.append(data["name"])
        except Exception:
            pass
    return sorted(names)


def list_profiles_with_summary() -> list[dict]:
    """Return rich profile metadata sorted by name."""
    summaries = []
    for p in PROGRESS_DIR.glob("*.json"):
        try:
            data = json.loads(p.read_text())
            mastery = data.get("mastery", {})
            total_answered = sum(
                v.get("total_seen", 0) for v in mastery.values()
            )
            today = date.today().isoformat()
            due_count = sum(
                1 for v in mastery.values()
                if v.get("next_review", "") and today >= v["next_review"]
            )
            summaries.append({
                "name":           data.get("name", "?"),
                "total_answered": total_answered,
                "last_seen":      data.get("last_seen", ""),
                "due_count":      due_count,
            })
        except Exception:
            pass
    return sorted(summaries, key=lambda s: s["name"])


def load_profile(name: str) -> Optional[UserProfile]:
    """Load a profile by name. Returns None if not found."""
    path = _profile_path(name)
    if not path.exists():
        # fallback: search all profiles for matching name
        for p in PROGRESS_DIR.glob("*.json"):
            try:
                data = json.loads(p.read_text())
                if data.get("name", "").lower() == name.lower():
                    return UserProfile.from_dict(data)
            except Exception:
                pass
        return None
    try:
        data = json.loads(path.read_text())
        return UserProfile.from_dict(data)
    except (json.JSONDecodeError, KeyError):
        backup = path.with_suffix(".backup.json")
        try:
            path.rename(backup)
            print(f"\n  Warning: profile '{name}' was corrupted and could not be loaded.")
            print(f"  A backup was saved to: {backup.name}")
            print(f"  Starting with a fresh profile.\n")
        except Exception:
            pass
        return None


def save_profile(profile: UserProfile) -> None:
    """Persist a UserProfile to disk (atomic write via temp file)."""
    path = _profile_path(profile.name)
    tmp  = path.with_suffix(".tmp")
    tmp.write_text(json.dumps(profile.to_dict(), indent=2))
    tmp.replace(path)


def create_profile(name: str) -> UserProfile:
    """Create and save a fresh UserProfile."""
    profile = UserProfile(user_id=str(uuid.uuid4()), name=name)
    save_profile(profile)
    return profile


def load_or_create(name: str) -> UserProfile:
    """Load an existing profile or create one if absent."""
    existing = load_profile(name)
    return existing if existing is not None else create_profile(name)


def delete_profile(name: str) -> bool:
    """Delete a profile by name. Returns True if it existed."""
    path = _profile_path(name)
    if path.exists():
        path.unlink()
        return True
    return False
