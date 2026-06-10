#!/usr/bin/env python3
"""
Set Theory: ZFC, Ordinals, Cardinals, and the Move to Type Theory
=================================================================
The classical foundation of mathematics — and why type theory offers
a different (and in many ways better) foundation.
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


def _section_zfc():
    clear()
    print(box("ZFC: The Axioms of Set Theory"))
    print()
    print(wrap(
        "Zermelo-Fraenkel set theory with the Axiom of Choice (ZFC) is the "
        "standard foundation of classical mathematics. Everything is a set; "
        "there is one primitive relation: membership (∈)."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The ZFC axioms:'))}\n")
    axioms = [
        ("Extensionality", "∀A B. (∀x. x∈A ↔ x∈B) → A=B",
         "Sets equal iff they have the same members"),
        ("Empty set",      "∃∅. ∀x. x∉∅",
         "The empty set exists"),
        ("Pairing",        "∀a b. ∃P. ∀x. x∈P ↔ (x=a ∨ x=b)",
         "{a,b} exists for any a,b"),
        ("Union",          "∀F. ∃U. ∀x. x∈U ↔ ∃A∈F. x∈A",
         "Union of a family of sets exists"),
        ("Power set",      "∀A. ∃P. ∀x. x∈P ↔ x⊆A",
         "The set of all subsets exists"),
        ("Separation",     "∀A. ∀φ. ∃S. ∀x. x∈S ↔ (x∈A ∧ φ(x))",
         "{x∈A | φ(x)} exists (bounded comprehension)"),
        ("Replacement",    "∀A. if F is a function, ∃B. B=F[A]",
         "Image of a set under a function is a set"),
        ("Infinity",       "∃ω. ∅∈ω ∧ ∀n∈ω. n∪{n}∈ω",
         "An infinite set (ℕ) exists"),
        ("Foundation",     "∀A≠∅. ∃m∈A. m∩A=∅",
         "No infinite descending ∈-chains"),
        ("Choice (AC)",    "∀F. (∀A∈F. A≠∅) → ∃f. ∀A∈F. f(A)∈A",
         "Choice function exists for nonempty families"),
    ]
    for name, formula, note in axioms:
        print(f"  {bold(cyan(name)):22} {dim(note)}")
        print(f"    {dim(formula)}")
        print()
    input(bold("  Press Enter to continue... "))


def _section_ordinals():
    clear()
    print(box("Ordinal Numbers"))
    print()
    print(wrap(
        "Ordinals measure order type — they generalize 'first, second, third...' "
        "beyond the finite. An ordinal is a transitive set well-ordered by ∈. "
        "Every ordinal α = {β | β < α}, so ordinals ARE sets of smaller ordinals."
    ))
    print()
    print(f"  {cyan('0 = ∅')}")
    print(f"  {cyan('1 = {0} = {∅}')}")
    print(f"  {cyan('2 = {0,1} = {∅, {∅}}')}")
    print(f"  {cyan('3 = {0,1,2}')}")
    print(f"  {cyan('ω = {0,1,2,3,...}   (first infinite ordinal = ℕ)')}")
    print(f"  {cyan('ω+1 = ω ∪ {ω} = {0,1,2,...,ω}')}")
    print(f"  {cyan('ω·2 = ω+ω = {0,1,...,ω,ω+1,...}')}")
    print(f"  {cyan('ω² = ω·ω,    ωω = ε₀  (limit of ω, ωω, ωωω,...)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The ordinal hierarchy:'))}\n")

    ordinals = [
        ("0,1,2,...",   "finite ordinals — the natural numbers"),
        ("ω",           "first infinite ordinal; order type of ℕ"),
        ("ω+1, ω+2,…",  "successor ordinals after ω"),
        ("ω·2",         "two copies of ω placed end-to-end"),
        ("ω²",          "ω copies of ω"),
        ("ε₀",          "least fixed point of α ↦ ω^α; used in proof theory"),
        ("ω₁",          "first uncountable ordinal"),
        ("Ω (proper class)", "the class of ALL ordinals — too big to be a set"),
    ]
    for name, desc in ordinals:
        print(f"  {bold(yellow(name)):25} {desc}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Burali-Forti paradox:'))}\n")
    print(wrap(
        "The class of all ordinals Ω would itself be an ordinal (it is "
        "transitive and well-ordered). But then Ω ∈ Ω — contradiction with "
        "Foundation. So Ω is a proper class, not a set. This is why ZFC "
        "has Foundation and distinguishes sets from classes."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_cardinals():
    clear()
    print(box("Cardinal Numbers"))
    print()
    print(wrap(
        "Cardinals measure SIZE — they tell us how many elements a set has. "
        "Two sets have the same cardinality iff there is a bijection between them. "
        "Every set has a unique cardinality (assuming AC)."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The cardinal hierarchy:'))}\n")

    cardinals = [
        ("0,1,2,...,ℕ",  "finite cardinals"),
        ("ℵ₀ = |ℕ|",     "countably infinite — the smallest infinite cardinal"),
        ("ℵ₁",           "the next cardinal after ℵ₀ (first uncountable)"),
        ("ℵ₂, ℵ₃, ...",  "the cardinal sequence"),
        ("ℵ_ω",          "limit of all finite-subscript ℵs"),
        ("𝔠 = 2^ℵ₀ = |ℝ|","the cardinality of the continuum"),
    ]
    for name, desc in cardinals:
        print(f"  {bold(cyan(name)):22} {desc}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key cardinality results:'))}\n")

    results = [
        ("|ℕ| = |ℤ| = |ℚ|", "all countable; bijections exist"),
        ("|ℕ| < |ℝ|",        "Cantor diagonal argument"),
        ("|A| < |𝒫(A)|",     "Cantor theorem: power set is strictly bigger"),
        ("|ℝ| = |ℝ²| = |ℝⁿ|","all have cardinality 𝔠 (not ℵ₀ copies)"),
        ("GCH: 2^ℵ_α = ℵ_{α+1}", "Generalized CH — independent of ZFC"),
        ("CH: 2^ℵ₀ = ℵ₁",    "Continuum Hypothesis — independent of ZFC"),
    ]
    for result, note in results:
        print(f"  {bold(yellow(result)):35} {dim(note)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Independence:'))}\n")
    print(wrap(
        "The Continuum Hypothesis (CH) and its generalization are INDEPENDENT "
        "of ZFC: both CH and its negation are consistent with ZFC. "
        "This was proved by Godel (1938, CH consistent) and Cohen (1963, not-CH consistent). "
        "Similar independence results exist for many large cardinal axioms."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_choice():
    clear()
    print(box("The Axiom of Choice"))
    print()
    print(wrap(
        "The Axiom of Choice (AC) says: given any collection of nonempty sets, "
        "we can simultaneously choose one element from each. It seems obvious "
        "but has surprising consequences — and is not constructive."
    ))
    print()
    print(f"  {cyan('AC: ∀F. (∀A∈F. A≠∅) → ∃f:∪F. ∀A∈F. f(A)∈A')}")
    print()
    print(rule())
    print(f"\n  {bold(green('AC is equivalent to (in ZF):'))}\n")
    equivalents = [
        ("Zorn's Lemma",
         "Every chain-complete poset has a maximal element"),
        ("Well-ordering theorem",
         "Every set can be well-ordered"),
        ("Tychonoff theorem",
         "Product of compact spaces is compact"),
        ("Vector space bases",
         "Every vector space has a Hamel basis"),
        ("Ultrafilter lemma",
         "Every filter extends to an ultrafilter (weaker form)"),
    ]
    for name, desc in equivalents:
        print(f"  {bold(cyan(name))}")
        print(f"    {dim(desc)}")
        print()
    print(rule())
    print(f"\n  {bold(yellow('Consequences of AC:'))}\n")
    consequences = [
        (green("Good"),  "Every vector space has a basis"),
        (green("Good"),  "Countable union of countable sets is countable"),
        (red("Strange"), "Banach-Tarski: decompose a ball into two equal balls"),
        (red("Strange"), "Non-measurable subsets of ℝ exist (Vitali set)"),
        (red("Strange"), "There exist sets with no definable well-ordering"),
    ]
    for label, result in consequences:
        print(f"  {label}  {result}")
    print()
    print(wrap(
        "In type theory and HoTT, AC fails in general for the same reason "
        "LEM fails: both are non-constructive. But a weaker form — AC for "
        "sets with decidable equality — often suffices."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_vs_type_theory():
    clear()
    print(box("Set Theory vs. Type Theory"))
    print()
    print(wrap(
        "Both ZFC and type theory are foundations for mathematics, but they "
        "have fundamentally different philosophies."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Comparing the foundations:'))}\n")
    print(f"  {'Feature':28}  {'ZFC':22}  {'Type Theory'}")
    print(f"  {dim('─'*75)}")
    comparisons = [
        ("Primitive notion",        "set membership ∈",   "type judgments A:Type"),
        ("Everything is",           "a set",              "a term with a type"),
        ("Propositions",            "sets of proofs",     "types (Curry-Howard)"),
        ("Proof relevance",         "no (props extensional)", "yes (proofs matter)"),
        ("Computation",             "separate (eval)",    "built in (reduction)"),
        ("Consistency proof",       "needs a model",      "normalization theorem"),
        ("Excluded middle",         "provable (axiom)",   "independent"),
        ("Axiom of choice",         "often assumed",      "independent"),
        ("Higher-dimensional",      "not naturally",      "yes! (HoTT)"),
        ("Univalence",              "forced externally",  "axiom (HoTT)"),
    ]
    for feature, zfc, tt in comparisons:
        print(f"  {feature:28}  {dim(zfc):30}  {cyan(tt)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Why type theory for HoTT?'))}\n")
    print(wrap(
        "Set theory treats equality as a single flat notion. Type theory, "
        "via identity types, makes equality itself a mathematical object. "
        "In HoTT, paths (proofs of equality) can have non-trivial structure — "
        "this is the key that unlocks homotopy theory inside type theory."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("zfc",      "ZFC: the axioms of set theory",              _section_zfc),
    ("ordinals", "Ordinal numbers",                             _section_ordinals),
    ("cardinals","Cardinal numbers",                            _section_cardinals),
    ("choice",   "The axiom of choice",                        _section_choice),
    ("vs",       "Set theory vs. type theory",                 _section_vs_type_theory),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Set Theory: ZFC, Ordinals, Cardinals", width=70))
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
