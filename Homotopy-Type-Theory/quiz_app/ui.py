"""
Terminal UI — ANSI colours, formatted output, and input helpers.
All display logic lives here; no business logic.
"""
from __future__ import annotations

import json
import re
import sys
import textwrap
from pathlib import Path
from typing import Optional

from .config import CHAPTER_META, PHASE_NAMES
from .models import Question, UserProfile

# ── Exceptions ─────────────────────────────────────────────────────────────────

class QuizQuit(Exception):
    """Raised when the user quits mid-session (Ctrl+C or 'q' at an answer prompt)."""


# ── ANSI helpers ───────────────────────────────────────────────────────────────

def _c(code: str, text: str) -> str:
    return f"\033[{code}m{text}\033[0m"

def bold(t: str)    -> str: return _c("1", t)
def green(t: str)   -> str: return _c("32", t)
def red(t: str)     -> str: return _c("31", t)
def yellow(t: str)  -> str: return _c("33", t)
def cyan(t: str)    -> str: return _c("36", t)
def dim(t: str)     -> str: return _c("2", t)
def blue(t: str)    -> str: return _c("34", t)
def magenta(t: str) -> str: return _c("35", t)

WIDTH = 80

def hr(char: str = "─") -> None:
    print(dim(char * WIDTH))

def section(title: str) -> None:
    print(f"\n{bold(cyan(title))}\n")

def wrap(text: str, indent: int = 2) -> str:
    prefix = " " * indent
    return textwrap.fill(
        text, width=WIDTH - indent,
        initial_indent=prefix, subsequent_indent=prefix,
    )

def print_wrap(text: str, indent: int = 2) -> None:
    print(wrap(text, indent))


# ── Input helpers ──────────────────────────────────────────────────────────────

def ask(prompt: str) -> str:
    try:
        return input(prompt).strip()
    except (EOFError, KeyboardInterrupt):
        print()
        raise QuizQuit()


def ask_int(prompt: str, lo: int, hi: int, default: Optional[int] = None) -> int:
    while True:
        raw = ask(prompt)
        if not raw and default is not None:
            return default
        if raw.lower() in ("q", "quit"):
            raise QuizQuit()
        try:
            v = int(raw)
            if lo <= v <= hi:
                return v
        except ValueError:
            pass
        print(yellow(f"  Please enter a number between {lo} and {hi}."))


def confirm(prompt: str, default: bool = True) -> bool:
    hint = "[Y/n]" if default else "[y/N]"
    raw = ask(f"  {prompt} {hint} → ").lower()
    if not raw:
        return default
    return raw in ("y", "yes")


def pause() -> None:
    ask(dim("  [Enter to continue] "))


# ── Question display ───────────────────────────────────────────────────────────

_LABELS = "ABCD"


def _diff_badge(diff: str) -> str:
    colours = {"beginner": green, "intermediate": yellow, "advanced": red}
    fn = colours.get(diff, dim)
    return fn(f"[{diff}]")


def _source_badge(generated: bool) -> str:
    return dim("[AI-generated]") if generated else ""


def present_question(
    q: Question,
    num: int,
    total: int,
    *,
    running_correct: Optional[int] = None,
    time_limit_secs: Optional[int] = None,
) -> tuple[bool, bool, int]:
    """
    Display a question and collect the user's answer.
    Returns (correct, flagged, confidence) where:
      correct    — whether the answer was right
      flagged    — whether the user manually flagged this for review
      confidence — 1=guessing, 2=unsure, 3=certain (from post-answer prompt)
    """
    ch_name = CHAPTER_META.get(q.chapter, {}).get("name", f"Ch.{q.chapter}")
    print()
    hr()
    score_s = f"  {dim(f'Score: {running_correct}/{num-1}')}" if running_correct is not None else ""
    timer_s = f"  {yellow(f'⏱ {time_limit_secs}s limit')}" if time_limit_secs else ""
    print(
        f"  {dim(ch_name)}  "
        f"{_diff_badge(q.difficulty)}  "
        f"{_source_badge(q.generated)}"
        f"{score_s}{timer_s}"
    )
    print(f"\n  {bold(f'Q{num}/{total}.')} ", end="")
    lines = textwrap.wrap(q.text, width=WIDTH - 4)
    print(lines[0])
    for line in lines[1:]:
        print(f"      {line}")
    print()

    import time as _time
    q_start = _time.monotonic()

    if q.kind == "mc":
        correct, flagged, confidence = _present_mc(q)
    elif q.kind == "tf":
        correct, flagged, confidence = _present_tf(q)
    else:
        correct, flagged, confidence = _present_blank(q)

    elapsed = _time.monotonic() - q_start
    if time_limit_secs and elapsed > time_limit_secs:
        print(yellow(f"  ⏱  {elapsed:.1f}s — time limit exceeded ({time_limit_secs}s)"))

    return correct, flagged, confidence


def _present_mc(q: Question) -> tuple[bool, bool, int]:
    for i, choice in enumerate(q.choices):
        print(f"    {bold(_LABELS[i])}. {choice}")
    print()
    while True:
        raw = ask("  Answer (A/B/C/D or Q to quit) → ").upper()
        if raw in ("Q", "QUIT"):
            raise QuizQuit()
        if raw and raw[0] in _LABELS[: len(q.choices)]:
            user_idx = _LABELS.index(raw[0])
            break
        print(yellow("  Please enter A, B, C, or D."))
    correct = user_idx == q.answer
    flagged, confidence = _show_result(correct, _LABELS[q.answer], q.choices[q.answer], q.explanation)
    return correct, flagged, confidence


def _present_tf(q: Question) -> tuple[bool, bool, int]:
    print("    A. True")
    print("    B. False")
    print()
    while True:
        raw = ask("  Answer (A/B or T/F or Q to quit) → ").upper()
        if raw in ("Q", "QUIT"):
            raise QuizQuit()
        if raw in ("A", "T", "TRUE"):
            user_idx = 0; break
        if raw in ("B", "F", "FALSE"):
            user_idx = 1; break
        print(yellow("  Please enter A (True) or B (False)."))
    correct = user_idx == q.answer
    label = "A (True)" if q.answer == 0 else "B (False)"
    flagged, confidence = _show_result(correct, label, q.choices[q.answer], q.explanation)
    return correct, flagged, confidence


def _present_blank(q: Question) -> tuple[bool, bool, int]:
    raw = ask("  Your answer (or Q to quit) → ").strip()
    if raw.lower() in ("q", "quit"):
        raise QuizQuit()
    raw = raw.lower()
    if not raw:
        flagged, confidence = _show_result(False, q.choices[0], q.choices[0], q.explanation)
        return False, flagged, confidence
    acceptable = [s.lower() for s in q.choices]
    # Accept exact match or user's input as a substring of the answer (min 3 chars).
    correct = any(
        raw == a or (len(raw) >= 3 and raw in a)
        for a in acceptable
    )
    if not correct:
        correct = any(
            raw.replace(" ", "") == a.replace(" ", "")
            for a in acceptable
        )
    flagged, confidence = _show_result(correct, q.choices[0], q.choices[0], q.explanation)
    return correct, flagged, confidence


def _show_result(correct: bool, label: str, text: str, explanation: str) -> tuple[bool, int]:
    """Display result + explanation; prompt for confidence and optional flag. Returns (flagged, confidence)."""
    print()
    if correct:
        print(f"  {green(bold('✓  Correct!'))}")
    else:
        print(f"  {red(bold('✗  Incorrect.'))}  Answer: {bold(label)}")
    print()
    print(dim("  Explanation:"))
    for line in textwrap.wrap(explanation, width=WIDTH - 6):
        print(f"    {line}")
    print()

    # Confidence + flag combined prompt
    hint = dim("1=guess · 2=unsure · 3=sure · f=flag · Enter=continue")
    raw = ask(f"  {hint} → ").strip().lower()

    confidence = 2
    flagged = False
    if raw == "1":
        confidence = 1
    elif raw == "3":
        confidence = 3
    elif raw == "f":
        flagged = True
        print(dim("  [Flagged for review]"))

    return flagged, confidence


# ── Progress bar ───────────────────────────────────────────────────────────────

def progress_bar(score: int, total: int, width: int = 30) -> str:
    pct  = score / max(total, 1)
    fill = int(pct * width)
    bar  = "█" * fill + "░" * (width - fill)
    colour = green if pct >= 0.7 else yellow if pct >= 0.5 else red
    return colour(bar) + f" {score}/{total}"


# ── Mastery dashboard ──────────────────────────────────────────────────────────

def show_dashboard(profile: UserProfile) -> None:
    from .adaptive import mastery_summary
    rows = mastery_summary(profile)

    section(f"Mastery Dashboard — {bold(profile.name)}")
    print(f"  Total answered: {bold(str(profile.total_answered()))}")
    due_count = len(profile.chapters_due_for_review())
    if due_count:
        print(f"  {yellow(bold(f'  {due_count} chapter(s) due for review today'))}")
    flagged = len(profile.review_queue)
    if flagged:
        print(f"  {cyan(f'  {flagged} question(s) flagged for review')}")
    print()

    current_phase = -1
    for row in rows:
        ph = row["phase"]
        if ph != current_phase:
            current_phase = ph
            print(f"\n  {cyan(bold(PHASE_NAMES.get(ph, f'Phase {ph}')))}:")

        score   = row["score"]
        pct     = int(score * 100)
        fill    = int(score * 20)
        bar     = "█" * fill + "░" * (20 - fill)
        col     = green if pct >= 70 else yellow if pct >= 40 else red

        due_flag   = yellow(" ⟳") if row["due"] else "  "
        started    = "★" if row["started"] else "·"
        seen_s     = f"({row['seen']} seen)" if row["started"] else "(not started)"

        print(
            f"    {started} Ch.{row['chapter']:02d} "
            f"{row['name'][:28]:<28} "
            f"{col(bar)} {pct:3d}%{due_flag} {dim(seen_s)}"
        )

    print()
    _show_session_history(profile)
    hr()

    # Offer cross-app view if peer apps are configured
    try:
        from .config import PEER_APP_DIRS
        peer_dirs = [d for d in PEER_APP_DIRS if (d / "data" / "progress").exists()]
        if peer_dirs and confirm("Show unified progress across all study apps?", default=False):
            show_cross_app_dashboard(profile.name, peer_dirs)
            return
    except (ImportError, AttributeError):
        pass

    pause()


def _show_session_history(profile: UserProfile) -> None:
    """Render the last 8 sessions at the bottom of the dashboard."""
    history = profile.session_history
    if not history:
        return

    print(f"\n  {cyan(bold('Recent sessions:'))}")
    for sess in reversed(history[-8:]):
        ts        = (sess.get("timestamp") or "")[:10]
        n_q       = sess.get("n_questions", 0)
        n_c       = sess.get("n_correct", 0)
        pct       = round(100 * n_c / max(n_q, 1))
        scope     = sess.get("scope_label", "")[:28]
        dur       = f"{sess.get('duration_secs', 0):.0f}s"
        streak    = sess.get("streak_max", 0)
        n_gen     = sess.get("n_generated", 0)
        col       = green if pct >= 70 else yellow if pct >= 50 else red

        streak_s  = f" streak:{streak}" if streak > 1 else ""
        gen_s     = f" +{n_gen}AI" if n_gen else ""
        print(
            f"    {dim(ts)}  {col(f'{pct:3d}%')}  {n_c}/{n_q}  "
            f"{scope:<28}  {dim(dur + streak_s + gen_s)}"
        )
    print()


# ── Session summary ────────────────────────────────────────────────────────────

def show_session_summary(
    score: int,
    total: int,
    wrong_names: list[str],
    duration_secs: float,
    streak_max: int = 0,
    per_difficulty: Optional[dict] = None,
    n_generated: int = 0,
) -> None:
    print()
    hr("═")
    pct = 100 * score // max(total, 1)
    colour = green if pct >= 70 else yellow if pct >= 50 else red
    print(f"\n  {bold('Result:')} {colour(bold(f'{score}/{total}'))}  ({pct}%)")
    print(f"  {dim(f'Time: {duration_secs:.0f}s')}", end="")
    if streak_max > 1:
        print(f"   {yellow(f'Best streak: {streak_max}')}", end="")
    if n_generated:
        print(f"   {dim(f'{n_generated} AI-generated')}", end="")
    print()

    # Per-difficulty breakdown
    if per_difficulty:
        parts = []
        for diff in ("beginner", "intermediate", "advanced"):
            vals = per_difficulty.get(diff, [0, 0])
            if vals[1] > 0:
                d_pct = round(100 * vals[0] / vals[1])
                parts.append(f"{diff}: {vals[0]}/{vals[1]} ({d_pct}%)")
        if parts:
            print(f"  {dim('By difficulty:')} {dim(' · '.join(parts))}")
    print()

    if pct == 100:
        print(green("  Perfect score! 🎓"))
    elif pct >= 70:
        print(yellow("  Good work. Review the topics you missed."))
    else:
        print(red("  Keep studying — re-read the relevant chapters and try again."))

    if wrong_names:
        print(f"\n  {bold('Topics to review:')}")
        for name in wrong_names:
            print(f"    • {name}")

    hr("═")
    pause()


# ── Cross-app unified dashboard ────────────────────────────────────────────────

def show_cross_app_dashboard(profile_name: str, peer_app_dirs: list[Path]) -> None:
    """Show a compact progress summary for the same user across all quiz apps."""
    section("Unified Study Dashboard")

    # Include the current app first by reading its own data
    from .config import PROGRESS_DIR, PROJECT_ROOT
    app_name = PROJECT_ROOT.name

    all_apps: list[tuple[str, Path]] = [(app_name, PROGRESS_DIR)]
    for d in peer_app_dirs:
        peer_root = d.parent
        peer_progress = d / "data" / "progress"
        all_apps.append((peer_root.name, peer_progress))

    for app_label, progress_dir in all_apps:
        if not progress_dir.exists():
            continue
        # Find profile for this user
        profile_data = None
        for p in progress_dir.glob("*.json"):
            try:
                data = json.loads(p.read_text())
                if data.get("name", "").lower() == profile_name.lower():
                    profile_data = data
                    break
            except Exception:
                continue

        print(f"  {bold(cyan(app_label))}")
        if profile_data is None:
            print(f"    {dim('No profile found for this user')}\n")
            continue

        mastery = profile_data.get("mastery", {})
        if not mastery:
            print(f"    {dim('No activity yet')}\n")
            continue

        total_seen = sum(v.get("total_seen", 0) for v in mastery.values())
        started    = [v for v in mastery.values() if v.get("total_seen", 0) > 0]
        if started:
            avg_score = sum(v.get("score", 0.5) for v in started) / len(started)
            pct = int(avg_score * 100)
            col = green if pct >= 70 else yellow if pct >= 40 else red
            fill = int(avg_score * 20)
            bar = "█" * fill + "░" * (20 - fill)
            print(f"    Chapters started: {len(started)}  |  Avg mastery: {col(bar)} {pct}%")
        print(f"    Total answered: {total_seen}\n")

    hr()
    pause()


# ── Progress export ────────────────────────────────────────────────────────────

def show_export_result(path: Path) -> None:
    print(f"\n  {green('✓')} Progress exported to:")
    print(f"    {bold(str(path))}\n")


# ── Prerequisite warning ───────────────────────────────────────────────────────

def show_prereq_warning(chapter: int, prev_phase: int, avg_mastery: float,
                        n_started: int, n_total: int) -> bool:
    """
    Warn the user if they're jumping ahead of their prerequisite mastery.
    Returns True if they want to continue, False to cancel.
    """
    from .config import PHASE_NAMES
    ch_name   = CHAPTER_META.get(chapter, {}).get("name", f"Ch.{chapter}")
    ch_phase  = CHAPTER_META.get(chapter, {}).get("phase", 0)
    prev_name = PHASE_NAMES.get(prev_phase, f"Phase {prev_phase}")

    print()
    print(yellow(f"  ⚠  {ch_name} (Phase {ch_phase}) builds on {prev_name}."))
    if n_started == 0:
        print(yellow(f"  You haven't started any chapters in {prev_name} yet."))
    else:
        print(yellow(f"  Your {prev_name} mastery: {int(avg_mastery*100)}% "
                     f"({n_started}/{n_total} chapters started)"))
    print()
    return confirm("Continue anyway?", default=True)


# ── Study mode ─────────────────────────────────────────────────────────────────

def _render_chapter_prose(raw: str) -> None:
    """Render stripped markdown prose to terminal."""
    prose = re.sub(r"```.*?```", "\n[code block omitted]\n", raw, flags=re.DOTALL)
    for paragraph in prose.split("\n\n"):
        paragraph = paragraph.strip()
        if not paragraph:
            continue
        if paragraph.startswith("#"):
            print(f"\n  {bold(cyan(paragraph.lstrip('#').strip()))}\n")
        elif paragraph.startswith("[code"):
            print(f"  {dim(paragraph)}\n")
        else:
            print_wrap(paragraph, indent=2)
            print()


def show_study_mode(chapter: int) -> None:
    """Display a chapter's content for studying.

    Reads from the richer subdirectory section files when they exist,
    falling back to the flat chapter summary file.  When section files
    are available the user can page through them one at a time.
    """
    from .config import CHAPTERS_DIR, CHAPTER_META

    meta = CHAPTER_META.get(chapter)
    if not meta:
        print_wrap(f"Chapter {chapter} not found.")
        return

    name = meta["name"]
    flat_file = meta["file"]
    subdir = CHAPTERS_DIR / flat_file.replace(".md", "")

    section_files = sorted(subdir.rglob("*.md")) if subdir.is_dir() else []

    section(f"Ch.{chapter:02d} — {name}")

    if section_files:
        # Show a table of contents first
        print(f"  {dim(str(len(section_files)))} section files found.  "
              f"Press Enter to page through, or {bold('s')} to skip to quiz.\n")
        for i, sf in enumerate(section_files, 1):
            rel = sf.relative_to(subdir)
            print(f"  {dim(str(i)+'.')}  {rel}")
        print()
        hr()

        for i, sf in enumerate(section_files, 1):
            rel = sf.relative_to(subdir)
            print(f"\n  {bold(cyan(str(rel)))}\n")
            raw = sf.read_text(errors="replace")
            _render_chapter_prose(raw)
            hr()
            if i < len(section_files):
                ans = ask(f"  [{i}/{len(section_files)}] Enter for next section, 's' to skip → ").strip().lower()
                if ans == "s":
                    break
    else:
        # Fall back to flat file
        path = CHAPTERS_DIR / flat_file
        if not path.exists():
            print_wrap(f"Chapter file not found: {flat_file}")
            return
        raw = path.read_text(errors="replace")
        truncated = len(raw) > 6_000
        _render_chapter_prose(raw[:6_000])
        if truncated:
            print(dim(f"  [Content truncated — open {flat_file} to read the full chapter]"))
        print()
        hr()

    if confirm("Quiz on this chapter now?"):
        return  # caller decides what to do


# ── Main menu ──────────────────────────────────────────────────────────────────

def show_main_menu(profile_name: str, api_available: bool,
                   due_count: int = 0,
                   flagged_count: int = 0) -> str:
    """Display main menu; return the user's selection."""
    print()
    hr()
    api_note = green("  (Claude AI enabled — dynamic questions on)") if api_available \
               else yellow("  (set ANTHROPIC_API_KEY to enable AI-generated questions)")
    print(f"\n  {bold(magenta('HoTT Adaptive Quiz'))}  •  {dim(profile_name)}")
    print(api_note)
    if due_count:
        print(f"  {yellow(bold(f'  {due_count} chapter(s) due for review'))}")
    if flagged_count:
        print(f"  {cyan(f'  {flagged_count} question(s) flagged for review')}")
    print()
    print(f"  {bold('1.')} Adaptive quiz   {dim('(focuses on your weakest areas)')}")
    print(f"  {bold('2.')} Custom quiz      {dim('(choose phase, chapter, topic, or flagged)')}")
    print(f"  {bold('3.')} Study a chapter  {dim('(read, then optionally quiz)')}")
    print(f"  {bold('4.')} Mastery dashboard")
    print(f"  {bold('5.')} Export progress  {dim('(saves a Markdown report)')}")
    print(f"  {bold('6.')} Switch profile")
    print(f"  {bold('7.')} Quit")
    print()
    hr()
    while True:
        ch = ask("  → ").strip()
        if ch in ("1", "2", "3", "4", "5", "6", "7"):
            return ch
        print(yellow("  Please enter 1–7."))


def show_scope_menu(pool_size: int, flagged_count: int = 0) -> "Scope":  # type: ignore[name-defined]
    """Let the user choose a scope. Returns a Scope object."""
    from .adaptive import Scope
    print()
    hr()
    section("Choose scope")
    print("  [1] All chapters")
    print("  [2] A specific phase")
    print("  [3] A specific chapter")
    print("  [4] A tag / topic")
    if flagged_count:
        print(f"  [5] Flagged for review  {dim(f'({flagged_count} questions)')}")
    hr()
    ch = ask("  → ").strip()

    if ch == "1":
        return Scope.all()

    if ch == "2":
        print()
        for k, v in PHASE_NAMES.items():
            print(f"    [{k}] {v}")
        ph = ask_int("\n  Phase number → ", 0, 8, default=0)
        return Scope.phase(ph)

    if ch == "3":
        print()
        for k, meta in CHAPTER_META.items():
            print(f"    [{k:2d}] Ch.{k:02d} {meta['name']}")
        ch_num = ask_int("\n  Chapter number → ", 0, 26, default=0)
        return Scope.chapter(ch_num)

    if ch == "4":
        tag = ask("  Tag → ").strip()
        return Scope.tag(tag) if tag else Scope.all()

    if ch == "5" and flagged_count:
        return Scope.due()   # caller will pass profile to get flagged IDs

    return Scope.all()


def show_chapter_menu() -> int:
    """Let the user pick a chapter number for study mode."""
    print()
    hr()
    section("Choose a chapter to study")
    for k, meta in CHAPTER_META.items():
        ph = meta["phase"]
        print(f"    [{k:2d}] Ph.{ph}  Ch.{k:02d}  {meta['name']}")
    hr()
    return ask_int("  Chapter number → ", 0, 26, default=0)


def ask_time_limit() -> Optional[int]:
    """Ask if the user wants a per-question time limit. Returns seconds or None."""
    print()
    raw = ask(dim("  Time limit per question? (30/60/90 seconds, or Enter for none) → ")).strip()
    if not raw:
        return None
    try:
        v = int(raw)
        if v > 0:
            return v
    except ValueError:
        pass
    return None
