#!/usr/bin/env python3
"""
Research Frontiers in HoTT
============================
Open problems, the Brunerie number, formalization frontiers,
and active research directions in homotopy type theory.
"""

import textwrap

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


def _section_open_problems():
    clear()
    print(box("Open Problems in HoTT"))
    print()
    print(wrap(
        "HoTT is a young field with many fundamental open problems. "
        "These range from foundational questions about the theory itself "
        "to concrete computations in homotopy theory."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Major open problems:'))}\n")

    problems = [
        ("Coherence",
         "Define a model of HoTT where all coherence laws hold definitionally. "
         "Current models (simplicial sets) are 'semi-strict' — some laws hold "
         "propositionally only."),
        ("Two-level TT",
         "A type theory combining strict (definitional) and weak (propositional) "
         "equality. 2LTT (Voevodsky/Annenkov) is a proposed solution but "
         "full metatheory is not complete."),
        ("Higher ind. rec.",
         "Can all HITs be defined with a single HITS constructor? "
         "The problem of giving a uniform account of all HITs in a single "
         "extension of MLTT."),
        ("Universe polymorphism",
         "Full universe polymorphism with consistent level arithmetic "
         "is complex. When can we prove theorems 'for all universe levels'?"),
        ("Real cohomology",
         "Define real-valued cohomology H^n(X;ℝ) purely synthetically "
         "in HoTT without reference to the reals as an external set."),
        ("LEM in sheaf models",
         "Characterize exactly which models of HoTT satisfy LEM "
         "or other classical principles."),
        ("Higher Galois theory",
         "Formalize the full Galois correspondence for (∞,1)-categories "
         "in HoTT — extending the covering space result."),
    ]

    for name, desc in problems:
        print(f"  {bold(cyan(name))}")
        print(wrap(desc, indent="    "))
        print()

    input(bold("  Press Enter to continue... "))


def _section_brunerie():
    clear()
    print(box("The Brunerie Number: π₄(S³) = ℤ/2ℤ"))
    print()
    print(wrap(
        "Guillaume Brunerie proved in his 2016 PhD thesis that π₄(S³) = ℤ/nℤ "
        "for some n, with n given by an explicit construction in HoTT. "
        "He conjectured n=2 based on the classical answer but could not "
        "prove it within HoTT at the time."
    ))
    print()
    print(f"  {bold(cyan('The Brunerie number:'))}")
    print(f"  {cyan('n = |π₄(S³)| = 2  (classical result: Freudenthal + Hopf)')}")
    print(f"  {dim('  but n is defined by a complex HoTT term that must be computed')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The construction:'))}\n")
    print(wrap(
        "Brunerie defined n as the composite:"
    ))
    print()
    print(f"  {cyan('n := |join(S¹,S¹)_-1| evaluated at the Hopf invariant')}")
    print(f"  {dim('  where the Hopf invariant is the degree of the Hopf fibration')}")
    print()
    print(wrap(
        "The Brunerie number is the order of the image of the Hopf map "
        "η: S³ → S² in π₄(S³) via the long exact sequence. In HoTT this "
        "is a specific term whose TYPE says it represents an element of ℤ/nℤ."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Resolution (2023):'))}\n")
    print(wrap(
        "In 2022-2023, the Brunerie number was COMPUTED to be 2 using "
        "Cubical Agda. Brunerie, Ljungstrom, and Mortberg formally verified "
        "the computation. This was a landmark result in formalized mathematics:"
    ))
    print()
    milestones = [
        ("2016", "Brunerie proves π₄(S³)=ℤ/nℤ in HoTT; conjectures n=2"),
        ("2017", "Term for n extracted; naive computation does not terminate"),
        ("2018", "Simplifications by Brunerie reduce the term"),
        ("2022", "Cubical Agda computational reductions make it feasible"),
        ("2023", "Machine verification that n=2 completes"),
    ]
    for year, event in milestones:
        print(f"  {bold(yellow(year))}  {dim(event)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Why this matters:'))}\n")
    print(wrap(
        "The Brunerie number computation is a proof of concept: HoTT can "
        "COMPUTE non-trivial algebraic topology results. The difficulty "
        "showed what improvements to HoTT (cubical computation) were needed."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_formalization():
    clear()
    print(box("Formalization Frontiers"))
    print()
    print(wrap(
        "Formalizing mathematics in proof assistants is an active area. "
        "HoTT offers advantages for formalization: univalence means that "
        "isomorphic structures are literally equal."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Major formalization projects:'))}\n")
    projects = [
        ("UniMath",
         "Voevodsky, Ahrens, et al. — formalization of mathematics "
         "using univalent foundations in Coq. Covers algebra, category theory."),
        ("HoTT Coq library",
         "The HoTT Book formalization: many results from the book "
         "formalized in Coq with HoTT axioms."),
        ("Agda-HoTT (HoTT-Agda)",
         "Formalization in Agda using the Book HoTT axioms. "
         "Includes πₙ(Sⁿ), Eilenberg-MacLane spaces."),
        ("Cubical Agda library",
         "agda/cubical: most comprehensive HoTT library. "
         "Includes the Brunerie number, James construction, HITs."),
        ("1Lab",
         "Comprehensive Cubical Agda library with full category theory, "
         "including ∞-categories (Rezk types)."),
        ("Mathlib4",
         "Not HoTT but classical: most comprehensive Lean 4 library. "
         "Includes algebraic topology, differential geometry."),
        ("Synthetic spectra",
         "Ongoing: formalizing stable homotopy theory in HoTT."),
    ]
    for name, desc in projects:
        print(f"  {bold(cyan(name))}")
        print(wrap(desc, indent="    "))
        print()
    input(bold("  Press Enter to continue... "))


def _section_active_research():
    clear()
    print(box("Active Research Areas"))
    print()
    print(wrap(
        "As of 2025, the following are the most active research areas "
        "in and around HoTT."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Foundational:'))}\n")
    foundational = [
        ("XTT / Observational TT",
         "Altenkirch-Kaposi: a type theory with decidable equality "
         "where univalence holds by construction, not by axiom"),
        ("Multimodal TT",
         "MTT: a general framework for adding modalities to type theory, "
         "unifying cohesive, directed, and guarded type theories"),
        ("Parametricity in TT",
         "Internal parametricity: a way to prove free theorems inside "
         "the type theory itself"),
        ("Infinity-type theories",
         "Riehl-Verity: ∞-type theories as the internal language "
         "of ∞-cosmoses"),
    ]
    for name, desc in foundational:
        print(f"  {bold(yellow(name))}")
        print(wrap(desc, indent="    "))
        print()

    print(rule())
    print(f"\n  {bold(green('Synthetic homotopy theory:'))}\n")
    synthetic = [
        ("Higher Blakers-Massey", "Fully formalized in HoTT; stronger than classical"),
        ("Synthetic spectra",     "Internal spectra in HoTT — Floris van Doorn"),
        ("Synthetic Thom spectra","Classifying bundles internally"),
        ("Rational homotopy",     "Sullivan minimal models in HoTT"),
        ("π_n(S^n) = ℤ",         "Proved; higher πₙ(Sᵐ) computation ongoing"),
    ]
    for name, desc in synthetic:
        print(f"  {bold(cyan(name)):28} {dim(desc)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_connections():
    clear()
    print(box("HoTT's Connections to Other Fields"))
    print()
    print(wrap(
        "HoTT sits at the intersection of several major areas of "
        "mathematics and computer science. Its development has influenced "
        "and been influenced by all of these."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Connections to:'))}\n")
    connections = [
        ("Algebraic Topology",
         "HoTT is a SYNTHETIC algebraic topology: homotopy groups, fibrations, "
         "and the Blakers-Massey theorem are proved type-theoretically.",
         "→ π₁(S¹)=ℤ, πₙ(Sⁿ)=ℤ, Freudenthal suspension"),
        ("Category Theory",
         "Every type is an ∞-groupoid; the universe is an ∞-topos. "
         "HoTT provides internal language for Lurie-style ∞-category theory.",
         "→ Yoneda, limits, adjunctions internally"),
        ("Logic",
         "HoTT extends constructive type theory with new axioms. "
         "It provides models where LEM fails and models where it holds.",
         "→ Kripke models, BHK, proof-theoretic strength"),
        ("Computer Science",
         "HoTT gives a foundation for programming with proofs. "
         "Cubical Agda extracts verified programs; proof-relevant data.",
         "→ Verified compilers, certified algorithms"),
        ("Physics",
         "Cohesive HoTT (Schreiber) formalizes quantum gauge field theory. "
         "Modal HoTT captures the geometry of string theory and M-theory.",
         "→ Chern-Simons theory, anomaly cancellation"),
        ("Foundations",
         "HoTT is an alternative to ZFC. It handles mathematical practice "
         "more naturally: isomorphic = equal, quotients are first-class.",
         "→ Univalent foundations program (Voevodsky)"),
    ]
    for name, desc, examples in connections:
        print(f"  {bold(cyan(name))}")
        print(wrap(desc, indent="    "))
        print(f"    {dim(examples)}")
        print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("open",    "Open problems in HoTT",             _section_open_problems),
    ("brunerie","The Brunerie number: π₄(S³) = ℤ/2ℤ", _section_brunerie),
    ("formal",  "Formalization frontiers",            _section_formalization),
    ("active",  "Active research areas",              _section_active_research),
    ("connect", "Connections to other fields",        _section_connections),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Research Frontiers in HoTT", width=70))
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
