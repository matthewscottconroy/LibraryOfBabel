#!/usr/bin/env python3
"""
Pushouts: Gluing Spaces Together
==================================
The pushout HIT as the universal "gluing" construction.

Every familiar topological space can be built by gluing simpler pieces together.
The pushout formalizes this: given maps A→B and A→C, the pushout B ⊔_A C
glues B and C together along their shared subspace A.
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


# ── Section 1: The pushout HIT ────────────────────────────────────────────────

def _section_pushout_def():
    clear()
    print(box("The Pushout: Universal Gluing"))
    print()
    print(wrap(
        "Given types A, B, C and maps f : A → B and g : A → C, the pushout "
        "B ⊔_A C glues B and C together by identifying f(a) ∈ B with g(a) ∈ C "
        "for each a : A."
    ))
    print()
    print(f"  {cyan('data B ⊔_A C where')}")
    print(f"  {cyan('  inl  : B → B ⊔_A C           -- inject from B')}")
    print(f"  {cyan('  inr  : C → B ⊔_A C           -- inject from C')}")
    print(f"  {cyan('  glue : ∀(a:A). inl(f a) = inr(g a)  -- identify along A')}")
    print()
    print(f"  {bold('The diagram:')}")
    print()
    print(f"       {bold(cyan('A'))}")
    print(f"      {dim('/ \\')} ")
    print(f"    {bold(cyan('f'))} {dim('/ \\')} {bold(cyan('g'))}")
    print(f"    {dim('↙')}   {dim('↘')}")
    print(f"   {bold(green('B'))}     {bold(green('C'))}")
    print(f"    {dim('\\')}   {dim('/')}")
    print(f"     {dim('\\')} {dim('/')}")
    print(f"      {bold(yellow('B ⊔_A C'))}")
    print()
    print(wrap(
        "The pushout is the COLIMIT of the diagram A ← A → C. It is the "
        "initial type that receives maps from both B and C, agreeing on A. "
        "Every space built by gluing is a pushout."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('The recursion principle:'))}\n")
    print(f"  {cyan('rec : (h_B : B → D) → (h_C : C → D)')}")
    print(f"  {cyan('    → (∀ a. h_B (f a) = h_C (g a))')}")
    print(f"  {cyan('    → B ⊔_A C → D')}")
    print()
    print(wrap(
        "To map OUT of the pushout, you need compatible maps from B and C. "
        "This forces you to provide a 'coherence' — a proof that both maps "
        "agree on the glued-along part A."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 2: S¹ as a pushout ────────────────────────────────────────────────

def _section_circle():
    clear()
    print(box("S¹ as a Pushout: Two Arcs Glued at Their Endpoints"))
    print()
    print(wrap(
        "The circle is two intervals glued at both endpoints. Let:"
    ))
    print()
    print(f"  {cyan('A = 𝟚 = {N, S}   (two points: north and south)')}")
    print(f"  {cyan('B = I = [0,1]    (one interval, left arc)')}")
    print(f"  {cyan('C = I = [0,1]    (one interval, right arc)')}")
    print(f"  {cyan('f(N) = 0, f(S) = 1   (endpoints of left arc)')}")
    print(f"  {cyan('g(N) = 0, g(S) = 1   (endpoints of right arc)')}")
    print()
    print(f"  {bold('S¹ ≃ I ⊔_𝟚 I')}")
    print()
    print(f"  {bold(green('ASCII picture:'))}")
    print()
    print(f"                {bold(green('N'))}")
    print(f"              {dim('/ \\')} ")
    print(f"       left  {dim('/   \\')}  right")
    print(f"       arc  {dim('/     \\')}  arc")
    print(f"            {dim('↙       ↘')}")
    print(f"           {bold(green('S'))}         {bold(green('S'))}  (same S, glued)")
    print()
    print(f"  Result: {bold(yellow('○'))}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Alternatively: S¹ = 𝟙 ⊔_𝟚 𝟙 (suspension of 𝟚)'))}\n")
    print(f"  {cyan('Σ(𝟚) = 𝟙 ⊔_𝟚 𝟙')}")
    print(f"  {cyan('  = north, south (from the two copies of 𝟙)')}")
    print(f"  {cyan('  + merid(t) for t : 𝟚, i.e. merid(0) and merid(1)')}")
    print()
    print(wrap(
        "The suspension ΣA of any type A is a pushout: two points (north, south) "
        "with a path from north to south for each element of A. "
        "S¹ = Σ(𝟚) has two meridians — which, composed, give the fundamental loop."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('π₁(S¹) = ℤ from the pushout structure:'))}\n")
    print(wrap(
        "By van Kampen's theorem (which holds in HoTT as a consequence of "
        "pushout induction), π₁(B ⊔_A C) is computed from π₁(B), π₁(C), "
        "and the images of π₁(A). For S¹ = I ⊔_𝟚 I, both arcs have "
        "trivial π₁ (they are contractible), and the gluing of two endpoints "
        "creates one free loop — giving π₁(S¹) = ℤ."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 3: Torus as a pushout ─────────────────────────────────────────────

def _section_torus():
    clear()
    print(box("The Torus T² as a Pushout"))
    print()
    print(wrap(
        "The torus is what you get when you take a square and identify "
        "opposite edges. As a pushout:"
    ))
    print()
    print(f"  {cyan('T² ≃ S¹ × S¹')}")
    print()
    print(wrap(
        "Or as a CW complex: one 0-cell (base), two 1-cells (a and b), "
        "and one 2-cell (the square) glued with boundary word aba⁻¹b⁻¹."
    ))
    print()
    print(f"  {bold(green('The square picture:'))}")
    print()
    print(f"    {bold('b')}→")
    print(f"  ┌──────┐")
    print(f"  │      │")
    print(f"  {bold('a')}↑  T²  ↑{bold('a')}")
    print(f"  │      │")
    print(f"  └──────┘")
    print(f"    {bold('b')}→")
    print()
    print(f"  {dim('Top and bottom edges (→) are identified as b.')}")
    print(f"  {dim('Left and right edges (↑) are identified as a.')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('As a HIT:'))}\n")
    print(f"  {cyan('data T² where')}")
    print(f"  {cyan('  base : T²')}")
    print(f"  {cyan('  p    : base = base    (loop in one direction)')}")
    print(f"  {cyan('  q    : base = base    (loop in other direction)')}")
    print(f"  {cyan('  surf : p · q = q · p  (the square = commutativity)')}")
    print()
    print(wrap(
        "The surf constructor is a 2-path that says p and q COMMUTE. "
        "This is the crucial difference from the free group on two generators: "
        "the torus has a RELATION between its loops."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('π₁(T²) = ℤ × ℤ:'))}\n")
    print(wrap(
        "The fundamental group of the torus is the FREE ABELIAN group on two "
        "generators. The surf constructor forces p·q = q·p, so the group is "
        "abelian: ℤ × ℤ (not the free group F₂). The loops p and q each "
        "generate their own copy of ℤ, and they commute."
    ))
    print()
    print(f"  π₁(T²) = ℤ × ℤ  =  {bold(cyan('⟨a, b | ab = ba⟩'))}")
    print()
    print(wrap(
        "Compare with the figure-8 space S¹ ∨ S¹ (wedge of two circles), "
        "which has π₁ = F₂ (free group on two generators, NO commutativity). "
        "The torus adds the surf 2-path that forces commutativity."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 4: Klein bottle ────────────────────────────────────────────────────

def _section_klein():
    clear()
    print(box("The Klein Bottle: A Non-Orientable Surface"))
    print()
    print(wrap(
        "The Klein bottle is like the torus, but one edge is identified with "
        "reversed orientation. It cannot be embedded in ℝ³ without "
        "self-intersection — it is a non-orientable surface."
    ))
    print()
    print(f"  {bold(green('The square picture:'))}")
    print()
    print(f"    {bold('b')}→")
    print(f"  ┌──────┐")
    print(f"  │      │")
    print(f"  {bold('a')}↑  Kl  ↓{bold('a')}  ← one edge REVERSED")
    print(f"  │      │")
    print(f"  └──────┘")
    print(f"    {bold('b')}→")
    print()
    print(f"  {dim('Top and bottom (→) identified as b (same direction).')}")
    print(f"  {dim('Left edge (↑) and right edge (↓) identified REVERSED as a.')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('As a HIT:'))}\n")
    print(f"  {cyan('data Klein where')}")
    print(f"  {cyan('  base : Klein')}")
    print(f"  {cyan('  p    : base = base')}")
    print(f"  {cyan('  q    : base = base')}")
    print(f"  {cyan('  surf : p · q = q · p⁻¹  (twisted identification)')}")
    print()
    print(wrap(
        "The surf constructor now says p·q = q·p⁻¹. This is the fundamental "
        "relation of the Klein bottle group — it is NOT abelian, unlike the torus."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('π₁(Klein) = ⟨a, b | abab⁻¹ = e⟩:'))}\n")
    print(wrap(
        "The Klein bottle group is the semidirect product ℤ ⋊ ℤ where the "
        "second ℤ acts on the first by negation. It has non-trivial 2-torsion: "
        "there is an element of order 2, unlike the torus group ℤ×ℤ."
    ))
    print()
    print(f"  π₁(Klein) = {bold(cyan('⟨a, b | abab⁻¹ = e⟩'))}")
    print()
    print(f"  {bold('Comparison:')}")
    print(f"  {'Torus:':20} π₁ = ℤ×ℤ  =  ⟨a,b | aba⁻¹b⁻¹⟩  (abelian)")
    print(f"  {'Klein:':20} π₁ = ℤ⋊ℤ  =  ⟨a,b | abab⁻¹⟩    (non-abelian)")
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 5: RP² ────────────────────────────────────────────────────────────

def _section_rp2():
    clear()
    print(box("Real Projective Plane RP²"))
    print()
    print(wrap(
        "RP² is the space of lines through the origin in ℝ³, equivalently S² "
        "with antipodal points identified: x ~ -x. As a CW complex: one 0-cell, "
        "one 1-cell (giving a circle), and one 2-cell glued by the degree-2 map."
    ))
    print()
    print(f"  {bold(green('The disk picture:'))}")
    print()
    print(f"       {bold(green('*'))}")
    print(f"     {dim('/ \\')} ")
    print(f"   {dim('a')}{dim('↑')}   {dim('↓')}{dim('a')}  (boundary identified with reversal)")
    print(f"     {dim('\\ /')}")
    print(f"       {bold(green('*'))}")
    print(f"   {dim('(disk with antipodal boundary points identified)')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('As a HIT:'))}\n")
    print(f"  {cyan('data RP² where')}")
    print(f"  {cyan('  base : RP²')}")
    print(f"  {cyan('  loop : base = base    (the single 1-cell, creating S¹ skeleton)')}")
    print(f"  {cyan('  surf : loop · loop = refl  (the 2-cell: loop² = 0)')}")
    print()
    print(wrap(
        "The surf constructor says the loop has order 2: loop · loop = refl. "
        "This forces π₁(RP²) to be a group with an element of order 2."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Homotopy groups:'))}\n")
    groups = [
        ("π₁(RP²)", "ℤ/2ℤ",       "The loop has order 2 (surf says loop² = refl)"),
        ("π₂(RP²)", "ℤ",           "The universal cover is S² (simply connected), π₂(S²) = ℤ"),
        ("π₃(RP²)", "ℤ",           "Same as π₃(S²) via the covering fibration"),
        ("π₁(RP³)", "ℤ/2ℤ",        "RP³ ≃ SO(3), double-covered by S³ ≃ SU(2)"),
    ]
    for grp, val, note in groups:
        print(f"  {bold(cyan(grp))} = {bold(yellow(val))}")
        print(f"    {dim(note)}")
        print()

    print(rule())
    print(f"\n  {bold(green('General pattern: RPⁿ'))}\n")
    print(f"  {cyan('RPⁿ = Sⁿ / (x ~ -x)')}")
    print()
    print(wrap(
        "The real projective spaces RPⁿ are a family of spaces with π₁ = ℤ/2ℤ "
        "and universal cover Sⁿ. They appear throughout algebraic topology, "
        "differential geometry (orientability), and physics (the configuration "
        "space of SO(3) rotations is RP³)."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 6: Van Kampen's theorem ───────────────────────────────────────────

def _section_van_kampen():
    clear()
    print(box("Van Kampen's Theorem: π₁ of Pushouts"))
    print()
    print(wrap(
        "Seifert-van Kampen's theorem computes π₁ of a pushout from the "
        "fundamental groups of its pieces. In HoTT, this is a theorem about "
        "pushouts — provable purely type-theoretically."
    ))
    print()
    print(f"  {bold('Setup:')} X = B ⊔_A C, with A, B, C connected and A connected.")
    print()
    print(f"  {cyan('π₁(B ⊔_A C) ≅ π₁(B) *_{π₁(A)} π₁(C)')}")
    print(f"  {dim('               (amalgamated free product)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key examples computed by van Kampen:'))}\n")

    examples = [
        ("S¹ = I ⊔_𝟚 I",
         "π₁(I)=0, π₁(I)=0, A=𝟚",
         "π₁(S¹) = 0 *_0 0 = ℤ  (the free group on the new loop)"),

        ("T² = S¹ × S¹",
         "π₁(S¹)=ℤ, π₁(S¹)=ℤ, A=𝟙",
         "π₁(T²) = ℤ *_1 ℤ = ℤ×ℤ  (free product over trivial = direct product when abelian)"),

        ("S¹ ∨ S¹  (wedge)",
         "π₁(S¹)=ℤ, π₁(S¹)=ℤ, A=𝟙",
         "π₁(S¹∨S¹) = ℤ * ℤ = F₂  (free group on two generators)"),

        ("RP² = D² ⊔_{S¹} S¹",
         "π₁(D²)=0, A=S¹→S¹ (degree 2 map)",
         "π₁(RP²) = 0 *_ℤ ℤ = ℤ/2ℤ  (ℤ quotiented by its image 2ℤ)"),

        ("Klein = T² with twist",
         "HIT with p·q = q·p⁻¹",
         "π₁(Klein) = ⟨a,b | abab⁻¹⟩  (non-abelian, order 2 torsion)"),
    ]

    for name, setup, result in examples:
        print(f"  {bold(cyan(name))}")
        print(f"    {dim(setup)}")
        print(f"    {green(result)}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('In HoTT: the encode-decode proof'))}\n")
    print(wrap(
        "Van Kampen in HoTT is proved using the encode-decode method: define "
        "a code family over the pushout by pushout induction, then show encode "
        "and decode are mutual inverses. The key is that the code for the pushout "
        "is the amalgamated free product — a group defined by generators and "
        "relations, where the relations come from the glue paths."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("def",    "The pushout HIT: universal gluing",        _section_pushout_def),
    ("circle", "S¹ as a pushout: two arcs",                _section_circle),
    ("torus",  "Torus T² and π₁(T²) = ℤ×ℤ",              _section_torus),
    ("klein",  "Klein bottle: non-orientable surface",     _section_klein),
    ("rp2",    "Real projective plane RP²",                _section_rp2),
    ("vk",     "Van Kampen's theorem: π₁ of pushouts",     _section_van_kampen),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Pushouts: Gluing Spaces Together", width=70))
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
