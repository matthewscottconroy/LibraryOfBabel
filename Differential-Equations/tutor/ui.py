"""Rich terminal UI — panels, prompts, progress, and stats."""

from __future__ import annotations

from rich.console import Console
from rich.panel import Panel
from rich.table import Table
from rich.text import Text
from rich.rule import Rule
from rich import box

from .adaptive import TOPIC_LABELS, elo_to_pct, elo_to_difficulty

console = Console()

STARS = {1: "★☆☆☆☆", 2: "★★☆☆☆", 3: "★★★☆☆", 4: "★★★★☆", 5: "★★★★★"}
DIFF_COLORS = {1: "green", 2: "cyan", 3: "yellow", 4: "orange3", 5: "red"}


# ── Banner ────────────────────────────────────────────────────────────────────

def banner() -> None:
    console.print()
    console.print(Panel(
        Text.assemble(
            ("  Math Tutor\n", "bold white"),
            ("  Adaptive Mathematics Problem Trainer", "dim white"),
        ),
        border_style="bright_blue",
        padding=(0, 4),
    ))
    console.print()


# ── Mastery dashboard ─────────────────────────────────────────────────────────

def stats_table(stats: list[dict], unlocked: list[str],
                session_count: int = 0) -> None:
    table = Table(
        title=f"Mastery Dashboard  [dim](sessions: {session_count})[/dim]",
        box=box.ROUNDED,
        border_style="bright_blue",
        title_style="bold white",
        show_lines=True,
    )
    table.add_column("Topic", style="bold")
    table.add_column("Status")
    table.add_column("ELO", justify="right")
    table.add_column("Level", justify="center")
    table.add_column("Mastery", justify="right")
    table.add_column("Seen", justify="right")
    table.add_column("Correct %", justify="right")
    table.add_column("Streak", justify="right")

    for row in stats:
        t = row["topic"]
        label = TOPIC_LABELS.get(t, t)
        elo = row["elo"]
        seen = row["seen"]
        correct = row["correct"]
        streak = row.get("streak", 0)
        best = row.get("best_streak", 0)
        pct = elo_to_pct(elo)
        diff = elo_to_difficulty(elo)
        stars = STARS[diff]
        color = DIFF_COLORS[diff]

        if t not in unlocked:
            status = Text("🔒 Locked", style="dim")
            label_text = Text(label, style="dim")
        else:
            status = Text("✓ Active", style="green")
            label_text = Text(label, style="bold")

        bar_filled = int(pct / 10)
        bar = "█" * bar_filled + "░" * (10 - bar_filled)
        pct_text = Text(f"{bar} {pct:4.0f}%", style=color)
        acc = f"{correct/seen*100:.0f}%" if seen > 0 else "—"
        streak_text = f"{streak} (best: {best})" if best > 0 else "—"

        table.add_row(
            label_text, status,
            Text(f"{elo:.0f}", style=color),
            Text(stars, style=color),
            pct_text, str(seen), acc, streak_text,
        )

    console.print(table)
    console.print()


# ── Session header ────────────────────────────────────────────────────────────

def session_header(problem_num: int, total: int, topic: str,
                   subtopic: str, difficulty: int, streak: int = 0) -> None:
    console.print()
    label = TOPIC_LABELS.get(topic, topic)
    stars = STARS[difficulty]
    color = DIFF_COLORS[difficulty]

    streak_part = (f" 🔥×{streak}" if streak >= 3 else "")
    console.print(Rule(
        Text.assemble(
            (f" Problem {problem_num}/{total}", "bold white"),
            (streak_part, "bold yellow"),
            (f"  {label} › {subtopic.replace('_', ' ').title()}  ", "dim"),
            (stars, color),
            (" ", ""),
        ),
        style="bright_blue",
    ))
    console.print()


# ── Problem display ───────────────────────────────────────────────────────────

def show_problem(question: str, problem_type: str,
                 choices: list[str] | None) -> None:
    body = Text(question, style="white")
    console.print(Panel(body, border_style="blue", padding=(1, 3)))

    if problem_type == "multiple_choice" and choices:
        console.print()
        for i, ch in enumerate(choices):
            letter = chr(ord("A") + i)
            console.print(f"  [bold cyan]{letter}.[/bold cyan] {ch}")

    console.print()
    if problem_type == "true_false":
        console.print("  [dim]Enter: True or False[/dim]")
    elif problem_type == "multiple_choice":
        console.print("  [dim]Enter the letter (A, B, C …) or the full answer[/dim]")
    else:
        console.print("  [dim]Enter your answer using Python syntax  "
                      "(^ → **, e^x → exp(x), ln → log)[/dim]")
    console.print()


# ── Prompting ─────────────────────────────────────────────────────────────────

def get_answer() -> str:
    console.print("  [dim]Commands: hint  skip  quit[/dim]")
    try:
        raw = console.input("  [bold yellow]Your answer:[/bold yellow] ").strip()
    except (EOFError, KeyboardInterrupt):
        raw = "quit"
    return raw


# ── Feedback ──────────────────────────────────────────────────────────────────

def show_correct(elo_delta: float, streak: int = 0,
                 hint_used: bool = False) -> None:
    delta_str = f"+{elo_delta:.0f}" if elo_delta >= 0 else f"{elo_delta:.0f}"
    hint_note = "  [dim](hint used — half credit)[/dim]" if hint_used else ""
    streak_note = f"  [bold yellow]🔥 {streak} in a row![/bold yellow]" if streak >= 3 else ""
    console.print(
        f"\n  [bold green]✓ Correct![/bold green]  [dim]ELO {delta_str}[/dim]"
        f"{hint_note}{streak_note}"
    )


def show_incorrect(correct_answer_str: str, elo_delta: float) -> None:
    delta_str = f"{elo_delta:.0f}"
    console.print(
        f"\n  [bold red]✗ Incorrect.[/bold red]  [dim]ELO {delta_str}[/dim]"
    )
    console.print(
        f"  [dim]Correct answer:[/dim] [bold white]{correct_answer_str}[/bold white]"
    )


def show_explanation(text: str) -> None:
    console.print()
    console.print(Panel(
        Text(text, style="dim white"),
        title="[dim]Explanation[/dim]",
        border_style="dim blue",
        padding=(1, 3),
    ))


def show_hint(text: str) -> None:
    console.print()
    console.print(Panel(
        Text(f"Hint: {text}", style="italic yellow"),
        border_style="yellow",
        padding=(0, 2),
    ))


def show_skip() -> None:
    console.print("  [dim]Skipped.[/dim]")


# ── Session summary ───────────────────────────────────────────────────────────

def session_summary(correct: int, total: int, seconds: float,
                    elo_changes: dict[str, float]) -> None:
    console.print()
    console.print(Rule(" Session Complete ", style="bright_blue"))
    console.print()

    pct = correct / total * 100 if total else 0
    mins = int(seconds // 60)
    secs = int(seconds % 60)
    color = "green" if pct >= 70 else "yellow" if pct >= 50 else "red"

    console.print(f"  Problems:  {total}")
    console.print(f"  Correct:   [{color}]{correct} ({pct:.0f}%)[/{color}]")
    console.print(f"  Time:      {mins}m {secs:02d}s")
    console.print()

    if elo_changes:
        console.print("  [bold]ELO changes this session:[/bold]")
        for topic, delta in elo_changes.items():
            label = TOPIC_LABELS.get(topic, topic)
            sign = "+" if delta >= 0 else ""
            col = "green" if delta >= 0 else "red"
            console.print(f"    {label:<35} [{col}]{sign}{delta:.0f}[/{col}]")
    console.print()


# ── Unlock notification ───────────────────────────────────────────────────────

def show_unlock(topic: str) -> None:
    label = TOPIC_LABELS.get(topic, topic)
    console.print()
    console.print(Panel(
        Text.assemble(
            ("  🔓 New topic unlocked: ", "bold yellow"),
            (label, "bold white"),
        ),
        border_style="yellow",
        padding=(0, 2),
    ))
