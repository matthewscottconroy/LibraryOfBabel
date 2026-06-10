#!/usr/bin/env python3
"""
HoTT Quiz — interactive quiz program covering all 27 chapters of the curriculum.

Run:  python3 quiz.py
"""

import json
import pathlib
import random
import sys
import textwrap
from dataclasses import dataclass, field
from typing import Literal

# ─── ANSI colours ─────────────────────────────────────────────────────────────

def _c(code: str, text: str) -> str:
    return f"\033[{code}m{text}\033[0m"

def bold(t):    return _c("1", t)
def green(t):   return _c("32", t)
def red(t):     return _c("31", t)
def yellow(t):  return _c("33", t)
def cyan(t):    return _c("36", t)
def dim(t):     return _c("2", t)
def blue(t):    return _c("34", t)
def magenta(t): return _c("35", t)

WIDTH = 78

def hr(char="─"): print(dim(char * WIDTH))
def section(title): print(f"\n{bold(cyan(title))}\n")

def wrap(text, indent=0):
    prefix = " " * indent
    return textwrap.fill(text, width=WIDTH - indent, initial_indent=prefix,
                         subsequent_indent=prefix)

# ─── Data model ───────────────────────────────────────────────────────────────

QuestionKind = Literal["mc", "tf", "blank"]

@dataclass
class Question:
    chapter: int
    phase: int
    kind: QuestionKind
    text: str
    choices: list[str]          # mc: [A,B,C,D]; tf: ["True","False"]; blank: synonyms
    answer: int | str           # mc/tf: 0-based index; blank: the exact string
    explanation: str
    tags: list[str] = field(default_factory=list)


# ─── Question bank loader ─────────────────────────────────────────────────────

def _load_questions() -> list[Question]:
    here = pathlib.Path(__file__).parent
    q_dir = here / "questions"
    questions: list[Question] = []
    for json_file in sorted(q_dir.rglob("*.json")):
        try:
            data = json.loads(json_file.read_text(encoding="utf-8"))
            questions.append(Question(
                chapter=int(data["chapter"]),
                phase=int(data["phase"]),
                kind=data["kind"],
                text=data["text"],
                choices=data["choices"],
                answer=data["answer"],
                explanation=data.get("explanation", ""),
                tags=data.get("tags", []),
            ))
        except Exception:
            pass  # skip malformed files silently
    return questions


Q: list[Question] = _load_questions()


# ─── Quiz engine ──────────────────────────────────────────────────────────────

PHASE_NAMES = {
    0: "Phase 0 — Mathematical Foundations",
    1: "Phase 1 — Logic and Computation",
    2: "Phase 2 — Dependent Types and MLTT",
    3: "Phase 3 — Category Theory and Categorical Logic",
    4: "Phase 4 — Topology and Homotopy Theory",
    5: "Phase 5 — Core HoTT",
    6: "Phase 6 — Proof Assistants",
    7: "Phase 7 — Advanced Foundations",
    8: "Phase 8 — Research Frontiers",
}

CH_NAMES = {
    0: "Ch.00 Logic and Proof",
    1: "Ch.01 Set Theory",
    2: "Ch.02 Abstract Algebra",
    3: "Ch.03 Real Analysis",
    4: "Ch.04 Proof Theory",
    5: "Ch.05 Intuitionistic Logic",
    6: "Ch.06 Curry-Howard",
    7: "Ch.07 STLC and System F",
    8: "Ch.08 Dependent Types",
    9: "Ch.09 MLTT",
    10: "Ch.10 Category Theory",
    11: "Ch.11 Categorical Logic",
    12: "Ch.12 Higher Categories",
    13: "Ch.13 Topology",
    14: "Ch.14 Homotopy Theory",
    15: "Ch.15 Simplicial Sets",
    16: "Ch.16 Identity Types",
    17: "Ch.17 H-Levels and Truncations",
    18: "Ch.18 Univalence",
    19: "Ch.19 Higher Inductive Types",
    20: "Ch.20 Synthetic Homotopy Theory",
    21: "Ch.21 Lean 4 and Mathlib",
    22: "Ch.22 Cubical Agda",
    23: "Ch.23 Cubical Type Theory",
    24: "Ch.24 Simplicial Type Theory",
    25: "Ch.25 Modal HoTT",
    26: "Ch.26 Research Frontiers",
}


def ask(prompt: str) -> str:
    try:
        return input(prompt).strip()
    except (EOFError, KeyboardInterrupt):
        print()
        sys.exit(0)


def choose_scope() -> list[Question]:
    print()
    hr()
    section("What would you like to be quizzed on?")
    print("  [a] All chapters (full curriculum)")
    print("  [p] A specific phase")
    print("  [c] A specific chapter")
    print("  [t] A tag / topic")
    hr()
    choice = ask("  → ").lower()

    if choice == "a":
        return list(Q)

    if choice == "p":
        print()
        for k, v in PHASE_NAMES.items():
            print(f"  [{k}] {v}  ({sum(1 for q in Q if q.phase==k)} questions)")
        ph = ask("\n  Phase number → ")
        try:
            ph = int(ph)
            pool = [q for q in Q if q.phase == ph]
            if not pool:
                print(red("No questions for that phase."))
                return choose_scope()
            return pool
        except ValueError:
            print(red("Please enter a number."))
            return choose_scope()

    if choice == "c":
        print()
        for k, v in CH_NAMES.items():
            n = sum(1 for q in Q if q.chapter == k)
            if n:
                print(f"  [{k:2d}] {v}  ({n} questions)")
        ch = ask("\n  Chapter number → ")
        try:
            ch = int(ch)
            pool = [q for q in Q if q.chapter == ch]
            if not pool:
                print(red("No questions for that chapter."))
                return choose_scope()
            return pool
        except ValueError:
            print(red("Please enter a number."))
            return choose_scope()

    if choice == "t":
        all_tags = sorted({tag for q in Q for tag in q.tags})
        print()
        for i, tag in enumerate(all_tags):
            n = sum(1 for q in Q if tag in q.tags)
            print(f"  [{i:2d}] {tag}  ({n} questions)")
        idx = ask("\n  Tag index → ")
        try:
            tag = all_tags[int(idx)]
            return [q for q in Q if tag in q.tags]
        except (ValueError, IndexError):
            print(red("Invalid selection."))
            return choose_scope()

    print(yellow("Defaulting to all chapters."))
    return list(Q)


def ask_how_many(pool: list[Question]) -> int:
    n = len(pool)
    print()
    hr()
    ans = ask(f"  How many questions? (1–{n}, or press Enter for 10) → ")
    if not ans:
        return min(10, n)
    try:
        k = int(ans)
        return max(1, min(k, n))
    except ValueError:
        return min(10, n)


def present_mc(q: Question, num: int, total: int) -> bool:
    print()
    hr()
    tag_str = dim(f"  {CH_NAMES.get(q.chapter, '')}  |  multiple choice")
    print(tag_str)
    print(f"\n  {bold(f'Q{num}/{total}.')} {wrap(q.text)}\n")
    labels = "ABCD"
    for i, choice in enumerate(q.choices):
        print(f"    {bold(labels[i])}. {choice}")
    print()
    while True:
        raw = ask("  Your answer (A/B/C/D) → ").upper()
        if raw in labels[:len(q.choices)]:
            break
        print(yellow("  Please enter A, B, C, or D."))
    user_idx = labels.index(raw)
    correct = (user_idx == q.answer)
    _show_result(correct, labels[q.answer], q.choices[q.answer], q.explanation)
    return correct


def present_tf(q: Question, num: int, total: int) -> bool:
    print()
    hr()
    print(dim(f"  {CH_NAMES.get(q.chapter, '')}  |  true / false"))
    print(f"\n  {bold(f'Q{num}/{total}.')} {wrap(q.text)}\n")
    print("    A. True")
    print("    B. False")
    print()
    while True:
        raw = ask("  Your answer (A/B or T/F) → ").upper()
        if raw in ("A", "T", "TRUE"):
            user_idx = 0; break
        if raw in ("B", "F", "FALSE"):
            user_idx = 1; break
        print(yellow("  Please enter A (True) or B (False)."))
    correct = (user_idx == q.answer)
    _show_result(correct, "A" if q.answer == 0 else "B",
                 q.choices[q.answer], q.explanation)
    return correct


def present_blank(q: Question, num: int, total: int) -> bool:
    print()
    hr()
    print(dim(f"  {CH_NAMES.get(q.chapter, '')}  |  fill in the blank"))
    print(f"\n  {bold(f'Q{num}/{total}.')} {wrap(q.text)}\n")
    raw = ask("  Your answer → ").strip().lower()
    acceptable = [s.lower() for s in q.choices]
    correct = any(raw == a or raw in a or a in raw for a in acceptable)
    if not correct and raw:
        correct = any(raw.replace(" ", "") == a.replace(" ", "") for a in acceptable)
    correct_str = q.choices[0]
    _show_result(correct, correct_str, correct_str, q.explanation)
    return correct


def _show_result(correct: bool, label: str, text: str, explanation: str):
    print()
    if correct:
        print(f"  {green(bold('✓  Correct!'))}")
    else:
        print(f"  {red(bold('✗  Incorrect.'))}  The answer was: {bold(label)}")
    print()
    print(dim("  Explanation:"))
    for line in textwrap.wrap(explanation, width=WIDTH - 4):
        print(f"    {line}")
    ask(dim("\n  [Press Enter to continue] "))


def run_quiz(pool: list[Question], n: int):
    questions = random.sample(pool, n)
    score = 0
    wrong = []

    for i, q in enumerate(questions, 1):
        if q.kind == "mc":
            ok = present_mc(q, i, n)
        elif q.kind == "tf":
            ok = present_tf(q, i, n)
        else:
            ok = present_blank(q, i, n)
        if ok:
            score += 1
        else:
            wrong.append(q)

    print()
    hr("═")
    pct = 100 * score // n
    colour = green if pct >= 70 else yellow if pct >= 50 else red
    print(f"\n  {bold('Final score:')} {colour(bold(f'{score}/{n}'))}  ({pct}%)\n")

    if pct == 100:
        print(green("  Perfect score! You know this material cold."))
    elif pct >= 70:
        print(yellow("  Good work. Review the topics you missed."))
    else:
        print(red("  Keep studying — re-read the relevant chapters and try again."))

    if wrong:
        print(f"\n  {bold('Questions to review:')}")
        for q in wrong:
            print(f"    • {CH_NAMES.get(q.chapter, f'Ch.{q.chapter}')}: "
                  f"{dim(q.text[:60] + ('...' if len(q.text)>60 else ''))}")

    hr("═")
    return pct >= 50


def main():
    print()
    hr("═")
    print(f"""
  {bold(magenta('Homotopy Type Theory — Quiz'))}

  {dim('Covers all 27 chapters: from logic and set theory through')}
  {dim('HoTT, cubical type theory, and research frontiers.')}
  {dim(f'Loaded {len(Q)} questions from the question bank.')}
    """)
    hr("═")

    while True:
        pool = choose_scope()
        n = ask_how_many(pool)
        run_quiz(pool, n)

        print()
        again = ask("  Play again? (y/n) → ").lower()
        if again not in ("y", "yes"):
            print(f"\n  {dim('Good luck with your studies.')}\n")
            break


if __name__ == "__main__":
    main()
