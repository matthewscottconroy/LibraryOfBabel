"""Terminal UI for the CSSB Adaptive Quiz."""
from __future__ import annotations

import re
import sys
import textwrap
from typing import Optional

from .config import CHAPTER_META, PHASE_NAMES
from .models import Question, UserProfile


class QuizQuit(Exception):
    """Raised when the user types 'q' to exit a quiz early."""

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
    return textwrap.fill(text, width=WIDTH - indent,
                         initial_indent=prefix, subsequent_indent=prefix)

def print_wrap(text: str, indent: int = 2) -> None:
    print(wrap(text, indent))

def ask(prompt: str) -> str:
    try:
        return input(prompt).strip()
    except (EOFError, KeyboardInterrupt):
        print()
        sys.exit(0)

def ask_int(prompt: str, lo: int, hi: int, default: Optional[int] = None) -> int:
    while True:
        raw = ask(prompt)
        if not raw and default is not None:
            return default
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

_LABELS = "ABCD"

def _diff_badge(diff: str) -> str:
    colours = {"beginner": green, "intermediate": yellow, "advanced": red}
    fn = colours.get(diff, dim)
    return fn(f"[{diff}]")

def _source_badge(generated: bool) -> str:
    return dim("[AI-generated]") if generated else ""

def _ask_confidence() -> int:
    """Ask how well the user knew the answer. Returns 1 (guessed), 2 (unsure), 3 (certain)."""
    raw = ask("  How well?  1=guessed  2=unsure  3=certain  [2] → ").strip()
    return {"1": 1, "3": 3}.get(raw, 2)


def present_question(
    q: Question,
    num: int,
    total: int,
    *,
    running_correct: Optional[int] = None,
    time_limit_secs: Optional[int] = None,
) -> tuple[bool, bool, int]:
    """
    Present a question and return (correct, flagged, confidence).
    Wrong answers are auto-flagged for the review queue.
    """
    ch_name = CHAPTER_META.get(q.chapter, {}).get("name", f"Ch.{q.chapter}")
    print()
    hr()
    score_note = f"  {dim(f'score so far: {running_correct}')}" if running_correct is not None else ""
    print(f"  {dim(ch_name)}  {_diff_badge(q.difficulty)}  {_source_badge(q.generated)}{score_note}")
    print(f"\n  {bold(f'Q{num}/{total}.')} ", end="")
    lines = textwrap.wrap(q.text, width=WIDTH - 4)
    print(lines[0])
    for line in lines[1:]:
        print(f"      {line}")
    print()

    if q.kind == "mc":
        correct = _present_mc(q)
    elif q.kind == "tf":
        correct = _present_tf(q)
    else:
        correct = _present_blank(q)

    confidence = _ask_confidence()
    pause()
    return correct, not correct, confidence  # flagged = not correct


def _present_mc(q: Question) -> bool:
    for i, choice in enumerate(q.choices):
        print(f"    {bold(_LABELS[i])}. {choice}")
    print()
    while True:
        raw = ask("  Answer (A/B/C/D or Q to quit) → ").upper()
        if raw == "Q":
            raise QuizQuit
        if raw and raw[0] in _LABELS[: len(q.choices)]:
            user_idx = _LABELS.index(raw[0])
            break
        print(yellow("  Please enter A–D (or Q to quit)."))
    correct = user_idx == q.answer
    _show_result(correct, _LABELS[q.answer], q.choices[q.answer], q.explanation)
    return correct


def _present_tf(q: Question) -> bool:
    print("    A. True")
    print("    B. False")
    print()
    while True:
        raw = ask("  Answer (A/B or T/F or Q to quit) → ").upper()
        if raw == "Q":
            raise QuizQuit
        if raw in ("A", "T", "TRUE"):
            user_idx = 0; break
        if raw in ("B", "F", "FALSE"):
            user_idx = 1; break
        print(yellow("  Please enter A (True) or B (False), or Q to quit."))
    correct = user_idx == q.answer
    label = "A (True)" if q.answer == 0 else "B (False)"
    _show_result(correct, label, q.choices[q.answer], q.explanation)
    return correct

def _normalize_blank(s: str) -> str:
    return re.sub(r'[^a-z0-9 ]', '', s.lower()).strip()

def _match_blank(raw: str, acceptable: list[str]) -> bool:
    raw_n = _normalize_blank(raw)
    raw_no_space = raw_n.replace(' ', '')
    for a in acceptable:
        a_n = _normalize_blank(a)
        if raw_n == a_n:
            return True
        if raw_no_space == a_n.replace(' ', ''):
            return True
        # Allow up to 2 extra words only for multi-word answers
        # (e.g., "the Hill coefficient" matches "Hill coefficient", but
        #  "membrane potential" does NOT match single-word "membrane")
        a_tokens = set(a_n.split())
        raw_tokens = set(raw_n.split())
        if len(a_tokens) > 1 and a_tokens <= raw_tokens and len(raw_tokens) <= len(a_tokens) + 2:
            return True
    return False

def _present_blank(q: Question) -> bool:
    raw = ask("  Your answer (or Q to quit) → ").strip()
    if raw.upper() == "Q":
        raise QuizQuit
    if not raw:
        _show_result(False, q.choices[0], q.choices[0], q.explanation)
        return False
    correct = _match_blank(raw, list(q.choices))
    _show_result(correct, q.choices[0], q.choices[0], q.explanation)
    return correct


def _show_result(correct: bool, label: str, text: str, explanation: str) -> None:
    print()
    if correct:
        print(f"  {green(bold('✓  Correct!'))}")
    else:
        print(f"  {red(bold('✗  Incorrect.'))}  Answer: {bold(label)}")
    print()
    print(dim("  Explanation:"))
    for line in textwrap.wrap(explanation, width=WIDTH - 6):
        print(f"    {line}")

def progress_bar(score: int, total: int, width: int = 30) -> str:
    pct  = score / max(total, 1)
    fill = int(pct * width)
    bar  = "█" * fill + "░" * (width - fill)
    colour = green if pct >= 0.7 else yellow if pct >= 0.5 else red
    return colour(bar) + f" {score}/{total}"

def show_dashboard(profile: UserProfile) -> None:
    from .adaptive import mastery_summary
    rows = mastery_summary(profile)

    section(f"Mastery Dashboard — {bold(profile.name)}")
    print(f"  Total answered: {bold(str(profile.total_answered()))}")
    due_count = len(profile.chapters_due_for_review())
    if due_count:
        print(f"  {yellow(bold(f'{due_count} topic(s) due for review today'))}")
    print()

    current_phase = -1
    for row in rows:
        ph = row["phase"]
        if ph != current_phase:
            current_phase = ph
            print(f"\n  {cyan(bold(PHASE_NAMES.get(ph, f'Tier {ph}')))}:")

        score      = row["score"]
        pct        = int(score * 100)
        fill       = int(score * 20)
        bar        = "█" * fill + "░" * (20 - fill)
        col        = green if pct >= 70 else yellow if pct >= 40 else red
        due_flag   = yellow(" ⟳") if row["due"] else "  "
        started    = "★" if row["started"] else "·"
        seen_s     = f"({row['seen']} seen)" if row["started"] else "(not started)"

        print(
            f"    {started} T{row['phase']}.{row['chapter']:02d} "
            f"{row['name'][:26]:<26} "
            f"{col(bar)} {pct:3d}%{due_flag} {dim(seen_s)}"
        )

    print()
    _show_session_history(profile)
    hr()
    pause()

def _show_session_history(profile: UserProfile) -> None:
    history = profile.session_history
    if not history:
        return
    print(f"\n  {cyan(bold('Recent sessions:'))}")
    for sess in reversed(history[-8:]):
        ts     = (sess.get("timestamp") or "")[:10]
        n_q    = sess.get("n_questions", 0)
        n_c    = sess.get("n_correct", 0)
        pct    = round(100 * n_c / max(n_q, 1))
        scope  = sess.get("scope_label", "")[:28]
        dur    = f"{sess.get('duration_secs', 0):.0f}s"
        streak = sess.get("streak_max", 0)
        col    = green if pct >= 70 else yellow if pct >= 50 else red
        streak_s = f" streak:{streak}" if streak > 1 else ""
        print(
            f"    {dim(ts)}  {col(f'{pct:3d}%')}  {n_c}/{n_q}  "
            f"{scope:<28}  {dim(dur + streak_s)}"
        )

    # Trend: compare last-3 average vs previous-3 average
    if len(history) >= 4:
        def _avg_pct(sessions: list) -> float:
            total_q = sum(s.get("n_questions", 0) for s in sessions)
            total_c = sum(s.get("n_correct", 0) for s in sessions)
            return 100 * total_c / max(total_q, 1)
        recent  = _avg_pct(history[-3:])
        prev    = _avg_pct(history[-6:-3])
        delta   = recent - prev
        if delta > 3:
            trend = green("↑ improving")
        elif delta < -3:
            trend = red("↓ declining")
        else:
            trend = yellow("→ steady")
        print(f"  Trend (last 3 vs previous): {trend}")
    print()

def show_session_summary(
    score: int,
    total: int,
    wrong_names: list[str],
    duration_secs: float,
    streak_max: int = 0,
    per_difficulty: Optional[dict] = None,
    n_generated: int = 0,
    per_chapter: Optional[dict] = None,
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

    if per_difficulty:
        parts = []
        for diff in ("beginner", "intermediate", "advanced"):
            vals = per_difficulty.get(diff, [0, 0])
            if vals[1] > 0:
                d_pct = round(100 * vals[0] / vals[1])
                parts.append(f"{diff}: {vals[0]}/{vals[1]} ({d_pct}%)")
        if parts:
            print(f"  {dim('By difficulty:')} {dim(' · '.join(parts))}")

    if per_chapter and len(per_chapter) > 1:
        print(f"  {dim('By chapter:')}")
        for ch in sorted(per_chapter):
            c, t = per_chapter[ch]
            name = CHAPTER_META.get(ch, {}).get("name", f"Ch.{ch}")
            ch_pct = round(100 * c / max(t, 1))
            col = green if ch_pct >= 70 else yellow if ch_pct >= 50 else red
            print(f"    Ch.{ch:02d} {name[:24]:<24}  {col(f'{c}/{t}')}  ({ch_pct}%)")
    print()

    if pct == 100:
        print(green("  Perfect score!"))
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

def show_study_mode(chapter: int) -> bool:
    """Display the chapter text and ask whether to quiz. Returns True if the user wants to quiz."""
    from pathlib import Path
    from .config import CHAPTERS_DIR, CHAPTER_META

    meta = CHAPTER_META.get(chapter)
    if not meta:
        print_wrap(f"Topic {chapter} not found.")
        return False

    name = meta["name"]
    path = CHAPTERS_DIR / meta["file"]
    if not path.exists():
        print_wrap(f"Topic file not found: {meta['file']}")
        return False

    raw = path.read_text(errors="replace")
    prose = re.sub(r"```.*?```", "\n[code block omitted]\n", raw, flags=re.DOTALL)
    truncated = len(prose) > 6_000
    prose = prose[:6_000]

    section(f"Topic {chapter:02d} — {name}")
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

    if truncated:
        print(dim("  [Content truncated — see the full chapter file for complete coverage.]\n"))
    hr()
    return confirm("Quiz on this topic now?")

def show_main_menu(profile_name: str, api_available: bool, due_count: int = 0) -> str:
    print()
    hr()
    api_note = green("  (Claude AI enabled — dynamic questions on)") if api_available \
               else yellow("  (set ANTHROPIC_API_KEY to enable AI-generated questions)")
    print(f"\n  {bold(magenta('CSSB Adaptive Quiz'))}  •  {dim(profile_name)}")
    print(api_note)
    if due_count:
        print(f"  {yellow(bold(f'  {due_count} topic(s) due for review'))}")
    print()
    print(f"  {bold('1.')} Adaptive quiz   {dim('(focuses on your weakest areas)')}")
    print(f"  {bold('2.')} Custom quiz      {dim('(choose tier, topic, or keyword)')}")
    print(f"  {bold('3.')} Study a topic    {dim('(read, then optionally quiz)')}")
    print(f"  {bold('4.')} Mastery dashboard")
    print(f"  {bold('5.')} Switch profile")
    print(f"  {bold('6.')} Cathedral readiness {dim('(research project prerequisites)')}")
    print(f"  {bold('7.')} Quit")
    print()
    hr()
    while True:
        ch = ask("  → ").strip()
        if ch in ("1", "2", "3", "4", "5", "6", "7"):
            return ch
        print(yellow("  Please enter 1–7."))

def show_scope_menu(pool_size: int) -> "Scope":  # type: ignore[name-defined]
    from .adaptive import Scope
    print()
    hr()
    section("Choose scope")
    print("  [1] All topics")
    print("  [2] A specific tier")
    print("  [3] A specific topic")
    print("  [4] A keyword / tag")
    hr()
    ch = ask("  → ").strip()

    if ch == "1":
        return Scope.all()
    if ch == "2":
        print()
        for k, v in PHASE_NAMES.items():
            print(f"    [{k}] {v}")
        ph = ask_int("\n  Tier number → ", 0, 5, default=0)
        return Scope.phase(ph)
    if ch == "3":
        print()
        for k, meta in CHAPTER_META.items():
            ph = meta["phase"]
            print(f"    [{k:2d}] T{ph}  {meta['name']}")
        ch_num = ask_int("\n  Topic number → ", 0, max(CHAPTER_META), default=0)
        return Scope.chapter(ch_num)
    if ch == "4":
        tag = ask("  Keyword → ").strip()
        return Scope.tag(tag) if tag else Scope.all()
    return Scope.all()

def show_chapter_menu() -> int:
    print()
    hr()
    section("Choose a topic to study")
    for k, meta in CHAPTER_META.items():
        ph = meta["phase"]
        print(f"    [{k:2d}] T{ph}  {meta['name']}")
    hr()
    return ask_int("  Topic number → ", 0, max(CHAPTER_META), default=0)


def show_cathedral_readiness(profile: UserProfile, cathedral_id: str) -> None:
    from .config import CATHEDRAL_PREREQS, DIFF_BEGINNER_MAX, DIFF_INTERMEDIATE_MAX

    data = CATHEDRAL_PREREQS.get(cathedral_id.upper())
    if not data:
        available = ", ".join(sorted(CATHEDRAL_PREREQS))
        print_wrap(f"Unknown cathedral '{cathedral_id}'. Available: {available}")
        return

    section(f"Cathedral {cathedral_id.upper()} — {data['title']}")
    print_wrap(data["description"])
    print()

    READY_THRESHOLD = DIFF_INTERMEDIATE_MAX   # ≥ 68% mastery = ready

    all_ready = True
    for ch in sorted(data["chapters"]):
        meta = CHAPTER_META.get(ch, {})
        name = meta.get("name", f"Ch.{ch}")
        rec  = profile.mastery.get(ch)
        score = rec.score if rec else 0.0
        seen  = rec.total_seen if rec else 0
        pct   = int(score * 100)
        fill  = int(score * 20)
        bar   = "█" * fill + "░" * (20 - fill)

        if score >= READY_THRESHOLD:
            col   = green
            flag  = green("  ✓ ready")
        elif seen == 0:
            col   = dim
            flag  = yellow("  ○ not started")
            all_ready = False
        else:
            col   = yellow if score >= DIFF_BEGINNER_MAX else red
            flag  = yellow("  ↑ needs work")
            all_ready = False

        print(f"    Ch.{ch:02d} {name[:28]:<28} {col(bar)} {pct:3d}%{flag}")

    print()
    if all_ready:
        print(green(bold("  ✓ All prerequisites met — you are ready to begin this cathedral.")))
    else:
        missing = [ch for ch in data["chapters"]
                   if (profile.mastery.get(ch) or type('', (), {'score': 0.0})()).score < READY_THRESHOLD]
        deficit = len(missing)
        print(yellow(f"  {deficit} chapter(s) below the readiness threshold (≥{int(READY_THRESHOLD*100)}%)."))
        print_wrap("Work through those chapters in the quiz until mastery turns green, then return.")
    print()
    hr()
    pause()


def show_cathedral_menu(profile: UserProfile) -> None:
    from .config import CATHEDRAL_PREREQS, DIFF_INTERMEDIATE_MAX

    section("Cathedral Readiness")
    print_wrap(
        "Each cathedral is a substantial research project. The table below shows "
        "how ready you are based on prerequisite chapter mastery (≥68% = ready)."
    )
    print()

    for cid, data in CATHEDRAL_PREREQS.items():
        chapters = data["chapters"]
        scores = [
            (profile.mastery[ch].score if ch in profile.mastery else 0.0)
            for ch in chapters
        ]
        ready_count = sum(1 for s in scores if s >= DIFF_INTERMEDIATE_MAX)
        total = len(chapters)
        overall = ready_count / total if total else 0.0
        fill  = int(overall * 12)
        bar   = "█" * fill + "░" * (12 - fill)
        col   = green if ready_count == total else yellow if ready_count > 0 else red
        print(f"  [{cid}] {data['title'][:38]:<38}  {col(bar)}  {ready_count}/{total} prereqs")

    print()
    hr()
    cid = ask("  Cathedral ID (I–VII) or Enter to go back → ").strip().upper()
    if cid in CATHEDRAL_PREREQS:
        show_cathedral_readiness(profile, cid)
