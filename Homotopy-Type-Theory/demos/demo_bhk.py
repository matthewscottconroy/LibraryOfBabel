#!/usr/bin/env python3
"""
The Brouwer-Heyting-Kolmogorov Interpretation
===============================================
What IS a proof? The BHK interpretation gives computational meaning to logic.

Each logical connective is assigned a TYPE of proof-object:
- A proof of A ∧ B is a pair (proof of A, proof of B)
- A proof of A → B is a function converting proofs of A into proofs of B
- A proof of A ∨ B is a tagged proof: either (left, proof of A) or (right, proof of B)
- A proof of ¬A is a function converting proofs of A into proofs of ⊥
- A proof of ∃x.P(x) is a pair (witness a, proof of P(a))
- A proof of ∀x.P(x) is a function sending each a to a proof of P(a)
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


# ── Section 1: Introduction ───────────────────────────────────────────────────

def _section_intro():
    clear()
    print(box("The BHK Interpretation: Proofs as Constructions"))
    print()
    print(wrap(
        "Classical logic defines truth via truth tables: φ is true iff it "
        "evaluates to T in every assignment. CONSTRUCTIVE logic (intuitionistic "
        "logic) instead asks: what does it mean to HAVE A PROOF of φ?"
    ))
    print()
    print(wrap(
        "The Brouwer-Heyting-Kolmogorov (BHK) interpretation answers this "
        "by giving each connective a TYPE of proof-object — a concrete "
        "mathematical construction that counts as evidence."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The BHK clauses:'))}\n")

    clauses = [
        ("A proof of ⊤",     "is trivial — the unique element of the unit type"),
        ("A proof of ⊥",     "does not exist — ⊥ is the empty type"),
        ("A proof of A ∧ B", "is a PAIR (p, q) where p proves A and q proves B"),
        ("A proof of A ∨ B", "is either (inl, p) where p proves A,"),
        ("",                  "or      (inr, q) where q proves B"),
        ("A proof of A → B", "is a FUNCTION f such that for any proof p of A,"),
        ("",                  "f(p) is a proof of B"),
        ("A proof of ¬A",    "is a function A → ⊥   (same as A → ⊥)"),
        ("A proof of ∀x.P(x)","is a FUNCTION f such that for any a, f(a) proves P(a)"),
        ("A proof of ∃x.P(x)","is a PAIR (a, p) where p proves P(a)"),
    ]

    for form, meaning in clauses:
        if form:
            print(f"  {bold(cyan(form))}")
        print(f"    {meaning}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('This IS Curry-Howard:'))}\n")
    print(wrap(
        "The BHK interpretation is exactly the Curry-Howard correspondence "
        "in disguise. Proofs are programs, propositions are types. "
        "A → B is both an implication and a function type. "
        "This is not a coincidence — it is the foundational insight of type theory."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 2: Conjunction and disjunction ────────────────────────────────────

def _section_conjunction():
    clear()
    print(box("Conjunction and Disjunction: Pairs and Tagged Unions"))
    print()
    print(f"  {bold(green('Conjunction A ∧ B:'))}\n")
    print(f"  {cyan('Proof(A ∧ B) = Proof(A) × Proof(B)')}")
    print()
    print(wrap(
        "To prove A ∧ B you must prove BOTH A and B independently. "
        "The proof-object is the pair (p_A, p_B). You can then extract "
        "either component: project₁(p_A, p_B) = p_A, project₂ = p_B."
    ))
    print()
    print(f"  {bold('Example:')}")
    print(f"  {cyan('  proof of (n>0) ∧ (n<10):  (proof_pos, proof_lt10)')}")
    print(f"  {cyan('  If n=5: (proof(5>0), proof(5<10))')}")
    print()
    print(f"  {bold(green('Proof terms (λ-calculus):'))}")
    print(f"  {cyan('  ∧I : A → B → A∧B     = λa b. (a, b)')}")
    print(f"  {cyan('  ∧E₁ : A∧B → A        = λ(a,b). a    = fst')}")
    print(f"  {cyan('  ∧E₂ : A∧B → B        = λ(a,b). b    = snd')}")

    print()
    print(rule())
    print(f"\n  {bold(green('Disjunction A ∨ B:'))}\n")
    print(f"  {cyan('Proof(A ∨ B) = Proof(A) + Proof(B)  (tagged union / sum type)')}")
    print()
    print(wrap(
        "To prove A ∨ B you must CHOOSE which disjunct holds AND provide "
        "a proof of that disjunct. The proof-object is tagged: inl(p_A) or inr(p_B). "
        "This is why disjunction in constructive logic is stronger than classically: "
        "you must KNOW which side is true, not merely that one of them is."
    ))
    print()
    print(f"  {bold('Example:')}")
    print(f"  {cyan('  proof of (n=0 ∨ n≠0) for n=5:  inr(proof(5≠0))')}")
    print(f"  {cyan('  proof of (n=0 ∨ n≠0) for n=0:  inl(proof(0=0))')}")
    print()
    print(f"  {bold(green('Proof terms:'))}")
    print(f"  {cyan('  ∨I₁ : A → A∨B    = λa. inl a')}")
    print(f"  {cyan('  ∨I₂ : B → A∨B    = λb. inr b')}")
    print(f"  {cyan('  ∨E  : A∨B → (A→C) → (B→C) → C  = case analysis')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Why disjunction is NOT classical:'))}\n")
    print(wrap(
        "Classically: n > 0 ∨ n ≤ 0 is always true by LEM — no computation needed. "
        "Constructively: you must decide WHICH case and produce evidence. "
        "For an arbitrary function f:ℕ→ℕ, proving f(n)>0 ∨ f(n)≤0 requires "
        "an algorithm to decide this — which may not exist."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 3: Implication and negation ───────────────────────────────────────

def _section_implication():
    clear()
    print(box("Implication and Negation: Functions and Absurdity"))
    print()
    print(f"  {bold(green('Implication A → B:'))}\n")
    print(f"  {cyan('Proof(A → B) = Proof(A) → Proof(B)  (function type)')}")
    print()
    print(wrap(
        "A proof of A → B is a FUNCTION that converts any proof of A into "
        "a proof of B. This must be a TOTAL, COMPUTABLE function — not just "
        "an assertion that such a function exists."
    ))
    print()
    print(f"  {bold('Example — modus ponens as function application:')}")
    print(f"  {cyan('  p : A→B,   q : A   ⊢   p(q) : B')}")
    print()
    print(f"  {bold('Example — hypothetical proof:')}")
    print(f"  {cyan('  proof of (n>0 → n+1>0):')}")
    print(f"  {cyan('  λh. succ_pos h   -- given proof h of n>0, produce proof of n+1>0')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Negation ¬A = A → ⊥:'))}\n")
    print(wrap(
        "Negation is not a primitive — it is DEFINED as implication into ⊥. "
        "A proof of ¬A is a function that would convert any proof of A into "
        "a proof of ⊥ (the empty type). Since ⊥ has no proof, this means "
        "A cannot be proved — providing a REFUTATION of A."
    ))
    print()
    print(f"  {cyan('¬A  :≡  A → ⊥')}")
    print(f"  {cyan('Proof(¬A) = Proof(A) → Proof(⊥) = Proof(A) → ∅')}")
    print()
    print(f"  {bold('Ex falso quodlibet:')}")
    print(f"  {cyan('  exfalso : ⊥ → A   (the empty function; no cases to handle)')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The double negation gap:'))}\n")
    print(wrap(
        "A → ¬¬A is provable (BHK): given proof p of A, return λf. f(p) "
        "— the function that takes a refutation f of A and applies it to p. "
        "This gives a proof of ⊥. So we have turned p : A into a proof of ¬¬A."
    ))
    print()
    print(f"  {cyan('dn : A → ¬¬A')}")
    print(f"  {cyan('dn p = λf. f p     -- f : A→⊥, p : A, so f p : ⊥')}")
    print()
    print(wrap(
        "But ¬¬A → A is NOT provable in general. A proof of ¬¬A is a function "
        "that takes a function f:A→⊥ and produces ⊥. From this, you cannot "
        "in general extract an element of A — there's no computational content."
    ))
    print()
    print(f"  {red('no proof of: ¬¬A → A  (double negation elimination)')}")
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 4: Quantifiers ────────────────────────────────────────────────────

def _section_quantifiers():
    clear()
    print(box("Quantifiers: Dependent Products and Sums"))
    print()
    print(f"  {bold(green('Universal quantifier ∀x:A. P(x):'))}\n")
    print(f"  {cyan('Proof(∀x:A. P(x)) = Π(x:A). Proof(P(x))')}")
    print()
    print(wrap(
        "A proof of ∀x:A. P(x) is a FUNCTION that, for each concrete a:A, "
        "produces a proof of P(a). This is a dependent function — its return "
        "type P(a) depends on the input a."
    ))
    print()
    print(f"  {bold('Example — ∀n:ℕ. n+0 = n:')}")
    print(f"  {cyan('  proof : Π(n:ℕ). n+0=n')}")
    print(f"  {cyan('  proof 0     = refl')}")
    print(f"  {cyan('  proof (S n) = ap succ (proof n)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Existential quantifier ∃x:A. P(x):'))}\n")
    print(f"  {cyan('Proof(∃x:A. P(x)) = Σ(x:A). Proof(P(x))')}")
    print()
    print(wrap(
        "A proof of ∃x:A. P(x) is a PAIR (a, p) where a:A is the explicit "
        "WITNESS and p is a proof of P(a). The witness must be concrete — "
        "not just 'some a exists' but 'HERE IS THE a, and here is the proof'."
    ))
    print()
    print(f"  {bold('Example — ∃n:ℕ. n>100:')}")
    print(f"  {cyan('  proof = (101, proof(101>100))')}")
    print(f"  {cyan('  fst proof = 101   -- you can extract the witness!')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The ∃ vs. mere ∃ distinction (again):'))}\n")
    print(wrap(
        "In HoTT, the Σ-type is 'constructive existence' with an extractable witness. "
        "The propositional truncation ||Σ(x:A). P(x)|| is 'mere existence' — "
        "you only know a witness exists but cannot extract it. "
        "The BHK ∃ corresponds to Σ, not to ||Σ||."
    ))
    print()
    print(f"  {bold('BHK table summary:')}\n")
    rows = [
        ("⊤",        "𝟙",              "Unit type"),
        ("⊥",        "𝟘",              "Empty type"),
        ("A ∧ B",    "A × B",          "Product type"),
        ("A ∨ B",    "A + B",          "Sum type (tagged union)"),
        ("A → B",    "A → B",          "Function type"),
        ("¬A",       "A → 𝟘",          "Function into empty"),
        ("∀x.P(x)", "Π(x:A). P(x)",   "Dependent function"),
        ("∃x.P(x)", "Σ(x:A). P(x)",   "Dependent pair (with witness)"),
    ]
    for logic, type_, note in rows:
        print(f"  {bold(cyan(logic)):18} ↔  {cyan(type_):25} {dim(note)}")
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 5: Why LEM has no BHK proof ───────────────────────────────────────

def _section_lem():
    clear()
    print(box("Why LEM Has No BHK Proof"))
    print()
    print(wrap(
        "The Law of Excluded Middle LEM = ∀P. P ∨ ¬P asserts that for every "
        "proposition P, either P holds or ¬P holds. Under BHK:"
    ))
    print()
    print(f"  {cyan('BHK proof of LEM = function f such that:')}")
    print(f"  {cyan('  for every P, f(P) is either inl(proof of P) or inr(proof of ¬P)')}")
    print()
    print(wrap(
        "But this requires a DECISION PROCEDURE for arbitrary propositions: "
        "an algorithm that, given any P, decides whether P or ¬P holds. "
        "By Gödel's incompleteness / Rice's theorem / the halting problem, "
        "no such algorithm exists."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The specific counterexample:'))}\n")
    print(wrap(
        "Let P = 'the Collatz conjecture is true'. Then LEM says: "
        "either Collatz is true or Collatz is false. But we don't know which! "
        "A BHK proof of LEM would have to decide this — and we have no algorithm."
    ))
    print()
    print(f"  {cyan('f(Collatz) = ?  -- inl(proof Collatz true) or inr(proof Collatz false)')}")
    print(f"  {dim('No one knows which to produce.')}")
    print()
    print(rule())
    print(f"\n  {bold(green('What IS provable:'))}\n")

    provable = [
        ("¬¬(P ∨ ¬P)",   "λk. k (inr (λp. k (inl p)))",
         "Continuation argument — you can't refute LEM."),
        ("P → P ∨ Q",     "λp. inl p",
         "Trivial disjunction introduction."),
        ("¬(P ∧ ¬P)",     "λ(p, np). np p",
         "Contradiction from P and ¬P."),
        ("(P→Q)→(¬Q→¬P)", "λf g p. g (f p)",
         "Modus tollens — fully constructive."),
        ("¬¬¬P → ¬P",     "λk p. k (λnp. np p)",
         "Triple negation reduces."),
    ]

    for formula, proof, note in provable:
        print(f"  {bold(cyan(formula))}")
        print(f"    proof: {dim(proof)}")
        print(f"    {note}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('Markov principle (a restricted form of DNE):'))}\n")
    print(wrap(
        "Markov's principle: if P is decidable (we have f:ℕ→Bool) and "
        "¬¬∃n. f(n)=true, then ∃n. f(n)=true. This is constructively "
        "acceptable to some (Russian constructivism) but not others (MLTT). "
        "It says: if a search cannot fail, you can run it."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 6: Double negation translation ────────────────────────────────────

def _section_double_neg():
    clear()
    print(box("The Double Negation Translation"))
    print()
    print(wrap(
        "Every classically provable proposition can be translated into an "
        "intuitionistically provable one via the DOUBLE NEGATION TRANSLATION "
        "(Gödel-Gentzen 1933). This shows classical logic is consistent "
        "RELATIVE TO intuitionistic logic."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The translation φ → φᴺ:'))}\n")

    trans = [
        ("pᴺ",          "¬¬p                 (for atoms p)"),
        ("⊥ᴺ",          "⊥"),
        ("⊤ᴺ",          "⊤"),
        ("(A∧B)ᴺ",      "Aᴺ ∧ Bᴺ"),
        ("(A∨B)ᴺ",      "¬(¬Aᴺ ∧ ¬Bᴺ)        (not  ¬Aᴺ ∨ ¬Bᴺ!)"),
        ("(A→B)ᴺ",      "Aᴺ → Bᴺ"),
        ("(¬A)ᴺ",       "¬Aᴺ"),
        ("(∀x.P)ᴺ",     "∀x. Pᴺ"),
        ("(∃x.P)ᴺ",     "¬∀x. ¬Pᴺ"),
    ]

    for form, result in trans:
        print(f"  {bold(cyan(form)):20}  {result}")

    print()
    print(rule())
    print(f"\n  {bold(green('Key theorem:'))}\n")
    print(f"  {cyan('If  ⊢_classical φ  then  ⊢_intuitionistic φᴺ')}")
    print()
    print(wrap(
        "Every classically valid formula, translated via ᴺ, becomes "
        "intuitionistically provable. In particular:"
    ))
    print()
    examples = [
        ("LEM",     "P ∨ ¬P",     "¬¬(P ∨ ¬P)       — intuitionistically valid ✓"),
        ("DNE",     "¬¬P → P",    "¬¬¬¬P → ¬¬P      — valid (¬¬ is idempotent) ✓"),
        ("Peirce",  "((P→Q)→P)→P","(¬¬(¬¬P→¬¬Q)→¬¬P)→¬¬P — valid ✓"),
    ]
    for name, classical, translated in examples:
        print(f"  {bold(name)}: {classical}")
        print(f"    ᴺ: {translated}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('Practical significance:'))}\n")
    print(wrap(
        "The double negation translation shows classical mathematics can be "
        "EMBEDDED into constructive mathematics. If you accept ¬¬-stability "
        "(that ¬¬P → P for the propositions you care about), you get classical "
        "reasoning. In HoTT, the propositions (h-level -1) behave classically "
        "if you postulate LEM for them — this is consistent with HoTT."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("intro",   "BHK: proofs as constructions",                  _section_intro),
    ("conj",    "Conjunction and disjunction: pairs and sums",   _section_conjunction),
    ("imp",     "Implication and negation: functions and ⊥",     _section_implication),
    ("quant",   "Quantifiers: Π-types and Σ-types",              _section_quantifiers),
    ("lem",     "Why LEM has no BHK proof",                      _section_lem),
    ("dnt",     "The double negation translation",               _section_double_neg),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("The Brouwer-Heyting-Kolmogorov Interpretation", width=70))
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
