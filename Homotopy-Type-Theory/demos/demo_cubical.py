#!/usr/bin/env python3
"""
Cubical Type Theory: Paths as Functions
=========================================
The interval 𝕀, paths as i:𝕀 → A, hcomp, and why cubical makes ua computable.

Book HoTT adds univalence as an axiom — true but with no computation rule.
Cubical Type Theory (CTT) adds the interval type 𝕀 with endpoints 0,1:
paths are literally functions 𝕀 → A, and univalence becomes a theorem
with a COMPUTATIONAL INTERPRETATION.
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


# ── Section 1: The interval type ──────────────────────────────────────────────

def _section_interval():
    clear()
    print(box("The Interval Type 𝕀: The Foundation of Cubical"))
    print()
    print(wrap(
        "In Book HoTT, paths a = b are a DEFINED type (the identity type). "
        "In Cubical Type Theory, paths are FUNCTIONS from an interval type 𝕀:"
    ))
    print()
    print(f"  {cyan('𝕀 : Type')}")
    print(f"  {cyan('0, 1 : 𝕀         -- two endpoints')}")
    print(f"  {cyan('i ∧ j, i ∨ j, ~ i : 𝕀  -- meets, joins, complement')}")
    print()
    print(f"  {bold('Path type:')}")
    print(f"  {cyan('Path A a b = (i : 𝕀) → A [ i=0 ↦ a, i=1 ↦ b ]')}")
    print()
    print(wrap(
        "A path from a to b in A is a function p : 𝕀 → A such that p(0) = a "
        "definitionally and p(1) = b definitionally. The bracket notation "
        "[ i=0 ↦ a ] is a FACE RESTRICTION: the function must equal a when i=0."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Reflexivity:'))}\n")
    print(f"  {cyan('refl_a : Path A a a')}")
    print(f"  {cyan('refl_a = λi. a           -- constant function')}")
    print()
    print(f"  {dim('refl_a(0) = a  ✓  (definitionally)')}")
    print(f"  {dim('refl_a(1) = a  ✓  (definitionally)')}")
    print()
    print(f"  {bold(green('Symmetry (path inversion):'))}\n")
    print(f"  {cyan('symm : Path A a b → Path A b a')}")
    print(f"  {cyan('symm p = λi. p (~ i)     -- reverse the interval')}")
    print()
    print(f"  {dim('symm(p)(0) = p(~0) = p(1) = b  ✓')}")
    print(f"  {dim('symm(p)(1) = p(~1) = p(0) = a  ✓')}")
    print()
    print(f"  {bold(green('Transitivity (path composition):'))}\n")
    print(f"  {cyan('comp : Path A a b → Path A b c → Path A a c')}")
    print(f"  {cyan('comp p q = λi. hcomp [i=0 ↦ a, i=1 ↦ p·q] (p (i ∨ 0))')}")
    print(f"  {dim('(simplified — actual cubical composition uses hcomp)')}")
    print()
    print(wrap(
        "The key advantage: refl, symm, and comp all have COMPUTATION RULES. "
        "symm(symm p) = p definitionally. This is not the case in Book HoTT."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 2: The right-unit law definitionally ──────────────────────────────

def _section_right_unit():
    clear()
    print(box("The Right-Unit Law: p · refl = p"))
    print()
    print(wrap(
        "In demo_groupoid_laws.py, we saw that in Book HoTT, the right unit law "
        "p · refl = p holds PROPOSITIONALLY but NOT DEFINITIONALLY. This is because "
        "path concatenation is defined by induction on the FIRST path, so the "
        "case p = refl works trivially, but for general p it requires a proof."
    ))
    print()
    print(rule())
    print(f"\n  {bold(red('Book HoTT (propositional only):'))}\n")
    print(f"  {cyan('_·refl : (p : a = b) → p · refl = p')}")
    print(f"  {cyan('(refl · refl) = refl   -- definitionally ✓')}")
    print(f"  {cyan('(p · refl) = p         -- requires path induction ✗ (not definitional)')}")
    print()
    print(wrap(
        "The asymmetry arises because the eliminator for the identity type "
        "eliminates on the LEFT endpoint: J eliminates when a = b is refl, "
        "where a is the LEFT endpoint. Concatenation defined via J is 'left-biased'."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Cubical HoTT (definitional):'))}\n")
    print(f"  {cyan('p : Path A a b  (a function 𝕀 → A)')}")
    print(f"  {cyan('refl_b : Path A b b  (the constant function at b)')}")
    print()
    print(f"  {cyan('comp p refl_b = λi. hcomp [i=0↦a, i=1↦b] (p i)')}")
    print(f"  {cyan('             = λi. p i  (hcomp with trivial system reduces)')}")
    print(f"  {cyan('             = p        (definitionally)')}")
    print()
    print(wrap(
        "In cubical, composition uses hcomp (homogeneous composition), which "
        "has a built-in reduction rule: when the 'cap' of the composition is "
        "already a face of the cube, hcomp reduces definitionally. The right "
        "unit follows definitionally — no proof needed."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Other definitional equalities in cubical:'))}\n")
    definitional = [
        ("Involution of inversion", "symm (symm p) = p"),
        ("Left unit",               "refl · p = p"),
        ("Right unit",              "p · refl = p  ← the key one"),
        ("Double negation",         "~ (~i) = i  (for the interval)"),
        ("Demorgan's laws",         "~(i ∧ j) = (~i) ∨ (~j)"),
        ("ua reduces",              "transport id (ua e) a = e a"),
        ("funext computes",         "happly (funext h) x = h x"),
    ]
    for name, eq in definitional:
        marker = bold(green("✓")) if "key" not in name else bold(yellow("★"))
        print(f"  {marker} {bold(name)}")
        print(f"    {cyan(eq)}")
        print()
    input(bold("  Press Enter to continue... "))


# ── Section 3: hcomp — homogeneous composition ────────────────────────────────

def _section_hcomp():
    clear()
    print(box("hcomp: Homogeneous Composition"))
    print()
    print(wrap(
        "hcomp is the fundamental operation in cubical type theory. It fills "
        "an 'open box' in a type — a cube with all faces specified except one. "
        "The missing face is what hcomp computes."
    ))
    print()
    print(f"  {cyan('hcomp : (i : 𝕀) → Partial (i=0 ∨ i=1) A → A → A')}")
    print()
    print(f"  {bold('Geometrically: filling a square')}\n")
    print(f"    {bold('a₀₁')} ─── {bold('a₁₁')}")
    print(f"    {dim('│')}           {dim('│')}")
    print(f"    {dim('│')}    ?      {dim('│')}  ← hcomp fills this")
    print(f"    {dim('│')}           {dim('│')}")
    print(f"    {bold('a₀₀')} ─── {bold('a₁₀')}")
    print()
    print(wrap(
        "Given three sides of a square (bottom, left, right), hcomp computes "
        "the TOP side — the path that completes the square. This is what "
        "makes path concatenation work: you glue two paths by filling a square."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Path concatenation via hcomp:'))}\n")
    print(f"  {cyan('comp p q = λj. hcomp [ j=0 ↦ p 0,  j=1 ↦ q j ] (p j)')}")
    print()
    print(wrap(
        "Here p : Path A a b and q : Path A b c. The hcomp fills the square:"
    ))
    print()
    print(f"     {bold('a')} ─── {bold('b')} ─── {bold('c')}")
    print(f"     │     p     │     q     │")
    print(f"     │     ─     │     ─     │")
    print(f"     {bold('a')} ──────────── {bold('c')}")
    print(f"                     p · q")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The cubical model:'))}\n")
    print(wrap(
        "The cubical model interprets types as presheaves over the category "
        "of cubes (products of intervals). A type A is a functor assigning "
        "to each cube Iⁿ a set of 'n-dimensional elements'. Paths are the "
        "1-dimensional elements, squares are 2-dimensional, etc. hcomp is "
        "the composition operation in this structure."
    ))
    print()
    print(wrap(
        "This gives a concrete COMPUTATIONAL INTERPRETATION: every proof "
        "in cubical type theory has a canonical normal form that can actually "
        "be computed. This makes cubical type theory executable in a way that "
        "Book HoTT with the univalence axiom is not."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 4: ua in cubical ──────────────────────────────────────────────────

def _section_ua_cubical():
    clear()
    print(box("ua in Cubical: Univalence as a Theorem"))
    print()
    print(wrap(
        "In Book HoTT, univalence is an AXIOM: ua : A ≃ B → A = B, asserted "
        "to exist with computation rule uaβ, but with no canonical definition. "
        "In Cubical HoTT, ua is a DEFINITION — it is GLUE types:"
    ))
    print()
    print(f"  {cyan('ua e = λ(i:𝕀). Glue A [ i=0 ↦ (A, id), i=1 ↦ (B, e) ]')}")
    print()
    print(wrap(
        "The Glue type constructor 'glues' types along an equivalence. At i=0, "
        "the type is A (with the identity equivalence). At i=1, the type is B "
        "(with equivalence e). The resulting path ua(e) : A = B is a FUNCTION "
        "𝕀 → 𝒰 from the interval to the universe."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The computation rule uaβ is now definitional:'))}\n")
    print(f"  {cyan('transport id (ua e) a')}")
    print(f"  {cyan('= transport id (λi. Glue A [ i=0 ↦ (A,id), i=1 ↦ (B,e) ]) a')}")
    print(f"  {cyan('= e a   (by the Glue reduction rule)')}")
    print()
    print(wrap(
        "The transport of a along ua(e) reduces definitionally to e(a). "
        "No axiom is needed — the Glue type has a built-in reduction rule "
        "that computes transport. This is the cubical model's power."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Consequences of computable ua:'))}\n")

    consequences = [
        ("funext is computable",
         "happly (funext h) x reduces definitionally to h x",
         "No stuck terms involving funext — proofs involving function equality compute."),
        ("Proof checking terminates",
         "Type checking is decidable in cubical",
         "Book HoTT with univalence has stuck terms; cubical has none."),
        ("Programs extract correctly",
         "Proof extraction via cubical gives running code",
         "You can extract a Haskell/OCaml program from a cubical proof."),
        ("HITs compute",
         "Eliminator for S¹ applied to base and loop both reduce",
         "S¹-recursion computes definitionally on the HIT constructors."),
    ]

    for title, form, note in consequences:
        print(f"  {bold(green(title))}")
        print(f"  {cyan(form)}")
        print(wrap(note, width=66, indent="    "))
        print()
    input(bold("  Press Enter to continue... "))


# ── Section 5: Book HoTT vs Cubical ───────────────────────────────────────────

def _section_comparison():
    clear()
    print(box("Book HoTT vs. Cubical Type Theory: A Comparison"))
    print()

    rows = [
        ("Paths",
         "Identity type a =_A b (eliminator J)",
         "Functions 𝕀 → A with face restrictions"),
        ("Univalence",
         "Axiom: ua : A ≃ B → A = B",
         "Theorem via Glue types"),
        ("ua computes",
         "No — transport along ua is stuck",
         "Yes — transport along ua reduces definitionally"),
        ("funext",
         "Theorem from univalence (but stuck terms)",
         "Definition: funext h = λi. λx. h x i (no axiom)"),
        ("Right unit p·refl",
         "Propositional only (requires proof)",
         "Definitional (hcomp reduction)"),
        ("HITs",
         "Postulated with computation rules",
         "Constructed via hcomp and higher cubes"),
        ("Decidable typechecking",
         "No — univalence blocks normalization",
         "Yes — all terms normalize"),
        ("Implementations",
         "Agda (with --without-K), Lean 4, Coq (HoTT library)",
         "Cubical Agda, redtt, cooltt, Arend"),
        ("η for functions",
         "Propositional (needs funext)",
         "Definitional (λx. f x ≡ f)"),
        ("Canonicity",
         "Fails with univalence axiom",
         "Holds: every closed term of type ℕ reduces to numeral"),
    ]

    print(f"  {'Property':<30}  {'Book HoTT':<30}  {'Cubical'}")
    print(f"  {dim('─'*90)}")
    for prop, book, cub in rows:
        print(f"  {bold(prop):<38}  {dim(book):<38}  {green(cub)}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('Which should you use?'))}\n")
    print(wrap(
        "Book HoTT: better for learning the theory, cleaner conceptually, "
        "closer to classical algebraic topology. The HoTT book uses it. "
        "Agda with --without-K is a good implementation."
    ))
    print()
    print(wrap(
        "Cubical: better for actually running proofs, extracting programs, "
        "and research on computational content of HoTT. Cubical Agda is the "
        "most mature implementation and is actively used for new results "
        "(e.g., formalizing the Brunerie number, Blakers-Massey, etc.)."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 6: The interval and ∞-groupoid structure ─────────────────────────

def _section_infinity_groupoid():
    clear()
    print(box("The Interval and ∞-Groupoid Structure"))
    print()
    print(wrap(
        "The interval 𝕀 generates ALL the higher structure in cubical type theory. "
        "An n-cube in a type A is a function (𝕀)ⁿ → A."
    ))
    print()

    cubes = [
        ("(𝕀)⁰ → A", "A point in A",                   "a : A"),
        ("(𝕀)¹ → A", "A path in A",                    "p : a = b"),
        ("(𝕀)² → A", "A square in A",                   "s : p = q  (a 2-path)"),
        ("(𝕀)³ → A", "A cube in A",                     "a 3-path (homotopy between homotopies)"),
        ("(𝕀)ⁿ → A", "An n-cube in A",                  "the n-dimensional identity type"),
    ]

    for ty, desc, ex in cubes:
        print(f"  {bold(cyan(ty)):25}  {desc}")
        print(f"    {dim(ex)}")
        print()

    print(rule())
    print(f"\n  {bold(green('The Kan condition:'))}\n")
    print(wrap(
        "Not every presheaf on cubes gives a valid type. A type must satisfy "
        "the KAN CONDITION: every open box (a cube with one face missing) can "
        "be filled. This is the hcomp operation — and it corresponds exactly "
        "to the COMPOSITION and FILLING operations of an ∞-groupoid."
    ))
    print()
    print(wrap(
        "Types in cubical = Kan cubical sets = models of ∞-groupoids. "
        "This is the precise mathematical meaning of 'types are ∞-groupoids' "
        "in Cubical Type Theory: they literally ARE Kan cubical sets."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Connections to other developments:'))}\n")

    developments = [
        ("Simplicial HoTT",   "Uses simplicial sets instead of cubical sets as the model"),
        ("Cartesian cubical",  "A variant with only product cubes (no diagonal, no connections)"),
        ("Directed type theory","Replaces paths with directed edges (morphisms) — for (∞,1)-categories"),
        ("Modal type theory",  "Adds modalities (♭, ♯, ○) for cohesive or differential structure"),
        ("XTT",                "Experimental extension with 'exact equality' and proof irrelevance"),
    ]
    for name, note in developments:
        print(f"  {bold(cyan(name))}")
        print(f"    {dim(note)}")
        print()

    print(wrap(
        "All of these are active research areas extending the cubical ideas. "
        "Cubical type theory is not the end of the story — it is a foundation "
        "that opens many new directions."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("interval",    "The interval type 𝕀: paths as functions",         _section_interval),
    ("right_unit",  "The right-unit law p·refl = p definitionally",    _section_right_unit),
    ("hcomp",       "hcomp: homogeneous composition",                  _section_hcomp),
    ("ua",          "ua in cubical: univalence as a theorem",          _section_ua_cubical),
    ("comparison",  "Book HoTT vs. Cubical: a comparison",             _section_comparison),
    ("infinity",    "The interval and ∞-groupoid structure",           _section_infinity_groupoid),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Cubical Type Theory: Paths as Functions", width=70))
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
