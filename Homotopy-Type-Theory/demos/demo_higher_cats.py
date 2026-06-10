#!/usr/bin/env python3
"""
Higher Category Theory: 2-Categories, ∞-Groupoids, and HoTT
============================================================
How the ladder of higher morphisms connects classical category theory
to homotopy theory and the foundations of HoTT.
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


def _section_2cats():
    clear()
    print(box("2-Categories and Bicategories"))
    print()
    print(wrap(
        "A 2-CATEGORY has objects, 1-morphisms (between objects), and "
        "2-morphisms (between 1-morphisms). There are two compositions: "
        "horizontal (◦) along objects and vertical (·) along 1-morphisms."
    ))
    print()
    print(f"  {cyan('2-category data:')}")
    print(f"  {cyan('  Objects:      A, B, C, ...')}")
    print(f"  {cyan('  1-morphisms:  f:A→B  (arrows between objects)')}")
    print(f"  {cyan('  2-morphisms:  α:f⟹g  (arrows between arrows)')}")
    print()
    print(f"  {dim('Vertical composition:   α:f⟹g,  β:g⟹h  →  β·α:f⟹h')}")
    print(f"  {dim('Horizontal composition: α:f⟹g (A→B),  β:h⟹k (B→C)  →  β◦α:h∘f⟹k∘g')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Examples of 2-categories:'))}\n")
    examples = [
        ("Cat",         "objects=cats, 1-mor=functors, 2-mor=natural transformations"),
        ("Top",         "objects=spaces, 1-mor=maps, 2-mor=homotopies"),
        ("Monoidal",    "a monoidal category = 2-category with one object"),
        ("Spans(Set)",  "1-mor=spans A←S→B, 2-mor=span maps"),
        ("Adj",         "objects=cats, 1-mor=adjunctions, 2-mor=compatible nattrans"),
        ("B²G",         "a 2-group G = 2-category with one object and one 1-morphism"),
    ]
    for name, desc in examples:
        print(f"  {bold(cyan(name)):18} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Bicategories (weak 2-categories):'))}\n")
    print(wrap(
        "In a BICATEGORY, associativity and unit laws hold only UP TO "
        "coherent 2-isomorphisms (not strictly). This is more common in "
        "practice — most naturally occurring 2-categories are bicategories."
    ))
    print()
    print(f"  {cyan('(f ∘ g) ∘ h ≅ f ∘ (g ∘ h)  via associator α_{f,g,h}')}")
    print(f"  {cyan('id ∘ f ≅ f ≅ f ∘ id          via unitors λ_f, ρ_f')}")
    print(f"  {dim('  with coherence axioms (pentagon, triangle)')}")
    print()
    print(wrap(
        "In HoTT: a type A is a 1-groupoid if the identity types a=_A b are "
        "sets. It is a (weak) 2-groupoid if the identity types are groupoids. "
        "All types in HoTT are ∞-groupoids (= ∞-types)."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_homotopy_hypothesis():
    clear()
    print(box("The Homotopy Hypothesis"))
    print()
    print(wrap(
        "GROTHENDIECK'S HOMOTOPY HYPOTHESIS (1983): ∞-groupoids are the "
        "same as homotopy types (topological spaces up to weak homotopy "
        "equivalence). This is the bridge between higher category theory "
        "and algebraic topology."
    ))
    print()
    print(f"  {bold(cyan('Homotopy types  ≃  ∞-groupoids'))}")
    print()
    print(rule())
    print(f"\n  {bold(green('The correspondence:'))}\n")
    corr = [
        ("space X",           "∞-groupoid Π_∞(X)"),
        ("points x:X",        "objects of Π_∞(X)"),
        ("paths γ:x=y",       "1-morphisms"),
        ("homotopies H:γ=δ",  "2-morphisms"),
        ("homotopies of homotopies", "3-morphisms"),
        ("π_n(X,x)",          "n-th homotopy group (automorphisms at level n)"),
        ("weak homotopy equiv","equivalence of ∞-groupoids"),
        ("fibration",         "Cartesian fibration of ∞-groupoids"),
        ("homotopy type",     "∞-groupoid up to equivalence"),
    ]
    for top, cat in corr:
        print(f"  {cyan(top):35} ↔  {yellow(cat)}")
    print()
    print(rule())
    print(f"\n  {bold(green('In HoTT:'))}\n")
    print(wrap(
        "The homotopy hypothesis is BUILT IN to HoTT: types ARE ∞-groupoids. "
        "The identity type a=_A b is the morphism type. This is not a theorem "
        "to be proved but an axiom of the foundation — the very meaning of '='."
    ))
    print()
    print(f"  {cyan('A type  ≡  an ∞-groupoid')}")
    print(f"  {cyan('a = b   ≡  a 1-morphism (path) from a to b')}")
    print(f"  {cyan('p = q   ≡  a 2-morphism (homotopy) between paths')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_infinity_cats():
    clear()
    print(box("(∞,1)-Categories"))
    print()
    print(wrap(
        "An (∞,1)-CATEGORY is an ∞-category where all k-morphisms for k≥2 "
        "are invertible. It has objects and morphisms, but also higher "
        "homotopies between morphisms — without a strict composition law."
    ))
    print()
    print(f"  {cyan('(n,k)-category: n-morphisms exist, k-morphisms are invertible')}")
    print(f"  {cyan('  (∞,0)-category = ∞-groupoid (all morphisms invertible)')}")
    print(f"  {cyan('  (∞,1)-category = ∞-category (all 2+ morphisms invertible)')}")
    print(f"  {cyan('  (1,1)-category = ordinary category')}")
    print(f"  {cyan('  (2,1)-category = (2,1)-category (groupoidal 2-cells)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Models of (∞,1)-categories:'))}\n")
    models = [
        ("Quasi-categories",    "simplicial sets where all inner horns fill"),
        ("Complete Segal spaces","simplicial spaces satisfying Segal and completeness"),
        ("Segal categories",    "simplicial spaces with discrete objects"),
        ("A∞-categories",       "categories with associative up to all higher homotopies"),
        ("Kan-enriched cats",   "categories enriched in Kan complexes"),
        ("Relative categories", "categories with weak equivalences"),
    ]
    for name, desc in models:
        print(f"  {bold(cyan(name)):25} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key (∞,1)-categories:'))}\n")
    key_cats = [
        ("Spaces",        "the (∞,1)-category of homotopy types"),
        ("Spectra",       "stable homotopy theory — infinite loop spaces"),
        ("Chain complexes","dg-categories — algebra and geometry"),
        ("∞-Groupoids",   "spaces again, via homotopy hypothesis"),
        ("∞-Toposes",     "(∞,1)-categories with topos structure"),
        ("Type theory",   "HoTT is the internal language of ∞-toposes"),
    ]
    for name, desc in key_cats:
        print(f"  {bold(cyan(name)):20} {dim(desc)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_infinity_groupoids():
    clear()
    print(box("∞-Groupoids and HoTT"))
    print()
    print(wrap(
        "An ∞-GROUPOID is an (∞,1)-category where every morphism at every "
        "level is invertible. These are exactly the homotopy types. The "
        "∞-groupoid structure of a type A in HoTT:"
    ))
    print()
    print(f"  {cyan('Level 0: objects = terms a:A')}")
    print(f"  {cyan('Level 1: paths   = p:a=_A b')}")
    print(f"  {cyan('Level 2: 2-paths = α:p=_{a=b} q')}")
    print(f"  {cyan('Level n: n-paths = p:(n-1)-paths in A')}")
    print()
    print(f"  {dim('Composition: transitivity (path concatenation)')}")
    print(f"  {dim('Inverses:    symmetry (reverse path)')}")
    print(f"  {dim('Identities:  reflexivity (refl : a = a)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('h-level = truncation level:'))}\n")
    hlevels = [
        ("-2", "contractible",   "all paths trivial; one point up to equivalence"),
        ("-1", "proposition",    "at most one element; paths unique"),
        ("0",  "set",            "paths are propositions; no higher structure"),
        ("1",  "groupoid",       "paths form sets; 2-paths are propositions"),
        ("2",  "2-groupoid",     "paths form groupoids; and so on"),
        ("n",  "n-groupoid",     "higher structure up to level n"),
        ("∞",  "∞-groupoid",     "all levels nontrivial — most types in HoTT"),
    ]
    print(f"  {'h-level':10}  {'Name':16}  {'Meaning'}")
    print(f"  {dim('─'*65)}")
    for lev, name, meaning in hlevels:
        print(f"  {bold(yellow(lev)):18} {cyan(name):24} {dim(meaning)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_higher_in_hott():
    clear()
    print(box("Higher Categories in HoTT"))
    print()
    print(wrap(
        "HoTT can DEFINE higher categorical structures using its type theory. "
        "The key examples show how the theory handles its own meta-structure."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Types as ∞-groupoids (automatic in HoTT):'))}\n")
    print(f"  {cyan('Every type A is automatically an ∞-groupoid:')}")
    print(f"  {cyan('  refl : a = a           (identity)')}")
    print(f"  {cyan('  sym  : a=b → b=a       (inverse)')}")
    print(f"  {cyan('  trans: a=b → b=c → a=c (composition)')}")
    print(f"  {cyan('  Coherences proved using J eliminator')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Defining (∞,1)-categories in HoTT:'))}\n")
    print(wrap(
        "A PRECATEGORY in HoTT has: Ob:Type, Hom:Ob×Ob→Set, composition, "
        "identities, and laws. A CATEGORY additionally requires the "
        "Rezk completeness condition: isomorphic objects are equal."
    ))
    print()
    print(f"  {cyan('Precategory = (Ob, Hom:Ob×Ob→Set, ∘, id, laws)')}")
    print(f"  {cyan('Category    = Precategory + (A≅B → A=B)')}")
    print(f"  {dim('  (Rezk completeness / univalence for categories)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The (∞,1)-category of types:'))}\n")
    print(wrap(
        "In HoTT, the universe 𝒰 itself forms an (∞,1)-category where: "
        "objects = types, morphisms = functions, 2-morphisms = homotopies, etc. "
        "Univalence makes this a COMPLETE Segal space — the definitive "
        "model of homotopy type theory."
    ))
    print()
    print(f"  {cyan('Type    : 𝒰')}")
    print(f"  {cyan('(A = B) : 𝒰  ≃  (A ≃ B)   (by univalence)')}")
    print(f"  {dim('  The identity types of 𝒰 are equivalences of types')}")
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("2cats",   "2-categories and bicategories",               _section_2cats),
    ("hyp",     "The homotopy hypothesis",                      _section_homotopy_hypothesis),
    ("inf1",    "(∞,1)-categories",                             _section_infinity_cats),
    ("infgrp",  "∞-groupoids and h-levels",                    _section_infinity_groupoids),
    ("hott",    "Higher categories in HoTT",                   _section_higher_in_hott),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Higher Category Theory", width=70))
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
