#!/usr/bin/env python3
"""
Category Theory: Categories, Functors, Yoneda, and Adjunctions
===============================================================
The language of modern mathematics — and the setting in which
HoTT finds its most natural models and applications.
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


def _section_categories():
    clear()
    print(box("Categories: Objects, Morphisms, Composition"))
    print()
    print(wrap(
        "A CATEGORY C consists of: objects (Ob C), morphisms (Hom(A,B) for "
        "each pair), an identity morphism idₐ:A→A, and composition "
        "(g∘f:A→C when f:A→B, g:B→C). These satisfy identity and "
        "associativity laws."
    ))
    print()
    print(f"  {cyan('Category laws:')}")
    print(f"  {cyan('  id ∘ f = f = f ∘ id         (identity)')}")
    print(f"  {cyan('  (h ∘ g) ∘ f = h ∘ (g ∘ f)  (associativity)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The zoo of categories:'))}\n")
    cats = [
        ("Set",    "sets",             "functions",              "all sets"),
        ("Grp",    "groups",           "homomorphisms",          "algebra"),
        ("Top",    "topological spaces","continuous maps",       "topology"),
        ("Vect_k", "vector spaces",    "linear maps",            "linear algebra"),
        ("Cat",    "small categories", "functors",               "meta-category"),
        ("Pos",    "posets",           "monotone maps",          "order theory"),
        ("1",      "one object ∗",     "only id",                "terminal category"),
        ("0",      "no objects",       "no morphisms",           "initial/empty"),
        ("2",      "two objects 0,1",  "one nonid: 0→1",         "walking morphism"),
        ("ω",      "objects ℕ",        "n→m iff n≤m",            "walking ω-chain"),
        ("Δ",      "finite ordinals",  "order-preserving maps",  "simplicial sets!"),
        ("HoTT",   "types",            "terms of function type", "π-pretopos"),
    ]
    print(f"  {'Category':12}  {'Objects':22}  {'Morphisms':22}  {'Context'}")
    print(f"  {dim('─'*75)}")
    for name, obs, mor, ctx in cats:
        print(f"  {bold(cyan(name)):20} {obs:28} {dim(mor):28} {dim(ctx)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Isomorphism in a category:'))}\n")
    print(f"  {cyan('f:A→B is iso iff ∃g:B→A. g∘f=idₐ ∧ f∘g=id_B')}")
    print()
    print(wrap(
        "In Set: isomorphism = bijection. In Grp: isomorphism = bijective "
        "homomorphism. In Top: isomorphism = homeomorphism. In HoTT: "
        "isomorphism between types = equivalence ≃."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_functors():
    clear()
    print(box("Functors and Natural Transformations"))
    print()
    print(wrap(
        "A FUNCTOR F:C→D maps objects to objects and morphisms to morphisms, "
        "preserving identity and composition. Functors are the structure-"
        "preserving maps between categories."
    ))
    print()
    print(f"  {cyan('Functor F:C→D:')}")
    print(f"  {cyan('  F(idₐ) = id_F(A)')}")
    print(f"  {cyan('  F(g∘f) = F(g) ∘ F(f)')}")
    print()
    print(f"  {bold('Covariant:')}")
    print(f"  {cyan('  f:A→B in C  →  F(f):F(A)→F(B) in D  (same direction)')}")
    print(f"  {bold('Contravariant:')}")
    print(f"  {cyan('  f:A→B in C  →  F(f):F(B)→F(A) in D  (flipped)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Examples of functors:'))}\n")
    functors = [
        ("Forgetful",   "Grp → Set",       "forget group structure"),
        ("Free",        "Set → Grp",        "free group on a set"),
        ("π₁",          "Top₊ → Grp",       "fundamental group (covariant)"),
        ("Hom(A,-)",    "C → Set",          "representable functor (covariant)"),
        ("Hom(-,A)",    "Cᵒᵖ → Set",        "representable functor (contra)"),
        ("Π₀",          "Top → Set",        "connected components"),
        ("𝒫",           "Set → Set",        "power set (covariant)"),
        ("H*(-, ℤ)",     "Topᵒᵖ → Ab",       "cohomology (contravariant)"),
        ("Identity",    "C → C",            "id_C; morphisms map to themselves"),
        ("Constant",    "C → D",            "maps everything to one object in D"),
    ]
    for name, sig, note in functors:
        print(f"  {bold(cyan(name)):16} {yellow(sig):22} {dim(note)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Natural transformations:'))}\n")
    print(f"  {cyan('α: F ⟹ G  is a family αₐ:F(A)→G(A) for each A:Ob(C)')}")
    print(f"  {dim('  such that for all f:A→B: αB ∘ F(f) = G(f) ∘ αA')}")
    print(f"  {dim('  (naturality square commutes)')}")
    print()
    print(wrap(
        "Natural transformations are morphisms between functors. They make Cat "
        "(the category of small categories) into a 2-category: objects = "
        "categories, 1-morphisms = functors, 2-morphisms = natural transformations."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_yoneda():
    clear()
    print(box("The Yoneda Lemma"))
    print()
    print(wrap(
        "The Yoneda lemma is arguably the most important theorem in category "
        "theory. It says: an object A is completely determined by the functor "
        "it represents — by how other objects map INTO it."
    ))
    print()
    print(f"  {bold(cyan('Yoneda lemma:'))}")
    print(f"  {cyan('For any F:C→Set and A:Ob(C):')}")
    print(f"  {cyan('  Nat(Hom(A,-), F)  ≅  F(A)')}")
    print(f"  {dim('  (natural transformations from the Hom functor to F')}")
    print(f"  {dim('   biject with elements of F(A))')}")
    print()
    print(rule())
    print(f"\n  {bold(green('What Yoneda means:'))}\n")
    print(wrap(
        "The bijection sends α:Hom(A,-)⟹F to α_A(id_A) ∈ F(A). "
        "This is natural in both A and F. The key consequences:"
    ))
    print()
    consequences = [
        ("Yoneda embedding",
         "Y: C → [Cᵒᵖ, Set]  given by A ↦ Hom(-,A)  is FULLY FAITHFUL",
         "Two objects are isomorphic iff their Hom functors are naturally iso"),
        ("Representability",
         "F:Cᵒᵖ→Set is representable iff F≃Hom(-,A) for some A",
         "Finding A is finding a UNIVERSAL ELEMENT of F"),
        ("Limits as Hom",
         "The limit of F is the object representing Nat(Δ(-), F)",
         "Limit is universal among cones — a Yoneda instance"),
        ("HoTT connection",
         "Types represent 'generalized elements'; univalence is Yoneda for types",
         "A ≃ B iff Hom(A,-) ≃ Hom(B,-) as functors"),
    ]
    for name, statement, note in consequences:
        print(f"  {bold(cyan(name))}")
        print(f"    {statement}")
        print(f"    {dim(note)}")
        print()
    input(bold("  Press Enter to continue... "))


def _section_limits():
    clear()
    print(box("Limits and Colimits"))
    print()
    print(wrap(
        "LIMITS and COLIMITS are the universal constructions in category theory. "
        "They generalize products, pullbacks, equalizers, and their duals — "
        "the raw material from which all constructions are built."
    ))
    print()
    print(f"  {cyan('Limit of F:J→C = universal cone over F')}")
    print(f"  {cyan('Colimit of F:J→C = universal cocone under F')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The menagerie of limits and colimits:'))}\n")
    print(f"  {'Construction':20}  {'As Limit':25}  {'As Colimit'}")
    print(f"  {dim('─'*68)}")
    constructions = [
        ("Terminal/Initial",   "limit over empty J",       "colimit over empty J"),
        ("Product A×B",        "limit over discrete {A,B}","—"),
        ("Coproduct A+B",      "—",                        "colimit over discrete {A,B}"),
        ("Equalizer",          "limit over A⇉B",           "—"),
        ("Coequalizer",        "—",                        "colimit over A⇉B"),
        ("Pullback",           "limit over A→C←B",         "—"),
        ("Pushout",            "—",                        "colimit over A←C→B"),
        ("ω-limit",            "limit over ℕ-chain",       "—"),
        ("Colimit over all",   "—",                        "= Σ-type in HoTT"),
        ("Dependent sum",      "—",                        "Σ(x:A).B(x) is a colimit"),
    ]
    for name, lim, colim in constructions:
        print(f"  {bold(yellow(name)):28} {dim(lim):32} {dim(colim)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('In HoTT:'))}\n")
    print(wrap(
        "HoTT has all HOMOTOPY limits and colimits. The pushout HIT is the "
        "homotopy colimit of A ← C → B. Identity types give homotopy "
        "pullbacks (path spaces). The ∞-category structure of HoTT naturally "
        "has all these constructions."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_adjunctions():
    clear()
    print(box("Adjunctions and Monads"))
    print()
    print(wrap(
        "An ADJUNCTION F ⊣ G between categories C and D is a natural "
        "bijection between morphisms: Hom_D(F(A), B) ≅ Hom_C(A, G(B)). "
        "Adjunctions are everywhere in mathematics."
    ))
    print()
    print(f"  {cyan('F ⊣ G  iff  Hom_D(FA,B) ≅ Hom_C(A,GB)  natural in A,B')}")
    print(f"  {dim('  F: C→D is left adjoint,  G: D→C is right adjoint')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Ubiquitous adjunctions:'))}\n")
    adjunctions = [
        ("Free ⊣ Forget",      "Free groups ⊣ underlying set",   "Hom_Grp(F(S),G) ≅ Hom_Set(S,|G|)"),
        ("× ⊣ →",              "A×- ⊣ A→-  (currying)",          "Hom(A×B,C) ≅ Hom(B, A→C)"),
        ("Σ ⊣ reindex",        "dependent sum ⊣ precompose",      "Σᶠ ⊣ f* in slice categories"),
        ("Π ⊣ reindex",        "dependent product ⊣ precompose",  "f* ⊣ Πᶠ in slice categories"),
        ("Lan ⊣ precompose",   "left Kan extension",              "Lan_F G ⊣ precompose by F"),
        ("Σ∞ ⊣ Ω∞",            "stabilization ⊣ infinite loop",  "stable homotopy theory"),
        ("Disc ⊣ Γ",           "discrete space ⊣ global sections","cohesive HoTT (♭ ⊣ ♯)"),
        ("|-| ⊣ sing",         "geometric realization ⊣ singular","simplicial sets ↔ spaces"),
    ]
    for name, instance, formula in adjunctions:
        print(f"  {bold(cyan(name)):22} {yellow(instance)}")
        print(f"    {dim(formula)}")
        print()
    print(rule())
    print(f"\n  {bold(green('Monads from adjunctions:'))}\n")
    print(wrap(
        "Every adjunction F ⊣ G gives a MONAD T = G∘F on C with unit η:id⟹GF "
        "and multiplication μ:GFGF⟹GF. Monads are used in functional "
        "programming (Maybe, IO, List) and model computational effects."
    ))
    print()
    monads = [
        ("Maybe",     "from ⊤⊔- ⊣ forget",     "partial computation"),
        ("List",      "from Free ⊣ forget",      "non-determinism"),
        ("State S",   "from S×- ⊣ S→-",         "stateful computation"),
        ("Powerset",  "from 𝒫 ⊣ membership",    "nondeterminism classically"),
        ("||·||_n",   "truncation modality",      "propositional/set truncation"),
    ]
    for name, origin, use in monads:
        print(f"  {bold(yellow(name)):14} {dim(origin):28} {cyan(use)}")
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("cats",      "Categories: objects, morphisms, composition",  _section_categories),
    ("functors",  "Functors and natural transformations",          _section_functors),
    ("yoneda",    "The Yoneda lemma",                             _section_yoneda),
    ("limits",    "Limits and colimits",                          _section_limits),
    ("adj",       "Adjunctions and monads",                       _section_adjunctions),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Category Theory: The Language of Structure", width=70))
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
