#!/usr/bin/env python3
"""
Simplicial Sets: The Combinatorial Model of Homotopy Theory
============================================================
Simplicial sets are the combinatorial machinery underlying both
classical homotopy theory and the models of HoTT.
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


def _section_simplex_category():
    clear()
    print(box("The Simplex Category Δ"))
    print()
    print(wrap(
        "The SIMPLEX CATEGORY Δ has as objects the finite nonempty ordinals "
        "[n] = {0,1,...,n} for n≥0, and as morphisms the order-preserving "
        "(weakly monotone) maps between them."
    ))
    print()
    print(f"  {cyan('Objects: [0]={0}, [1]={0,1}, [2]={0,1,2}, ...')}")
    print(f"  {cyan('Morphisms: order-preserving maps f:[m]→[n]  (f(i)≤f(j) when i≤j)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The generating morphisms:'))}\n")
    print(f"  {bold('Coface maps')}  {cyan('δⁱₙ:[n-1]→[n]  (skip i)')}")
    print(f"  {dim('  δ⁰₁: [0]→[1]: 0↦1')}")
    print(f"  {dim('  δ¹₁: [0]→[1]: 0↦0')}")
    print(f"  {dim('  δ⁰₂: [1]→[2]: 0↦1, 1↦2')}")
    print(f"  {dim('  δ¹₂: [1]→[2]: 0↦0, 1↦2  (skip 1)')}")
    print(f"  {dim('  δ²₂: [1]→[2]: 0↦0, 1↦1  (skip 2 = endpoint missing)')}")
    print()
    print(f"  {bold('Codegeneracy maps')}  {cyan('σⁱₙ:[n+1]→[n]  (repeat i)')}")
    print(f"  {dim('  σ⁰₁: [2]→[1]: 0↦0, 1↦0, 2↦1  (collapse 0 and 1)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Simplicial identities:'))}\n")
    print(f"  {cyan('δʲ ∘ δⁱ = δⁱ ∘ δʲ⁻¹   when i < j')}")
    print(f"  {cyan('σʲ ∘ σⁱ = σⁱ ∘ σʲ⁺¹   when i ≤ j')}")
    print(f"  {cyan('σʲ ∘ δⁱ = δⁱ ∘ σʲ⁻¹   when i < j')}")
    print(f"  {cyan('         = id            when i = j or i = j+1')}")
    print(f"  {cyan('         = δⁱ⁻¹ ∘ σʲ   when i > j+1')}")
    print()
    print(wrap(
        "These identities completely determine Δ. Every morphism in Δ "
        "factors uniquely as a surjection followed by an injection "
        "(codegeneracies then cofaces — the Eilenberg-Zilber factorization)."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_simplicial_sets():
    clear()
    print(box("Simplicial Sets"))
    print()
    print(wrap(
        "A SIMPLICIAL SET is a presheaf on Δ — a contravariant functor "
        "X:Δᵒᵖ→Set. It assigns to each [n] a set Xₙ of n-simplices, "
        "with face and degeneracy maps relating them."
    ))
    print()
    print(f"  {cyan('X : Δᵒᵖ → Set')}")
    print(f"  {cyan('Xₙ = X([n])  (the set of n-simplices)')}")
    print(f"  {cyan('dᵢ : Xₙ → Xₙ₋₁  (face maps, from cofaces δⁱ)')}")
    print(f"  {cyan('sᵢ : Xₙ → Xₙ₊₁  (degeneracy maps, from codegeneracies σⁱ)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Geometric intuition:'))}\n")
    print(f"  {dim('X₀ = vertices (0-simplices)')}")
    print(f"  {dim('X₁ = edges (1-simplices, each has two endpoints d₀,d₁:X₁→X₀)')}")
    print(f"  {dim('X₂ = triangles (2-simplices, with 3 edge faces d₀,d₁,d₂:X₂→X₁)')}")
    print(f"  {dim('X₃ = tetrahedra (4 triangle faces)')}")
    print(f"  {dim('degenerate: sᵢ(x) = constant simplex sitting at x')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key examples:'))}\n")
    examples = [
        ("Δⁿ",             "the representable: Δ([n],-) — the standard n-simplex"),
        ("∂Δⁿ",            "boundary of Δⁿ: all faces except the top one"),
        ("Λᵏₙ",            "n-horn: Δⁿ minus the k-th face and interior"),
        ("Nerve(C)",        "nerve of a category: n-simplices = chains of n composable morphisms"),
        ("Sing(X)",         "singular complex of a space: n-simplices = maps Δⁿ→X"),
        ("N(G)",            "classifying space of a group: nerve of B(G)"),
        ("Δ¹",             "the walking edge: vertices {0,1}, one edge 0→1"),
    ]
    for name, desc in examples:
        print(f"  {bold(cyan(name)):15} {dim(desc)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_kan_complexes():
    clear()
    print(box("Kan Complexes: The Homotopical Simplicial Sets"))
    print()
    print(wrap(
        "A KAN COMPLEX is a simplicial set satisfying the KAN FILLING "
        "CONDITION: every HORN (a simplex with one face missing) "
        "can be filled to a full simplex. Kan complexes are the "
        "'fibrant' objects — the ones with the right homotopy theory."
    ))
    print()
    print(f"  {cyan('X is a Kan complex iff:')}")
    print(f"  {cyan('  for all 0≤k≤n and any horn  Λᵏₙ → X')}")
    print(f"  {cyan('  there exists a filler      Δⁿ → X  extending it')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Horn filling intuitively:'))}\n")
    print(wrap(
        "A HORN Λᵏₙ is Δⁿ with the interior and the k-th face removed. "
        "Filling the horn means completing the shape:"
    ))
    print()
    print(f"  {dim('Λ¹₂: triangle with left edge missing — fill with the hypotenuse')}")
    print(f"  {dim('Λ⁰₂: triangle with bottom edge missing — fill it in')}")
    print()
    print(f"  {bold('Outer horns')} {dim('(k=0 or k=n):')}")
    print(f"  {dim('  Filling Λ⁰₁: given endpoint 1, find an edge ending at 1')}")
    print(f"  {dim('  = existence of pre-images; only Kan complexes do this')}")
    print()
    print(f"  {bold('Inner horns')} {dim('(0<k<n):')}")
    print(f"  {dim('  Filling Λ¹₂: given two composable edges, fill the triangle')}")
    print(f"  {dim('  = composition law; quasi-categories fill only inner horns')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Kan complexes as ∞-groupoids:'))}\n")
    corr = [
        ("vertices x:X₀",      "objects"),
        ("edges p:X₁",         "morphisms (paths)"),
        ("d₀(p)=x, d₁(p)=y",  "p goes from y to x (convention varies)"),
        ("triangle f:X₂",      "homotopy between two paths"),
        ("outer horn fill",    "inverses exist (all morphisms invertible)"),
        ("inner horn fill",    "composition exists"),
        ("uniqueness",         "not required — homotopy coherence suffices"),
    ]
    for sset, grpd in corr:
        print(f"  {cyan(sset):35} ↔  {dim(grpd)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_model_structure():
    clear()
    print(box("The Quillen Model Structure"))
    print()
    print(wrap(
        "QUILLEN (1967) put a MODEL STRUCTURE on simplicial sets (SSet) making "
        "it a model for homotopy theory. The model structure consists of three "
        "classes of maps satisfying lifting properties."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The three classes:'))}\n")
    print(f"  {bold(cyan('Cofibrations:'))}")
    print(f"    {dim('= levelwise injections (monomorphisms of simplicial sets)')}")
    print(f"    {dim('= boundary inclusions ∂Δⁿ ↪ Δⁿ generate all cofibrations')}")
    print()
    print(f"  {bold(cyan('Weak equivalences:'))}")
    print(f"    {dim('= maps inducing isomorphisms on ALL homotopy groups')}")
    print(f"    {dim('= geometric realization gives weak homotopy equivalence')}")
    print()
    print(f"  {bold(cyan('Fibrations (Kan fibrations):'))}")
    print(f"    {dim('= maps with the right lifting property against horn inclusions')}")
    print(f"    {dim('= fibers are Kan complexes; the fibrant replacement is Kan')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key facts:'))}\n")
    facts = [
        ("Fibrant objects",   "= Kan complexes (all objects are cofibrant)"),
        ("Ho(SSet)",          "= homotopy category ≃ CW complexes up to weak equiv"),
        ("Geometric real.",   "|·|:SSet → Top  is a Quillen equivalence"),
        ("Singular functor",  "Sing:Top → SSet  is right adjoint to |·|"),
        ("Quillen equiv.",    "SSet ≃ Top  (same homotopy theory!)"),
        ("Fibration seqs.",   "long exact sequence of homotopy groups"),
    ]
    for name, desc in facts:
        print(f"  {bold(yellow(name)):20} {dim(desc)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_model_of_hott():
    clear()
    print(box("Simplicial Sets as a Model of HoTT"))
    print()
    print(wrap(
        "VOEVODSKY (2006) discovered that SIMPLICIAL SETS provide a model "
        "for Homotopy Type Theory, including the UNIVALENCE AXIOM. "
        "This was the key insight that launched HoTT as a field."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The interpretation:'))}\n")
    print(f"  {'HoTT concept':30}  {'Simplicial sets'}")
    print(f"  {dim('─'*65)}")
    interp = [
        ("Type A",              "Kan complex A"),
        ("Term a:A",            "vertex a∈A₀"),
        ("Path p:a=b",          "edge p∈A₁ with d₁p=a, d₀p=b"),
        ("Type family P:A→Type","Kan fibration E→A"),
        ("Σ(x:A).P(x)",         "total space E (vertex set over A)"),
        ("Π(x:A).P(x)",         "section space of fibration"),
        ("ua:A≃B → A=B",        "path in the universe Kan complex 𝒰"),
        ("Universe 𝒰",          "the simplicial set of small Kan complexes"),
        ("Univalence",          "Voevodsky: fibers of 𝒰₁→𝒰₀ are Kan"),
        ("Identity type",       "path space fibration E^I ×_E E → E"),
    ]
    for hott, sset in interp:
        print(f"  {cyan(hott):38} {yellow(sset)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Why this was revolutionary:'))}\n")
    print(wrap(
        "Before Voevodsky, univalence was known to be CONSISTENT with type "
        "theory but no one had a MODEL. The simplicial set model proved "
        "consistency and showed that HoTT is not just formal play — it "
        "has a concrete mathematical interpretation in homotopy theory."
    ))
    print()
    print(wrap(
        "The model also showed WHY univalence is true: in spaces, two spaces "
        "are 'the same' iff they are homotopy equivalent. Univalence makes "
        "this into a TYPE-THEORETIC axiom."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("delta",   "The simplex category Δ",                _section_simplex_category),
    ("ssets",   "Simplicial sets: presheaves on Δ",      _section_simplicial_sets),
    ("kan",     "Kan complexes: ∞-groupoids",             _section_kan_complexes),
    ("model",   "The Quillen model structure",           _section_model_structure),
    ("hott",    "Simplicial sets as a model of HoTT",    _section_model_of_hott),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Simplicial Sets: Combinatorial Homotopy Theory", width=70))
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
