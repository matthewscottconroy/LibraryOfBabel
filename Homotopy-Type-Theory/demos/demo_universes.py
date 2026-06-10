#!/usr/bin/env python3
"""
Universes: The Hierarchy 𝒰₀ : 𝒰₁ : 𝒰₂ : ⋯
=============================================
Why we need a tower of universes and what goes wrong without one.

A universe 𝒰 is a type whose elements are themselves types. Universes let
us state quantified statements about types ("for all types A, ..."). Without
a hierarchy, we get Girard's paradox — a type-theoretic version of Russell's.
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


def _section_why():
    clear()
    print(box("Why We Need Universes"))
    print()
    print(wrap(
        "In Martin-Löf Type Theory, every expression has a type. Natural numbers "
        "have type ℕ. Functions have function types. But what is the TYPE of ℕ itself? "
        "We need a 'type of types' — a universe."
    ))
    print()
    print(f"  {cyan('ℕ : 𝒰₀              -- ℕ is a small type')}")
    print(f"  {cyan('Bool : 𝒰₀           -- Bool is a small type')}")
    print(f"  {cyan('ℕ → ℕ : 𝒰₀         -- function types are small')}")
    print(f"  {cyan('𝒰₀ : 𝒰₁             -- the universe 𝒰₀ lives in 𝒰₁')}")
    print(f"  {cyan('𝒰₁ : 𝒰₂             -- 𝒰₁ lives in 𝒰₂')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Why we need universes:'))}\n")
    uses = [
        ("Polymorphism",
         "id : Π(A:𝒰). A → A   requires quantifying over 𝒰"),
        ("Type families",
         "Vec : ℕ → 𝒰           requires 𝒰 as a codomain"),
        ("Propositions as types",
         "isProp : 𝒰 → 𝒰₁      (the statement isProp A lives one level up)"),
        ("Induction on types",
         "codes for π₁(S¹): code : S¹ → 𝒰  requires a universe as target"),
        ("Large structures",
         "Group : 𝒰₁            the type of all groups is large"),
        ("Univalence",
         "ua : (A ≃ B) → (A = B)  requires A, B : 𝒰 for some universe 𝒰"),
    ]
    for name, example in uses:
        print(f"  {bold(cyan(name))}")
        print(f"    {dim(example)}")
        print()
    input(bold("  Press Enter to continue... "))


def _section_paradox():
    clear()
    print(box("Girard's Paradox: Why 𝒰 : 𝒰 Is Inconsistent"))
    print()
    print(wrap(
        "If we allowed 𝒰 : 𝒰 (a universe containing itself), the type theory "
        "would be INCONSISTENT — every type, including ⊥, would be inhabited. "
        "This is Girard's paradox, the type-theoretic version of Russell's paradox."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Russell: the set of all sets that do not contain themselves'))}\n")
    print(f"  {cyan('R := { x | x ∉ x }')}")
    print(f"  {cyan('R ∈ R  ↔  R ∉ R   -- contradiction')}")
    print()
    print(wrap(
        "In set theory: if such a set R existed, we get R∈R iff R∉R. "
        "Resolution: restrict to a hierarchy of sets (the Von Neumann universe)."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Girard: type-theoretic paradox via System U'))}\n")
    print(wrap(
        "In a type theory with 𝒰:𝒰, one can encode a form of impredicativity "
        "strong enough to derive a contradiction. The argument is complex but "
        "the intuition is:"
    ))
    print()
    print(f"  {dim('1. Define the type of all ordinals: Ord : 𝒰')}")
    print(f"  {dim('2. Since 𝒰:𝒰, Ord : 𝒰 can contain itself')}")
    step3 = "3. Construct a largest ordinal — which must be larger than itself"
    print(f"  {dim(step3)}")
    print(f"  {dim('4. Contradiction (Burali-Forti paradox in type theory)')}")
    print()
    print(wrap(
        "The resolution: impose a HIERARCHY. 𝒰₀ does not contain 𝒰₀ itself; "
        "𝒰₀ lives in 𝒰₁, which lives in 𝒰₂, etc. No universe contains itself."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Python analogy: sets and types'))}\n")
    print(wrap(
        "Python's type system is not stratified, so it allows paradoxical constructions:"
    ))
    print()
    print(f"  {cyan('type(int)   = <class type>')}")
    print(f"  {cyan('type(type)  = <class type>   # type contains itself!')}")
    print(f"  {cyan('type(object) = <class type>')}")
    print()
    import builtins
    print(f"  type(int)  = {type(int)}")
    print(f"  type(type) = {type(type)}")
    print(f"  type is an instance of type: {isinstance(type, type)}")
    print()
    print(wrap(
        "Python doesn't derive a contradiction from this because it's not a "
        "proof-relevant system. But in type theory, where types ARE proofs, "
        "this self-reference leads to inconsistency."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_hierarchy():
    clear()
    print(box("The Universe Hierarchy: 𝒰₀ : 𝒰₁ : 𝒰₂ : ⋯"))
    print()
    print(f"  {cyan('𝒰ₙ : 𝒰ₙ₊₁   for each n : ℕ')}")
    print(f"  {cyan('A : 𝒰ₙ  implies  A : 𝒰ₙ₊₁   (cumulativity)')}")
    print()
    print(wrap(
        "Cumulativity means smaller universes are included in larger ones. "
        "A type in 𝒰₀ automatically inhabits every 𝒰ₙ for n≥0."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('What lives at each level:'))}\n")

    levels = [
        ("𝒰₀", "small types",
         ["ℕ, Bool, 𝟙, 𝟘", "ℕ→ℕ, Bool→Bool", "Fin n, Vec n A",
          "S¹, S², all HITs defined from small types",
          "identity types a=b for a b : A : 𝒰₀"]),
        ("𝒰₁", "types of small structures",
         ["𝒰₀  (the universe of small types)",
          "isProp : 𝒰₀→𝒰₁  (propositional predicate on small types)",
          "isSet, isGroupoid  (higher h-level predicates)",
          "Group = Σ(G:𝒰₀). GroupStr(G)  (type of all small groups)",
          "Functor (between categories whose objects are small)"]),
        ("𝒰₂", "types of large structures",
         ["𝒰₁", "Category (large categories)", "Σ(G:𝒰₁). GroupStr(G)  (large groups)"]),
        ("𝒰ω", "the limit universe (if postulated)",
         ["Union of all 𝒰ₙ  (often not included — uses ω-rule)"]),
    ]

    for name, desc, contents in levels:
        print(f"  {bold(cyan(name))}  — {desc}")
        for item in contents:
            print(f"    {dim('·')} {item}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('Universe polymorphism:'))}\n")
    print(wrap(
        "Many definitions should work AT ANY LEVEL. For example, the identity "
        "function id should work for any type, regardless of its universe level."
    ))
    print()
    print(f"  {cyan('id : Π(ℓ:Level). Π(A:𝒰ℓ). A → A')}")
    print(f"  {cyan('id ℓ A a = a')}")
    print()
    print(wrap(
        "Universe-polymorphic definitions are parameterized by a LEVEL variable ℓ. "
        "Agda supports this with 'Level' and 'Set ℓ'. The HoTT book often writes "
        "'Type' to mean an arbitrary universe, leaving the level implicit."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_resizing():
    clear()
    print(box("Propositional Resizing"))
    print()
    print(wrap(
        "In the universe hierarchy, a proposition P might live in 𝒰₁ while we "
        "want to use it where 𝒰₀ is expected. PROPOSITIONAL RESIZING asserts "
        "that every proposition is equivalent to a small one:"
    ))
    print()
    print(f"  {cyan('PropResizing : ∀(P : 𝒰₁). isProp P → Σ(Q:𝒰₀). P ≃ Q')}")
    print()
    print(wrap(
        "This is not provable in HoTT but can be added as an axiom. It says: "
        "propositions (h-level -1 types) are 'small' — they fit in 𝒰₀ even "
        "if originally defined in a larger universe."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Why resizing matters:'))}\n")

    examples = [
        ("isProp A for A:𝒰₀",
         "isProp A : 𝒰₀  (without resizing, it's in 𝒰₁)",
         "With resizing: treat isProp as a small predicate."),
        ("Power sets",
         "𝒫(A) = A → 𝒰₀  (without resizing, 𝒫(A) : 𝒰₁)",
         "Resizing lets 𝒫(A) live in 𝒰₀, recovering classical power set."),
        ("Propositional truncation",
         "||A|| : 𝒰₀ when A : 𝒰₀  (standard; needs care otherwise)",
         "Truncation can raise universe level without resizing."),
        ("Classifying spaces",
         "π₁(S¹) = ℤ stated without universe issues",
         "With resizing: the path type (base=base) is equivalent to small ℤ."),
    ]

    for title, issue, resolution in examples:
        print(f"  {bold(cyan(title))}")
        print(f"    Issue:       {issue}")
        print(f"    Resolution:  {green(resolution)}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('Consistency:'))}\n")
    print(wrap(
        "Propositional resizing is consistent with HoTT (it holds in the "
        "simplicial set model). It is NOT derivable from univalence alone. "
        "Voevodsky conjectured (and it was verified) that adding resizing "
        "to HoTT gives a conservative extension — it proves no new theorems "
        "about concrete types like ℕ."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_large_small():
    clear()
    print(box("Large vs. Small: Structures in the Hierarchy"))
    print()
    print(wrap(
        "A key distinction in mathematics is between 'small' structures "
        "(defined within a fixed universe) and 'large' ones (ranging over all types). "
        "The universe hierarchy makes this precise."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Small vs. large groups:'))}\n")
    print(f"  {cyan('SmallGroup = Σ(G:𝒰₀). GroupStr(G)      -- in 𝒰₁')}")
    print(f"  {cyan('LargeGroup = Σ(G:𝒰₁). GroupStr(G)      -- in 𝒰₂')}")
    print()
    print(wrap(
        "The type of all small groups is a large structure (lives in 𝒰₁). "
        "The type of all large groups is even larger (lives in 𝒰₂). "
        "In classical category theory, this is the small/large category distinction."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Typical level requirements:'))}\n")

    table = [
        ("A : 𝒰₀",             "𝒰₀", "A is a small type"),
        ("A → B  (A B:𝒰₀)",   "𝒰₀", "Function types are closed"),
        ("Σ(x:A). P(x)",       "max(level A, level P)", "Sigma type"),
        ("𝒰₀",                 "𝒰₁", "Small universe"),
        ("isProp A",            "𝒰₁", "Without resizing"),
        ("Group",               "𝒰₁", "Type of small groups"),
        ("Category",            "𝒰₁", "Small-hom categories"),
        ("Functor Cat→Cat",     "𝒰₂", "Large functor category"),
        ("𝒰₁",                 "𝒰₂", "The next universe"),
    ]

    print(f"  {'Expression':35}  {'Lives in':12}  {'Note'}")
    print(f"  {dim('─'*65)}")
    for expr, level, note in table:
        print(f"  {cyan(expr):43}  {bold(yellow(level)):20}  {dim(note)}")

    print()
    print(rule())
    print(f"\n  {bold(yellow('Impredicativity (special case):'))}\n")
    print(wrap(
        "Some systems (Coq's Prop, System F) allow IMPREDICATIVE universes: "
        "quantifying over all types of the same universe. This avoids the "
        "hierarchy but requires careful restriction to avoid paradox. "
        "HoTT uses a PREDICATIVE hierarchy — each level is closed only "
        "under operations on SMALLER types."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("why",       "Why we need universes",                        _section_why),
    ("paradox",   "Girard's paradox: why 𝒰:𝒰 is inconsistent",  _section_paradox),
    ("hierarchy", "The hierarchy 𝒰₀:𝒰₁:𝒰₂:⋯",                  _section_hierarchy),
    ("resizing",  "Propositional resizing",                       _section_resizing),
    ("large",     "Large vs. small structures",                   _section_large_small),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Universes: The Hierarchy 𝒰₀ : 𝒰₁ : 𝒰₂ : ⋯", width=70))
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
