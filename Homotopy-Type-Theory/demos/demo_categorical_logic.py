#!/usr/bin/env python3
"""
Categorical Logic: CCCs, Toposes, and the Internal Language
============================================================
How category theory provides MODELS for type theory and logic —
and how each type theory has a natural categorical semantics.
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


def _section_ccc():
    clear()
    print(box("Cartesian Closed Categories (CCCs)"))
    print()
    print(wrap(
        "A CARTESIAN CLOSED CATEGORY (CCC) is a category with finite products "
        "AND internal hom (function objects). This is exactly the categorical "
        "model of the simply-typed lambda calculus."
    ))
    print()
    print(f"  {cyan('CCC = category with:')}")
    print(f"  {cyan('  terminal object 1  (models unit type 𝟙)')}")
    print(f"  {cyan('  products A × B     (models pair types / conjunction)')}")
    print(f"  {cyan('  exponentials Bᴬ    (models function types A → B)')}")
    print()
    print(f"  {dim('Adjunction: Hom(C×A, B) ≅ Hom(C, Bᴬ)  (currying)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The CCC / lambda calculus correspondence:'))}\n")
    print(f"  {'Type theory':30}  {'CCC'}")
    print(f"  {dim('─'*55)}")
    correspondence = [
        ("type A",              "object A"),
        ("term a:A",            "morphism 1 → A (global element)"),
        ("function type A→B",   "exponential Bᴬ"),
        ("application f a",     "eval: Bᴬ × A → B"),
        ("abstraction λx.t",    "curry(t): C → Bᴬ"),
        ("product type A×B",    "product A×B"),
        ("unit type 𝟙",         "terminal object 1"),
        ("substitution [a/x]t", "composition"),
        ("beta reduction",      "β: eval ∘ (curry(f)×id) = f"),
        ("eta reduction",       "η: curry(eval) = id"),
    ]
    for tt, cat in correspondence:
        print(f"  {cyan(tt):35} {yellow(cat)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Examples of CCCs:'))}\n")
    examples = [
        ("Set",             "the paradigm example"),
        ("Pos",             "posets with monotone maps — models intuitionistic logic"),
        ("[C, Set]",        "presheaf categories — always CCC"),
        ("FinSet",          "finite sets — bounded computation"),
        ("SSet (simplicial)","simplicial sets — models HoTT"),
        ("Homotopy types",  "∞-category of spaces (∞-CCC)"),
    ]
    for name, note in examples:
        print(f"  {bold(cyan(name)):25} {dim(note)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_lccc():
    clear()
    print(box("Locally Cartesian Closed Categories (LCCCs)"))
    print()
    print(wrap(
        "An LCCC is a category where every SLICE category C/A is cartesian "
        "closed. Slice categories C/A have objects = morphisms X→A into A, "
        "and morphisms = commuting triangles. LCCCs model dependent type theory."
    ))
    print()
    print(f"  {cyan('LCCC: each slice C/A is a CCC')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The LCCC / dependent type theory correspondence:'))}\n")
    print(f"  {'Dependent type theory':35}  {'LCCC'}")
    print(f"  {dim('─'*70)}")
    correspondence = [
        ("context Γ",                "object Γ in C"),
        ("type A in context Γ",      "morphism A → Γ  (a family over Γ)"),
        ("term a:A in context Γ",    "section Γ → A of the projection"),
        ("Σ(x:A). B(x)",            "composition A ∘ B  (dependent sum)"),
        ("Π(x:A). B(x)",            "right adjoint to pullback (Π_f)"),
        ("context extension Γ,x:A", "pullback along Γ → 1"),
        ("substitution f*",          "pullback functor C/Γ → C/Δ"),
        ("weakening",               "projection A×Γ → Γ"),
        ("identity type a=_A b",    "diagonal Δ:A → A×A  (needs homotopy)"),
    ]
    for tt, cat in correspondence:
        print(f"  {cyan(tt):40} {yellow(cat)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The key theorem:'))}\n")
    print(wrap(
        "Every LCCC with a natural numbers object models intensional "
        "Martin-Löf Type Theory (without univalence). Adding identity types "
        "with the full J-eliminator is subtle and requires the LCCC to be "
        "a homotopy-theoretic LCCC (each slice is a homotopy CCC)."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_fibrations():
    clear()
    print(box("Fibered Categories and Display Maps"))
    print()
    print(wrap(
        "A FIBRATION (Grothendieck fibration) is a functor p:E→B where "
        "morphisms in B can be LIFTED to morphisms in E. The fiber over "
        "an object b:B is the subcategory Eᵦ = p⁻¹(b). This is the "
        "categorical model of type families."
    ))
    print()
    print(f"  {cyan('p:E→B fibration iff for each f:a→b in B and e∈E_b,')}")
    print(f"  {cyan('  ∃ cartesian lifting: e′→e over f')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Types as fibrations:'))}\n")
    print(wrap(
        "In dependent type theory, a type family P:A→Type corresponds to "
        "a fibration Σ(a:A).P(a) → A. The fiber over a:A is P(a). "
        "Substitution (pullback) corresponds to change-of-base."
    ))
    print()
    print(f"  {cyan('type family P : A → Type')}")
    print(f"  {cyan('  ↔  fibration  (Σa.P(a)) → A')}")
    print(f"  {cyan('  ↔  display map  (in display map categories)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Grothendieck construction:'))}\n")
    print(wrap(
        "The Grothendieck construction turns a FUNCTOR F:Cᵒᵖ→Cat into a "
        "fibration ∫F → C, where ∫F has objects = pairs (c, x) with c:C "
        "and x:F(c). This unifies indexed families and fibrations."
    ))
    print()
    print(f"  {cyan('∫F: objects = (c, x) where c:Ob(C), x:F(c)')}")
    print(f"  {cyan('    morphisms (c,x)→(d,y) = Σ(f:c→d). F(f)(x)→y')}")
    print()
    print(wrap(
        "In HoTT, the Grothendieck construction is the TOTAL SPACE: "
        "∫P = Σ(a:A).P(a) for a type family P:A→Type. The univalence "
        "axiom makes this construction well-behaved up to equivalence."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_toposes():
    clear()
    print(box("Elementary Toposes"))
    print()
    print(wrap(
        "A TOPOS is a CCC with a subobject classifier Ω — an object that "
        "classifies subobjects the way 2={T,F} classifies subsets in Set. "
        "Every topos has an internal logic that is intuitionistic."
    ))
    print()
    print(f"  {cyan('Topos = CCC + subobject classifier Ω')}")
    print(f"  {cyan('  Ω has true:1→Ω  and for every mono m:A↪B,')}")
    print(f"  {cyan('  a unique χ_m:B→Ω  with m = χ_m⁻¹(true)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key examples of toposes:'))}\n")
    examples = [
        ("Set",             "Ω = 2 = {T,F}",                    "the canonical example"),
        ("[C, Set]",        "Ω = Hom(-, Ω_Set)",               "presheaf topos"),
        ("Sh(X)",           "sheaves on a space X",             "Ω = sheaf of opens"),
        ("Fin",             "finite sets",                       "boolean topos"),
        ("Eff",             "effective topos",                  "models of realizability"),
        ("SSet",            "simplicial sets",                  "models HoTT (∞-topos)"),
        ("Sch",             "Lawvere-Tierney topos variant",    "algebraic geometry"),
    ]
    for name, omega, note in examples:
        print(f"  {bold(cyan(name)):14} {dim(omega):32} {yellow(note)}")
    print()
    print(rule())
    print(f"\n  {bold(green('The internal logic of a topos:'))}\n")
    print(wrap(
        "In any topos, one can do logic INTERNALLY using the subobject "
        "classifier Ω as the object of truth values. The internal logic is "
        "HIGHER-ORDER intuitionistic logic:"
    ))
    print()
    logic = [
        ("propositions",  "subobjects A ↪ B  (i.e., monos)"),
        ("truth",         "true : 1 → Ω"),
        ("conjunction",   "∧ : Ω × Ω → Ω"),
        ("implication",   "⇒ : Ω × Ω → Ω"),
        ("universal ∀",   "∀_A : Ω^A → Ω  (right adjoint to pullback)"),
        ("existential ∃", "∃_A : Ω^A → Ω  (left adjoint to pullback)"),
    ]
    for item, meaning in logic:
        print(f"  {bold(yellow(item)):20} {dim(meaning)}")
    print()
    print(wrap(
        "The internal logic of Set is classical (LEM holds, Ω=2). "
        "The internal logic of a general topos is intuitionistic. "
        "The ∞-topos of spaces has HoTT as its internal language."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_internal_language():
    clear()
    print(box("The Internal Language Correspondence"))
    print()
    print(wrap(
        "Every category with enough structure has an INTERNAL LANGUAGE — "
        "a type theory whose models are exactly that class of categories. "
        "This correspondence is the central result of categorical logic."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The correspondence table:'))}\n")
    print(f"  {'Category':30}  {'Internal language'}")
    print(f"  {dim('─'*65)}")
    correspondence = [
        ("Cartesian category",        "type theory with ×, 1 (no →)"),
        ("CCC",                       "simply-typed lambda calculus"),
        ("LCCC",                      "Martin-Löf dependent type theory"),
        ("Topos",                     "higher-order intuitionistic logic"),
        ("Boolean topos",             "higher-order classical logic"),
        ("Heyting algebra (pos.)",    "propositional intuitionistic logic"),
        ("Boolean algebra (pos.)",    "propositional classical logic"),
        ("Linear category",           "intuitionistic linear logic"),
        ("∞-topos (Lurie)",           "homotopy type theory (HoTT)"),
        ("∞-LCCC + univalence",       "HoTT with univalence axiom"),
    ]
    for cat, lang in correspondence:
        print(f"  {bold(cyan(cat)):38} {yellow(lang)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The central insight:'))}\n")
    print(wrap(
        "Type theories and categories are EQUIVALENT descriptions of "
        "mathematical structure. Every theorem in type theory is a theorem "
        "about the corresponding category, and vice versa. This is why "
        "HoTT and ∞-topos theory are really the same subject."
    ))
    print()
    print(f"  {cyan('HoTT ↔ ∞-topos theory')}")
    print(f"  {dim('  (as internal language ↔ categorical model)')}")
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("ccc",     "Cartesian closed categories",          _section_ccc),
    ("lccc",    "LCCCs and dependent types",            _section_lccc),
    ("fib",     "Fibered categories",                   _section_fibrations),
    ("topos",   "Elementary toposes",                   _section_toposes),
    ("lang",    "The internal language correspondence", _section_internal_language),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Categorical Logic: Types, Toposes, Language", width=70))
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
