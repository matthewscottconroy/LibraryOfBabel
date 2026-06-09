"""
Terminal UI for the Abstract Algebra Adaptive Quiz.
ANSI colours, formatted output, and input helpers.
"""
from __future__ import annotations

import re
import sys
import textwrap
from typing import Optional

from .config import CHAPTER_META, PHASE_NAMES
from .models import Question, UserProfile


class QuizQuit(Exception):
    """Raised when the user types 'q' to exit mid-session."""

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

def present_question(
    q: Question,
    num: int,
    total: int,
    running_correct: int = 0,
    time_limit_secs: Optional[int] = None,  # accepted but not yet enforced
) -> tuple[bool, bool, int]:
    """Present one question; return (correct, flagged, confidence)."""
    ch_name = CHAPTER_META.get(q.chapter, {}).get("name", f"Ch.{q.chapter}")
    print()
    hr()
    score_str = dim(f"  {running_correct}/{num - 1} correct") if num > 1 else ""
    print(f"  {dim(ch_name)}  {_diff_badge(q.difficulty)}  {_source_badge(q.generated)}{score_str}")
    print(f"\n  {bold(f'Q{num}/{total}.')} ", end="")
    lines = textwrap.wrap(q.text, width=WIDTH - 4)
    print(lines[0])
    for line in lines[1:]:
        print(f"      {line}")
    print()

    if q.kind == "mc":
        return _present_mc(q)
    if q.kind == "tf":
        return _present_tf(q)
    if q.kind == "proof":
        return _present_proof(q)
    return _present_blank(q)

def _present_mc(q: Question) -> tuple[bool, bool, int]:
    for i, choice in enumerate(q.choices):
        print(f"    {bold(_LABELS[i])}. {choice}")
    print()
    while True:
        raw = ask("  Answer (A/B/C/D  or q to quit) → ").upper()
        if raw == "Q":
            raise QuizQuit
        if raw and raw[0] in _LABELS[: len(q.choices)]:
            user_idx = _LABELS.index(raw[0])
            break
        print(yellow("  Please enter A, B, C, or D."))
    correct = user_idx == q.answer
    flagged, confidence = _show_result(correct, _LABELS[q.answer], q.choices[q.answer], q.explanation, chapter=q.chapter)
    return correct, flagged, confidence

def _present_tf(q: Question) -> tuple[bool, bool, int]:
    print("    A. True")
    print("    B. False")
    print()
    while True:
        raw = ask("  Answer (A/B or T/F  or q to quit) → ").upper()
        if raw == "Q":
            raise QuizQuit
        if raw in ("A", "T", "TRUE"):
            user_idx = 0; break
        if raw in ("B", "F", "FALSE"):
            user_idx = 1; break
        print(yellow("  Please enter A (True) or B (False)."))
    correct = user_idx == q.answer
    label = "A (True)" if q.answer == 0 else "B (False)"
    flagged, confidence = _show_result(correct, label, q.choices[q.answer], q.explanation, chapter=q.chapter)
    return correct, flagged, confidence

def _normalize_blank(s: str) -> str:
    """Collapse whitespace and lowercase for blank-answer comparison."""
    return re.sub(r"\s+", " ", s.strip().lower())


def _blank_matches(raw: str, acceptable: list[str]) -> bool:
    """
    Return True if raw matches any acceptable answer.
    Allows the user to type a suffix of a multi-word answer (e.g. "induction"
    matches "mathematical induction") but requires at least 4 characters to
    guard against trivially short substrings.  Plain substring-of-user-input
    matching ("a in raw") is intentionally excluded because it would accept
    any sentence containing a keyword.
    """
    r = _normalize_blank(raw)
    for a in [_normalize_blank(a) for a in acceptable]:
        if r == a:
            return True
        if len(r) >= 4 and a.endswith(r):
            return True
    return False


def _present_blank(q: Question) -> tuple[bool, bool, int]:
    raw = ask("  Your answer (or q to quit) → ").strip()
    if raw.lower() == "q":
        raise QuizQuit
    raw = raw.lower()
    if not raw:
        flagged, confidence = _show_result(False, q.choices[0], q.choices[0], q.explanation, chapter=q.chapter)
        return False, flagged, confidence
    correct = _blank_matches(raw, q.choices)
    flagged, confidence = _show_result(correct, q.choices[0], q.choices[0], q.explanation, chapter=q.chapter)
    return correct, flagged, confidence

def _present_proof(q: Question) -> tuple[bool, bool, int]:
    """
    Present a proof-scaffold question.
    q.text    = the theorem to prove.
    q.choices = list of proof lines, some containing '___'.
    q.answer  = pipe-separated canonical fills (one per blank, in order).
    """
    fills = [f.strip() for f in q.answer.split("|") if f.strip()]

    print_wrap(f"Prove: {bold(q.text)}", indent=2)
    print()
    print(dim("  Complete the following proof by filling in each blank:"))
    print()

    # Number the blanks across all lines in order
    blank_idx = 0
    for line in q.choices:
        if "___" not in line:
            print(f"    {dim(line)}")
        else:
            # Replace each ___ with a numbered placeholder for display
            display = line
            count_in_line = line.count("___")
            for _ in range(count_in_line):
                display = display.replace("___", cyan(f"[{blank_idx + 1}]"), 1)
                blank_idx += 1
            print(f"    {display}")
    print()

    user_fills: list[str] = []
    for i in range(len(fills)):
        raw = ask(f"  Fill blank [{i + 1}] (or q to quit) → ").strip()
        if raw.lower() == "q":
            raise QuizQuit
        user_fills.append(raw)

    # Score: all blanks must be correct (case-insensitive, allow suffix matching)
    all_correct = all(
        _blank_matches(user_fills[i], [fills[i]])
        for i in range(len(fills))
    )

    # Show the completed proof with correct fills highlighted
    print()
    if all_correct:
        print(f"  {green(bold('✓  Correct!'))}")
    else:
        print(f"  {red(bold('✗  Incorrect.'))}  Correct fills:")
        for i, f in enumerate(fills):
            mark = green("✓") if _blank_matches(user_fills[i], [f]) else red("✗")
            print(f"    [{i + 1}] {mark} {bold(f)}"
                  + (f"  {dim('(you wrote: ' + user_fills[i] + ')')}"
                     if not _blank_matches(user_fills[i], [f]) else ""))

    flagged, confidence = _show_result(all_correct, "", "", q.explanation, chapter=q.chapter)
    return all_correct, flagged, confidence


def _show_result(
    correct: bool, label: str, text: str, explanation: str, chapter: int = -1
) -> tuple[bool, int]:
    """Print the result + explanation, then collect (flagged, confidence) from user."""
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
    if not correct:
        demos = CHAPTER_META.get(chapter, {}).get("demos", [])
        if demos:
            cmds = "  ".join(cyan(bold(f"cargo run -p {d}")) for d in demos)
            print(f"  {dim('Explore it hands-on:')}  {cmds}")
            print()
        ask(dim("  [Enter to continue] "))
        return False, 1
    # Correct: collect confidence + optional flag in one prompt
    raw = ask(
        dim("  [1]guess [2]unsure [3]certain  [f]flag for review  Enter=2 → ")
    ).strip().lower()
    flagged    = "f" in raw
    nums       = [c for c in raw if c in "123"]
    confidence = int(nums[0]) if nums else 2
    return flagged, confidence

def progress_bar(score: int, total: int, width: int = 30) -> str:
    pct  = score / max(total, 1)
    fill = int(pct * width)
    bar  = "█" * fill + "░" * (width - fill)
    colour = green if pct >= 0.7 else yellow if pct >= 0.5 else red
    return colour(bar) + f" {score}/{total}"

def show_dashboard(profile: UserProfile) -> None:
    from .adaptive import mastery_summary, tag_weakness
    from .question_bank import QUESTIONS
    rows = mastery_summary(profile)

    section(f"Mastery Dashboard — {bold(profile.name)}")
    print(f"  Total answered: {bold(str(profile.total_answered()))}")
    due_count = len(profile.chapters_due_for_review())
    if due_count:
        print(f"  {yellow(bold(f'  {due_count} chapter(s) due for review today'))}")
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
    _show_tag_weakness(profile, QUESTIONS)
    _show_session_history(profile)
    hr()
    pause()

def _show_tag_weakness(profile: UserProfile, questions: list) -> None:
    from .adaptive import tag_weakness
    weak = tag_weakness(profile, questions, top_n=8)
    if not weak:
        return
    print(f"  {cyan(bold('Weakest topics (by tag):'))}")
    for row in weak:
        pct  = int(row["mastery"] * 100)
        fill = int(row["mastery"] * 16)
        bar  = "█" * fill + "░" * (16 - fill)
        col  = green if pct >= 70 else yellow if pct >= 40 else red
        print(f"    {row['tag'][:24]:<24} {col(bar)} {pct:3d}%")
    print()


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
    print()

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
    per_q = duration_secs / max(total, 1)
    print(f"  {dim(f'Time: {duration_secs:.0f}s  ({per_q:.0f}s/question)')}", end="")
    if streak_max > 1:
        print(f"   {yellow(f'Best streak: {streak_max}')}", end="")
    if n_generated > 0:
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

def show_study_mode(chapter: int) -> None:
    from pathlib import Path
    from .config import CHAPTERS_DIR, CHAPTER_META

    meta = CHAPTER_META.get(chapter)
    if not meta:
        print_wrap(f"Chapter {chapter} not found.")
        return

    name = meta["name"]
    path = CHAPTERS_DIR / meta["file"]
    if not path.exists():
        print_wrap(f"Chapter file not found: {meta['file']}")
        return

    raw = path.read_text(errors="replace")
    prose = re.sub(r"```.*?```", "\n[code block omitted]\n", raw, flags=re.DOTALL)

    # Split into paragraphs, then group into ~2500-char pages
    paragraphs = [p.strip() for p in prose.split("\n\n") if p.strip()]
    pages: list[list[str]] = []
    current_page: list[str] = []
    current_len = 0
    for para in paragraphs:
        current_page.append(para)
        current_len += len(para)
        if current_len >= 2500:
            pages.append(current_page)
            current_page = []
            current_len = 0
    if current_page:
        pages.append(current_page)

    section(f"Ch.{chapter:02d} — {name}")

    def _render_paragraph(paragraph: str) -> None:
        if paragraph.startswith("#"):
            print(f"\n  {bold(cyan(paragraph.lstrip('#').strip()))}\n")
        elif paragraph.startswith("[code"):
            print(f"  {dim(paragraph)}\n")
        else:
            print_wrap(paragraph, indent=2)
            print()

    for page_num, page_paras in enumerate(pages, 1):
        for para in page_paras:
            _render_paragraph(para)
        if page_num < len(pages):
            hr()
            raw_in = ask(dim(f"  Page {page_num}/{len(pages)} — [Enter] to continue, q to stop → "))
            if raw_in.strip().lower() == "q":
                break
            print()

    hr()
    if confirm("Quiz on this chapter now?"):
        return

def show_main_menu(
    profile_name: str,
    api_available: bool,
    due_count: int = 0,
    review_queue_count: int = 0,
) -> str:
    print()
    hr()
    api_note = green("  (Claude AI enabled — dynamic questions on)") if api_available \
               else yellow("  (set ANTHROPIC_API_KEY to enable AI-generated questions)")
    print(f"\n  {bold(magenta('Abstract Algebra Adaptive Quiz'))}  •  {dim(profile_name)}")
    print(api_note)
    print()
    print(f"  {bold('1.')} Adaptive quiz        {dim('(focuses on your weakest areas)')}")
    due_note = f"  {yellow(f'{due_count} due')}" if due_count else ""
    print(f"  {bold('2.')} Review due chapters  {dim('(spaced repetition)')}{due_note}")
    rq_note  = f"  {yellow(f'{review_queue_count} queued')}" if review_queue_count else ""
    print(f"  {bold('3.')} Re-quiz wrong answers{dim('(retry your mistakes)')}{rq_note}")
    print(f"  {bold('4.')} Custom quiz          {dim('(choose phase, chapter, or topic)')}")
    print(f"  {bold('5.')} Study a chapter      {dim('(read, then optionally quiz)')}")
    print(f"  {bold('6.')} Mastery dashboard")
    print(f"  {bold('7.')} Switch profile")
    print(f"  {bold('8.')} Quit")
    print()
    hr()
    while True:
        ch = ask("  → ").strip()
        if ch in ("1", "2", "3", "4", "5", "6", "7", "8"):
            return ch
        print(yellow("  Please enter 1–8."))

def show_scope_menu(questions: list) -> "Scope":  # type: ignore[name-defined]
    from .adaptive import Scope
    tags = sorted({t for q in questions for t in (q.tags if hasattr(q, "tags") else [])})
    print()
    hr()
    section("Choose scope")
    print("  [1] All chapters")
    print("  [2] A specific phase")
    print("  [3] A specific chapter")
    print("  [4] A tag / topic")
    hr()
    while True:
        ch = ask("  → ").strip()
        if ch in ("1", "2", "3", "4"):
            break
        print(yellow("  Please enter 1–4."))

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
        ch_num = ask_int("\n  Chapter number → ", 0, 27, default=0)
        return Scope.chapter(ch_num)
    # ch == "4"
    if tags:
        tag_display = ", ".join(tags[:24])
        suffix = f" … +{len(tags) - 24} more" if len(tags) > 24 else ""
        print(f"\n  {dim('Available tags:')} {tag_display}{dim(suffix)}")
    tag = ask("  Tag → ").strip()
    return Scope.tag(tag) if tag else Scope.all()

def show_chapter_menu() -> int:
    print()
    hr()
    section("Choose a chapter to study")
    for k, meta in CHAPTER_META.items():
        ph = meta["phase"]
        print(f"    [{k:2d}] Ph.{ph}  Ch.{k:02d}  {meta['name']}")
    hr()
    return ask_int("  Chapter number → ", 0, 27, default=0)


def show_profile_summary_list(summaries: list[dict]) -> None:
    """Print a rich profile list with total answered and last-seen context."""
    section("Saved profiles")
    for i, s in enumerate(summaries, 1):
        name    = s.get("name", "?")
        total   = s.get("total_answered", 0)
        last    = (s.get("last_seen") or "")[:10]
        due     = s.get("due_count", 0)
        due_s   = f"  {yellow(f'{due} due')}" if due else ""
        seen_s  = dim(f"({total} answered, last: {last})") if total else dim("(new)")
        print(f"  [{i}] {bold(name):20s}  {seen_s}{due_s}")
    print(f"  [{len(summaries) + 1}] {dim('Create new profile')}")
