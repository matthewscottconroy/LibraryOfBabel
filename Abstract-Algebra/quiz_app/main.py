"""
Entry point for the Abstract Algebra Adaptive Quiz.

Usage:
    python3 -m quiz_app                          # interactive menus
    python3 -m quiz_app --chapter 8 --n 15       # direct chapter quiz
    python3 -m quiz_app --phase 2                # phase quiz
    python3 -m quiz_app --tag groups             # tag quiz
    python3 -m quiz_app --study 12               # study then optionally quiz
    python3 -m quiz_app --profile Alice          # load named profile
    python3 -m quiz_app --list-profiles          # print all saved profiles
    python3 -m quiz_app --delete-profile Alice   # delete a profile
    python3 -m quiz_app --export                 # export progress summary to stdout
"""
from __future__ import annotations

import argparse
import csv
import io
import random
import sys
from typing import Optional

from .adaptive import Scope, filter_by_scope, mastery_summary
from .generator import ClaudeGenerator
from .session import QuizSession
from .storage import (
    create_profile, delete_profile, list_profiles, list_profiles_with_summary,
    load_or_create, save_profile,
)
from . import ui


def run_onboarding(profile, generator: Optional[ClaudeGenerator]) -> None:
    from .question_bank import QUESTIONS
    from .config import PHASE_NAMES

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
        candidates = [q for q in QUESTIONS if q.phase == ph and q.difficulty == "beginner"]
        if candidates:
            selected.append(rng.choice(candidates))

    if not selected:
        profile.onboarded = True
        save_profile(profile)
        return

    for i, q in enumerate(selected, 1):
        correct = ui.present_question(q, i, len(selected))
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


def _choose_or_create_profile(initial_name: Optional[str] = None):
    if initial_name:
        return load_or_create(initial_name)

    summaries = list_profiles_with_summary()

    if not summaries:
        print()
        ui.section("Welcome to the Abstract Algebra Adaptive Quiz!")
        ui.print_wrap(
            "This quiz covers all 28 chapters of the Abstract Algebra curriculum — "
            "from logic and set theory through Lie theory and foundations."
        )
        ui.print_wrap(
            "It tracks your mastery per chapter and adjusts question difficulty and "
            "topic selection based on where you need the most practice."
        )
        print()
        name = ui.ask("  Your name → ").strip() or "Student"
        return create_profile(name)

    print()
    ui.show_profile_summary_list(summaries)
    ui.hr()
    choice = ui.ask_int("  → ", 1, len(summaries) + 1, default=1)

    if choice <= len(summaries):
        return load_or_create(summaries[choice - 1]["name"])

    name = ui.ask("  New profile name → ").strip() or "Student"
    return create_profile(name)


def _ask_n_questions(pool_size: int, default: int = 10) -> int:
    if pool_size <= 0:
        return 0
    cap = min(pool_size, 50)
    print()
    ui.hr()
    n = ui.ask_int(
        f"  How many questions? (1–{cap}, Enter for {min(default, cap)}) → ",
        1, cap, default=min(default, cap),
    )
    return n


def _run_direct(args: argparse.Namespace) -> None:
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


def _show_stats(profile) -> None:
    """Print a quick stats summary without entering interactive mode."""
    from .adaptive import mastery_summary, weak_topics
    rows = mastery_summary(profile)
    started = [r for r in rows if r["started"]]
    total   = profile.total_answered()
    due     = len(profile.chapters_due_for_review())
    review  = len(profile.review_queue)
    avg     = sum(r["score"] for r in started) / max(len(started), 1)
    print(f"\nProfile: {profile.name}")
    print(f"  Total answered : {total}")
    print(f"  Chapters started: {len(started)} / {len(rows)}")
    print(f"  Average mastery : {avg:.0%}")
    print(f"  Due for review  : {due}")
    print(f"  Review queue    : {review}")
    if started:
        print("\n  Weakest chapters:")
        for name in weak_topics(profile, n=5):
            print(f"    • {name}")
    if due:
        print("\n  Due chapters:")
        for r in rows:
            if r["due"]:
                print(f"    • Ch.{r['chapter']:02d} {r['name']}")


def _export_progress_csv(profile) -> None:
    """Write a CSV of chapter mastery to stdout."""
    rows = mastery_summary(profile)
    out = io.StringIO()
    writer = csv.DictWriter(
        out,
        fieldnames=["chapter", "phase", "name", "score", "seen",
                    "correct", "started", "due", "next_review"],
    )
    writer.writeheader()
    for row in rows:
        writer.writerow(row)
    print(out.getvalue(), end="")


def main(argv: Optional[list[str]] = None) -> None:
    parser = argparse.ArgumentParser(
        prog="python3 -m quiz_app",
        description="Abstract Algebra Adaptive Quiz",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--profile", "-p", metavar="NAME")
    parser.add_argument("--chapter", "-c", type=int, metavar="N",
                        help="Quiz directly on chapter N (0–27)")
    parser.add_argument("--phase", type=int, metavar="N",
                        help="Quiz directly on phase N (0–8)")
    parser.add_argument("--tag", "-t", metavar="TAG")
    parser.add_argument("--n", "-n", type=int, default=10, metavar="N")
    parser.add_argument("--study", type=int, metavar="CHAPTER")
    parser.add_argument("--list-profiles", action="store_true",
                        help="Print all saved profiles and exit")
    parser.add_argument("--delete-profile", metavar="NAME",
                        help="Delete a saved profile and exit")
    parser.add_argument("--export", action="store_true",
                        help="Export progress CSV for --profile to stdout and exit")
    parser.add_argument("--stats", action="store_true",
                        help="Print a quick stats summary for --profile and exit")
    args = parser.parse_args(argv)

    if args.list_profiles:
        names = list_profiles()
        if names:
            for n in names:
                print(n)
        else:
            print("(no profiles saved)")
        return

    if args.delete_profile:
        removed = delete_profile(args.delete_profile)
        if removed:
            print(f"Deleted profile: {args.delete_profile}")
        else:
            print(f"No profile found for: {args.delete_profile}", file=sys.stderr)
            sys.exit(1)
        return

    if args.export:
        profile = load_or_create(args.profile or "Student")
        _export_progress_csv(profile)
        return

    if args.stats:
        profile = load_or_create(args.profile or "Student")
        _show_stats(profile)
        return

    direct = any([
        args.chapter is not None, args.phase is not None,
        args.tag is not None, args.study is not None,
    ])
    if direct:
        _run_direct(args)
        return

    print()
    ui.hr("═")
    print(f"\n  {ui.bold(ui.magenta('Abstract Algebra Adaptive Quiz'))}\n")
    ui.hr("═")

    profile   = _choose_or_create_profile(args.profile)
    generator = ClaudeGenerator()

    if not profile.onboarded:
        run_onboarding(profile, generator)

    while True:
        from .question_bank import QUESTIONS
        due_count    = len(profile.chapters_due_for_review())
        review_count = len(profile.review_queue)
        choice = ui.show_main_menu(
            profile.name, generator.available, due_count, review_count
        )

        if choice == "1":
            n = _ask_n_questions(len(QUESTIONS))
            if n:
                sess = QuizSession(profile, Scope.adaptive(), n, generator)
                sess.run()

        elif choice == "2":
            due_pool = filter_by_scope(QUESTIONS, Scope.due(), profile)
            if not due_pool:
                ui.print_wrap("No chapters are currently due for review.")
                ui.pause()
            else:
                n = _ask_n_questions(len(due_pool), default=10)
                if n:
                    sess = QuizSession(profile, Scope.due(), n, generator, pool=due_pool)
                    sess.run()

        elif choice == "3":
            if not profile.review_queue:
                ui.print_wrap("Your re-quiz queue is empty — answer some questions wrong first!")
                ui.pause()
            else:
                scope    = Scope.review(list(profile.review_queue))
                rq_pool  = filter_by_scope(QUESTIONS, scope)
                n = _ask_n_questions(max(len(rq_pool), 1), default=min(len(rq_pool), 10))
                if n:
                    sess = QuizSession(profile, scope, n, generator, pool=rq_pool)
                    sess.run()

        elif choice == "4":
            scope  = ui.show_scope_menu(QUESTIONS)
            scoped = filter_by_scope(QUESTIONS, scope, profile)
            n = _ask_n_questions(max(len(scoped), 1))
            if n:
                sess = QuizSession(profile, scope, n, generator)
                sess.run()

        elif choice == "5":
            chapter = ui.show_chapter_menu()
            ui.show_study_mode(chapter)
            scoped = filter_by_scope(QUESTIONS, Scope.chapter(chapter))
            if scoped:
                n = _ask_n_questions(len(scoped), default=5)
                if n:
                    sess = QuizSession(profile, Scope.chapter(chapter), n, generator)
                    sess.run()

        elif choice == "6":
            ui.show_dashboard(profile)

        elif choice == "7":
            profile = _choose_or_create_profile()
            if not profile.onboarded:
                run_onboarding(profile, generator)

        elif choice == "8":
            print(f"\n  {ui.dim('See you next time.')}\n")
            sys.exit(0)


if __name__ == "__main__":
    main()
