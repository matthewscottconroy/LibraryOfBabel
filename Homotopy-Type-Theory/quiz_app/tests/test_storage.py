"""Tests for storage.py — save/load/delete UserProfile on disk."""
import json
import pytest

from quiz_app.models import MasteryRecord, UserProfile
from quiz_app import storage


# ── Fixtures ──────────────────────────────────────────────────────────────────

@pytest.fixture(autouse=True)
def temp_progress_dir(tmp_path, monkeypatch):
    """Redirect all storage operations to a temp directory."""
    monkeypatch.setattr(storage, "PROGRESS_DIR", tmp_path)
    # Also patch _profile_path to use the tmp directory
    original_fn = storage._profile_path
    def patched(name):
        p = original_fn(name)
        return tmp_path / p.name
    monkeypatch.setattr(storage, "_profile_path", patched)
    yield tmp_path


def _make_profile(name="Alice") -> UserProfile:
    p = UserProfile(user_id="u-test", name=name)
    p.mastery[3] = MasteryRecord(score=0.7, total_seen=5, total_correct=4)
    return p


# ── save & load ───────────────────────────────────────────────────────────────

class TestSaveLoad:
    def test_save_creates_file(self, temp_progress_dir):
        p = _make_profile()
        storage.save_profile(p)
        files = list(temp_progress_dir.glob("*.json"))
        assert len(files) == 1

    def test_load_returns_equivalent_profile(self, temp_progress_dir):
        p = _make_profile()
        storage.save_profile(p)
        loaded = storage.load_profile(p.name)
        assert loaded is not None
        assert loaded.name == p.name
        assert loaded.user_id == p.user_id
        assert loaded.mastery[3].score == pytest.approx(0.7)

    def test_load_nonexistent_returns_none(self, temp_progress_dir):
        assert storage.load_profile("nobody") is None

    def test_save_is_atomic_on_success(self, temp_progress_dir):
        """No .tmp file left behind after a successful save."""
        storage.save_profile(_make_profile())
        tmps = list(temp_progress_dir.glob("*.tmp"))
        assert tmps == []

    def test_overwrite_updates_profile(self, temp_progress_dir):
        p = _make_profile()
        storage.save_profile(p)
        p.mastery[3].score = 0.95
        storage.save_profile(p)
        loaded = storage.load_profile(p.name)
        assert loaded.mastery[3].score == pytest.approx(0.95)

    def test_corrupted_file_returns_none(self, temp_progress_dir):
        p = _make_profile()
        storage.save_profile(p)
        # Corrupt the JSON
        path = next(temp_progress_dir.glob("*.json"))
        path.write_text("{ not valid json !!!")
        assert storage.load_profile(p.name) is None


# ── load_or_create ────────────────────────────────────────────────────────────

class TestLoadOrCreate:
    def test_creates_when_absent(self, temp_progress_dir):
        p = storage.load_or_create("Bob")
        assert p.name == "Bob"
        assert (temp_progress_dir / "bob.json").exists()

    def test_loads_when_present(self, temp_progress_dir):
        p = _make_profile("Carol")
        storage.save_profile(p)
        loaded = storage.load_or_create("Carol")
        assert loaded.user_id == p.user_id


# ── list_profiles ─────────────────────────────────────────────────────────────

class TestListProfiles:
    def test_empty_directory_returns_empty(self, temp_progress_dir):
        assert storage.list_profiles() == []

    def test_returns_names_sorted(self, temp_progress_dir):
        for name in ["Charlie", "Alice", "Bob"]:
            storage.save_profile(_make_profile(name))
        names = storage.list_profiles()
        assert names == sorted(names)
        assert set(names) == {"Alice", "Bob", "Charlie"}

    def test_ignores_non_json_files(self, temp_progress_dir):
        (temp_progress_dir / "notes.txt").write_text("hello")
        assert storage.list_profiles() == []


# ── delete_profile ────────────────────────────────────────────────────────────

class TestDeleteProfile:
    def test_delete_existing_returns_true(self, temp_progress_dir):
        p = _make_profile()
        storage.save_profile(p)
        assert storage.delete_profile(p.name) is True
        assert storage.load_profile(p.name) is None

    def test_delete_nonexistent_returns_false(self, temp_progress_dir):
        assert storage.delete_profile("nobody") is False


# ── create_profile ────────────────────────────────────────────────────────────

class TestCreateProfile:
    def test_creates_file_on_disk(self, temp_progress_dir):
        p = storage.create_profile("Dave")
        files = list(temp_progress_dir.glob("*.json"))
        assert len(files) == 1

    def test_returned_profile_has_correct_name(self, temp_progress_dir):
        p = storage.create_profile("Eve")
        assert p.name == "Eve"

    def test_returned_profile_has_user_id(self, temp_progress_dir):
        p = storage.create_profile("Frank")
        assert p.user_id != ""

    def test_creates_loadable_profile(self, temp_progress_dir):
        storage.create_profile("Grace")
        loaded = storage.load_profile("Grace")
        assert loaded is not None
        assert loaded.name == "Grace"


# ── _profile_path name normalization ─────────────────────────────────────────

class TestProfilePath:
    def test_uppercase_normalised(self):
        from quiz_app.storage import _profile_path
        p = _profile_path("ALICE")
        assert "alice" in p.name

    def test_spaces_become_underscores(self):
        from quiz_app.storage import _profile_path
        p = _profile_path("John Doe")
        assert " " not in p.name
        assert "john_doe" in p.name

    def test_special_chars_replaced(self):
        from quiz_app.storage import _profile_path
        p = _profile_path("user@host.com")
        assert "@" not in p.name
        assert "." not in p.name.replace(".json", "")

    def test_same_name_same_path(self):
        from quiz_app.storage import _profile_path
        assert _profile_path("Alice") == _profile_path("Alice")


# ── load_profile fallback (name search) ──────────────────────────────────────

class TestLoadProfileFallback:
    def test_fallback_search_by_name_case_insensitive(self, temp_progress_dir):
        """
        If a profile file doesn't match the normalised path (e.g. written
        under a different encoding), load_profile falls back to scanning all
        files for a matching name field.
        """
        import json, uuid
        from quiz_app.models import UserProfile
        # Write a profile file with a name that won't match the normalised path
        p = UserProfile(user_id=str(uuid.uuid4()), name="ZéRo")
        data = json.dumps(p.to_dict())
        # Store it under an unusual filename (not what _profile_path would pick)
        (temp_progress_dir / "unusual_name.json").write_text(data)
        loaded = storage.load_profile("ZéRo")
        assert loaded is not None
        assert loaded.name == "ZéRo"

    def test_fallback_returns_none_when_no_match(self, temp_progress_dir):
        assert storage.load_profile("completely_unknown_xyz") is None
