#!/usr/bin/env python3
"""
Truncations: Squashing Higher Structure
========================================
Propositional truncation ||A||, set-truncation ||A||₀, and n-truncations.

Truncation answers: "what if I want A but don't care about *which* element?"
The propositional truncation ||A|| remembers only WHETHER A is inhabited,
not the specific witness. This is the type-theoretic version of ∃.
"""

import textwrap, os

def _c(code, t): return f"\033[{code}m{t}\033[0m"
bold    = lambda t: _c("1",    t)
green   = lambda t: _c("32",   t)
yellow  = lambda t: _c("33",   t)
cyan    = lambda t: _c("36",   t)
red     = lambda t: _c("31",   t)
dim     = lambda t: _c("2",    t)
magenta = lambda t: _c("35",   t)

def clear(): print("\033[2J\033[H", end="")

def wrap(text, width=70, indent="  "):
    lines = []
    for para in text.strip().split("\n"):
        if para.strip() == "":
            lines.append("")
        else:
            lines.extend(textwrap.wrap(para, width, initial_indent=indent,
                                       subsequent_indent=indent))
    return "\n".join(lines)

def box(title, width=68):
    inner = width - 2
    return (f"  ╔{'═'*inner}╗\n"
            f"  ║  {bold(title):<{inner-2}}║\n"
            f"  ╚{'═'*inner}╝")

def rule(width=70): return "  " + dim("─"*width)


# ── Section 1: h-levels and the truncation ladder ───────────────────────────

def _section_intro():
    clear()
    print(box("Truncations: Squashing Higher Homotopy Structure"))
    print()
    print(wrap(
        "Every type in HoTT has a HOMOTOPY LEVEL (h-level): a measure of the "
        "highest dimension of non-trivial path structure it contains. "
        "Truncation is the operation that FORCES a type down to a given h-level "
        "by identifying all paths above that level."
    ))
    print()
    print(rule())
    print(f"\n  {bold('The h-level ladder:')}\n")

    levels = [
        ("-2", "Contractible",      "||1|| = 1. Exactly one element. All paths trivial.",         "𝟙, Σ(x:A). a=x"),
        ("-1", "Propositions",      "||0|| or ||1||. At most one element. All proofs equal.",    "⊤, ⊥, a=b (in sets)"),
        ("0",  "Sets",              "No higher path structure beyond propositional equality.",    "ℕ, Bool, ℤ"),
        ("1",  "Groupoids",         "Paths form sets; no interesting 2-paths.",                  "Groups, categories"),
        ("2",  "2-groupoids",       "2-paths form sets; no interesting 3-paths.",                "2-categories"),
        ("n",  "n-groupoids",       "The general case.",                                          "..."),
        ("∞",  "∞-groupoids",       "Untruncated types. All higher structure present.",          "S¹, S², all HITs"),
    ]

    for lvl, name, desc, ex in levels:
        if lvl == "∞":
            col = magenta
        elif lvl in ("-2", "-1", "0"):
            col = green
        else:
            col = yellow
        print(f"  {bold(col(f'h-level {lvl:>2}'))}  {bold(name):<18}  {dim(ex)}")
        print(wrap(desc, width=65, indent="                    "))
        print()

    print(rule())
    print(wrap(
        "The n-TRUNCATION of A, written ||A||ₙ (or trunc n A), is a universal "
        "way to make A into an n-type. Propositional truncation is ||A||₋₁, "
        "set-truncation is ||A||₀. We'll explore both in this demo."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 2: Propositional truncation ──────────────────────────────────────

def _section_prop_trunc():
    clear()
    print(box("Propositional Truncation: ||A||"))
    print()
    print(wrap(
        "The propositional truncation ||A|| (also written ‖A‖ or ∥A∥) is a "
        "HIT with one point constructor and one path constructor:"
    ))
    print()
    print(f"  {cyan('data ||A|| where')}")
    print(f"  {cyan('  |_| : A → ||A||           -- wrap any element of A')}")
    print(f"  {cyan('  squash : ∀(x y : ||A||). x = y  -- all elements are equal')}")
    print()
    print(wrap(
        "The squash constructor says: any two elements of ||A|| are equal. "
        "This makes ||A|| a PROPOSITION (h-level -1): at most one element "
        "up to propositional equality."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('What ||A|| remembers:'))}")
    print()

    rows = [
        ("A = 𝟘 (Empty)",   "||𝟘|| = 𝟘",  "No elements → ||A|| has no elements → False"),
        ("A = 𝟙 (Unit)",    "||𝟙|| = 𝟙",  "One element → ||A|| has one element → True"),
        ("A = Bool",        "||Bool|| = 𝟙", "Two elements → squashed to one → True (inhabited)"),
        ("A = ℕ",           "||ℕ|| = 𝟙",  "Infinitely many → squashed to one → True"),
        ("A = 𝟘 + 𝟙",      "||𝟘+𝟙|| = 𝟙","Sum type, nonEmpty → True"),
        ("A = S¹",         "||S¹|| = 𝟙",  "Circle → squashed (connected space) → True"),
    ]

    for a, trunc, note in rows:
        print(f"  {bold(cyan(a)):<30} →  {bold(yellow(trunc))}")
        print(f"  {dim(note)}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('The recursion principle:'))}")
    print()
    print(wrap(
        "You can define a function ||A|| → B ONLY IF B is a proposition. "
        "Otherwise you would be able to extract a specific element of A from "
        "the truncation, violating the 'forgetfulness'."
    ))
    print()
    print(f"  {cyan('rec||_|| : isProp B → (A → B) → ||A|| → B')}")
    print()
    print(wrap(
        "If B is a mere proposition, all paths in B are trivial, so the "
        "squash constructor is automatically satisfied. This is why you "
        "CAN eliminate into other propositions but NOT into sets or higher types."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 3: ∃ vs Σ — the crucial distinction ──────────────────────────────

def _section_existence():
    clear()
    print(box("∃ vs. Σ: Mere Existence vs. Constructive Witness"))
    print()
    print(wrap(
        "This is one of the most philosophically important distinctions in HoTT. "
        "In constructive mathematics there are TWO notions of existence:"
    ))
    print()

    print(f"  {bold(green('Σ-type (constructive existence):'))}")
    print(f"  {cyan('  Σ(x:A). P(x)')}")
    print(f"  {'':4}A pair (a, p) where a : A and p : P(a).")
    print(f"  {'':4}{dim('The SPECIFIC WITNESS a is part of the data.')}")
    print()
    print(f"  {bold(yellow('∃ (mere existence / propositional truncation):'))}")
    print(f"  {cyan('  ∃(x:A). P(x)  :≡  ||Σ(x:A). P(x)||')}")
    print(f"  {'':4}Merely: some a with P(a) exists.")
    print(f"  {'':4}{dim('The witness is HIDDEN — you cannot extract it.')}")
    print()
    print(rule())
    print(f"\n  {bold('Concrete example: does there exist an even prime?')}\n")

    print(f"  {bold(green('Σ-version (gives you the number):'))}")
    print(f"  {cyan('  proof : Σ(n:ℕ). isEven n × isPrime n')}")
    print(f"  {cyan('  proof = (2, refl, isPrime-2)')}")
    print(f"  {'':4}You CAN extract: fst proof = 2")
    print()
    print(f"  {bold(yellow('∃-version (only tells you one exists):'))}")
    print(f"  {cyan('  proof : ∃(n:ℕ). isEven n × isPrime n')}")
    print(f"  {cyan('  proof = |( 2, refl, isPrime-2 )|')}")
    print(f"  {'':4}You CANNOT extract: there is no function ∃(n:ℕ). P n → ℕ")
    print(f"  {'':4}{dim('(unless ℕ is a proposition, which it is not)')}")
    print()
    print(rule())
    print(f"\n  {bold('Why does this matter?')}\n")
    print(wrap(
        "Consider: 'There exists a natural number satisfying P.' "
        "In classical math, this means you could IN PRINCIPLE find it. "
        "In constructive math, Σ means you have an ALGORITHM that produces it; "
        "∃ means only that the existence is CONSISTENT — you might not be able "
        "to compute the witness. The truncation hides computational content."
    ))
    print()
    print(f"  {bold(green('Concrete consequence:'))}")
    print()
    print(f"  {cyan('  decidable : ∀(n:ℕ). isPrime n ∨ ¬isPrime n')}")
    print(f"  {'':4}This is  {dim('∀n. ||isPrime n + ¬isPrime n||')}")
    print(f"  {'':4}It does NOT give you a function ℕ → Bool that computes primality.")
    print(f"  {'':4}{dim('(That requires Σ, not ∃, and an actual decision procedure)')}")
    print()
    print(rule())
    print(f"\n  {bold('The axiom of choice in HoTT:')}\n")
    print(wrap(
        "Classical AC: ∀(x:A). ∃(y:B). R(x,y) → ∃(f:A→B). ∀(x:A). R(x, f(x))."
    ))
    print(wrap(
        "In HoTT, the 'trivial' choice Π-Σ (without truncation) is provable: "
        "∀(x:A). Σ(y:B). R(x,y) → Σ(f:A→B). ∀(x:A). R(x, f(x))."
    ))
    print(wrap(
        "But with truncation, the statement involves extracting a FUNCTION from "
        "a truncated type, which is generally not possible — this is exactly the "
        "content of the classical vs. constructive debate about choice."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 4: Set-truncation ─────────────────────────────────────────────────

def _section_set_trunc():
    clear()
    print(box("Set-Truncation: ||A||₀"))
    print()
    print(wrap(
        "The set-truncation ||A||₀ makes A into a set (h-level 0) by "
        "identifying all paths between any two elements, but NOT identifying "
        "distinct elements. It is a HIT:"
    ))
    print()
    print(f"  {cyan('data ||A||₀ where')}")
    print(f"  {cyan('  |_|₀ : A → ||A||₀')}")
    print(f"  {cyan('  squash₀ : ∀(x y : ||A||₀). ∀(p q : x=y). p = q')}")
    print()
    print(wrap(
        "The squash₀ constructor collapses 2-paths: any two proofs of x=y are "
        "identified. This leaves at most one path between any two points — "
        "making ||A||₀ a set."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Key examples:'))}\n")

    examples = [
        ("||S¹||₀",  "𝟙",
         "The circle has one connected component. All paths between base and "
         "base (which form ℤ) get squashed — only the truth 'base=base' survives."),
        ("||S²||₀",  "𝟙",
         "Same: S² is connected. The 2-sphere's surface gets squashed."),
        ("||Bool||₀", "Bool",
         "Bool is already a set! ||Bool||₀ = Bool."),
        ("||ℕ||₀",   "ℕ",
         "ℕ is already a set. No higher structure to squash."),
        ("||ℤ||₀",   "ℤ",
         "ℤ is a set. Set-truncation is idempotent on sets."),
        ("||A+B||₀", "||A||₀ + ||B||₀",
         "Set-truncation distributes over sums (up to equivalence)."),
    ]

    for term, result, note in examples:
        print(f"  {bold(cyan(term)):<22} ≃  {bold(yellow(result))}")
        print(wrap(note, width=65, indent="      "))
        print()

    print(rule())
    print(f"\n  {bold(yellow('The recursion principle for set-truncation:'))}\n")
    print(f"  {cyan('rec||_||₀ : isSet B → (A → B) → ||A||₀ → B')}")
    print()
    print(wrap(
        "You can eliminate from ||A||₀ into any SET B. The squash₀ constructor "
        "is automatically satisfied because B is a set (all its path spaces are "
        "propositions, so 2-paths are automatically trivial)."
    ))
    print()
    print(wrap(
        "In practice, this means: to define a function from ||A||₀ to a set, "
        "you only need to give its behavior on elements of A (not on paths), "
        "because all path information has been squashed away."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 5: General n-truncation ──────────────────────────────────────────

def _section_n_trunc():
    clear()
    print(box("General n-Truncation: ||A||ₙ"))
    print()
    print(wrap(
        "The n-truncation ||A||ₙ is defined for any n ≥ -2. It is the "
        "universal map from A to an n-type: the 'best approximation' of A "
        "that has no homotopy above dimension n."
    ))
    print()
    print(f"  {cyan('data ||A||ₙ where')}")
    print(f"  {cyan('  |_|ₙ : A → ||A||ₙ')}")
    print(f"  {cyan('  hub  : (Sⁿ → ||A||ₙ) → ||A||ₙ')}")
    print(f"  {cyan('  spoke: ∀(r : Sⁿ → ||A||ₙ)(x : Sⁿ). hub r = r x')}")
    print()
    print(wrap(
        "Alternatively (Lumsdaine-Shulman): the n-truncation is built by "
        "'coning off' all (n+1)-spheres. Every sphere Sⁿ⁺¹ → ||A||ₙ gets "
        "a filler — so no non-trivial (n+1)-paths can persist."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The truncation table:'))}\n")

    rows = [
        ("-2", "||A||₋₂", "Contractification",  "Collapses A to a point (if nonempty) or leaves empty"),
        ("-1", "||A||₋₁", "Propositional trunc", "Squashes all paths: at most one element up to equality"),
        ("0",  "||A||₀",  "Set-truncation",      "Squashes 2-paths: sets have UIP (unique identity proofs)"),
        ("1",  "||A||₁",  "Groupoid-truncation", "Squashes 3-paths: 1-groupoids, categories"),
        ("n",  "||A||ₙ",  "n-truncation",        "Squashes (n+2)-paths"),
    ]

    for n, term, name, desc in rows:
        print(f"  {bold(cyan(term)):<22} {bold(name)}")
        print(wrap(desc, width=65, indent="      "))
        print()

    print(rule())
    print(f"\n  {bold(yellow('The truncation-reflection adjunction:'))}\n")
    print(wrap(
        "There is an adjunction between the inclusion of n-types into all "
        "types and the n-truncation functor:"
    ))
    print()
    print(f"  {cyan('(||A||ₙ → B) ≃ (A → B)   (when B is an n-type)')}")
    print()
    print(wrap(
        "This says: maps FROM ||A||ₙ to an n-type B are the same as maps "
        "from A to B. Truncation is the LEFT ADJOINT to the inclusion of n-types. "
        "This is the universal property that defines truncation uniquely."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Iterated truncation:'))}\n")
    print(f"  {cyan('|||| A ||ₙ ||ₘ ≃ ||A||_{min(n,m)}')}")
    print()
    print(wrap(
        "Truncating twice: the lower truncation wins. Truncating a proposition "
        "further (to set-level or contractible) doesn't change it. "
        "Propositional truncation is idempotent: |||| A ||₋₁ ||₋₁ ≃ ||A||₋₁."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 6: When can you extract from a truncation? ───────────────────────

def _section_extraction():
    clear()
    print(box("Extraction: When Can You Escape the Truncation?"))
    print()
    print(wrap(
        "Once you put something into ||A||, you generally cannot get it back. "
        "But there are important cases where you CAN escape:"
    ))
    print()
    print(rule())

    cases = [
        ("Into a proposition",
         "rec : isProp B → (A → B) → ||A|| → B",
         "Always works. If B has at most one element, the squash is trivially satisfied.",
         green),
        ("Into a set (from ||A||₀)",
         "rec₀ : isSet B → (A → B) → ||A||₀ → B",
         "Works for set-truncation. The target must be a set.",
         green),
        ("Using double-negation (classical logic)",
         "DNE : ||A|| → ¬¬A   and classically  ¬¬A → A",
         "In classical logic, ||A|| ↔ ¬¬A. Under LEM you can extract. "
         "But LEM is not provable in HoTT — it's an additional axiom.",
         yellow),
        ("A decidable proposition",
         "decide : (A + ¬A) → ||A|| → A",
         "If A is decidable (A + ¬A holds, giving an actual element or refutation), "
         "then from ||A|| you can extract: since ¬A is impossible (A is inhabited), "
         "you must be in the A case.",
         green),
        ("The axiom of choice",
         "AC : (∀x. ||F x||) → ||∀x. F x||",
         "This is the 'choice' that is NOT automatically available in HoTT. "
         "It requires an explicit axiom. Without it, you cannot in general "
         "collect pointwise truncated proofs into a global function.",
         red),
    ]

    for title, form, note, col in cases:
        print(f"  {bold(col(title))}")
        print(f"  {cyan(form)}")
        print(wrap(note, width=66, indent="    "))
        print()

    print(rule())
    print(f"\n  {bold(yellow('The key insight:'))}\n")
    print(wrap(
        "Truncation separates EXISTENCE from COMPUTATION. The truncated ∃ "
        "says 'a witness exists' without giving you one. This is essential for "
        "doing classical mathematics (where you use existence without construction) "
        "inside a constructive system. The truncation ||A|| is your 'classical "
        "existence' while Σ is your 'constructive existence'."
    ))
    print()
    print(wrap(
        "In formal verification, this means: if you want to verify a theorem "
        "classically, you can use ||A||. If you want to extract a certified "
        "algorithm, you need Σ. HoTT lets you be precise about which you mean."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("intro",      "The h-level ladder and what truncation does",      _section_intro),
    ("prop",       "Propositional truncation ||A||",                   _section_prop_trunc),
    ("existence",  "∃ vs. Σ: mere existence vs. constructive witness", _section_existence),
    ("set",        "Set-truncation ||A||₀",                            _section_set_trunc),
    ("general",    "General n-truncation ||A||ₙ",                      _section_n_trunc),
    ("extraction", "Extraction: when can you escape the truncation?",  _section_extraction),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Truncations: Squashing Higher Homotopy Structure", width=70))
        print()
        for i, (_, title, _fn) in enumerate(SECTIONS):
            marker = bold(cyan("▶")) if i == idx else " "
            print(f"  {marker} {bold(str(i+1))}   {title}")
        print()
        print(rule())
        print(f"  {dim('1-6  jump   n  next   p  prev   q  quit')}")
        print()
        try:
            ch = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break
        if ch in ("q", "quit", "exit"):
            break
        elif ch in ("n", ""):
            SECTIONS[idx][2]()
            idx = min(idx + 1, len(SECTIONS) - 1)
        elif ch == "p":
            idx = max(idx - 1, 0)
        else:
            try:
                v = int(ch) - 1
                if 0 <= v < len(SECTIONS):
                    idx = v
                    SECTIONS[idx][2]()
                    idx = min(idx + 1, len(SECTIONS) - 1)
            except ValueError:
                pass

if __name__ == "__main__":
    main()
