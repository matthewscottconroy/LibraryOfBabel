"""
Entry point — profile management, onboarding, CLI flags, and main menu loop.

Usage:
    python3 -m quiz_app                          # interactive menus
    python3 -m quiz_app --chapter 18 --n 15      # direct chapter quiz
    python3 -m quiz_app --phase 5                # phase quiz
    python3 -m quiz_app --tag paths              # tag quiz
    python3 -m quiz_app --study 18               # study then optionally quiz
    python3 -m quiz_app --profile Alice          # load named profile
"""
from __future__ import annotations

import argparse
import random
import sys
from datetime import date
from pathlib import Path
from typing import Optional

from .adaptive import Scope, filter_by_scope
from .config import CHAPTER_META, PHASE_NAMES
from .generator import ClaudeGenerator
from .session import QuizSession
from .storage import create_profile, list_profiles, load_or_create, save_profile
from . import ui


# ── Onboarding ────────────────────────────────────────────────────────────────

def run_onboarding(profile, generator: Optional[ClaudeGenerator]) -> None:
    """
    One-time diagnostic for fresh users.
    Picks one beginner question per phase, runs them, and uses the results
    to set initial mastery priors so the adaptive engine starts calibrated.
    """
    from .question_bank import QUESTIONS

    print()
    ui.section("Welcome Diagnostic")
    ui.print_wrap(
        "Since this is your first time, let's run a short diagnostic to calibrate "
        "your starting level. One beginner question per phase — answer honestly. "
        "This sets your initial mastery and makes the adaptive quiz immediately useful."
    )
    print()
    if not ui.confirm("Run the diagnostic now? (recommended)"):
        profile.onboarded = True
        save_profile(profile)
        return

    rng = random.Random()
    selected = []
    for ph in sorted(PHASE_NAMES.keys()):
        candidates = [
            q for q in QUESTIONS
            if q.phase == ph and q.difficulty == "beginner"
        ]
        if candidates:
            selected.append(rng.choice(candidates))

    if not selected:
        profile.onboarded = True
        save_profile(profile)
        return

    for i, q in enumerate(selected, 1):
        correct, _flagged, _conf = ui.present_question(q, i, len(selected))
        profile.record_answer(q.chapter, correct, q.question_id)
        save_profile(profile)

    print()
    ui.print_wrap(
        "Diagnostic complete. Your mastery levels have been initialised — "
        "the adaptive quiz will now focus on the areas where you need practice."
    )
    ui.pause()

    profile.onboarded = True
    save_profile(profile)


# ── Profile selection ──────────────────────────────────────────────────────────

def _choose_or_create_profile(initial_name: Optional[str] = None):
    """Prompt the user to pick an existing profile or create a new one."""
    if initial_name:
        return load_or_create(initial_name)

    profiles = list_profiles()

    if not profiles:
        print()
        ui.section("Welcome to the HoTT Adaptive Quiz!")
        ui.print_wrap(
            "This quiz covers all 27 chapters of the Homotopy Type Theory curriculum — "
            "from logic and set theory through cubical type theory and research frontiers."
        )
        ui.print_wrap(
            "It tracks your mastery per chapter and adjusts question difficulty and "
            "topic selection based on where you need the most practice."
        )
        print()
        name = ui.ask("  Your name → ").strip() or "Student"
        return create_profile(name)

    print()
    ui.section("Select a profile")
    for i, name in enumerate(profiles):
        print(f"  [{i + 1}] {name}")
    print(f"  [{len(profiles) + 1}] Create new profile")
    ui.hr()
    choice = ui.ask_int("  → ", 1, len(profiles) + 1, default=1)

    if choice <= len(profiles):
        return load_or_create(profiles[choice - 1])

    name = ui.ask("  New profile name → ").strip() or "Student"
    return create_profile(name)


# ── Quiz configuration ─────────────────────────────────────────────────────────

def _ask_n_questions(pool_size: int, default: int = 10) -> int:
    cap = min(pool_size, 50)
    print()
    ui.hr()
    n = ui.ask_int(
        f"  How many questions? (1–{cap}, Enter for {min(default, cap)}) → ",
        1, cap, default=min(default, cap),
    )
    return n


# ── Prerequisite check ─────────────────────────────────────────────────────────

def _check_prereqs(profile, chapter: int) -> bool:
    """
    Warn the user if their mastery in the preceding phase is low.
    Returns False if they chose to cancel, True to proceed.
    """
    ch_phase = CHAPTER_META.get(chapter, {}).get("phase", 0)
    if ch_phase == 0:
        return True  # Phase 0 has no prerequisites

    prev_phase = ch_phase - 1
    prev_chs   = [k for k, v in CHAPTER_META.items() if v["phase"] == prev_phase]
    started    = [ch for ch in prev_chs if ch in profile.mastery]

    avg = (
        sum(profile.mastery[ch].score for ch in started) / len(started)
        if started else 0.0
    )

    # Only warn when fewer than half the previous-phase chapters have been attempted
    # or when the average mastery is below 40%
    if avg >= 0.4 and len(started) >= len(prev_chs) * 0.5:
        return True

    return ui.show_prereq_warning(
        chapter, prev_phase, avg, len(started), len(prev_chs)
    )


# ── Progress export ────────────────────────────────────────────────────────────

def _export_progress(profile) -> None:
    from .adaptive import mastery_summary
    from .config import DATA_DIR

    rows  = mastery_summary(profile)
    today = date.today().isoformat()
    lines = [
        f"# HoTT Quiz Progress — {profile.name}",
        f"Exported: {today}  |  Total answered: {profile.total_answered()}",
        "",
        "| Phase | Ch | Name | Score | Seen | Correct | Due | Next Review |",
        "|------:|---:|------|------:|-----:|--------:|-----|-------------|",
    ]
    for row in rows:
        due = "✓" if row["due"] else ""
        nr  = row["next_review"] or ""
        lines.append(
            f"| {row['phase']} "
            f"| {row['chapter']:02d} "
            f"| {row['name']} "
            f"| {row['score']:.2f} "
            f"| {row['seen']} "
            f"| {row['correct']} "
            f"| {due} "
            f"| {nr} |"
        )

    out = DATA_DIR / f"progress_export_{today}.md"
    out.write_text("\n".join(lines))
    ui.show_export_result(out)


# ── Direct (CLI) session ───────────────────────────────────────────────────────

def _run_direct(args: argparse.Namespace) -> None:
    """Non-interactive path: run a single session from CLI arguments."""
    profile   = _choose_or_create_profile(args.profile)
    generator = ClaudeGenerator()

    if args.study is not None:
        ui.show_study_mode(args.study)
        scope = Scope.chapter(args.study)
    elif args.chapter is not None:
        scope = Scope.chapter(args.chapter)
    elif args.phase is not None:
        scope = Scope.phase(args.phase)
    elif args.tag is not None:
        scope = Scope.tag(args.tag)
    else:
        scope = Scope.adaptive()

    from .question_bank import QUESTIONS
    pool = filter_by_scope(QUESTIONS, scope)
    n    = min(args.n, max(len(pool), 1))

    sess = QuizSession(profile, scope, n, generator)
    sess.run()


# ── Main loop ──────────────────────────────────────────────────────────────────

def main(argv: Optional[list[str]] = None) -> None:
    parser = argparse.ArgumentParser(
        prog="python3 -m quiz_app",
        description="HoTT Adaptive Quiz",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--profile", "-p", metavar="NAME",
                        help="Use or create profile NAME without prompting")
    parser.add_argument("--chapter", "-c", type=int, metavar="N",
                        help="Quiz directly on chapter N (0–26)")
    parser.add_argument("--phase", type=int, metavar="N",
                        help="Quiz directly on phase N (0–8)")
    parser.add_argument("--tag", "-t", metavar="TAG",
                        help="Quiz directly on questions tagged TAG")
    parser.add_argument("--n", "-n", type=int, default=10, metavar="N",
                        help="Number of questions (default: 10)")
    parser.add_argument("--study", type=int, metavar="CHAPTER",
                        help="Study chapter CHAPTER, then optionally quiz on it")
    args = parser.parse_args(argv)

    # Direct mode: at least one quiz-scoping flag was given
    direct = any([
        args.chapter is not None,
        args.phase is not None,
        args.tag is not None,
        args.study is not None,
    ])
    if direct:
        _run_direct(args)
        return

    # ── Interactive menu loop ─────────────────────────────────────────────────
    print()
    ui.hr("═")
    print(f"\n  {ui.bold(ui.magenta('HoTT Adaptive Quiz'))}\n")
    ui.hr("═")

    profile   = _choose_or_create_profile(args.profile)
    generator = ClaudeGenerator()

    # Run onboarding diagnostic for new users
    if not profile.onboarded:
        run_onboarding(profile, generator)

    try:
        while True:
            due_count     = len(profile.chapters_due_for_review())
            flagged_count = len(profile.review_queue)
            choice = ui.show_main_menu(
                profile.name, generator.available,
                due_count=due_count, flagged_count=flagged_count,
            )

            if choice == "1":
                from .question_bank import QUESTIONS
                n    = _ask_n_questions(len(QUESTIONS))
                tlim = ui.ask_time_limit()
                sess = QuizSession(profile, Scope.adaptive(), n, generator,
                                   time_limit_secs=tlim)
                sess.run()

            elif choice == "2":
                from .question_bank import QUESTIONS
                scope = ui.show_scope_menu(len(QUESTIONS), flagged_count=flagged_count)

                # If user selected "flagged", build a review scope
                if scope.kind == "due" and flagged_count:
                    scope = Scope.review(list(profile.review_queue))

                # Prerequisite check for chapter scopes
                if scope.kind == "chapter" and isinstance(scope.value, int):
                    if not _check_prereqs(profile, scope.value):
                        continue

                scoped = filter_by_scope(QUESTIONS, scope, profile)
                if not scoped:
                    ui.print_wrap("No questions match that scope.")
                    continue
                n    = _ask_n_questions(max(len(scoped), 1))
                tlim = ui.ask_time_limit()
                sess = QuizSession(profile, scope, n, generator,
                                   time_limit_secs=tlim)
                sess.run()

            elif choice == "3":
                chapter = ui.show_chapter_menu()
                ui.show_study_mode(chapter)
                from .question_bank import QUESTIONS
                scoped = filter_by_scope(QUESTIONS, Scope.chapter(chapter))
                if scoped:
                    n = _ask_n_questions(len(scoped), default=5)
                    sess = QuizSession(profile, Scope.chapter(chapter), n, generator)
                    sess.run()

            elif choice == "4":
                ui.show_dashboard(profile)

            elif choice == "5":
                _export_progress(profile)

            elif choice == "6":
                profile = _choose_or_create_profile()
                if not profile.onboarded:
                    run_onboarding(profile, generator)

            elif choice == "7":
                print(f"\n  {ui.dim('See you next time.')}\n")
                sys.exit(0)

    except ui.QuizQuit:
        print(f"\n  {ui.dim('See you next time.')}\n")
        sys.exit(0)


if __name__ == "__main__":
    main()
