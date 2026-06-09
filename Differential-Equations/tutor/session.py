"""Main quiz session loop."""

from __future__ import annotations

import time
from typing import Optional

from . import db, ui
from .adaptive import update_elo, select_topic, select_difficulty, unlocked_topics
from .problems import get_problem

DEFAULT_SESSION_LENGTH = 20


class Session:
    def __init__(self, profile: str = "default") -> None:
        self.profile = profile
        db.init_db(profile)

    # ── Public entry points ───────────────────────────────────────────────────

    def run(self, n_problems: int = DEFAULT_SESSION_LENGTH,
            topic_filter: Optional[str] = None) -> None:
        ui.banner()

        mastery = db.get_mastery(self.profile)
        stats = db.get_stats(self.profile)
        available = unlocked_topics(mastery)

        if topic_filter:
            if topic_filter not in available:
                ui.console.print(
                    f"  [red]Topic '{topic_filter}' is locked or unknown.[/red]\n"
                    f"  Available: {', '.join(available)}"
                )
                return
            available = [topic_filter]

        self._show_available(available, mastery)

        session_id = db.start_session(self.profile)
        correct_count = 0
        skip_count = 0
        elo_start = dict(mastery)
        session_start = time.time()
        last_topic: Optional[str] = None
        problem_num = 0
        streak = 0

        while problem_num < n_problems:
            mastery = db.get_mastery(self.profile)
            stats = db.get_stats(self.profile)

            if topic_filter:
                topic = topic_filter
            else:
                topic = select_topic(mastery, last_topic, stats, self.profile)

            elo = mastery[topic]
            difficulty = select_difficulty(elo)

            prob = get_problem(topic, difficulty)
            if prob is None:
                continue

            problem_num += 1
            hint_used = False

            ui.session_header(problem_num, n_problems, topic,
                              prob.subtopic, difficulty, streak)
            ui.show_problem(prob.question, prob.problem_type, prob.choices)

            t_start = time.time()
            answered = False

            while not answered:
                raw = ui.get_answer()
                cmd = raw.lower().strip()

                if cmd in ("quit", "q"):
                    elapsed = time.time() - session_start
                    db.end_session(session_id, problem_num - 1, correct_count, elapsed)
                    self._finish(correct_count, problem_num - 1, elapsed, elo_start)
                    return

                elif cmd in ("skip", "s"):
                    ui.show_skip()
                    skip_count += 1
                    streak = 0
                    t_sec = time.time() - t_start
                    old_elo = mastery[topic]
                    new_elo = update_elo(old_elo, difficulty, 0.0, prob.problem_type)
                    db.update_mastery(
                        topic, new_elo, False, 0.0, t_sec,
                        prob.subtopic, difficulty,
                        hint_used=False, skipped=True, profile=self.profile,
                    )
                    ui.show_explanation(prob.explanation)
                    self._check_unlocks(mastery, db.get_mastery(self.profile))
                    answered = True

                elif cmd in ("hint", "h"):
                    if prob.hint:
                        ui.show_hint(prob.hint)
                        hint_used = True
                    else:
                        ui.console.print("  [dim]No hint available for this problem.[/dim]")

                else:
                    t_sec = time.time() - t_start
                    ok, parse_err = prob.check_answer(raw)

                    if parse_err:
                        ui.console.print(f"  [yellow]{parse_err}[/yellow]")
                        continue

                    old_elo = mastery[topic]
                    actual_score = 0.5 if (ok and hint_used) else float(ok)
                    new_elo = update_elo(old_elo, difficulty, actual_score, prob.problem_type)
                    db.update_mastery(
                        topic, new_elo, ok, actual_score, t_sec,
                        prob.subtopic, difficulty,
                        hint_used=hint_used, skipped=False, profile=self.profile,
                    )

                    delta = new_elo - old_elo
                    if ok:
                        streak += 1
                        ui.show_correct(delta, streak, hint_used)
                        correct_count += 1
                    else:
                        streak = 0
                        ui.show_incorrect(prob.answer_display(), delta)

                    ui.show_explanation(prob.explanation)
                    self._check_unlocks(mastery, db.get_mastery(self.profile))
                    answered = True

            last_topic = topic

        elapsed = time.time() - session_start
        db.end_session(session_id, n_problems, correct_count, elapsed)
        self._finish(correct_count, n_problems, elapsed, elo_start)

    def show_stats(self) -> None:
        ui.banner()
        mastery = db.get_mastery(self.profile)
        stats = db.get_stats(self.profile)
        unlocked = unlocked_topics(mastery)
        session_count = db.get_session_count(self.profile)
        ui.stats_table(stats, unlocked, session_count)

    # ── Helpers ───────────────────────────────────────────────────────────────

    def _show_available(self, available: list[str], mastery: dict[str, float]) -> None:
        from .adaptive import TOPIC_LABELS, elo_to_pct
        ui.console.print("  [bold]Active topics:[/bold]")
        for t in available:
            pct = elo_to_pct(mastery.get(t, 450.0))
            label = TOPIC_LABELS.get(t, t)
            bar_filled = int(pct / 10)
            bar = "█" * bar_filled + "░" * (10 - bar_filled)
            ui.console.print(f"    {label:<38} {bar} {pct:3.0f}%")
        ui.console.print()

    def _check_unlocks(self, old_mastery: dict[str, float],
                       new_mastery: dict[str, float]) -> None:
        old_avail = set(unlocked_topics(old_mastery))
        new_avail = set(unlocked_topics(new_mastery))
        for topic in new_avail - old_avail:
            ui.show_unlock(topic)

    def _finish(self, correct: int, total: int, elapsed: float,
                elo_start: dict[str, float]) -> None:
        final_mastery = db.get_mastery(self.profile)
        changes = {
            t: final_mastery[t] - elo_start[t]
            for t in elo_start
            if abs(final_mastery.get(t, elo_start[t]) - elo_start[t]) > 0.5
        }
        ui.session_summary(correct, total, elapsed, changes)

        try:
            raw = ui.console.input(
                "  Show mastery dashboard? [dim][y/N][/dim]: "
            ).strip().lower()
        except (EOFError, KeyboardInterrupt):
            raw = ""
        if raw in ("y", "yes"):
            stats = db.get_stats(self.profile)
            unlocked = unlocked_topics(final_mastery)
            session_count = db.get_session_count(self.profile)
            ui.stats_table(stats, unlocked, session_count)
