"""Unit tests for quiz_app.storage."""
import json
import pytest
from pathlib import Path
from unittest.mock import patch

from quiz_app.models import MasteryRecord, UserProfile
from quiz_app import storage


def _tmp_dir(tmp_path: Path):
    """Patch PROGRESS_DIR to an isolated temp directory."""
    return patch.object(storage, "PROGRESS_DIR", tmp_path)


class TestProfilePath:
    def test_sanitizes_spaces(self, tmp_path):
        with _tmp_dir(tmp_path):
            path = storage._profile_path("John Doe")
        assert " " not in path.name

    def test_sanitizes_special_chars(self, tmp_path):
        with _tmp_dir(tmp_path):
            path = storage._profile_path("Alice/Bob!?")
        assert "/" not in path.name
        assert "!" not in path.name
        assert "?" not in path.name

    def test_lowercase(self, tmp_path):
        with _tmp_dir(tmp_path):
            path = storage._profile_path("UPPERCASE")
        assert path.name == path.name.lower()


class TestSaveAndLoad:
    def test_save_then_load(self, tmp_path):
        with _tmp_dir(tmp_path):
            profile = UserProfile(user_id="u1", name="Alice")
            profile.mastery[0] = MasteryRecord(score=0.75, total_seen=4, total_correct=3)
            storage.save_profile(profile)
            loaded = storage.load_profile("Alice")

        assert loaded is not None
        assert loaded.name == "Alice"
        assert loaded.user_id == "u1"
        assert loaded.mastery[0].score == pytest.approx(0.75)

    def test_load_nonexistent_returns_none(self, tmp_path):
        with _tmp_dir(tmp_path):
            result = storage.load_profile("Nobody")
        assert result is None

    def test_save_is_atomic(self, tmp_path):
        """tmp file should not exist after save completes."""
        with _tmp_dir(tmp_path):
            profile = UserProfile(user_id="u1", name="Bob")
            storage.save_profile(profile)
            tmp_file = tmp_path / "bob.tmp"
        assert not tmp_file.exists()

    def test_corrupt_json_returns_none(self, tmp_path):
        with _tmp_dir(tmp_path):
            bad = tmp_path / "corrupt.json"
            bad.write_text("{not valid json")
            result = storage.load_profile("corrupt")
        assert result is None

    def test_load_by_name_fallback(self, tmp_path):
        """load_profile searches all files if the expected filename mismatches."""
        with _tmp_dir(tmp_path):
            profile = UserProfile(user_id="u2", name="Charlie")
            storage.save_profile(profile)
            # Rename file so expected path won't match
            (tmp_path / "charlie.json").rename(tmp_path / "other.json")
            loaded = storage.load_profile("Charlie")
        assert loaded is not None
        assert loaded.name == "Charlie"


class TestCreateProfile:
    def test_creates_and_saves(self, tmp_path):
        with _tmp_dir(tmp_path):
            profile = storage.create_profile("Dana")
        assert profile.name == "Dana"
        assert profile.user_id != ""
        assert profile.onboarded is False

    def test_creates_unique_ids(self, tmp_path):
        with _tmp_dir(tmp_path):
            p1 = storage.create_profile("E1")
            p2 = storage.create_profile("E2")
        assert p1.user_id != p2.user_id


class TestLoadOrCreate:
    def test_loads_existing(self, tmp_path):
        with _tmp_dir(tmp_path):
            storage.create_profile("Frank")
            loaded = storage.load_or_create("Frank")
        assert loaded.name == "Frank"

    def test_creates_if_absent(self, tmp_path):
        with _tmp_dir(tmp_path):
            profile = storage.load_or_create("Grace")
        assert profile.name == "Grace"


class TestDeleteProfile:
    def test_deletes_existing(self, tmp_path):
        with _tmp_dir(tmp_path):
            storage.create_profile("Hank")
            result = storage.delete_profile("Hank")
            still_there = storage.load_profile("Hank")
        assert result is True
        assert still_there is None

    def test_returns_false_if_not_found(self, tmp_path):
        with _tmp_dir(tmp_path):
            result = storage.delete_profile("Nobody")
        assert result is False


class TestListProfiles:
    def test_empty_directory(self, tmp_path):
        with _tmp_dir(tmp_path):
            names = storage.list_profiles()
        assert names == []

    def test_lists_saved_profiles(self, tmp_path):
        with _tmp_dir(tmp_path):
            storage.create_profile("Zoe")
            storage.create_profile("Adam")
            names = storage.list_profiles()
        assert sorted(names) == ["Adam", "Zoe"]

    def test_ignores_corrupt_files(self, tmp_path):
        with _tmp_dir(tmp_path):
            (tmp_path / "bad.json").write_text("!!!invalid!!!")
            storage.create_profile("Valid")
            names = storage.list_profiles()
        assert names == ["Valid"]
