#!/usr/bin/env python3
"""
The Fundamental Theorem of Identity Types
==========================================
Characterizing path spaces: (a = b) ≃ R(a, b)

The fundamental theorem tells us HOW to prove that two things are equal:
give a relation R, show it is equivalent to equality. This unifies all the
specific path-space calculations in HoTT (pairs, functions, Σ-types, HITs)
under one theorem.
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


# ── Section 1: The theorem ────────────────────────────────────────────────────

def _section_theorem():
    clear()
    print(box("The Fundamental Theorem of Identity Types"))
    print()
    print(wrap(
        "The fundamental theorem characterizes when a relation R : A → A → Type "
        "is the SAME as equality. It is a complete criterion."
    ))
    print()
    print(f"  {bold(cyan('Theorem (FTID):'))}")
    print(f"  {cyan('Given R : A → A → Type and r : ∀a. R a a,')}")
    print(f"  {cyan('the following are equivalent:')}")
    print()
    print(f"  {bold('(i)')}  R a b is an equivalence of types, for all a b : A:")
    print(f"        {cyan('(a = b) ≃ R a b')}")
    print()
    print(f"  {bold('(ii)')} The type Σ(b:A). R a b is contractible, for each a : A:")
    print(f"        {cyan('isContr (Σ(b:A). R a b)')}")
    print()
    print(wrap(
        "These are equivalent, and either can be proved to establish that R "
        "characterizes equality. Condition (ii) is often easier to check: "
        "you show that Σ(b:A). R a b has a unique element (a, r a)."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Why this works:'))}\n")
    print(wrap(
        "We always have encode : a = b → R a b defined by path induction "
        "(encode refl = r a). The FTID says: if Σb. R a b is contractible "
        "(with center (a, r a)), then encode is an equivalence. "
        "Contractibility of the total space is the key checkable condition."
    ))
    print()
    print(wrap(
        "The encode-decode pattern you saw in demo_encode_decode.py is an "
        "APPLICATION of this theorem: the 'code' family IS the R, and proving "
        "the total space contractible is the roundtrip argument."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 2: Paths in products ──────────────────────────────────────────────

def _section_products():
    clear()
    print(box("Example 1: Paths in Products A × B"))
    print()
    print(wrap(
        "Claim: (a, b) = (a', b') in A × B is equivalent to (a = a') × (b = b')."
    ))
    print()
    r_def = "R((a,b), (a',b')) :≡ (a = a') × (b = b')"
    print(f"  {cyan(r_def)}")
    print()
    print(f"  {bold(green('Proof using FTID:'))}")
    print()
    print(wrap(
        "Fix (a, b). We must show Σ((a',b'):A×B). R((a,b),(a',b')) is contractible."
    ))
    print()
    sigma_str1 = "Σ((a',b'):A×B). (a=a') × (b=b')"
    sigma_str2 = "≃ Σ(a':A). (a=a') × Σ(b':B). (b=b')"
    sigma_str3 = "≃ (Σ(a':A). a=a') × (Σ(b':B). b=b')"
    print(f"  {cyan(sigma_str1)}")
    print(f"  {cyan(sigma_str2)}")
    print(f"  {cyan(sigma_str3)}")
    print(f"  {cyan('≃ 𝟙 × 𝟙  (each singleton is contractible)')}")
    print(f"  {cyan('≃ 𝟙  □')}")
    print()
    print(wrap(
        "Each factor Σ(a':A). a=a' is a 'singleton' — contractible with "
        "center (a, refl). The product of contractible types is contractible. "
        "So the total space is contractible, and FTID gives the equivalence."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('The result:'))}\n")
    print(f"  {cyan('(a, b) = (a′, b′)  ≃  (a = a′) × (b = b′)')}")
    print()
    print(wrap(
        "PATHS in a PRODUCT are PAIRS of PATHS. This is the general pattern: "
        "the path space of a structured type is the 'structured path space'."
    ))
    print()
    print(f"  {bold('In Python, we can see the analogous structure:')}")
    print()

    pairs = [((1, 'a'), (1, 'a')), ((2, 'b'), (2, 'b')), ((1, 'a'), (1, 'b'))]
    for p, q in pairs:
        eq = p == q
        component_eq = (p[0] == q[0], p[1] == q[1])
        same = green("✓") if eq == (component_eq[0] and component_eq[1]) else red("✗")
        print(f"  {str(p):15} = {str(q):15}  ↔  {str(component_eq[0]):<6} and {str(component_eq[1]):<6}  {same}")

    print()
    print(dim("  (Python's == checks component-wise — exactly the FTID result)"))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 3: Paths in Σ-types ───────────────────────────────────────────────

def _section_sigma():
    clear()
    print(box("Example 2: Paths in Σ-Types"))
    print()
    print(wrap(
        "For a dependent pair type Σ(x:A). P(x), the path space is:"
    ))
    print()
    print(f"  {cyan('(a, p) = (a′, p′)  ≃  Σ(q : a = a′).  transport P q p = p′')}")
    print()
    print(wrap(
        "A path between dependent pairs consists of TWO PIECES: "
        "(1) a path q : a = a' between the first components, and "
        "(2) a path showing the second components are equal AFTER transporting "
        "p along q to make them live in the same fiber."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Why transport appears:'))}\n")
    print(wrap(
        "The second components live in different types: p : P(a) and p' : P(a'). "
        "To compare them, we must TRANSPORT p to P(a'), using the path q. "
        "Then transport P q p : P(a') and p' : P(a') can be compared."
    ))
    print()
    print(f"  {cyan('p  : P(a)')}")
    print(f"  {cyan('p′ : P(a′)')}")
    print(f"  {cyan('q  : a = a′')}")
    print(f"  {cyan('transport P q p : P(a′)    ← now they live in the same type')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Proof using FTID:'))}\n")
    print(wrap(
        "Fix (a, p). We show Σ((a',p'):Σx.Px). path-space is contractible:"
    ))
    print()
    print(f"  {cyan('Σ((a′,p′): Σx.Px). Σ(q:a=a′). transport P q p = p′')}")
    print(f"  {cyan('≃ Σ(a′:A). Σ(q:a=a′). Σ(p′:Pa′). transport P q p = p′')}")
    print(f"  {cyan('≃ Σ(a′:A). Σ(q:a=a′). 𝟙   (transport p = p′ has unique solution)')}")
    print(f"  {cyan('≃ Σ(a′:A). (a=a′)  ≃  𝟙  □')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Special case: (a=a′) ≃ (a=a′)  (trivial)'))}\n")
    print(wrap(
        "When P is the constant family P(x) = B, Σ(x:A). B ≃ A × B, and "
        "transport is trivial. We recover the product path space as a special case."
    ))
    print()
    print(f"\n  {bold(yellow('Special case: paths in the universe'))}\n")
    print(f"  {cyan('(A = B : 𝒰)  ≃  (A ≃ B)  -- by univalence!')}")
    print()
    print(wrap(
        "The universe 𝒰 is itself a type. Paths in 𝒰 between types A and B "
        "correspond to equivalences A ≃ B — this is exactly univalence. "
        "Univalence is the FTID applied to the universe with R = equivalence."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 4: Paths in function types (funext) ───────────────────────────────

def _section_funext():
    clear()
    print(box("Example 3: Paths in Function Types (funext)"))
    print()
    print(wrap(
        "For functions f g : A → B, the path space (f = g) is characterized by:"
    ))
    print()
    print(f"  {cyan('(f = g : A → B)  ≃  ∀(x:A). f x = g x')}")
    print()
    print(wrap(
        "This IS function extensionality (funext). It follows from FTID "
        "by taking R(f, g) = ∀x. f x = g x."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Proof using FTID:'))}\n")
    print(wrap(
        "Fix f : A → B. We show Σ(g:A→B). ∀x. f x = g x is contractible:"
    ))
    print()
    print(f"  {cyan('Σ(g:A→B). ∀(x:A). f x = g x')}")
    print(f"  {cyan('≃ ∀(x:A). Σ(y:B). f x = y   (Σ-Π swap, using choice-like principle)')}")
    print(f"  {cyan('≃ ∀(x:A). 𝟙               (each Σ(y:B). fx=y is a singleton)')}")
    print(f"  {cyan('≃ 𝟙  □')}")
    print()
    print(wrap(
        "The Σ-Π swap used here is the 'type-theoretic axiom of choice' which "
        "holds WITHOUT any axiom when the Σ type is over a structure-preserving "
        "family. The swap is an equivalence, and singletons are contractible."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Dependent version:'))}\n")
    print(f"  {cyan('(f = g : Π(x:A). P x)  ≃  ∀(x:A). f x =_{{Px}} g x')}")
    print()
    print(wrap(
        "For dependent functions, pointwise paths live in the fibers P(x). "
        "The proof is the same: the total space Σg. ∀x. fx =_{Px} gx is "
        "contractible because each fiber Σy. fx=y is a singleton."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Summary: path spaces of type formers'))}\n")

    rows = [
        ("A × B",           "(a,b) = (a',b')",    "(a=a') × (b=b')"),
        ("Σ(x:A). P x",     "(a,p) = (a',p')",    "Σ(q:a=a'). transport P q p = p'"),
        ("Π(x:A). P x",     "f = g",              "∀x. f x = g x"),
        ("A + B",           "inl a = inl a'",     "a = a' in A"),
        ("A + B",           "inr b = inr b'",     "b = b' in B"),
        ("A + B",           "inl a = inr b",      "𝟘 (impossible)"),
        ("𝒰 (universe)",    "A = B",              "A ≃ B (univalence)"),
    ]

    print(f"  {'Type':22}  {'Path':26}  {'Equivalent to'}")
    print(f"  {dim('─'*70)}")
    for ty, path, eq in rows:
        print(f"  {cyan(ty):30}  {path:26}  {bold(yellow(eq))}")

    print()
    input(bold("  Press Enter to continue... "))


# ── Section 5: The encode-decode connection ───────────────────────────────────

def _section_encode_decode():
    clear()
    print(box("The Encode-Decode Connection"))
    print()
    print(wrap(
        "The encode-decode method is precisely the FTID in action. "
        "Let's see how the circle computation π₁(S¹) = ℤ fits the pattern."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('For S¹:'))}\n")
    print(f"  {cyan('code : S¹ → Type')}")
    print(f"  {cyan('code base = ℤ')}")
    print(f"  {cyan('code (loop i) = ua(+1)(i)  (transport adds 1 around the loop)')}")
    print()
    print(wrap(
        "FTID asks: is Σ(x:S¹). code x contractible? "
        "The total space Σ(x:S¹). code x is the UNIVERSAL COVER of S¹ — which is ℝ "
        "(the real line). ℝ is contractible. So yes!"
    ))
    print()
    print(f"  {cyan('Σ(x:S¹). code x  ≃  ℝ  ≃  𝟙   (contractible)')}")
    print()
    print(f"  {bold('Therefore:')}  {bold(cyan('(base = base) ≃ ℤ'))}  {dim('i.e. π₁(S¹) = ℤ  □')}")
    print()
    print(rule())
    print(f"\n  {bold(green('General pattern for HITs:'))}\n")

    print(f"  {'HIT':20}  {'code family':25}  {'total space'}")
    print(f"  {dim('─'*65)}")
    pattern = [
        ("S¹",  "code base = ℤ",     "ℝ (universal cover)"),
        ("S²",  "code base = ℤ",     "ℝ³ - {0}  (not easy!)"),
        ("ℝP²", "code base = ℤ/2ℤ",  "S² (2-fold cover)"),
        ("T²",  "code base = ℤ×ℤ",   "ℝ² (universal cover of T²)"),
        ("BG",  "code base = G",      "EG (contractible total space)"),
    ]
    for hit, code, total in pattern:
        print(f"  {cyan(hit):20}  {code:25}  {dim(total)}")

    print()
    print(rule())
    print(f"\n  {bold(yellow('The contractibility criterion is the hard part:'))}\n")
    print(wrap(
        "For S¹, proving the universal cover ℝ is contractible is immediate "
        "(ℝ deformation retracts to a point). For S², the corresponding space "
        "is more complex (it leads to the calculation π₃(S²) = ℤ via Hopf). "
        "The FTID tells us WHAT to prove; the geometry tells us HOW."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 6: The path space fibration ───────────────────────────────────────

def _section_path_fibration():
    clear()
    print(box("The Path Space Fibration"))
    print()
    print(wrap(
        "There is a canonical fibration associated to any map f : A → B, "
        "called the PATH SPACE FIBRATION or MAPPING PATH SPACE."
    ))
    print()
    print(f"  {cyan('Pf = Σ(a:A). Σ(b:B). f a = b')}")
    print(f"  {cyan('Pf ≃ A  (by projection: the Σ(b:B). fa=b fiber is contractible)')}")
    print()
    print(wrap(
        "The mapping path space Pf is equivalent to A, but it comes with a "
        "MAP to B (by projection to b). The fiber over a point b : B is "
        "the HOMOTOPY FIBER of f at b:"
    ))
    print()
    print(f"  {cyan('hofib(f, b) = Σ(a:A). f a = b')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The fibration sequence:'))}\n")
    print(f"  {cyan('hofib(f,b) → A → B')}")
    print()
    print(wrap(
        "The homotopy fiber fits into a fibration sequence, which gives a "
        "long exact sequence of homotopy groups (generalizing the Hopf fibration "
        "sequence from demo_hopf.py):"
    ))
    print()
    print(f"  {cyan('⋯ → πₙ(hofib(f,b)) → πₙ(A) → πₙ(B) → πₙ₋₁(hofib(f,b)) → ⋯')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('FTID as contractibility of homotopy fibers:'))}\n")
    print(wrap(
        "The FTID says that encode : (a=b) → R(a,b) is an equivalence "
        "iff the total space Σb. R(a,b) is contractible."
    ))
    print(wrap(
        "In fibration language: encode is the 'transport' for the fibration "
        "with total space Σb. R(a,b). The total space being contractible "
        "means the fibration is TRIVIAL — the fibers (a=b) and R(a,b) are equivalent."
    ))
    print()
    print(f"  {bold('Summary of the three equivalent conditions (FTID):')}\n")
    print(f"  {bold('(1)')} {cyan('encode : (a=b) → R(a,b)  is an equivalence')}")
    print(f"  {bold('(2)')} {cyan('Σ(b:A). R(a,b)  is contractible')}")
    print(f"  {bold('(3)')} {cyan('R is a reflexive relation total-equivalent to equality')}")
    print()
    print(wrap(
        "Any one of these can be used to prove the others. In practice, "
        "(2) is easiest to prove (show the total space is a singleton or "
        "use contractibility lemmas). Then (1) gives the actual equivalence "
        "between (a=b) and R(a,b) that you want."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("theorem",   "The Fundamental Theorem of Identity Types",         _section_theorem),
    ("products",  "Example 1: paths in products A × B",               _section_products),
    ("sigma",     "Example 2: paths in Σ-types",                      _section_sigma),
    ("funext",    "Example 3: paths in function types (funext)",       _section_funext),
    ("encode",    "The encode-decode connection",                      _section_encode_decode),
    ("fibration", "The path space fibration",                          _section_path_fibration),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("The Fundamental Theorem of Identity Types", width=70))
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
