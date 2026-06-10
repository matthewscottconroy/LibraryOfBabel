#!/usr/bin/env python3
"""
Eilenberg-MacLane Spaces and Cohomology
=========================================
K(G, n): types with exactly one non-trivial homotopy group.

K(G, n) is the unique (up to equivalence) type with πₙ = G and all other
homotopy groups trivial. These spaces are the building blocks of cohomology
theory: Hⁿ(X; G) = ||X → K(G,n)||₀.
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


# ── Section 1: What is K(G,n)? ───────────────────────────────────────────────

def _section_definition():
    clear()
    print(box("Eilenberg-MacLane Spaces: K(G, n)"))
    print()
    print(wrap(
        "An Eilenberg-MacLane space K(G, n) is a type (space) with:"
    ))
    print()
    print(f"  {cyan('πₙ(K(G,n)) = G      -- exactly one non-trivial homotopy group')}")
    print(f"  {cyan('πₖ(K(G,n)) = 0      -- all other homotopy groups trivial (k ≠ n)')}")
    print()
    print(wrap(
        "Here G must be a GROUP for n=1, and an ABELIAN GROUP for n≥2 "
        "(since all higher homotopy groups are abelian by Eckmann-Hilton)."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The hierarchy of K(G,n) spaces:'))}\n")

    print(f"  {'n':6}  {'G':15}  {'K(G,n)':20}  {'Description'}")
    print(f"  {dim('─'*68)}")
    rows = [
        ("0", "ℤ/2ℤ", "𝟚 (Bool)",        "Two-point space, π₀=ℤ/2ℤ"),
        ("1", "ℤ",    "S¹",              "Circle: π₁(S¹) = ℤ"),
        ("1", "ℤ/2ℤ", "RP^∞",            "Infinite real projective space"),
        ("1", "ℤ/nℤ", "L^∞(n)  (lens)",  "Infinite lens space"),
        ("2", "ℤ",    "CP^∞",            "Infinite complex projective space"),
        ("2", "ℤ/2ℤ", "B(ℤ/2ℤ) ≃ RP^∞", "Twice delooped ℤ/2ℤ"),
        ("n", "ℤ",    "Kₙ(ℤ)",           "Constructed iteratively by delooping"),
        ("n", "G",    "K(G,n)",           "Universal example for nth cohomology"),
    ]
    for n, G, space, desc in rows:
        print(f"  {n:6}  {G:15}  {bold(cyan(space)):28}  {dim(desc)}")

    print()
    print(rule())
    print(f"\n  {bold(yellow('In HoTT:'))}\n")
    print(f"  {cyan('K(G, 0) = ||G||₋₁   (propositional truncation, if G is a set)')}")
    print(f"  {cyan('K(G, 1) = BG        (the delooping/classifying space of G)')}")
    print(f"  {cyan('K(G, n) = B^n G     (n-fold delooping of G)')}")
    print()
    print(wrap(
        "Delooping a group G gives a connected type BG with π₁(BG) = G. "
        "Iterating: K(G, n) = B^n G, delooping n times. This requires G to be "
        "abelian for n ≥ 2 (so that the group structure survives the delooping)."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 2: K(ℤ,1) = S¹ ───────────────────────────────────────────────────

def _section_circle():
    clear()
    print(box("K(ℤ, 1) = S¹: The Circle as a Classifying Space"))
    print()
    print(wrap(
        "The circle S¹ is an Eilenberg-MacLane space for n=1, G=ℤ:"
    ))
    print()
    print(f"  {cyan('π₁(S¹) = ℤ')}")
    print(f"  {cyan('πₙ(S¹) = 0  for n ≥ 2')}")
    print()
    print(wrap(
        "The vanishing of higher homotopy groups follows from the universal "
        "cover: S¹ is covered by ℝ (contractible), and the long exact sequence "
        "of the covering gives πₙ(S¹) = πₙ(ℝ) = 0 for n ≥ 2."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('S¹ as the classifying space BZ:'))}\n")
    print(wrap(
        "A principal ℤ-bundle over a base X is a covering space p : E → X "
        "where the fiber is ℤ and ℤ acts by deck transformations. Classifying "
        "such bundles means finding a space BZ such that:"
    ))
    print()
    print(f"  {cyan('Prin_ℤ(X) ≃ [X, S¹]  (homotopy classes of maps X → S¹)')}")
    print()
    print(wrap(
        "This is the universal property of K(ℤ,1) = S¹. Every ℤ-bundle over X "
        "corresponds to a map X → S¹ (up to homotopy), which assigns to each "
        "loop in X its 'winding number' around S¹."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('The universal cover tower:'))}\n")
    print(f"  {'ℝ':>12}  (contractible, universal cover of S¹)")
    print(f"  {'│':>12}")
    print(f"  {'│ ℤ action':>12}  (deck transformations: n · x = x + n)")
    print(f"  {'↓':>12}")
    print(f"  {'S¹ = ℝ/ℤ':>12}  (quotient by ℤ action)")
    print()
    print(wrap(
        "The fiber sequence is: ℤ → ℝ → S¹. In homotopy groups:"
    ))
    print()
    print(f"  {cyan('⋯ → πₙ(ℤ) → πₙ(ℝ) → πₙ(S¹) → πₙ₋₁(ℤ) → ⋯')}")
    print(f"  {cyan('⋯ →   0   →   0   → πₙ(S¹) →    0    → ⋯')}")
    print(f"  {cyan('⟹ πₙ(S¹) = 0  for n ≥ 2')}")
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 3: K(ℤ/2ℤ, 1) = RP^∞ ────────────────────────────────────────────

def _section_rp_inf():
    clear()
    print(box("K(ℤ/2ℤ, 1) = RP^∞: Infinite Projective Space"))
    print()
    print(wrap(
        "The infinite real projective space RP^∞ is the colimit of the sequence:"
    ))
    print()
    print(f"  {cyan('RP⁰ ⊂ RP¹ ⊂ RP² ⊂ RP³ ⊂ ⋯ ⊂ RP^∞')}")
    print()
    print(wrap(
        "Equivalently, RP^∞ = S^∞ / (x ~ -x) where S^∞ is the infinite-dimensional "
        "sphere (which is contractible). As a result:"
    ))
    print()
    print(f"  {cyan('π₁(RP^∞) = ℤ/2ℤ')}")
    print(f"  {cyan('πₙ(RP^∞) = 0  for n ≥ 2')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Why contractible total space?'))}\n")
    print(wrap(
        "The infinite sphere S^∞ is the colimit of the Sⁿ chain. Each suspension "
        "increases connectivity: Sⁿ is (n-1)-connected, and the colimit S^∞ is "
        "∞-connected — hence contractible. The ℤ/2ℤ action (antipodal map) gives "
        "the covering S^∞ → RP^∞ with contractible total space."
    ))
    print()
    print(f"  {bold('Homotopy groups:')}")
    rp_groups = [
        ("RP^∞", 1, "ℤ/2ℤ"),
        ("RP^∞", 2, "0"),
        ("RP^∞", 3, "0"),
        ("RP^∞", 4, "0"),
        ("RP²",  1, "ℤ/2ℤ"),
        ("RP²",  2, "ℤ"),
        ("RP²",  3, "ℤ"),
        ("RP³",  1, "ℤ/2ℤ"),
        ("RP³",  2, "0"),
        ("RP³",  3, "ℤ"),
    ]
    for space, k, g in rp_groups:
        marker = bold(yellow("★")) if (space == "RP^∞") else dim("·")
        print(f"  {marker} π_{k}({space}) = {bold(cyan(g))}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('In HoTT: RP^∞ as a HIT'))}\n")
    print(f"  {cyan('data RP^∞ where')}")
    print(f"  {cyan('  base : RP^∞')}")
    print(f"  {cyan('  loop : base = base         (generator of ℤ/2ℤ)')}")
    print(f"  {cyan('  trunc : ∀(p q : base=base). p = q  (force π₁ = ℤ/2ℤ, not ℤ)')}")
    print()
    print(wrap(
        "The trunc constructor makes the path space a proposition — squashing "
        "the free loop (which would give ℤ) down to ℤ/2ℤ. This is one way "
        "to build K(ℤ/2ℤ, 1) directly as a HIT."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 4: Cohomology via K(G,n) ─────────────────────────────────────────

def _section_cohomology():
    clear()
    print(box("Cohomology: Hⁿ(X; G) = ||X → K(G,n)||₀"))
    print()
    print(wrap(
        "Cohomology with coefficients in G can be defined elegantly in HoTT:"
    ))
    print()
    print(f"  {bold(cyan('Hⁿ(X; G)  :=  ||X → K(G, n)||₀'))}")
    print()
    print(f"  {dim('(set-truncation of the function type X → K(G,n))')}")
    print()
    print(wrap(
        "A cohomology class in Hⁿ(X; G) is a homotopy class of maps X → K(G,n). "
        "Two maps f, g : X → K(G,n) are in the same class iff f and g are "
        "homotopic (there exists a homotopy H : X × 𝕀 → K(G,n) with H(−,0)=f, H(−,1)=g). "
        "The set-truncation forgets which specific homotopy witnesses this."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Computing H¹(S¹; ℤ):'))}\n")
    print(f"  {cyan('H¹(S¹; ℤ) = ||S¹ → K(ℤ,1)||₀ = ||S¹ → S¹||₀')}")
    print()
    print(wrap(
        "Maps S¹ → S¹ up to homotopy are classified by their DEGREE (winding number). "
        "The degree is an integer, and every integer arises as the degree of some map. "
        "So [S¹, S¹] (homotopy classes of maps) = ℤ."
    ))
    print()
    print(f"  {bold('H¹(S¹; ℤ) = ℤ')}")
    print()

    degrees = list(range(-3, 4))
    print(f"  {bold('Sample maps S¹ → S¹ and their degrees:')}")
    for d in degrees:
        if d == 0:
            desc = "constant map to base"
        elif d == 1:
            desc = "identity map"
        elif d == -1:
            desc = "reflection (reverse orientation)"
        elif d > 0:
            desc = f"wraps {d}× around the circle"
        else:
            desc = f"wraps {abs(d)}× backwards"
        print(f"    degree {d:>2}: {desc}")

    print()
    print(rule())
    print(f"\n  {bold(green('More cohomology computations:'))}\n")

    computations = [
        ("H⁰(X; ℤ)",    "π₀(X) as an ℤ-module",        "Connected components, weighted by ℤ"),
        ("H¹(X; ℤ)",    "||X → S¹||₀ ≃ Hom(π₁(X), ℤ)", "Homomorphisms from π₁(X) to ℤ"),
        ("H²(X; ℤ)",    "||X → CP^∞||₀",                "Line bundles over X (Picard group)"),
        ("H¹(X; ℤ/2ℤ)", "||X → RP^∞||₀",                "ℤ/2ℤ-torsors, orientation classes"),
        ("Hⁿ(Sⁿ; ℤ)",   "ℤ",                            "The fundamental class of Sⁿ"),
        ("Hⁿ(Sᵐ; ℤ)",   "0 for n ≠ m (and n > 0)",     "Spheres have concentrated cohomology"),
    ]

    for form, result, note in computations:
        print(f"  {bold(cyan(form))} = {bold(yellow(result))}")
        print(f"    {dim(note)}")
        print()

    input(bold("  Press Enter to continue... "))


# ── Section 5: Delooping ──────────────────────────────────────────────────────

def _section_delooping():
    clear()
    print(box("Delooping: Building K(G, n) Iteratively"))
    print()
    print(wrap(
        "Given K(G, n), we can build K(G, n+1) by DELOOPING: finding a type "
        "BX such that ΩBX ≃ X (the loop space of BX is X). Iterating:"
    ))
    print()
    print(f"  {cyan('K(G, 0) = G  (as a discrete type)')}")
    print(f"  {cyan('K(G, 1) = BG  (delooping of G)')}")
    print(f"  {cyan('K(G, 2) = B²G  (delooping of BG)')}")
    print(f"  {cyan('K(G, n) = BⁿG  (n-fold delooping)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('In HoTT: the delooping of ℤ'))}\n")
    print(wrap(
        "We want BZ such that Ω(BZ) ≃ ℤ. The loop space of the circle is ℤ:"
    ))
    print()
    print(f"  {cyan('Ω(S¹) = (base = base) ≃ ℤ')}")
    print()
    print(wrap(
        "So S¹ is the delooping of ℤ. Conversely, ℤ is the loop space of S¹. "
        "This is the content of π₁(S¹) = ℤ — the fundamental theorem of the circle."
    ))
    print()
    print(f"  {bold('The delooping tower for ℤ:')}")
    print()
    print(f"  {dim('K(ℤ,-1)')}  =  𝟘 (empty)          {dim('(no type with π₋₁=ℤ)')}")
    print(f"  {cyan('K(ℤ, 0)')}  =  ℤ (discrete)        {dim('(zero-type: points, no paths)')}")
    print(f"  {cyan('K(ℤ, 1)')}  =  S¹                  {dim('(π₁ = ℤ)')}")
    print(f"  {cyan('K(ℤ, 2)')}  =  CP^∞                {dim('(π₂ = ℤ, built by delooping S¹)')}")
    print(f"  {cyan('K(ℤ, 3)')}  =  B(CP^∞)             {dim('(π₃ = ℤ, highly abstract)')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The delooping problem:'))}\n")
    print(wrap(
        "Not every type X can be delooped. To deloop X, you need X to be a "
        "CONNECTED type with a group structure on its loop space. Specifically, "
        "you need X to be a 'group-like' A∞-space (an E₁-space). "
        "For abelian groups, you can deloop infinitely many times."
    ))
    print()
    print(wrap(
        "In HoTT, the delooping of a group G is constructed as a HIT: "
        "one point (the base), and for each group element g, a loop g : base = base. "
        "The group multiplication becomes path composition, and the group axioms "
        "become the groupoid laws — automatically satisfied by path structure."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 6: K(G,n) in HoTT ────────────────────────────────────────────────

def _section_hott_construction():
    clear()
    print(box("K(G, n) as HITs in HoTT"))
    print()
    print(wrap(
        "The Eilenberg-MacLane spaces can be defined directly as HITs. "
        "Here is the construction for K(G, 1) = BG:"
    ))
    print()
    print(f"  {cyan('data BG where')}")
    print(f"  {cyan('  base : BG')}")
    print(f"  {cyan('  path : G → base = base          -- one loop per group element')}")
    print(f"  {cyan('  mult : path(g·h) = path g · path h  -- multiplication = composition')}")
    print(f"  {cyan('  trunc : isTrunc 1 BG             -- force to be a 1-type')}")
    print()
    print(wrap(
        "The trunc constructor ensures BG is at h-level 1 (a groupoid). "
        "Without it, BG would have interesting higher homotopy."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('For K(G, 2):'))}\n")
    print(f"  {cyan('data K(G,2) where')}")
    print(f"  {cyan('  base : K(G,2)')}")
    print(f"  {cyan('  surf : G → refl = refl           -- one 2-loop per group element')}")
    print(f"  {cyan('  mult : surf(g+h) = surf g ∙₂ surf h  (+ for abelian G)')}")
    print(f"  {cyan('  trunc : isTrunc 2 K(G,2)')}")
    print()
    print(wrap(
        "Note G must be ABELIAN for K(G,2): the multiplication of 2-loops "
        "is commutative by Eckmann-Hilton, so only abelian group structures survive."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Why this matters:'))}\n")
    print(wrap(
        "Cohomology Hⁿ(X; G) = ||X → K(G,n)||₀ is now a DEFINITION in HoTT, "
        "not an axiom. You can compute with it. The Mayer-Vietoris sequence, "
        "the cup product, characteristic classes — all follow from properties "
        "of HITs and the definition of K(G,n). Algebraic topology becomes "
        "a branch of type theory."
    ))
    print()
    print(wrap(
        "This is part of the program of SYNTHETIC homotopy theory: doing "
        "algebraic topology directly in type theory, without first building "
        "a model in topological spaces and then translating. The type theory "
        "IS the topology."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("def",     "What is K(G,n)?",                              _section_definition),
    ("circle",  "K(ℤ,1) = S¹: the circle as a classifying space", _section_circle),
    ("rp",      "K(ℤ/2ℤ,1) = RP^∞: infinite projective space", _section_rp_inf),
    ("cohom",   "Cohomology: Hⁿ(X;G) = ||X → K(G,n)||₀",       _section_cohomology),
    ("deloop",  "Delooping: building K(G,n) iteratively",       _section_delooping),
    ("hott",    "K(G,n) as HITs in HoTT",                       _section_hott_construction),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Eilenberg-MacLane Spaces and Cohomology", width=70))
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
