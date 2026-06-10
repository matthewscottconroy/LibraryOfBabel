#!/usr/bin/env python3
"""
Univalence: Equivalences Are Paths
=====================================
idtoeqv, ua, uaβ, transport along ua, and consequences.

The univalence axiom says that the canonical map idtoeqv : (A = B) → (A ≃ B)
is an equivalence. This identifies paths between types with equivalences
between types — "equivalence is equality."
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


# ── Section 1: idtoeqv and the statement ──────────────────────────────────────

def _section_statement():
    clear()
    print(box("Univalence: The Precise Statement"))
    print()
    print(wrap(
        "Before the axiom, there is a canonical map from paths to equivalences, "
        "provable by path induction alone:"
    ))
    print()
    print(f"  {cyan('idtoeqv : A = B → A ≃ B')}")
    print(f"  {cyan('idtoeqv refl = id_A  (the identity equivalence)')}")
    print()
    print(wrap(
        "Path induction says: if we can handle the case where A = B is refl "
        "(i.e., A = B = A and the path is trivial), then we can handle any path. "
        "For refl, the equivalence is just the identity. QED."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('The univalence axiom:'))}\n")
    print(f"  {bold(cyan('ua : A ≃ B → A = B'))}")
    print(f"  {bold(cyan('univalence : isEquiv idtoeqv'))}")
    print()
    print(wrap(
        "The axiom says: idtoeqv is an EQUIVALENCE. This means it has an "
        "inverse, which we call ua (for 'univalence'). So:"
    ))
    print()
    print(f"  {'ua   : A ≃ B → A = B':40}  (inverse of idtoeqv)")
    print(f"  {'uaβ  : idtoeqv (ua e) = e':40}  (computation rule)")
    print(f"  {'uaη  : ua (idtoeqv p) = p':40}  (uniqueness rule)")
    print()
    print(rule())
    print(f"\n  {bold(green('What this means:'))}\n")
    print(wrap(
        "An equivalence e : A ≃ B is a function with a quasi-inverse — a bijection "
        "in the homotopy-theoretic sense. Univalence says: EVERY such bijection "
        "IS a path between the types. Types that are equivalent are literally "
        "IDENTICAL (as types in the universe)."
    ))
    print()
    print(f"  {bold('Slogan:')}  {yellow('Equivalent types are equal types.')}")
    print()
    print(wrap(
        "This seems radical. But consider: in mathematics, we routinely treat "
        "isomorphic objects as 'the same'. Univalence makes this precise and "
        "formally valid."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 2: ua in practice ─────────────────────────────────────────────────

def _section_ua_practice():
    clear()
    print(box("ua in Practice: Building Paths Between Types"))
    print()
    print(wrap(
        "Given any equivalence e : A ≃ B, we get a path ua(e) : A = B. "
        "Let's see concrete examples."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Example 1: Bool ≃ Bool (two equivalences)'))}\n")
    print(f"  {cyan('id_Bool   : Bool → Bool  (identity)')}")
    print(f"  {cyan('flip_Bool : Bool → Bool  (true↦false, false↦true)')}")
    print()
    print(wrap(
        "Both are self-equivalences Bool ≃ Bool. They give TWO DIFFERENT paths "
        "Bool = Bool in the universe. The type Bool has an interesting path space."
    ))
    print()
    print(f"  ua(id_Bool)   : Bool = Bool   {dim('(the trivial path refl)')}")
    print(f"  ua(flip_Bool) : Bool = Bool   {dim('(a non-trivial loop at Bool in the universe)')}")
    print()

    # Simulate the two Bool equivalences
    id_bool   = {True: True, False: False}
    flip_bool = {True: False, False: True}

    print(f"  {bold('Identity:')}")
    for k, v in id_bool.items():
        print(f"    {k} ↦ {v}")
    print(f"  {bold('Flip:')}")
    for k, v in flip_bool.items():
        print(f"    {k} ↦ {v}")

    print()
    print(f"  {dim('Both are bijections (equivalences). Both give paths Bool = Bool.')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Example 2: ℕ ≃ ℕ (many equivalences)'))}\n")
    print(wrap(
        "The type ℕ has many self-equivalences: the identity, shift by 1 "
        "(requires fixing 0), swap any two elements, etc. But most of these "
        "are NOT well-behaved algebraically. The key examples:"
    ))
    print()
    print(f"  {cyan('id_ℕ        : ℕ ≃ ℕ')}")
    print(f"  {cyan('succ_equiv  : ℕ ≃ ℕ   -- impossible! succ has no left inverse at 0')}")
    print()
    print(wrap(
        "succ is NOT an equivalence: it's an injection but not a surjection "
        "(0 is not in the image). So ua(succ) does NOT give a path ℕ = ℕ."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Example 3: (A × B) ≃ (B × A)'))}\n")
    print(f"  {cyan('swap : A × B → B × A')}")
    print(f"  {cyan('swap (a, b) = (b, a)')}")
    print()
    print(wrap(
        "swap is its own inverse: swap ∘ swap = id. So swap is an equivalence, "
        "and ua(swap) : A × B = B × A. This path can be used to transport "
        "structures from A × B to B × A — the types are literally the same."
    ))
    print()

    pairs = [(1, 'x'), (True, 3), ([1,2], 'hello')]
    print(f"  {'(a, b)':20} → {'(b, a)'}")
    for a, b in pairs:
        print(f"  {str((a,b)):20} → {str((b,a))}")
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 3: Aut(Bool) = ℤ/2ℤ ─────────────────────────────────────────────

def _section_aut_bool():
    clear()
    print(box("Aut(Bool) = ℤ/2ℤ: The Automorphism Group of Bool"))
    print()
    print(wrap(
        "The AUTOMORPHISM GROUP of Bool is the set of self-equivalences Bool ≃ Bool, "
        "with composition as the group operation. By univalence, this equals "
        "the loop space of the universe at Bool:"
    ))
    print()
    print(f"  {cyan('Aut(Bool) = (Bool = Bool) = Ω(𝒰, Bool)')}")
    print()
    print(wrap(
        "We claim Aut(Bool) = ℤ/2ℤ (two elements). Let's verify:"
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The two self-equivalences of Bool:'))}\n")

    equivs = [
        ("id",   lambda b: b,     "T↦T, F↦F", "identity, trivial loop (= refl)"),
        ("flip", lambda b: not b, "T↦F, F↦T", "swap, non-trivial loop"),
    ]

    for name, fn, mapping, note in equivs:
        t_result = fn(True)
        f_result = fn(False)
        comp_tt = fn(fn(True)) == True
        comp_ff = fn(fn(False)) == False
        inverse_ok = green("self-inverse ✓") if (comp_tt and comp_ff) else red("not self-inverse")
        print(f"  {bold(cyan(name)):20}  {mapping}")
        print(f"    {dim(note)}")
        print(f"    {name} ∘ {name} = id?  {inverse_ok}")
        print()

    print(f"  {bold('Group table for Aut(Bool):')}\n")
    print(f"  {'∘':12}{'id':12}{'flip':12}")
    print(f"  {dim('-'*36)}")
    ops = {
        ('id',   'id'):   'id',
        ('id',   'flip'): 'flip',
        ('flip', 'id'):   'flip',
        ('flip', 'flip'): 'id',
    }
    for a in ['id', 'flip']:
        row = f"  {a:12}"
        for b in ['id', 'flip']:
            row += f"{ops[(a,b)]:12}"
        print(row)

    print()
    print(wrap(
        "The group table is that of ℤ/2ℤ: two elements, each its own inverse, "
        "with flip·flip = id. By univalence, the path space Bool = Bool has "
        "exactly two elements — the trivial loop (refl = ua(id)) and the "
        "non-trivial loop (ua(flip))."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Generalizing: Aut(Fin n) = Σₙ (symmetric group)'))}\n")
    print(wrap(
        "More generally, the automorphism group of any finite type with n elements "
        "is the symmetric group Σₙ on n elements. For Bool = Fin 2, this is Σ₂ = ℤ/2ℤ. "
        "Univalence lets us DERIVE this group-theoretically from the type structure."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 4: Transport along ua ─────────────────────────────────────────────

def _section_transport_ua():
    clear()
    print(box("Transport Along ua: Moving Data Across Equivalences"))
    print()
    print(wrap(
        "Given an equivalence e : A ≃ B and a value a : A, we can transport "
        "a along the path ua(e) to get a value in B:"
    ))
    print()
    print(f"  {cyan('transport P (ua e) : P A → P B')}")
    print()
    print(wrap(
        "For the identity type family P = id (i.e., P(X) = X), this gives:"
    ))
    print()
    print(f"  {cyan('transport id (ua e) : A → B')}")
    print(f"  {cyan('transport id (ua e) a = e(a)   (by uaβ)')}")
    print()
    print(wrap(
        "The computation rule uaβ says: transporting along ua(e) IS applying e. "
        "The path ua(e) carries values from A to B precisely by applying the "
        "underlying function of the equivalence."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Example: transport along ua(flip) : Bool = Bool'))}\n")

    flip = lambda b: not b

    values = [True, False]
    print(f"  {'a : Bool':20}  {'transport id (ua flip) a':30}  {'= flip a'}")
    print(f"  {dim('─'*60)}")
    for v in values:
        transported = flip(v)
        print(f"  {str(v):20}  {str(transported):30}  {green('✓')}")

    print()
    print(rule())
    print(f"\n  {bold(green('Example: transport a predicate along ua(flip)'))}\n")
    print(wrap(
        "Let P(X) = (X → Prop) — the type of predicates on X. "
        "Transporting the predicate isTrue : Bool → Prop along ua(flip) gives "
        "the predicate isFalse (the negation)."
    ))
    print()
    print(f"  {cyan('transport (λX. X → Prop) (ua flip) isTrue')}")
    print(f"  {cyan('= isTrue ∘ flip⁻¹')}")
    print(f"  {cyan('= isTrue ∘ flip')}")
    print(f"  {cyan('= isFalse')}")
    print()
    print(wrap(
        "More generally, transporting a function type along ua(e) gives you "
        "pre-composition with e. Transport 'acts contravariantly' on predicates "
        "and functions — and this is exactly what equalities between types should do."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Key computation rules:'))}\n")
    print(f"  {cyan('transport id (ua e) a = e a           -- apply the equivalence')}")
    print(f"  {cyan('transport (λX. X → P) (ua e) f = f ∘ e⁻¹  -- precompose with inverse')}")
    print(f"  {cyan('transport (λX. P → X) (ua e) f = e ∘ f     -- postcompose with e')}")
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 5: Consequences ───────────────────────────────────────────────────

def _section_consequences():
    clear()
    print(box("Consequences of Univalence"))
    print()

    consequences = [
        ("Function extensionality",
         "funext : (∀x. f x = g x) → f = g",
         "Follows from univalence. See demo_funext.py for the full derivation.",
         green),
        ("Propositional extensionality",
         "propext : (P ↔ Q) → P = Q",
         "For propositions (h-level -1), logical equivalence implies equality. "
         "This is a special case of univalence: propositions that are logically "
         "equivalent are equivalent as types (trivially, since both have at most one element).",
         green),
        ("Structure transport",
         "If A ≃ B and A is a group, then B is a group (with the transported structure)",
         "Any structure or property on A can be transported along ua(e) to B. "
         "This formalizes the mathematical practice of 'working up to isomorphism'.",
         green),
        ("The structure identity principle (SIP)",
         "Two structures (A, s) ≃ (B, t) iff (A, s) = (B, t) in the universe of structures",
         "SIP is a strengthening of univalence for structured types. "
         "Proved by Coquand-Danielsson and formalized in cubical Agda.",
         yellow),
        ("No strict set model",
         "Sets (classical) do NOT model univalence — they satisfy UIP (Uniqueness of Identity Proofs)",
         "In classical set theory, A = B implies the elements are literally the same, "
         "not just bijectable. Univalence requires richer models (simplicial sets, cubical sets).",
         red),
        ("Proof irrelevance follows for propositions",
         "isProp A → ∀(p q : a = b). p = q",
         "In sets (h-level 0), all paths between two points are equal. "
         "This follows from univalence applied to path spaces.",
         green),
    ]

    for title, form, note, col in consequences:
        print(f"  {bold(col(title))}")
        print(f"  {cyan(form)}")
        print(wrap(note, width=66, indent="    "))
        print()

    print(rule())
    print(f"\n  {bold(yellow('The philosophical shift:'))}\n")
    print(wrap(
        "Classical mathematics treats isomorphic objects as interchangeable but "
        "technically distinct. Every time a mathematician says 'without loss of "
        "generality, we may assume...' or 'identify A with B', they are using an "
        "informal version of ua. Univalence FORMALIZES this, making it rigorous "
        "rather than a handwave. The price: you must work in a richer foundational "
        "system (HoTT) rather than classical set theory."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("statement",   "idtoeqv and the precise statement of univalence", _section_statement),
    ("practice",    "ua in practice: building paths between types",    _section_ua_practice),
    ("aut",         "Aut(Bool) = ℤ/2ℤ: automorphism groups from ua",  _section_aut_bool),
    ("transport",   "Transport along ua: moving data across equiv.",   _section_transport_ua),
    ("consequences","Consequences: funext, propext, SIP",              _section_consequences),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Univalence: Equivalences Are Paths", width=70))
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
