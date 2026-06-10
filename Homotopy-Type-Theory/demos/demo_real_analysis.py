#!/usr/bin/env python3
"""
Real Analysis: Metric Spaces, Convergence, and Reals in HoTT
=============================================================
The classical foundations of continuity and limits — and how the
real line is constructed and used in homotopy type theory.
"""

import textwrap
import math

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


def _section_metric_spaces():
    clear()
    print(box("Metric Spaces"))
    print()
    print(wrap(
        "A METRIC SPACE is a set X with a distance function d:X×X→ℝ≥0 "
        "that measures how far apart points are. The axioms capture what "
        "we expect from any reasonable notion of distance."
    ))
    print()
    print(f"  {cyan('Metric axioms for d:X×X→ℝ≥0:')}")
    print(f"  {cyan('  (M1) d(x,y) = 0 ↔ x = y          (identity of indiscernibles)')}")
    print(f"  {cyan('  (M2) d(x,y) = d(y,x)              (symmetry)')}")
    print(f"  {cyan('  (M3) d(x,z) ≤ d(x,y) + d(y,z)    (triangle inequality)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Examples of metric spaces:'))}\n")

    examples = [
        ("ℝ with |x-y|",         "the standard real line"),
        ("ℝⁿ with Euclidean",    "||x-y|| = sqrt(Σᵢ(xᵢ-yᵢ)²)"),
        ("ℝⁿ with sup-norm",     "max |xᵢ-yᵢ| — all norms equivalent on ℝⁿ"),
        ("C([0,1]) with sup",    "continuous functions — infinite-dimensional"),
        ("Discrete metric",      "d(x,y) = 0 if x=y, 1 otherwise"),
        ("Graph distance",       "shortest path length in a graph"),
        ("Hamming distance",     "bits differing in two binary strings"),
        ("p-adic metric",        "|x-y|_p — ultrametric, non-Archimedean"),
    ]
    for name, note in examples:
        print(f"  {bold(cyan(name)):30} {dim(note)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Live demo: distances in ℝ²'))}\n")
    points = [(0,0), (1,0), (0,1), (3,4), (-1,2)]
    labels = ['O', 'A', 'B', 'C', 'D']
    p = points[0]; q = points[3]
    d = math.sqrt((p[0]-q[0])**2 + (p[1]-q[1])**2)
    print(f"  Points: {dict(zip(labels, points))}")
    print()
    print(f"  {'Pair':12}  {'Euclidean':12}  {'Manhattan':12}  {'Sup-norm'}")
    print(f"  {dim('─'*55)}")
    for i in range(len(points)):
        for j in range(i+1, len(points)):
            x1,y1 = points[i]; x2,y2 = points[j]
            euclid = math.sqrt((x1-x2)**2 + (y1-y2)**2)
            manhat = abs(x1-x2) + abs(y1-y2)
            sup    = max(abs(x1-x2), abs(y1-y2))
            pair   = f"{labels[i]}{labels[j]}"
            print(f"  {pair:12}  {euclid:12.3f}  {manhat:12.1f}  {sup:.1f}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_convergence():
    clear()
    print(box("Sequences and Convergence"))
    print()
    print(wrap(
        "A sequence in X is a function ℕ→X. A sequence (xₙ) CONVERGES to "
        "a limit L if for every ε>0 there is N such that for all n>N, "
        "d(xₙ,L) < ε. This is the epsilon-N definition."
    ))
    print()
    print(f"  {cyan('xₙ → L  :≡  ∀ε>0. ∃N. ∀n≥N. d(xₙ,L) < ε')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Live demo: sequences and limits:'))}\n")

    sequences = [
        ("1/n",       lambda n: 1/n,          "→ 0"),
        ("(n+1)/n",   lambda n: (n+1)/n,      "→ 1"),
        ("(-1)^n/n",  lambda n: (-1)**n / n,  "→ 0 (oscillating, damped)"),
        ("(1+1/n)^n", lambda n: (1+1/n)**n,   "→ e ≈ 2.71828"),
    ]

    for name, seq, limit in sequences:
        vals = [seq(n) for n in [1, 2, 5, 10, 50, 100]]
        vs = "  ".join(f"{v:8.5f}" for v in vals)
        print(f"  {bold(cyan(name)):18} {limit}")
        print(f"  {dim('n=1,2,5,10,50,100:')}  {dim(vs)}")
        print()

    print(rule())
    print(f"\n  {bold(green('Cauchy sequences:'))}\n")
    print(f"  {cyan('(xₙ) Cauchy :≡ ∀ε>0. ∃N. ∀m,n≥N. d(xₙ,xₘ) < ε')}")
    print()
    print(wrap(
        "A Cauchy sequence is one where the terms get arbitrarily close to "
        "EACH OTHER, without reference to a limit. In a COMPLETE metric space, "
        "every Cauchy sequence converges. ℝ is complete; ℚ is not (Cauchy "
        "sequences in ℚ can converge to irrationals)."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_continuity():
    clear()
    print(box("Continuity"))
    print()
    print(wrap(
        "A function f:X→Y between metric spaces is CONTINUOUS at x if "
        "nearby inputs map to nearby outputs. This is the epsilon-delta "
        "definition generalizing to metric spaces."
    ))
    print()
    print(f"  {cyan('f continuous at x :≡ ∀ε>0. ∃δ>0. d(x,y)<δ → d(f(x),f(y))<ε')}")
    print(f"  {cyan('f continuous :≡ ∀x. f continuous at x')}")
    print()
    print(f"  {cyan('Equivalently: xₙ → x → f(xₙ) → f(x)  (sequential continuity)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Characterizations of continuity:'))}\n")
    chars = [
        ("Epsilon-delta", "∀ε>0. ∃δ>0. d(x,y)<δ → d(f(x),f(y))<ε"),
        ("Sequential",    "xₙ→x implies f(xₙ)→f(x)"),
        ("Open sets",     "f⁻¹(U) open for every open U  (topological definition)"),
        ("Closed sets",   "f⁻¹(C) closed for every closed C"),
        ("Nets",          "directed-net convergence (for non-metrizable spaces)"),
    ]
    for name, def_str in chars:
        print(f"  {bold(cyan(name)):20} {dim(def_str)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Live demo: continuous vs. discontinuous'))}\n")

    def step(x):
        return 1.0 if x >= 0 else 0.0

    def smooth(x):
        return x**3

    xs = [-0.1, -0.01, -0.001, 0.0, 0.001, 0.01, 0.1]
    print(f"  {'x':10}  {'step(x)':12}  {'x³':12}")
    print(f"  {dim('─'*38)}")
    for x in xs:
        s = step(x)
        c = smooth(x)
        jump = red("←jump") if x == 0.0 else ""
        print(f"  {x:10.3f}  {s:12.3f}  {c:12.6f}  {jump}")
    print()
    print(wrap(
        "The step function jumps at 0: limits from left and right differ. "
        "The function x³ is continuous everywhere — and uniformly continuous on "
        "every bounded interval."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_compactness():
    clear()
    print(box("Compactness"))
    print()
    print(wrap(
        "COMPACTNESS is one of the most useful properties in analysis. "
        "It generalizes 'closed and bounded' from ℝⁿ to abstract spaces, "
        "and it guarantees that continuous functions achieve their extrema."
    ))
    print()
    print(f"  {cyan('X compact :≡ every open cover has a finite subcover')}")
    print(f"  {cyan('  (equivalently: every sequence has a convergent subsequence)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key theorems involving compactness:'))}\n")
    theorems = [
        ("Heine-Cantor",
         "f:X→Y continuous, X compact → f uniformly continuous"),
        ("Extreme value",
         "f:X→ℝ continuous, X compact → f attains max and min"),
        ("Heine-Borel (ℝⁿ)",
         "A ⊆ ℝⁿ compact ↔ A closed and bounded"),
        ("Tychonoff",
         "Product of compact spaces is compact (needs AC in general)"),
        ("Sequential compactness",
         "X metric: compact ↔ every sequence has convergent subsequence"),
        ("Finite covers",
         "Any open cover of [a,b] has a finite subcover"),
    ]
    for name, statement in theorems:
        print(f"  {bold(cyan(name))}")
        print(f"    {dim(statement)}")
        print()
    print(rule())
    print(f"\n  {bold(yellow('Compact vs. not compact:'))}\n")
    examples = [
        (green("Compact"),     "[0,1] ⊂ ℝ",          "closed and bounded"),
        (green("Compact"),     "Sⁿ",                  "spheres are compact"),
        (green("Compact"),     "finite discrete sets", "any finite space"),
        (red("Not compact"),   "(0,1) ⊂ ℝ",           "open — no finite subcover of {(1/n, 1)}"),
        (red("Not compact"),   "ℝ itself",             "unbounded"),
        (red("Not compact"),   "ℓ²",                   "closed unit ball in infinite dim. not compact"),
    ]
    for status, space, reason in examples:
        print(f"  {status:20} {bold(space):25} {dim(reason)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_reals_hott():
    clear()
    print(box("The Real Numbers in HoTT"))
    print()
    print(wrap(
        "Constructing ℝ in HoTT is subtle. There are several approaches, "
        "and they are NOT all equivalent without extra axioms. The key "
        "question: what does it mean for a real number to EXIST constructively?"
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Two constructions of ℝ:'))}\n")
    print(f"  {bold(cyan('1. Cauchy reals (ℝ_C):'))}")
    print(f"     {dim('Equivalence classes of Cauchy sequences in ℚ')}")
    print(f"     {dim('Two sequences equivalent iff they converge to the same limit')}")
    print(f"     {dim('Constructively well-behaved; arithmetic is computable')}")
    print(f"     {dim('ℝ_C = Σ(s:ℕ→ℚ). isCauchy(s) / ~')}")
    print()
    print(f"  {bold(cyan('2. Dedekind reals (ℝ_D):'))}")
    print(f"     {dim('A Dedekind cut is a pair (L,U) of subsets of ℚ')}")
    print(f"     {dim('splitting ℚ into rationals below and above a real')}")
    print(f"     {dim('ℝ_D = {(L,U) : 𝒫(ℚ)×𝒫(ℚ) | DedekindCutAxioms(L,U)}')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Comparison:'))}\n")
    print(f"  {'Property':35}  {'Cauchy':15}  {'Dedekind'}")
    print(f"  {dim('─'*65)}")
    comparisons = [
        ("Quotient required",          "yes (~ on sequences)", "no"),
        ("Constructively complete",    "yes",                  "yes"),
        ("Isomorphic constructively",  "not always",           "—"),
        ("In HoTT with LEM",          "≃ ℝ_D",               "standard"),
        ("Propositional resizing",     "needed for ℝ_C≃ℝ_D",  "—"),
        ("Computation",               "very computable",      "less so"),
    ]
    for prop, cauchy, dedekind in comparisons:
        print(f"  {prop:35}  {dim(cauchy):23}  {cyan(dedekind)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The interval [0,1] in HoTT:'))}\n")
    print(wrap(
        "The unit interval [0,1] is crucial for homotopy theory. In HoTT, "
        "paths are NOT modeled as functions [0,1]→X (that would be circular). "
        "Instead, paths are PRIMITIVE — given by identity types. The synthetic "
        "approach avoids needing [0,1] as a metric space."
    ))
    print()
    print(wrap(
        "In CUBICAL HoTT, the interval is a primitive 𝕀 with two endpoints "
        "i0,i1:𝕀 and a connection algebra. This gives computation for paths "
        "without the analytic definition of [0,1]."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("metric",    "Metric spaces",                         _section_metric_spaces),
    ("conv",      "Sequences and convergence",             _section_convergence),
    ("cont",      "Continuity",                            _section_continuity),
    ("compact",   "Compactness",                           _section_compactness),
    ("reals",     "The real numbers in HoTT",              _section_reals_hott),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Real Analysis: Metric Spaces and Continuity", width=70))
        print()
        for i, (_, title, _fn) in enumerate(SECTIONS):
            marker = bold(cyan("▶")) if i == idx else " "
            print(f"  {marker} {bold(str(i+1))}   {title}")
        print()
        print(rule())
        print(f"  {dim('1-5  jump   n  next   p  prev   q  quit')}")
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
