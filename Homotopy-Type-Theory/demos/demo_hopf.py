#!/usr/bin/env python3
"""
The Hopf Fibration: S¹ → S³ → S²
==================================
One of the most beautiful constructions in topology, formalized in HoTT.

The Hopf fibration is a fiber bundle where S³ is "twisted" over S²
with fiber S¹. The long exact sequence of homotopy groups forces
π₃(S²) = ℤ — a non-trivial result that once required deep geometric
arguments, and is now a theorem in HoTT provable by type-theoretic means.
"""

import textwrap, os

# ─── ANSI helpers ────────────────────────────────────────────────────────────
def _c(code, t): return f"\033[{code}m{t}\033[0m"
bold    = lambda t: _c("1",    t)
green   = lambda t: _c("32",   t)
yellow  = lambda t: _c("33",   t)
cyan    = lambda t: _c("36",   t)
red     = lambda t: _c("31",   t)
dim     = lambda t: _c("2",    t)
magenta = lambda t: _c("35",   t)

def clear():
    print("\033[2J\033[H", end="")

def wrap(text, width=70, indent="  "):
    lines = []
    for paragraph in text.strip().split("\n"):
        if paragraph.strip() == "":
            lines.append("")
        else:
            lines.extend(textwrap.wrap(paragraph, width, initial_indent=indent,
                                       subsequent_indent=indent))
    return "\n".join(lines)

def box(title, width=66):
    inner = width - 2
    return (f"  ╔{'═' * inner}╗\n"
            f"  ║  {bold(title):<{inner - 2}}║\n"
            f"  ╚{'═' * inner}╝")

def rule(width=68):
    return "  " + dim("─" * width)

# ─── Homotopy group data ─────────────────────────────────────────────────────

# πₖ(Sⁿ) — key groups relevant to the Hopf fibration
# Format: (k, n) → string
HOMOTOPY_GROUPS = {
    (1, 1): "ℤ",
    (2, 1): "0",
    (3, 1): "0",
    (4, 1): "0",
    (1, 2): "0",
    (2, 2): "ℤ",
    (3, 2): "ℤ",      # ← THE KEY RESULT
    (4, 2): "ℤ/2ℤ",
    (5, 2): "ℤ/2ℤ",
    (1, 3): "0",
    (2, 3): "0",
    (3, 3): "ℤ",
    (4, 3): "ℤ/2ℤ",
    (5, 3): "ℤ/2ℤ",
    (1, 4): "0",
    (2, 4): "0",
    (3, 4): "0",
    (4, 4): "ℤ",
    (5, 4): "ℤ/2ℤ",
}

def pi(k, n):
    return HOMOTOPY_GROUPS.get((k, n), "?")

# ─── Section 1: What is a fiber bundle? ─────────────────────────────────────

def _section_fiber_bundle():
    clear()
    print(box("Fiber Bundles: Spaces That Are 'Locally Trivial'"))
    print()
    print(wrap(
        "A fiber bundle p : E → B with fiber F is a map where every point "
        "b : B has a neighborhood U such that p⁻¹(U) ≅ U × F. Globally, "
        "however, E need not be the product B × F — it can be 'twisted'."
    ))
    print()
    print(f"  {bold('Notation:')}  F → E → B")
    print(f"  {dim('           fiber  total space  base space')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Familiar examples:'))}")
    print()
    print(f"  {bold(cyan('Möbius band:'))}   S¹ ← total space → S¹")
    print(f"  {dim('  Locally: interval × arc of circle')}")
    print(f"  {dim('  Globally: twisted — fiber flips around once')}")
    print()
    print(f"  {bold(cyan('Cylinder:'))}      S¹ ← S¹×ℝ → S¹")
    print(f"  {dim('  Locally AND globally: trivial product')}")
    print()
    print(f"  {bold(cyan('Tangent bundle:'))} ℝ² ← TM → M")
    print(f"  {dim('  Total space = all tangent vectors at all points of M')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The Hopf fibration:'))}  S¹ → S³ → S²")
    print()
    print(wrap(
        "The total space S³ fibers over the base S² with fiber S¹. "
        "It is NON-TRIVIAL: S³ ≠ S¹ × S². "
        "This non-triviality is the source of all the interesting homotopy theory."
    ))
    print()
    print(wrap(
        "Why does this matter? The long exact sequence of a fibration "
        "relates πₖ(F), πₖ(E), and πₖ(B). Since we know πₖ(S¹) and πₖ(S³) "
        "rather well, the sequence pins down πₖ(S²) — most dramatically, "
        "π₃(S²) = ℤ."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ─── Section 2: The Hopf fibration geometrically ────────────────────────────

def _section_geometry():
    clear()
    print(box("The Hopf Map: Geometry"))
    print()
    print(wrap(
        "S³ can be viewed as the unit sphere in ℂ² = ℝ⁴:"
    ))
    print()
    print(f"  {cyan('S³ = {(z₀, z₁) : ℂ² | |z₀|² + |z₁|² = 1}')}")
    print()
    print(wrap(
        "The Hopf map h : S³ → S² = ℂP¹ sends (z₀, z₁) to the complex line "
        "[z₀ : z₁] — the ratio z₀/z₁ in projective space. More explicitly:"
    ))
    print()
    print(f"  {cyan('h(z₀, z₁) = (2·Re(z̄₀z₁), 2·Im(z̄₀z₁), |z₀|²−|z₁|²)')}")
    print(f"  {dim('             ──────────────────────────────────────────── ∈ S²')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The fiber over each point:'))}")
    print()
    print(wrap(
        "The preimage h⁻¹(b) for any b : S² is a great circle S¹ in S³. "
        "These fibers are Hopf circles — they are LINKED with each other. "
        "Any two distinct Hopf circles in S³ have linking number 1."
    ))
    print()

    # ASCII visualization of the fibration
    print(f"  {bold('Fibration diagram:')}")
    print()
    print(f"  {'S³':>12}  (total space, 3-sphere in ℝ⁴)")
    print(f"  {'│':>12}")
    print(f"  {'│ h':>12}  (Hopf map)")
    print(f"  {'↓':>12}")
    print(f"  {'S²':>12}  (base, 2-sphere)")
    print()
    print(f"  {dim('Each point of S² has fiber h⁻¹(b) = S¹  (a Hopf circle)')}")
    print()

    print(rule())
    print(f"\n  {bold(green('Stereographic picture:'))}")
    print()
    print(wrap(
        "After stereographic projection of S³ to ℝ³, the Hopf circles become:"
    ))
    print()
    print(f"  {dim('  N (north pole)')} ──────── maps to ────────── {dim('the z-axis (a line)')}")
    print(f"  {dim('  equator of S²')} ──────────────────────────── {dim('nested tori in ℝ³')}")
    print(f"  {dim('  S (south pole)')} ──────── maps to ────────── {dim('the unit circle in xy-plane')}")
    print()
    print(wrap(
        "The Hopf circles fill all of ℝ³ ∪ {∞} = S³, and any two of them "
        "are linked. This is the 'Hopf invariant one' property: the fibration "
        "has Hopf invariant η = 1 ∈ π₃(S²) = ℤ, where η is the generator."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ─── Section 3: Long exact sequence ─────────────────────────────────────────

def _section_les():
    clear()
    print(box("The Long Exact Sequence of a Fibration"))
    print()
    print(wrap(
        "Every fiber bundle F → E → B induces a long exact sequence of "
        "homotopy groups. 'Exact' means the image of each map equals "
        "the kernel of the next."
    ))
    print()
    print(f"  {cyan('⋯ → πₙ(F) → πₙ(E) → πₙ(B) → πₙ₋₁(F) → ⋯ → π₀(F) → π₀(E) → π₀(B)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('For S¹ → S³ → S²:'))}")
    print()

    les_rows = [
        ("n=4:", f"π₄(S¹)={pi(4,1)}", "→", f"π₄(S³)={pi(4,3)}", "→", f"π₄(S²)=?", "→", f"π₃(S¹)={pi(3,1)}", "→", "⋯"),
        ("n=3:", f"π₃(S¹)={pi(3,1)}", "→", f"π₃(S³)={pi(3,3)}", "→", f"π₃(S²)=?", "→", f"π₂(S¹)={pi(2,1)}", "→", "⋯"),
        ("n=2:", f"π₂(S¹)={pi(2,1)}", "→", f"π₂(S³)={pi(2,3)}", "→", f"π₂(S²)=?", "→", f"π₁(S¹)={pi(1,1)}", "→", "⋯"),
        ("n=1:", f"π₁(S¹)={pi(1,1)}", "→", f"π₁(S³)={pi(1,3)}", "→", f"π₁(S²)=?", "→", f"π₀(S¹)=0", "→", "⋯"),
    ]

    for row in les_rows:
        print("  " + "  ".join(row))

    print()
    print(rule())
    print(f"\n  {bold(yellow('Now fill in the knowns:'))}")
    print()
    print(wrap(
        "Using πₙ(S¹) = ℤ if n=1, else 0; and πₙ(S³) = 0 for n<3, ℤ for n=3:"
    ))
    print()

    filled_rows = [
        ("n=4:", f"π₄(S¹)={pi(4,1)}", "→", f"π₄(S³)={pi(4,3)}", "→", bold(cyan("π₄(S²)=?")), "→", f"π₃(S¹)={pi(3,1)}", "→"),
        ("n=3:", f"π₃(S¹)={pi(3,1)}", "→", f"π₃(S³)={pi(3,3)}", "→", bold(yellow("π₃(S²)=?")), "→", f"π₂(S¹)={pi(2,1)}", "→"),
        ("n=2:", f"π₂(S¹)={pi(2,1)}", "→", f"π₂(S³)={pi(2,3)}", "→", bold(cyan("π₂(S²)=?")), "→", f"π₁(S¹)={pi(1,1)}", "→"),
        ("n=1:", f"π₁(S¹)={pi(1,1)}", "→", f"π₁(S³)={pi(1,3)}", "→", bold(cyan("π₁(S²)=?")), "→", f"π₀(S¹)={0}",       "→"),
    ]

    for row in filled_rows:
        print("  " + "  ".join(row))

    print()
    input(bold("  Press Enter to continue to the derivation... "))


# ─── Section 4: Deriving π₃(S²) = ℤ ────────────────────────────────────────

def _section_derivation():
    clear()
    print(box("Deriving π₃(S²) = ℤ Step by Step"))
    print()

    print(f"  {bold('The long exact sequence at n=3:')}")
    print()
    print(f"  π₃(S¹) → π₃(S³) → π₃(S²) → π₂(S¹) → π₂(S³)")
    print(f"    {dim('↓')}         {dim('↓')}         {dim('↓')}         {dim('↓')}         {dim('↓')}")
    print(f"    0     →   ℤ   → π₃(S²) →    0    →    0")
    print()
    print(rule())
    print()

    steps = [
        ("Step 1", "π₃(S¹) = 0",
         "The circle has trivial higher homotopy: πₙ(S¹) = 0 for n ≥ 2. "
         "This follows from the universal cover ℝ → S¹; since ℝ is contractible, "
         "it has trivial homotopy, and the long exact sequence gives πₙ(S¹) = 0."),
        ("Step 2", "π₂(S¹) = 0",
         "Same reasoning. The universal cover is ℝ ≃ *, so π₂(S¹) = π₂(ℝ) = 0."),
        ("Step 3", "π₃(S³) = ℤ",
         "Every sphere has πₙ(Sⁿ) = ℤ — the degree of the map. For S³, a "
         "loop in πₙ is a map S³ → S³, classified by its degree (winding number). "
         "The identity map has degree 1, generating π₃(S³) = ℤ."),
        ("Step 4", "Exactness forces π₃(S²) = ℤ",
         "The sequence  0 → ℤ → π₃(S²) → 0  is exact. This means the map "
         "ℤ → π₃(S²) is both injective (kernel = image of 0 = 0) and surjective "
         "(image = kernel of 0 → 0 = π₃(S²)). So π₃(S²) ≅ ℤ. □"),
    ]

    for key, claim, explanation in steps:
        print(f"  {bold(green(key))}: {bold(claim)}")
        print(wrap(explanation))
        print()

    print(rule())
    print()
    print(f"  {bold(yellow('The generator: the Hopf element η ∈ π₃(S²)'))}")
    print()
    print(wrap(
        "The generator η is the Hopf map h : S³ → S² itself. Any map "
        "S³ → S² is homotopic to n · h for some integer n (with n · h "
        "meaning 'compose with itself n times via the group structure'). "
        "The Hopf invariant of h is 1, distinguishing it from the trivial map."
    ))
    print()
    print(wrap(
        "This was SURPRISING: S² and S³ have different dimensions, yet there "
        "are non-trivial maps from S³ to S². Before Hopf (1931), it was "
        "conjectured that all such maps were null-homotopic. Hopf proved otherwise."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ─── Section 5: HoTT formalization ──────────────────────────────────────────

def _section_hott():
    clear()
    print(box("The Hopf Fibration in HoTT"))
    print()
    print(wrap(
        "The Hopf fibration was formalized in HoTT by Brunerie, Licata, and "
        "Lumsdaine (2013), and later by Buchholtz and Rijke. The construction "
        "uses the HIT for the 2-sphere and the join construction for S³."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Key ingredients in HoTT:'))}")
    print()

    ingredients = [
        ("S¹ as a HIT",
         "data S¹ where\n"
         "  base : S¹\n"
         "  loop : base = base",
         "The circle with one point and one non-trivial loop."),
        ("S² as a HIT",
         "data S² where\n"
         "  base : S²\n"
         "  surf : refl_base = refl_base",
         "The 2-sphere with one point and one non-trivial 2-loop (a surface)."),
        ("S³ as a suspension",
         "S³ = ΣS²",
         "S³ is the suspension of S². It has north, south, and for each "
         "point on S², a meridian from north to south."),
        ("Join construction",
         "S³ = S¹ ★ S¹  (join of two circles)",
         "Alternatively, S³ is the join of S¹ with itself. Points of S¹ ★ S¹ "
         "are formal convex combinations t·x + (1-t)·y with t:I, x,y:S¹."),
    ]

    for name, code, desc in ingredients:
        print(f"  {bold(cyan(name))}")
        for line in code.split("\n"):
            print(f"  {cyan('  ' + line)}")
        print(wrap(desc))
        print()

    print(rule())
    print(f"\n  {bold(green('Defining the Hopf map in HoTT:'))}")
    print()
    print(wrap(
        "The Hopf map h : S³ → S² is defined using the action of S¹ on itself "
        "by multiplication (S¹ as the unit complex numbers). Given the join "
        "S³ = S¹ ★ S¹, define:"
    ))
    print()
    print(f"  {cyan('h(inl x)       = base')}")
    print(f"  {cyan('h(inr y)       = base')}")
    print(f"  {cyan('h(push x y t)  = rot_x(loop)(t)')}")
    print(f"  {dim('  where rot_x rotates the loop by x : S¹')}")
    print()
    print(wrap(
        "The key: for each x : S¹, we get a loop at base in S². The rotation "
        "by x traces out the surface of S², and the resulting map is the Hopf map."
    ))
    print()
    print(rule())
    brunerie_header = "Brunerie's theorem (2019):"
    print(f"\n  {bold(green(brunerie_header))}")
    print()
    print(wrap(
        "Guillaume Brunerie proved in his PhD thesis that π₃(S²) = ℤ in HoTT, "
        "by showing the Hopf invariant of the Hopf map is ±1. The proof uses "
        "the James construction, cup products in cohomology (defined via HITs), "
        "and the James splitting S²ⁿ⁺¹ ≃ S^(2n+1) ∨ ΣΩS^(2n+1)."
    ))
    print()
    print(wrap(
        "Independently, Licata and Brunerie gave a shorter proof using the "
        "long exact sequence of the Hopf fibration — essentially the argument "
        "we followed in this demo, now fully formalized in type theory."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ─── Section 6: Consequences and table ──────────────────────────────────────

def _section_consequences():
    clear()
    print(box("Consequences and Homotopy Groups of Spheres"))
    print()
    print(wrap(
        "The Hopf fibration is not an isolated curiosity — it is the "
        "foundation for an entire family of fibrations and results."
    ))
    print()

    fibrations = [
        ("Real:    S⁰ → S¹ → S¹",    "ℤ/2ℤ → S¹ → RP¹ = S¹",   "Generates π₁(RP¹) = ℤ"),
        ("Complex: S¹ → S³ → S²",    "Hopf fibration",             "Generates π₃(S²) = ℤ  ← this demo"),
        ("Quaternion: S³ → S⁷ → S⁴", "Quaternionic Hopf",         "Generates π₇(S⁴) = ℤ ⊕ ℤ/12ℤ"),
        ("Octonionic: S⁷ → S¹⁵ → S⁸","Octonionic Hopf",           "Generates π₁₅(S⁸) = ℤ ⊕ ℤ/120ℤ"),
    ]

    print(f"  {bold('The four Hopf fibrations (one per normed division algebra):')}\n")
    for name, alt, consequence in fibrations:
        marker = yellow("★") if "S³ → S²" in name else dim("◆")
        print(f"  {marker} {bold(name)}")
        print(f"    {dim(alt)}")
        print(f"    {cyan(consequence)}")
        print()

    print(rule())
    print(f"\n  {bold('Selected homotopy groups πₖ(Sⁿ)  (rows: k, columns: n):')}\n")

    ns = [1, 2, 3, 4]
    ks = [1, 2, 3, 4, 5]

    header = f"  {'k\\n':6}" + "".join(f"  {'S'+str(n):8}" for n in ns)
    print(bold(header))
    print("  " + dim("─" * 50))
    for k in ks:
        row = f"  {'k='+str(k):6}"
        for n in ns:
            g = pi(k, n)
            if g == "ℤ" and (k == n or (k == 3 and n == 2)):
                row += f"  {bold(green(g)):18}"
            elif g == "0":
                row += f"  {dim(g):12}"
            else:
                row += f"  {yellow(g):18}"
        print(row)

    print()
    print(f"  {dim('Green: computed or determined in this demo')}")
    print(f"  {dim('Yellow: non-trivial stable or unstable groups')}")
    print()
    print(rule())
    print()
    print(wrap(
        "Computing homotopy groups of spheres is one of the central open "
        "problems in algebraic topology. HoTT provides a new framework where "
        "these groups are defined synthetically and some can be computed "
        "purely by type-theoretic reasoning."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ─── Section 7: Hopf invariant ──────────────────────────────────────────────

def _section_hopf_invariant():
    clear()
    print(box("The Hopf Invariant"))
    print()
    print(wrap(
        "The Hopf invariant is an integer H(f) associated to any map "
        "f : S^(2n-1) → Sⁿ. It measures the 'linking' of the preimages "
        "of two regular values."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Definition via cohomology:'))}")
    print()
    print(wrap(
        "Given f : S^(2n-1) → Sⁿ, form the mapping cone Cf = Sⁿ ∪_f D^(2n). "
        "The cohomology H*(Cf; ℤ) has generators α ∈ H^n and β ∈ H^(2n). "
        "The Hopf invariant is the integer H(f) such that:"
    ))
    print()
    print(f"  {cyan('α ∪ α = H(f) · β  ∈ H^(2n)(Cf; ℤ)')}")
    print()
    print(wrap(
        "The cup product of α with itself measures self-linking of the fiber."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Hopf invariant one problem:'))}")
    print()
    print(wrap(
        "For which n does there exist f : S^(2n-1) → Sⁿ with H(f) = 1?"
    ))
    print()
    hopf_one = [
        (1,  "S¹",  "S⁰ → S¹ → S¹",    "Real numbers ℝ"),
        (2,  "S²",  "S¹ → S³ → S²",    "Complex numbers ℂ  ← Hopf"),
        (4,  "S⁴",  "S³ → S⁷ → S⁴",   "Quaternions ℍ"),
        (8,  "S⁸",  "S⁷ → S¹⁵ → S⁸",  "Octonions 𝕆"),
    ]
    for n, sn, fib, algebra in hopf_one:
        print(f"  {bold(cyan(f'n={n}'))}:  {sn}  {dim(fib):35} ← {yellow(algebra)}")

    print()
    print(wrap(
        "Adams (1960) proved this is ONLY possible for n = 1, 2, 4, 8 — "
        "corresponding exactly to the four normed division algebras over ℝ. "
        "The proof uses secondary cohomology operations (Adams operations). "
        "In HoTT, this theorem connects to the classification of H-spaces."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('In HoTT:'))}")
    print()
    print(wrap(
        "The Hopf invariant can be defined type-theoretically using the "
        "cohomology theory of HITs. The statement 'H(f) = 1 implies n ∈ {1,2,4,8}' "
        "is a theorem about the multiplication on Sⁿ⁻¹ — the existence of a "
        "map Sⁿ⁻¹ × Sⁿ⁻¹ → Sⁿ⁻¹ with unit. This is related to the existence "
        "of H-space structures, which HoTT can reason about synthetically."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


# ─── Main loop ───────────────────────────────────────────────────────────────

SECTIONS = [
    ("bundle",      "Fiber Bundles: Spaces That Are Locally Trivial",  _section_fiber_bundle),
    ("geometry",    "The Hopf Map: Geometry",                          _section_geometry),
    ("les",         "Long Exact Sequence of a Fibration",              _section_les),
    ("derivation",  "Deriving π₃(S²) = ℤ Step by Step",               _section_derivation),
    ("hott",        "The Hopf Fibration in HoTT",                      _section_hott),
    ("consequences","Consequences and Homotopy Groups of Spheres",     _section_consequences),
    ("invariant",   "The Hopf Invariant",                              _section_hopf_invariant),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("The Hopf Fibration: S¹ → S³ → S²", width=70))
        print()
        for i, (key, title, _) in enumerate(SECTIONS):
            marker = bold(cyan("▶")) if i == idx else " "
            print(f"  {marker} {bold(str(i+1))}   {title}")
        print()
        print(rule())
        print(f"  {dim('1-7  jump to section   n  next   p  prev   q  quit')}")
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
