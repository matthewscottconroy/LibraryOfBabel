#!/usr/bin/env python3
"""
Simplicial Homotopy Type Theory: Directed Types
================================================
Riehl-Shulman simplicial HoTT adds DIRECTED structure to type theory
— types can have non-invertible morphisms, enabling synthetic ∞-category theory.
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


def _section_motivation():
    clear()
    print(box("The Problem: HoTT is Undirected"))
    print()
    print(wrap(
        "Standard HoTT has UNDIRECTED paths: p:a=b is automatically "
        "reversible (sym p:b=a). This models ∞-groupoids perfectly. "
        "But ∞-CATEGORIES have non-invertible morphisms — how do we "
        "add directionality to type theory?"
    ))
    print()
    print(f"  {bold('HoTT paths:')}  {cyan('p:a=b → sym p:b=a  (always reversible)')}")
    print(f"  {bold('Categories:')} {cyan('f:A→B  may NOT be reversible')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Why we want directed type theory:'))}\n")
    reasons = [
        ("∞-category theory",
         "Functors, natural transformations, adjunctions — all directed"),
        ("Algebraic K-theory",
         "K-groups come from directed structures on rings"),
        ("Higher algebra",
         "E_n-algebras, monoidal ∞-categories — need non-inv. morphisms"),
        ("Directed homotopy",
         "Concurrency theory: paths in time have a direction"),
        ("Synthetic Yoneda",
         "Yoneda for ∞-categories needs a directed notion of path"),
        ("Rezk completion",
         "Turning a precategory into a category — directed process"),
    ]
    for title, desc in reasons:
        print(f"  {bold(cyan(title))}")
        print(f"    {dim(desc)}")
        print()
    print(rule())
    print(f"\n  {bold(yellow('The solution: Simplicial HoTT (Riehl-Shulman 2017):'))}\n")
    print(wrap(
        "Add TWO interval types: the undirected interval 𝟚 (= Bool = {0,1}) "
        "and a DIRECTED interval 2 = {0 ≤ 1} (a poset, not a groupoid). "
        "Types are SIMPLICIAL TYPES — they have both path structure (from 𝟚) "
        "and morphism structure (from 2)."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_two_intervals():
    clear()
    print(box("Two Intervals: 𝟚 and 2"))
    print()
    print(wrap(
        "Simplicial HoTT has two interval objects with different characters."
    ))
    print()
    print(f"  {bold(cyan('𝟚'))} — the UNDIRECTED interval  (= Bool = {'{0, 1}'})")
    print(f"  {dim('  Both endpoints connected symmetrically')}")
    print(f"  {dim('  Functions out of 𝟚 give PATHS (reversible morphisms)')}")
    print(f"  {dim('  Hom_𝟚(0,1) ≅ Hom_𝟚(1,0) — direction does not matter')}")
    print()
    print(f"  {bold(cyan('2'))} — the DIRECTED interval  (= {'{0 ≤ 1}'}, walking arrow)")
    print(f"  {dim('  One endpoint 0 is before 1 — ordering matters')}")
    print(f"  {dim('  Functions out of 2 give MORPHISMS (possibly non-invertible)')}")
    print(f"  {dim('  Hom_2(0,1) ≠ Hom_2(1,0) — there is no map from 1 to 0')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key definitions:'))}\n")
    print(f"  {cyan('-- Undirected paths (as before):')}")
    print(f"  {cyan('path(A, a, b) := 𝟚 → A  such that 0 ↦ a, 1 ↦ b')}")
    print(f"  {cyan('a = b  (the identity type)')}")
    print()
    print(f"  {cyan('-- Directed morphisms (new):')}")
    print(f"  {cyan('hom(A, a, b) := 2 → A  such that 0 ↦ a, 1 ↦ b')}")
    print(f"  {cyan('a → b  (the morphism type — directed!)')}")
    print()
    print(wrap(
        "In an ∞-groupoid, every hom(A,a,b) is contractible (at most one morphism, "
        "and it is invertible). In a non-trivial ∞-category, hom(A,a,b) "
        "can be a complex space of morphisms."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_segal_types():
    clear()
    print(box("Segal Types: Unique Composition"))
    print()
    print(wrap(
        "A SEGAL TYPE is a type A where the composition of morphisms is "
        "UNIQUE UP TO CONTRACTIBILITY. This is the simplicial HoTT version "
        "of a category — objects are terms, morphisms are maps from 2."
    ))
    print()
    print(f"  {cyan('-- The Segal condition (spine inclusion is an equivalence):')}")
    print(f"  {cyan('A is Segal iff:')}")
    print(f"  {cyan('  for all f:a→b and g:b→c,')}")
    print(f"  {cyan('  the type of composites {{h:a→c | h agrees with f,g}} is contractible')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Segal condition via horn filling:'))}\n")
    print(wrap(
        "The Segal condition says: for every INNER HORN Λ¹₂ → A "
        "(two composable morphisms without the composite), there is a "
        "UNIQUE filling Δ² → A (the composite). This is exactly uniqueness "
        "of composition in a category!"
    ))
    print()
    print(f"  {cyan('  f:a→b,  g:b→c  ⊢  ∃! h:a→c  (the composite g∘f)')}")
    print()
    print(f"  {dim('(Unlike Kan complexes: INNER horns fill UNIQUELY; outer horns fill non-uniquely)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Examples of Segal types:'))}\n")
    examples = [
        ("Every type in HoTT",     "Segal with unique (trivial) composition via paths"),
        ("BG (classifying space)", "Segal type for a group G — one object, all morphisms"),
        ("Nerve of a category",    "the nerve N(C) is Segal"),
        ("Hom(A,B)",               "if B Segal → Hom(A,B) is Segal (function category)"),
        ("Disc(S)",                "discrete set S — only identity morphisms"),
    ]
    for name, desc in examples:
        print(f"  {bold(cyan(name)):30} {dim(desc)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_rezk_types():
    clear()
    print(box("Rezk Types: Complete Segal Types"))
    print()
    print(wrap(
        "A REZK TYPE (or complete Segal type) is a Segal type that also "
        "satisfies a COMPLETENESS condition: isomorphic objects are EQUAL. "
        "Rezk types model (∞,1)-categories where the objects form a space "
        "with the correct homotopy type."
    ))
    print()
    print(f"  {cyan('A is Rezk iff:')}")
    print(f"  {cyan('  A is Segal  AND')}")
    print(f"  {cyan('  for all a b : A, the map  (a = b) → (a ≅ b)  is an equivalence')}")
    print()
    print(f"  {dim('(a ≅ b = iso a b = type of invertible morphisms from a to b)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The Rezk completeness condition:'))}\n")
    print(wrap(
        "In a precategory, we can have isomorphic objects that are not equal. "
        "Rezk completeness (= univalence for categories) says: the ONLY reason "
        "two objects are isomorphic is that they ARE equal. This is the "
        "categorical analog of the univalence axiom."
    ))
    print()
    print(f"  {cyan('Univalence for types:    (A = B) ≃ (A ≃ B)')}")
    print(f"  {cyan('Rezk for Segal types:    (a = b) ≃ (a ≅ b)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The hierarchy of simplicial types:'))}\n")
    hierarchy = [
        ("Any type",      "no composition required"),
        ("Segal type",    "unique composition of morphisms"),
        ("Rezk type",     "Segal + isomorphic = equal"),
        ("∞-groupoid",    "Rezk where every morphism is invertible"),
        ("Set",           "Rezk where all morphisms are trivial"),
    ]
    for name, desc in hierarchy:
        print(f"  {bold(cyan(name)):20} {dim(desc)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_directed_yoneda():
    clear()
    print(box("The Directed Yoneda Lemma"))
    print()
    print(wrap(
        "One of the key results in simplicial HoTT is the SYNTHETIC YONEDA "
        "LEMMA for Segal types. It says: the type of natural transformations "
        "from a representable functor to F is equivalent to F at the "
        "representing object."
    ))
    print()
    print(f"  {bold(cyan('Synthetic Yoneda for Segal types A:'))}")
    print(f"  {cyan('For a Segal type A, a:A, and P:A→Type,')}")
    print(f"  {cyan('  (Π b:A. hom(A,a,b) → P b) ≃ P a')}")
    print()
    print(f"  {dim('  (The type of natural transformations from hom(a,-) to P)')}")
    print(f"  {dim('   is equivalent to P a = just the value at the representing point)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Consequences:'))}\n")
    consequences = [
        ("Representability",
         "F:Aᵒᵖ→Type representable iff F≃hom(-,a) for some a:A"),
        ("Codomain fibration",
         "the codomain fibration π₁:A^2→A is representable (by hom)"),
        ("Uniqueness of adjoints",
         "if F⊣G and F⊣H then G≃H  (adjoints are unique up to iso)"),
        ("Fully faithful functor",
         "f:A→B ff iff the induced hom map is an equivalence"),
        ("Limits as right adjoints",
         "the limit of a diagram is the right Kan extension along 1"),
    ]
    for name, desc in consequences:
        print(f"  {bold(cyan(name))}")
        print(f"    {dim(desc)}")
        print()
    print(rule())
    print(f"\n  {bold(yellow('Why simplicial HoTT matters:'))}\n")
    print(wrap(
        "Standard HoTT can only talk about ∞-groupoids (spaces). "
        "Simplicial HoTT allows SYNTHETIC ∞-CATEGORY THEORY — proving "
        "theorems about (∞,1)-categories using type-theoretic methods. "
        "This is essential for applications to algebraic geometry, "
        "representation theory, and higher algebra."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("motiv",   "The problem: HoTT is undirected",      _section_motivation),
    ("intervals","Two intervals: 𝟚 and 2",               _section_two_intervals),
    ("segal",   "Segal types: unique composition",       _section_segal_types),
    ("rezk",    "Rezk types: complete Segal types",      _section_rezk_types),
    ("yoneda",  "The directed Yoneda lemma",             _section_directed_yoneda),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Simplicial HoTT: Directed Type Theory", width=70))
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
