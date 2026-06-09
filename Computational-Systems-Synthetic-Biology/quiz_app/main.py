"""
Entry point for the CSSB Adaptive Quiz.

Usage:
    python3 -m quiz_app                           # interactive menus
    python3 -m quiz_app --chapter 10 --n 15       # direct topic quiz
    python3 -m quiz_app --phase 2                 # tier quiz
    python3 -m quiz_app --tag crispr              # tag quiz
    python3 -m quiz_app --study 17                # study then optionally quiz
    python3 -m quiz_app --app 11                  # quiz on the chapter taught by app 11
    python3 -m quiz_app --cathedral V             # check readiness for cathedral V
    python3 -m quiz_app --list-topics             # print all chapters and exit
    python3 -m quiz_app --export-progress         # dump profile to JSON and exit
    python3 -m quiz_app --export-progress out.json
    python3 -m quiz_app --import-progress backup.json
    python3 -m quiz_app --dry-run --chapter 10    # preview a generated question
    python3 -m quiz_app --profile Alice           # load named profile
"""
from __future__ import annotations

import argparse
import json
import random
import sys
from typing import Optional

from .adaptive import Scope, filter_by_scope
from .config import APP_CHAPTER_MAP, CATHEDRAL_PREREQS, CHAPTER_META, PHASE_NAMES
from .generator import ClaudeGenerator
from .session import QuizSession
from .storage import create_profile, list_profiles, load_or_create, save_profile
from . import ui


def run_onboarding(profile, generator: Optional[ClaudeGenerator]) -> None:
    from .question_bank import QUESTIONS
    from .config import PHASE_NAMES

    print()
    ui.section("Welcome Diagnostic")
    ui.print_wrap(
        "Since this is your first time, let's run a short diagnostic to calibrate "
        "your starting level. One beginner question per tier — answer honestly."
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
        ui.print_wrap("No beginner questions found — skipping diagnostic.")
        profile.onboarded = True
        save_profile(profile)
        return

    for i, q in enumerate(selected, 1):
        correct, _flagged, confidence = ui.present_question(q, i, len(selected))
        profile.record_answer(q.chapter, correct, q.question_id, confidence=confidence)
        save_profile(profile)

    print()
    ui.print_wrap("Diagnostic complete. Your mastery levels have been initialised.")
    ui.pause()
    profile.onboarded = True
    save_profile(profile)


def _choose_or_create_profile(initial_name: Optional[str] = None):
    if initial_name:
        return load_or_create(initial_name)

    profiles = list_profiles()

    if not profiles:
        print()
        ui.section("Welcome to the CSSB Adaptive Quiz!")
        ui.print_wrap(
            "This quiz covers 29 topics across 6 tiers of the Computational "
            "Systems & Synthetic Biology curriculum — from mathematical bedrock "
            "through synthetic biology, computational tools, and research craft."
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


def _ask_n_questions(pool_size: int, default: int = 10) -> int:
    cap = min(pool_size, 50)
    print()
    ui.hr()
    n = ui.ask_int(
        f"  How many questions? (1–{cap}, Enter for {min(default, cap)}) → ",
        1, cap, default=min(default, cap),
    )
    return n


def _list_topics() -> None:
    for k, meta in CHAPTER_META.items():
        ph = meta["phase"]
        pname = PHASE_NAMES.get(ph, f"Tier {ph}")
        print(f"  {k:2d}  T{ph}  {meta['name']:<38}  ({pname})")
    sys.exit(0)


def _export_progress(profile, dest: Optional[str]) -> None:
    data = json.dumps(profile.to_dict(), indent=2)
    if dest:
        from pathlib import Path
        Path(dest).write_text(data)
        print(f"  Progress exported to: {dest}")
    else:
        print(data)
    sys.exit(0)


def _import_progress(filepath: str, override_name: Optional[str]) -> None:
    from pathlib import Path
    from .models import UserProfile
    from .storage import save_profile

    try:
        data = json.loads(Path(filepath).read_text())
    except FileNotFoundError:
        print(f"  File not found: {filepath}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"  Invalid JSON in {filepath}: {e}")
        sys.exit(1)

    try:
        profile = UserProfile.from_dict(data)
    except (KeyError, TypeError) as e:
        print(f"  Could not parse profile: {e}")
        sys.exit(1)

    if override_name:
        profile.name = override_name
    save_profile(profile)
    n_sessions = len(profile.session_history)
    n_chapters = len(profile.mastery)
    print(
        f"  Imported profile '{profile.name}': "
        f"{n_chapters} chapters tracked, {n_sessions} session(s)."
    )
    sys.exit(0)


def _dry_run_generate(scope_args: argparse.Namespace) -> None:
    from .generator import ClaudeGenerator
    from .adaptive import preferred_difficulty
    from .config import CHAPTER_META

    generator = ClaudeGenerator()
    if not generator.available:
        print("  --dry-run requires ANTHROPIC_API_KEY to be set.")
        sys.exit(1)

    if scope_args.chapter is not None:
        chapter = scope_args.chapter
    elif getattr(scope_args, "app", None) is not None:
        chapter = APP_CHAPTER_MAP.get(scope_args.app, 0)
    else:
        chapter = 0

    difficulty = preferred_difficulty(0.5)
    meta = CHAPTER_META.get(chapter, {})
    ch_name = meta.get("name", f"Ch.{chapter}")

    print(f"\n  Generating preview: Ch.{chapter} — {ch_name}  [{difficulty}]\n")
    q = generator._generate(chapter, difficulty, 50, [])
    if not q:
        print("  Generation failed (API error or invalid response).")
        sys.exit(1)

    print(f"  [{q.kind.upper()}] {q.text}")
    if q.kind in ("mc", "tf"):
        for i, c in enumerate(q.choices):
            marker = "✓" if i == q.answer else " "
            print(f"    {marker} {c}")
    else:
        print(f"    acceptable: {q.choices}")
    print(f"\n  Explanation: {q.explanation}")
    print(f"  Tags: {q.tags}")
    print()
    sys.exit(0)


def _run_direct(args: argparse.Namespace) -> None:
    # Non-profile flags that don't need a quiz session
    if getattr(args, "list_topics", False):
        _list_topics()

    if getattr(args, "import_progress", None):
        _import_progress(args.import_progress, args.profile)

    if getattr(args, "dry_run", False):
        _dry_run_generate(args)

    profile   = _choose_or_create_profile(args.profile)
    generator = ClaudeGenerator()

    if getattr(args, "export_progress", None) is not False:
        _export_progress(profile, args.export_progress)

    if getattr(args, "cathedral", None):
        ui.show_cathedral_readiness(profile, args.cathedral)
        return

    if getattr(args, "app", None) is not None:
        chapter = APP_CHAPTER_MAP.get(args.app)
        if chapter is None:
            valid = sorted(APP_CHAPTER_MAP)
            ui.print_wrap(f"App {args.app} not in map. Valid app numbers: {valid[0]}–{valid[-1]}.")
            sys.exit(1)
        scope = Scope.chapter(chapter)
        ch_name = CHAPTER_META.get(chapter, {}).get("name", f"Ch.{chapter}")
        print(f"\n  App {args.app:02d} → {ui.cyan(ch_name)} (Chapter {chapter})\n")
    elif args.study is not None:
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


def main(argv: Optional[list[str]] = None) -> None:
    parser = argparse.ArgumentParser(
        prog="python3 -m quiz_app",
        description="CSSB Adaptive Quiz",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--profile", "-p", metavar="NAME")
    parser.add_argument("--chapter", "-c", type=int, metavar="N",
                        help="Quiz directly on topic N (0–28)")
    parser.add_argument("--phase", type=int, metavar="N",
                        help="Quiz directly on tier N (0–5)")
    parser.add_argument("--tag", "-t", metavar="TAG")
    parser.add_argument("--n", "-n", type=int, default=10, metavar="N")
    parser.add_argument("--study", type=int, metavar="TOPIC")
    parser.add_argument("--app", type=int, metavar="N",
                        help="Quiz on the chapter taught by interactive app N (1–22)")
    parser.add_argument("--cathedral", metavar="ID",
                        help="Show prerequisite readiness for cathedral I–VII")
    parser.add_argument("--list-topics", action="store_true",
                        help="Print all curriculum topics and exit")
    parser.add_argument("--export-progress", nargs="?", const=None, default=False,
                        metavar="FILE",
                        help="Export profile to JSON (stdout if FILE omitted)")
    parser.add_argument("--import-progress", metavar="FILE",
                        help="Import a profile from a JSON file exported by --export-progress")
    parser.add_argument("--dry-run", action="store_true",
                        help="Generate and preview a question without caching (requires API key)")
    args = parser.parse_args(argv)

    direct = any([
        args.chapter is not None, args.phase is not None,
        args.tag is not None, args.study is not None,
        getattr(args, "app", None) is not None,
        getattr(args, "cathedral", None) is not None,
        getattr(args, "list_topics", False),
        getattr(args, "export_progress", False) is not False,
        getattr(args, "import_progress", None) is not None,
        getattr(args, "dry_run", False),
    ])
    if direct:
        _run_direct(args)
        return

    print()
    ui.hr("═")
    print(f"\n  {ui.bold(ui.magenta('CSSB Adaptive Quiz'))}\n")
    ui.hr("═")

    profile   = _choose_or_create_profile(args.profile)
    generator = ClaudeGenerator()

    if not profile.onboarded:
        run_onboarding(profile, generator)

    while True:
        due_count = len(profile.chapters_due_for_review())
        choice = ui.show_main_menu(profile.name, generator.available, due_count)

        if choice == "1":
            from .question_bank import QUESTIONS
            n = _ask_n_questions(len(QUESTIONS))
            sess = QuizSession(profile, Scope.adaptive(), n, generator)
            sess.run()

        elif choice == "2":
            from .question_bank import QUESTIONS
            scope  = ui.show_scope_menu(len(QUESTIONS))
            scoped = filter_by_scope(QUESTIONS, scope)
            n = _ask_n_questions(max(len(scoped), 1))
            sess = QuizSession(profile, scope, n, generator)
            sess.run()

        elif choice == "3":
            chapter = ui.show_chapter_menu()
            wants_quiz = ui.show_study_mode(chapter)
            if wants_quiz:
                from .question_bank import QUESTIONS
                scoped = filter_by_scope(QUESTIONS, Scope.chapter(chapter))
                if scoped:
                    n = _ask_n_questions(len(scoped), default=5)
                    sess = QuizSession(profile, Scope.chapter(chapter), n, generator)
                    sess.run()

        elif choice == "4":
            ui.show_dashboard(profile)

        elif choice == "5":
            profile = _choose_or_create_profile()
            if not profile.onboarded:
                run_onboarding(profile, generator)

        elif choice == "6":
            ui.show_cathedral_menu(profile)

        elif choice == "7":
            print(f"\n  {ui.dim('See you next time.')}\n")
            sys.exit(0)


if __name__ == "__main__":
    main()
