#!/usr/bin/env python3
"""
Modal HoTT: Cohesion, Shape, and Differential Geometry
=======================================================
Lawvere-Schreiber cohesive HoTT adds modalities (♭, ♯, ○) that
capture the distinction between discrete, continuous, and smooth structure.
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


def _section_modalities():
    clear()
    print(box("What Are Modalities?"))
    print()
    print(wrap(
        "A MODALITY is a type-theoretic operation ○ that sends types to types, "
        "with a unit map η:A→○A, satisfying a universal property. "
        "Modalities generalize propositions (the -1-truncation) and "
        "set-truncation, and they are the key tool in modal type theory."
    ))
    print()
    print(f"  {cyan('Modality ○ : Type → Type  with  η : A → ○A')}")
    print(f"  {dim('  ○ is idempotent:  ○(○A) ≃ ○A')}")
    print(f"  {dim('  ○ reflects maps:  (○A → B) ≃ (A → B) when B is ○-modal')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Examples of modalities:'))}\n")
    examples = [
        ("||A||_{-1}",  "propositional truncation (n = -1)"),
        ("||A||_0",     "set-truncation"),
        ("||A||_n",     "n-truncation — a modality for each n"),
        ("♭A",          "flat/discrete — forget continuous structure"),
        ("♯A",          "sharp/codiscrete — maximally continuous"),
        ("○A",          "shape — topological quotient"),
        ("L_S A",       "Bousfield localization at a set S"),
        ("◻A",          "necessity in modal logic"),
        ("◇A",          "possibility (= cofibrant replacement modality)"),
    ]
    for symbol, desc in examples:
        print(f"  {bold(cyan(symbol)):14} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Lex vs non-lex modalities:'))}\n")
    print(wrap(
        "A LEX modality preserves finite limits (pullbacks, terminal object). "
        "Lex modalities are also called SUBTOPOSES. The n-truncations are lex. "
        "Non-lex modalities like ◻ in S4 modal logic are also useful but "
        "less well-behaved."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_flat():
    clear()
    print(box("The ♭ (Flat) Modality: Discrete Types"))
    print()
    print(wrap(
        "The FLAT modality ♭ (pronounced 'flat') takes a type A and returns "
        "the TYPE OF CRISP TERMS of A — the terms that are DISCRETE, i.e., "
        "constant on any path in the base space. It forgets continuous "
        "or cohesive structure."
    ))
    print()
    print(f"  {cyan('♭ : Type → Type')}")
    print(f"  {cyan('♭A = {a:A | a is crisp (globally constant)}')}")
    print(f"  {dim('  A term a:A is crisp if it does not depend on any point in space')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key properties of ♭:'))}\n")
    properties = [
        ("♭ is a monad",      "η:A→♭A (every term is crisp in the discrete world)"),
        ("♭(A→B) ≃ ♭A→♭B",   "♭ is strong — it distributes over function types"),
        ("♭A is discrete",    "all paths in ♭A are trivial (♭A is a set if A is)"),
        ("♭ ⊣ Γ",             "♭ is left adjoint to global sections functor Γ"),
        ("Γ ⊣ ♯",             "global sections is left adjoint to ♯"),
        ("♭A ≃ A for sets",   "if A has no continuous structure, ♭A ≃ A"),
    ]
    for name, desc in properties:
        print(f"  {bold(cyan(name)):22} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Use: crisp induction'))}\n")
    print(wrap(
        "Crisp induction allows reasoning about types without depending on "
        "the cohesive/continuous structure. A crisp term a:♭A can be used "
        "in any context — it is a 'constant' that does not vary with position."
    ))
    print()
    print(f"  {cyan('crisp-ind : (a:♭A) → P(a)')}")
    print(f"  {dim('  where P need not be a discrete family')}")
    print()
    print(wrap(
        "This lets us use both the continuous type and its discrete shadow "
        "in the same proof — essential for connecting topology to algebra."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_shape():
    clear()
    print(box("The ○ (Shape) Modality"))
    print()
    print(wrap(
        "The SHAPE modality ○ (sometimes written ∫ or Π_∞) sends a type A "
        "to its UNDERLYING HOMOTOPY TYPE — it forgets the geometric/smooth "
        "structure and retains only the topological shape."
    ))
    print()
    print(f"  {cyan('○ : Type → Type')}")
    print(f"  {cyan('○A = topological quotient of A by the relation')}")
    print(f"  {cyan('     a ~ b iff they are connected by a continuous path')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key examples:'))}\n")
    examples = [
        ("○ ℝⁿ = *",          "Euclidean space is contractible — shape is a point"),
        ("○ S¹ = S¹",         "the circle has shape S¹ (already discrete/topological)"),
        ("○ ℝ\\{0} = S⁰",    "line minus origin has shape two points"),
        ("○ Torus = BZ²",     "torus shape = K(Z²,1)"),
        ("○ * = *",           "the point maps to itself"),
        ("π_n(○X) = π_n(X)", "shape preserves homotopy groups"),
    ]
    for name, desc in examples:
        print(f"  {bold(cyan(name)):25} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(green('The cohesive adjoint quadruple:'))}\n")
    print(f"  {cyan('♭ ⊣ ○ ⊣ ♯    (or   Disc ⊣ Π ⊣ Codisc  in Lawvere notation)')}")
    print()
    print(wrap(
        "The three modalities form an adjoint triple. Together with the identity "
        "functor, this gives the COHESIVE STRUCTURE that makes Lawvere's "
        "synthetic differential geometry possible inside type theory."
    ))
    print()
    print(f"  {'Modality':10}  {'Operation':28}  {'Intuition'}")
    print(f"  {dim('─'*60)}")
    modalities = [
        ("♭",     "discrete: forget geometry",    "freeze all paths"),
        ("○",     "shape: forget geometry, keep top", "quotient by paths"),
        ("♯",     "codiscrete: maximally continuous", "everything is connected"),
        ("id",    "the actual type",               "geometry + topology"),
    ]
    for sym, op, intuition in modalities:
        print(f"  {bold(cyan(sym)):18} {dim(op):35} {yellow(intuition)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_cohesive():
    clear()
    print(box("Cohesive HoTT"))
    print()
    print(wrap(
        "COHESIVE HoTT (Schreiber-Shulman) is HoTT extended with the "
        "adjoint triple ♭ ⊣ ○ ⊣ ♯. This allows SYNTHETIC differential "
        "geometry and algebraic topology inside type theory."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('What cohesive HoTT can express:'))}\n")
    expressions = [
        ("Smooth manifolds",
         "A type M with local charts → ℝⁿ; tangent spaces are fibers"),
        ("Differential forms",
         "Ω^n(M) = maps M → ℝⁿ satisfying antisymmetry"),
        ("de Rham cohomology",
         "H^n_dR(M) = ker(d:Ω^n→Ω^{n+1}) / im(d:Ω^{n-1}→Ω^n)"),
        ("Principal bundles",
         "P → M with G-action; classified by BG = K(G,1)"),
        ("Connection on a bundle",
         "A splitting of TP as vertical + horizontal"),
        ("Curvature",
         "F_A = dA + A∧A ∈ Ω²(M,g)  (obstruction to flatness)"),
        ("Characteristic classes",
         "Chern classes, Pontryagin classes — via cohesive cohomology"),
        ("Flat ♭G-connections",
         "π₁(M) → G homomorphisms — via the flat modality"),
    ]
    for name, desc in expressions:
        print(f"  {bold(cyan(name)):25} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Key theorem: de Rham theorem internally'))}\n")
    print(f"  {cyan('H^n_dR(M) ≅ H^n(○M ; ℝ)  (de Rham = singular cohomology)')}")
    print()
    print(wrap(
        "This is the de Rham theorem, proved SYNTHETICALLY inside cohesive HoTT "
        "by using the shape modality ○. The left side uses smooth forms; "
        "the right side uses the homotopy type ○M. The bridge is cohesion."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_applications():
    clear()
    print(box("Applications of Modal HoTT"))
    print()
    print(wrap(
        "Modal HoTT provides a uniform framework for several areas of "
        "mathematics that were previously studied with separate tools."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Differential geometry:'))}\n")
    print(wrap(
        "Synthetic differential geometry (SDG) adds nilpotent infinitesimals. "
        "Cohesive HoTT combines SDG with HoTT: manifolds are types, "
        "differential forms are internal to the theory."
    ))
    print()
    print(f"  {cyan('D = {x:ℝ | x² = 0}  (nilpotent infinitesimals)')}")
    print(f"  {cyan('TM = M^D            (tangent bundle = exponential by D)')}")
    print(f"  {cyan('A form is a function TM → ℝ satisfying linearity axioms')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Quantum field theory:'))}\n")
    print(wrap(
        "Schreiber has proposed cohesive HoTT as a foundation for "
        "QUANTUM GAUGE FIELD THEORY. The modalities capture the transition "
        "between local and global structure, which is central to gauge theory."
    ))
    print()
    gauge = [
        ("Gauge field",    "connection A on principal bundle P→M"),
        ("Gauge transform","automorphism of P — an element of Aut(P)"),
        ("Moduli space",   "[A]/Aut(P) — quotient by gauge symmetry"),
        ("BV formalism",   "derived critical locus of action functional"),
        ("Prequantum",     "degree-2 cohomology class in H²(M;B²U(1))"),
    ]
    for name, desc in gauge:
        print(f"  {bold(cyan(name)):20} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The dream: physics in HoTT'))}\n")
    print(wrap(
        "Schreiber has formalized much of classical and quantum field theory "
        "in cohesive HoTT, including anomaly cancellation, string backgrounds, "
        "and the M-theory C-field. The modalities ♭, ♯, ○ capture exactly "
        "the geometric structure needed for modern mathematical physics."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("modal",    "What are modalities?",                    _section_modalities),
    ("flat",     "The ♭ (flat) modality: discrete types",   _section_flat),
    ("shape",    "The ○ (shape) modality",                  _section_shape),
    ("cohesive", "Cohesive HoTT",                           _section_cohesive),
    ("apps",     "Applications: geometry and physics",      _section_applications),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Modal HoTT: Cohesion and Differential Geometry", width=70))
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
