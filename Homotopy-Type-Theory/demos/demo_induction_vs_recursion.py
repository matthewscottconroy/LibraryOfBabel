#!/usr/bin/env python3
"""
Induction vs. Recursion: The Motive Makes the Difference
=========================================================
The recursor (non-dependent eliminator) vs. the inductor (dependent eliminator).

Both define functions out of inductive types by case analysis, but they differ
in what the RETURN TYPE can be. Recursion returns a fixed type; induction
returns a type that can depend on the element. The motive is what makes the
difference between computation and proof.
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


def _section_rec_vs_ind():
    clear()
    print(box("Recursor vs. Inductor: What Is the Difference?"))
    print()
    print(wrap(
        "For the natural numbers ℕ, there are two elimination principles. "
        "Both handle the zero and successor cases, but their types differ."
    ))
    print()
    print(f"  {bold(green('Recursor (non-dependent):'))}")
    print(f"  {cyan('rec_ℕ : C → (ℕ → C → C) → ℕ → C')}")
    print(f"  {cyan('rec_ℕ z s 0       = z')}")
    print(f"  {cyan('rec_ℕ z s (succ n) = s n (rec_ℕ z s n)')}")
    print()
    print(wrap(
        "The return type C is FIXED and does not depend on the natural number. "
        "This is enough for computing functions ℕ → C."
    ))
    print()
    print(f"  {bold(yellow('Inductor (dependent eliminator):'))}")
    print(f"  {cyan('ind_ℕ : (P : ℕ → Type) → P 0 → (∀n. P n → P (succ n)) → ∀n. P n')}")
    print(f"  {cyan('ind_ℕ P z s 0       = z')}")
    print(f"  {cyan('ind_ℕ P z s (succ n) = s n (ind_ℕ P z s n)')}")
    print()
    print(wrap(
        "The return type P(n) DEPENDS ON n — the MOTIVE P is a TYPE FAMILY. "
        "This is required for proofs, where the statement to be proved depends "
        "on the specific natural number."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Relationship:'))}\n")
    print(f"  {dim('rec_ℕ = ind_ℕ (const C)   (motive is constant family)')}")
    print()
    print(wrap(
        "The recursor is a special case of the inductor where P = λn. C is "
        "constant. Every function definable by rec is definable by ind. "
        "But ind is strictly more powerful."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_what_rec_can_do():
    clear()
    print(box("What the Recursor Can Do: Computing Functions"))
    print()
    print(wrap(
        "The recursor is enough to define all PRIMITIVE RECURSIVE functions: "
        "functions computed by structural recursion on ℕ."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Addition: rec_ℕ 0 (λn r. succ r) m'))}\n")
    print(f"  {cyan('add : ℕ → ℕ → ℕ')}")
    print(f"  {cyan('add m n = rec_ℕ n (λ_ r. succ r) m')}")
    print()
    def rec_add(m, n):
        if m == 0: return n
        return rec_add(m-1, n) + 1
    for a, b in [(3,4),(0,5),(7,0)]:
        print(f"  add {a} {b} = {rec_add(a,b)}")

    print()
    print(f"\n  {bold(green('Multiplication: rec_ℕ 0 (λn r. add n r) m'))}\n")
    print(f"  {cyan('mul : ℕ → ℕ → ℕ')}")
    print(f"  {cyan('mul m n = rec_ℕ 0 (λ_ r. add n r) m')}")
    print()
    def rec_mul(m, n):
        if m == 0: return 0
        return n + rec_mul(m-1, n)
    for a, b in [(3,4),(0,5),(2,7)]:
        print(f"  mul {a} {b} = {rec_mul(a,b)}")

    print()
    print(f"\n  {bold(green('Fibonacci: rec_ℕ (0,1) (λ_ (a,b). (b, a+b)) n'))}\n")
    print(f"  {cyan('fib n = fst (rec_ℕ (0,1) (λ_ (a,b). (b, a+b)) n)')}")
    print()
    def rec_fib(n):
        a, b = 0, 1
        for _ in range(n):
            a, b = b, a+b
        return a
    for k in range(10):
        print(f"  fib {k} = {rec_fib(k)}", end="   ")
    print()
    print()
    print(rule())
    print(f"\n  {bold(yellow('What rec CANNOT do: proofs about computation'))}\n")
    print(wrap(
        "The recursor cannot prove n + 0 = n, because the TYPE of the proof "
        "depends on n. We need P(n) = (n + 0 = n), which varies with n. "
        "rec_ℕ requires a FIXED return type — so the motive P cannot appear."
    ))
    print()
    print(f"  {red('rec_ℕ refl (λn r. ?) n  -- what goes in ?? return type is fixed')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_what_ind_can_do():
    clear()
    print(box("What the Inductor Can Do: Proofs by Induction"))
    print()
    print(wrap(
        "The inductor (dependent eliminator) allows the return type to depend "
        "on the number being eliminated. This enables PROOFS BY INDUCTION."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Example: proving n + 0 = n'))}\n")
    print(f"  {bold('Motive:')}")
    print(f"  {cyan('P : ℕ → Type')}")
    print(f"  {cyan('P n = (n + 0 = n)')}")
    print()
    print(f"  {bold('Base case (n=0):')}")
    print(f"  {cyan('z : P 0 = (0 + 0 = 0)')}")
    print(f"  {cyan('z = refl      -- 0+0 reduces to 0 definitionally')}")
    print()
    print(f"  {bold('Inductive step:')}")
    print(f"  {cyan('s : ∀n. P n → P (succ n)')}")
    print(f"  {cyan('s n ih = ap succ ih')}")
    print(f"  {dim('  -- ih : n+0=n, apply succ to both sides')}")
    print()
    print(f"  {bold('Conclusion:')}")
    print(f"  {cyan('ind_ℕ P z s : ∀n. n + 0 = n')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The key: why this needs induction, not recursion'))}\n")
    print(wrap(
        "At n=5, P(5) = (5+0=5). At n=0, P(0) = (0+0=0). "
        "These are DIFFERENT TYPES — a function P:ℕ→Type. The recursor "
        "requires a single fixed type C; it cannot handle a varying P."
    ))
    print()
    print(f"  {'n':6}  {'P(n) = (n+0=n)':30}  {'proof'}")
    print(f"  {dim('─'*55)}")
    for n in range(6):
        statement = f"({n}+0={n})"
        proof = "refl" if n == 0 else f"ap succ (proof({n-1}+0={n-1}))"
        print(f"  {n:<6}  {statement:30}  {dim(proof)}")
    print()
    print(rule())
    print(f"\n  {bold(green('More induction proofs:'))}\n")

    proofs = [
        ("succ_ne_zero",  "∀n. succ n ≠ 0",
         "P n = (succ n ≠ 0);  base: trivial (n not succ);  step: easy"),
        ("add_comm",      "∀m n. m + n = n + m",
         "Double induction. Inner P n = (m+n=n+m); outer P m = (∀n. m+n=n+m)"),
        ("le_refl",       "∀n. n ≤ n",
         "P n = (n ≤ n);  base: 0≤0;  step: if n≤n then succ n ≤ succ n"),
        ("strong_ind",    "∀n. P n  if  ∀n. (∀m<n. P m) → P n",
         "Strong induction follows from ordinary induction on a 'cumulant'"),
    ]

    for name, stmt, hint in proofs:
        print(f"  {bold(cyan(name))}: {stmt}")
        print(f"    {dim(hint)}")
        print()

    input(bold("  Press Enter to continue... "))


def _section_motive():
    clear()
    print(box("The Motive: Making the Return Type Depend on the Argument"))
    print()
    print(wrap(
        "The MOTIVE P : ℕ → Type is the family of types returned by induction. "
        "Choosing the right motive is the key skill in dependent type theory "
        "proofs. A bad motive makes the proof impossible; the right motive "
        "makes it immediate."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Motive selection examples:'))}\n")

    cases = [
        ("Goal: n + 0 = n",
         "P n = (n + 0 = n)",
         "Motive is exactly the goal with n free."),
        ("Goal: m + n = n + m",
         "Outer: P m = (∀n. m+n=n+m)\nInner: Q n = (m+n=n+m)",
         "Double induction; the outer motive quantifies over n."),
        ("Goal: ∀n. n < n+1",
         "P n = (n < n+1)",
         "Direct; base: 0<1; step: if n<n+1 then succ n < succ(n+1)."),
        ("Goal: the predecessor function is total",
         "P 0 = 𝟙  and  P (succ n) = ℕ",
         "Different types in different cases! This is the 'no confusion' motive."),
        ("Goal: transport along n=m gives an isomorphism",
         "P n = (Vec n A ≃ Vec n A)",
         "Motive lives in the universe — induction on the INDEX."),
    ]

    for goal, motive, note in cases:
        print(f"  {bold(green(goal))}")
        for line in motive.split("\n"):
            print(f"  {cyan('  ' + line)}")
        print(f"    {dim(note)}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('The predecessor function: a surprising motive'))}\n")
    print(wrap(
        "Define pred : ℕ → ℕ by pred(0)=0 and pred(succ n)=n. "
        "The challenge: how do we prove that pred(succ n) = n? "
        "We cannot use refl because pred reduces to a case expression. "
        "The trick is a motive that returns DIFFERENT TYPES in the two cases:"
    ))
    print()
    print(f"  {cyan('P : ℕ → Type')}")
    print(f"  {cyan('P 0       = 𝟙   (unit: zero case is trivial)')}")
    print(f"  {cyan('P (succ n) = ℕ   (we want to return n)')}")
    print()
    print(f"  {cyan('ind_ℕ P (tt) (λn _. n) : ∀m. P m')}")
    print(f"  {dim('  -- at (succ n): returns n, which is of type P(succ n) = ℕ')}")
    print()
    print(wrap(
        "This is the standard technique for defining predecessor without "
        "a separate 'no confusion' axiom. The motive IS the case distinction."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_j_eliminator():
    clear()
    print(box("The J Eliminator: Path Induction"))
    print()
    print(wrap(
        "The J eliminator is the inductor for the IDENTITY TYPE a = b. "
        "It is the fundamental tool for all path reasoning in HoTT."
    ))
    print()
    print(f"  {bold(green('J (path induction):'))}")
    print(f"  {cyan('J : (C : ∀(a b:A). a=b → Type)')}")
    print(f"  {cyan('  → (∀a. C a a refl)')}")
    print(f"  {cyan('  → ∀(a b:A)(p: a=b). C a b p')}")
    print()
    print(wrap(
        "The motive C depends on BOTH endpoints and the path. "
        "The only case to handle is when a=b and the path is refl. "
        "J then handles any path p:a=b by 'reducing' to the refl case."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Symmetry: p → p⁻¹'))}\n")
    print(f"  {cyan('symm : a=b → b=a')}")
    print(f"  {cyan('symm p = J (λa b _. b=a) (λa. refl) a b p')}")
    print(f"  {dim('  Motive C a b p = (b=a)')}")
    print(f"  {dim('  Base case:  C a a refl = (a=a), proved by refl')}")
    print()
    print(f"\n  {bold(green('Transitivity: p · q'))}\n")
    print(f"  {cyan('trans : a=b → b=c → a=c')}")
    print(f"  {cyan('trans p q = J (λa b _. b=c → a=c) (λa r. r) a b p q')}")
    print(f"  {dim('  Motive C a b p = (b=c → a=c)')}")
    print(f"  {dim('  Base: C a a refl = (a=c → a=c), proved by identity function')}")
    print()
    print(f"\n  {bold(green('Transport'))}\n")
    print(f"  {cyan('transport : (P:A→Type) → a=b → P a → P b')}")
    print(f"  {cyan('transport P p = J (λa b _. P a → P b) (λa. id) a b p')}")
    print(f"  {dim('  Motive C a b p = (P a → P b)')}")
    print(f"  {dim('  Base: C a a refl = (P a → P a), proved by id')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('J is the ONLY axiom for path reasoning:'))}\n")
    print(wrap(
        "Every theorem about paths in Book HoTT — symmetry, transitivity, "
        "transport, ap, apd, path induction — is derivable from J alone. "
        "J is the path recursor; it says paths behave like an inductive type "
        "with one constructor (refl) and one base case."
    ))
    print()
    print(wrap(
        "This is why 'computation' in Book HoTT can get stuck: J does not "
        "reduce on non-refl paths definitionally. Cubical HoTT replaces J "
        "with the interval and hcomp, which compute on ALL paths."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("compare", "Recursor vs. inductor: the key difference",       _section_rec_vs_ind),
    ("rec",     "What the recursor can do: computing functions",   _section_what_rec_can_do),
    ("ind",     "What the inductor can do: proofs by induction",   _section_what_ind_can_do),
    ("motive",  "The motive: choosing the right return type",      _section_motive),
    ("j",       "The J eliminator: path induction",                _section_j_eliminator),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Induction vs. Recursion: The Motive Makes the Difference", width=70))
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
