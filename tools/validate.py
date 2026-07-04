#!/usr/bin/env python3
"""
validate.py — repo-wide validation for the LibraryOfBabel textbooks.

Runs a battery of per-book checks and prints a report.  Exits non-zero if any
*unexpected* failure is found.  Failures that are currently expected (because a
concurrent cleanup is in flight) are allowlisted in
``tools/validate-exceptions.toml`` and reported separately without affecting the
exit code.

Checks
------
    a. subject.toml   — located at <book>/subject.toml OR <book>/quiz-app/
                        subject.toml; every chapters[].file must resolve
                        against chapters_dir.
    b. questions      — every <book>/questions/**/*.json (except files starting
                        with '_') must parse and carry the required fields.
    c. build coverage — tools/build_book.py --check for each book.
    d. links          — every relative markdown link in each book's README.md
                        and the root README.md must resolve.
    e. zero-byte      — census of zero-byte / whitespace-only .md files.

Usage
-----
    python3 tools/validate.py            # full run
    python3 tools/validate.py --book X   # single book (+ root-level checks)
    python3 tools/validate.py --list-exceptions
"""

from __future__ import annotations

import argparse
import fnmatch
import json
import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BUILD = ROOT / "tools" / "build_book.py"
EXCEPTIONS_FILE = ROOT / "tools" / "validate-exceptions.toml"

VALID_KINDS = {"mc", "tf", "blank"}
VALID_DIFFICULTY = {"beginner", "intermediate", "advanced"}
REQUIRED_QUESTION_FIELDS = (
    "kind", "text", "choices", "answer", "explanation", "difficulty", "chapter",
)

LINK_RE = re.compile(r"\[[^\]]*\]\(([^)]+)\)")


# --------------------------------------------------------------------------- #
# Failure / exception model
# --------------------------------------------------------------------------- #

class Failure:
    def __init__(self, book: str, check: str, detail: str, message: str):
        self.book = book
        self.check = check      # subject_toml | questions | build_check | links | zero_byte
        self.detail = detail    # a stable string (path) an exception can match
        self.message = message

    def __str__(self):
        return f"[{self.check}] {self.detail}: {self.message}"


def load_exceptions() -> list[dict]:
    if not EXCEPTIONS_FILE.exists():
        return []
    with EXCEPTIONS_FILE.open("rb") as fh:
        data = tomllib.load(fh)
    return data.get("exception", [])


def is_expected(f: Failure, exceptions: list[dict]) -> dict | None:
    for exc in exceptions:
        if exc.get("book") not in (f.book, "*", None):
            continue
        if exc.get("check") not in (f.check, "*", None):
            continue
        pattern = exc.get("pattern", "*")
        if fnmatch.fnmatch(f.detail, pattern):
            return exc
    return None


# --------------------------------------------------------------------------- #
# Discovery
# --------------------------------------------------------------------------- #

def book_dirs() -> list[Path]:
    return sorted(
        p for p in ROOT.iterdir()
        if p.is_dir() and (p / "book.toml").exists()
    )


def find_subject_toml(book: Path) -> Path | None:
    for cand in (book / "subject.toml", book / "quiz-app" / "subject.toml"):
        if cand.exists():
            return cand
    return None


# --------------------------------------------------------------------------- #
# Checks
# --------------------------------------------------------------------------- #

def check_subject_toml(book: Path) -> list[Failure]:
    fails: list[Failure] = []
    st = find_subject_toml(book)
    if st is None:
        fails.append(Failure(book.name, "subject_toml", "subject.toml",
                             "no subject.toml at book root or quiz-app/"))
        return fails
    try:
        with st.open("rb") as fh:
            data = tomllib.load(fh)
    except (tomllib.TOMLDecodeError, OSError) as exc:
        fails.append(Failure(book.name, "subject_toml", str(st),
                             f"cannot parse: {exc}"))
        return fails

    chapters_dir = data.get("chapters_dir", ".")
    base = (st.parent / chapters_dir).resolve()
    for ch in data.get("chapters", []):
        rel = ch.get("file")
        if not rel:
            continue
        if not (base / rel).exists():
            fails.append(Failure(book.name, "subject_toml", rel,
                                 f"chapter file does not resolve under {chapters_dir}"))
    return fails


def check_questions(book: Path) -> list[Failure]:
    fails: list[Failure] = []
    qdir = book / "questions"
    if not qdir.exists():
        return fails
    for jf in sorted(qdir.rglob("*.json")):
        if jf.name.startswith("_"):
            continue
        rel = jf.relative_to(book).as_posix()
        try:
            data = json.loads(jf.read_text(encoding="utf-8"))
        except (json.JSONDecodeError, OSError) as exc:
            fails.append(Failure(book.name, "questions", rel, f"invalid JSON: {exc}"))
            continue

        items = data if isinstance(data, list) else [data]
        for i, q in enumerate(items):
            tag = rel if len(items) == 1 else f"{rel}[{i}]"
            if not isinstance(q, dict):
                fails.append(Failure(book.name, "questions", tag, "not an object"))
                continue
            missing = [k for k in REQUIRED_QUESTION_FIELDS if k not in q]
            if missing:
                fails.append(Failure(book.name, "questions", tag,
                                     f"missing fields: {', '.join(missing)}"))
                continue
            kind = q["kind"]
            if kind not in VALID_KINDS:
                fails.append(Failure(book.name, "questions", tag,
                                     f"bad kind: {kind!r}"))
            if not isinstance(q["text"], str) or not q["text"].strip():
                fails.append(Failure(book.name, "questions", tag, "empty text"))
            if not isinstance(q["choices"], list):
                fails.append(Failure(book.name, "questions", tag, "choices not a list"))
            if not isinstance(q["explanation"], str) or not q["explanation"].strip():
                fails.append(Failure(book.name, "questions", tag, "empty explanation"))
            if q["difficulty"] not in VALID_DIFFICULTY:
                fails.append(Failure(book.name, "questions", tag,
                                     f"bad difficulty: {q['difficulty']!r}"))
            if not isinstance(q["chapter"], int) or isinstance(q["chapter"], bool):
                fails.append(Failure(book.name, "questions", tag,
                                     f"chapter not an int: {q['chapter']!r}"))
            # answer shape must match the Rust quiz-core Question enum so the
            # engine can load the bank: mc/tf → 0-based int index into choices;
            # blank → the canonical answer STRING (choices are acceptable answers,
            # all strings). This mirrors quiz/quiz-core/src/question.rs.
            choices = q["choices"] if isinstance(q["choices"], list) else []
            if kind in ("mc", "tf"):
                ans = q["answer"]
                if not isinstance(ans, int) or isinstance(ans, bool):
                    fails.append(Failure(book.name, "questions", tag,
                                         f"answer not an int: {ans!r}"))
                elif not (0 <= ans < len(choices)):
                    fails.append(Failure(book.name, "questions", tag,
                                         f"answer {ans} out of range (0..{len(choices) - 1})"))
            elif kind == "blank":
                if not isinstance(q["answer"], str):
                    fails.append(Failure(book.name, "questions", tag,
                                         f"blank answer must be a string (the canonical fill), got {q['answer']!r}"))
                if any(not isinstance(c, str) for c in choices):
                    fails.append(Failure(book.name, "questions", tag,
                                         "blank choices must all be strings (acceptable answers)"))
    return fails


def check_build(book: Path) -> list[Failure]:
    result = subprocess.run(
        [sys.executable, str(BUILD), str(book), "--check"],
        capture_output=True, text=True,
    )
    if result.returncode == 0:
        return []
    tail = "\n      ".join(
        line for line in result.stdout.splitlines()
        if line.strip().startswith(("- ", "UNCAPTURED", "MANIFEST", "EMPTY", "FAIL"))
    )
    return [Failure(book.name, "build_check", book.name,
                    "build_book.py --check failed:\n      " + tail)]


def check_links(readme: Path, book_name: str) -> list[Failure]:
    fails: list[Failure] = []
    if not readme.exists():
        return fails
    text = readme.read_text(encoding="utf-8", errors="replace")
    for m in LINK_RE.finditer(text):
        target = m.group(1).strip()
        if target.startswith(("http://", "https://", "#", "mailto:", "<")):
            continue
        path = target.split("#", 1)[0].split("?", 1)[0].strip()
        if not path:
            continue
        if not (readme.parent / path).exists():
            fails.append(Failure(book_name, "links", path,
                                 f"broken relative link in {readme.name}"))
    return fails


def zero_byte_census() -> list[Failure]:
    fails: list[Failure] = []
    skip = {".git", "output", "node_modules", "target", "__pycache__"}
    for md in ROOT.rglob("*.md"):
        if any(part in skip for part in md.parts):
            continue
        try:
            if not md.read_text(encoding="utf-8", errors="replace").strip():
                rel = md.relative_to(ROOT).as_posix()
                fails.append(Failure(rel.split("/", 1)[0], "zero_byte", rel,
                                     "zero-byte / whitespace-only markdown"))
        except OSError:
            continue
    return fails


# --------------------------------------------------------------------------- #
# Runner
# --------------------------------------------------------------------------- #

def run(only_book: str | None) -> int:
    exceptions = load_exceptions()
    books = book_dirs()
    if only_book:
        books = [b for b in books if b.name == only_book]
        if not books:
            sys.exit(f"ERROR: no book named {only_book!r} (has book.toml)")

    all_fails: list[Failure] = []

    for book in books:
        bfails: list[Failure] = []
        bfails += check_subject_toml(book)
        bfails += check_questions(book)
        bfails += check_build(book)
        bfails += check_links(book / "README.md", book.name)

        expected = [f for f in bfails if is_expected(f, exceptions)]
        unexpected = [f for f in bfails if f not in expected]

        status = "OK" if not unexpected else "FAIL"
        extra = f"  ({len(expected)} expected)" if expected else ""
        print(f"{status:4} {book.name}{extra}")
        for f in unexpected:
            print(f"       {f}")
        for f in expected:
            exc = is_expected(f, exceptions)
            print(f"       (expected) {f}  -- {exc.get('reason', '')}")
        all_fails += bfails

    # Repo-level checks
    print("---- repo-wide ----")
    root_fails = check_links(ROOT / "README.md", "<root>")
    zb = zero_byte_census()
    repo_fails = root_fails + zb

    zb_expected = [f for f in zb if is_expected(f, exceptions)]
    print(f"zero-byte markdown: {len(zb)} total, "
          f"{len(zb_expected)} expected, {len(zb) - len(zb_expected)} unexpected")
    for f in root_fails:
        marker = "(expected) " if is_expected(f, exceptions) else ""
        print(f"       {marker}{f}")

    all_fails += repo_fails

    unexpected_total = [f for f in all_fails if not is_expected(f, exceptions)]
    expected_total = [f for f in all_fails if is_expected(f, exceptions)]

    print("=" * 60)
    print(f"books checked      : {len(books)}")
    print(f"total failures     : {len(all_fails)}")
    print(f"expected (allowed) : {len(expected_total)}")
    print(f"unexpected         : {len(unexpected_total)}")
    if unexpected_total:
        print("\nUNEXPECTED FAILURES:")
        for f in unexpected_total:
            print(f"  {f.book}: {f}")
        return 1
    print("\nAll checks passed (only allowlisted exceptions remain).")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description="Repo-wide textbook validation.")
    ap.add_argument("--book", help="Validate only this book (by directory name).")
    ap.add_argument("--list-exceptions", action="store_true",
                    help="Print the allowlisted exceptions and exit.")
    args = ap.parse_args()

    if args.list_exceptions:
        for exc in load_exceptions():
            print(f"- {exc.get('book')} / {exc.get('check')} / "
                  f"{exc.get('pattern', '*')}\n    {exc.get('reason', '')}")
        return 0

    return run(args.book)


if __name__ == "__main__":
    sys.exit(main())
