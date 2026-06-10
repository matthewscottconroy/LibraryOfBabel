#!/usr/bin/env python3
"""
Normal Forms: Beta, Eta, and Definitional Equality
====================================================
Reduction rules, Church-Rosser, stuck terms, and what 'computes' in HoTT.

In type theory, two expressions are DEFINITIONALLY EQUAL if they reduce to
the same NORMAL FORM — a term that cannot be reduced further. The reduction
system must be confluent (Church-Rosser) and terminating for type checking
to be decidable.
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


# ── Simple lambda term evaluator ──────────────────────────────────────────────

class Expr:
    pass

class Var(Expr):
    def __init__(self, name): self.name = name
    def __repr__(self): return self.name

class Lam(Expr):
    def __init__(self, var, body): self.var = var; self.body = body
    def __repr__(self): return f"(λ{self.var}. {self.body})"

class App(Expr):
    def __init__(self, func, arg): self.func = func; self.arg = arg
    def __repr__(self): return f"({self.func} {self.arg})"

class Nat(Expr):
    def __init__(self, n): self.n = n
    def __repr__(self): return str(self.n)

def subst(expr, var, val):
    if isinstance(expr, Var):
        return val if expr.name == var else expr
    if isinstance(expr, Lam):
        if expr.var == var:
            return expr
        return Lam(expr.var, subst(expr.body, var, val))
    if isinstance(expr, App):
        return App(subst(expr.func, var, val), subst(expr.arg, var, val))
    return expr

def beta_reduce_once(expr):
    """Return (reduced_expr, did_reduce)."""
    if isinstance(expr, App):
        if isinstance(expr.func, Lam):
            result = subst(expr.func.body, expr.func.var, expr.arg)
            return result, True
        new_func, r1 = beta_reduce_once(expr.func)
        if r1:
            return App(new_func, expr.arg), True
        new_arg, r2 = beta_reduce_once(expr.arg)
        if r2:
            return App(expr.func, new_arg), True
    if isinstance(expr, Lam):
        new_body, r = beta_reduce_once(expr.body)
        if r:
            return Lam(expr.var, new_body), True
    return expr, False

def normalize(expr, max_steps=20):
    steps = [repr(expr)]
    for _ in range(max_steps):
        new_expr, reduced = beta_reduce_once(expr)
        if not reduced:
            break
        expr = new_expr
        steps.append(repr(expr))
    return expr, steps


# ── Sections ──────────────────────────────────────────────────────────────────

def _section_definitional():
    clear()
    print(box("Definitional Equality: Computation as Proof"))
    print()
    print(wrap(
        "In type theory, there are two notions of equality:"
    ))
    print()
    print(f"  {bold(green('Definitional equality (≡):'))}")
    print(f"  {cyan('  a ≡ b  iff  a and b reduce to the same normal form')}")
    print(f"  {dim('  Checked algorithmically by the type checker.')}")
    print(f"  {dim('  No proof term needed — it is TRUE BY COMPUTATION.')}")
    print()
    print(f"  {bold(yellow('Propositional equality (=):'))}")
    print(f"  {cyan('  a = b  is a TYPE; a proof of a=b is a TERM of that type')}")
    print(f"  {dim('  May require a non-trivial proof.')}")
    print(f"  {dim('  Examples: symmetry, transitivity, induction arguments.')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Examples of definitional equalities:'))}\n")
    defeqs = [
        ("(λx. x) a ≡ a",           "β-reduction: apply identity"),
        ("(λx. f x) ≡ f",           "η-expansion: functions equal their η-expansions"),
        ("fst (a, b) ≡ a",           "projection reduction"),
        ("rec_ℕ z s 0 ≡ z",          "recursor base case"),
        ("rec_ℕ z s (succ n) ≡ s n (rec_ℕ z s n)", "recursor step"),
        ("0 + 3 ≡ 3",               "add reduces on first arg"),
        ("2 + 2 ≡ 4",               "arithmetic computes"),
        ("if true then a else b ≡ a", "conditional reduces"),
    ]
    for eq, note in defeqs:
        print(f"  {bold(cyan(eq))}")
        print(f"    {dim(note)}")
        print()
    print(rule())
    print(f"\n  {bold(yellow('Examples of propositional (not definitional) equalities:'))}\n")
    propeqs = [
        ("n + 0 = n",            "requires induction on n  (0+n≡n but n+0 needs proof)"),
        ("n + m = m + n",        "commutativity: requires double induction"),
        ("f = g  (functions)",   "requires funext (not definitional in Book HoTT)"),
        ("p · refl = p  (paths)","right unit: requires path induction (not refl)"),
        ("A = B  (types)",       "requires ua(e) for some equivalence e  (univalence)"),
    ]
    for eq, note in propeqs:
        print(f"  {red('→')} {eq}")
        print(f"    {dim(note)}")
        print()
    input(bold("  Press Enter to continue... "))


def _section_beta():
    clear()
    print(box("Beta Reduction: Function Application"))
    print()
    print(wrap(
        "Beta reduction is the fundamental computation rule: applying a lambda "
        "abstraction to an argument substitutes the argument for the variable."
    ))
    print()
    print(f"  {bold(green('Rule:'))}")
    print(f"  {cyan('(λx. M) N  →β  M[N/x]   (substitute N for x in M)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Step-by-step reductions:'))}\n")

    # Identity
    print(f"  {bold('Identity function:')}")
    e1 = App(Lam('x', Var('x')), Nat(42))
    _, steps = normalize(e1)
    for i, s in enumerate(steps):
        arrow = "→β" if i > 0 else "  "
        print(f"    {arrow}  {s}")
    print()

    # Constant function
    print(f"  {bold('Constant function:')}")
    e2 = App(App(Lam('x', Lam('y', Var('x'))), Nat(1)), Nat(2))
    _, steps = normalize(e2)
    for i, s in enumerate(steps):
        arrow = "→β" if i > 0 else "  "
        print(f"    {arrow}  {s}")
    print()

    # Composition
    print(f"  {bold('(λf. λg. λx. f (g x)) succ succ 0:')}")
    succ = Lam('n', App(Var('succ'), Var('n')))
    comp = App(App(App(Lam('f', Lam('g', Lam('x', App(Var('f'), App(Var('g'), Var('x')))))),
                      succ), succ), Nat(0))
    _, steps = normalize(comp)
    for i, s in enumerate(steps[:6]):
        arrow = "→β" if i > 0 else "  "
        print(f"    {arrow}  {s}")
    print()

    print(rule())
    print(f"\n  {bold(yellow('Church-Rosser (confluence):'))}\n")
    print(wrap(
        "Reduction is CONFLUENT: if M →* N₁ and M →* N₂ by different "
        "reduction sequences, then there exists P with N₁ →* P and N₂ →* P. "
        "This means the ORDER of reduction doesn't matter — all paths lead "
        "to the same normal form."
    ))
    print()
    print(f"  {bold('Example: two different reductions of (λx. (λy. y) x) a:')}")
    e = App(Lam('x', App(Lam('y', Var('y')), Var('x'))), Nat(5))
    print(f"    {repr(e)}")
    print(f"  {dim('  Path 1: reduce outer beta first')}")
    print(f"    →β  {repr(App(Lam('y', Var('y')), Nat(5)))}")
    print(f"    →β  5")
    print(f"  {dim('  Path 2: reduce inner beta first')}")
    print(f"    →β  {repr(App(Lam('x', Var('x')), Nat(5)))}")
    print(f"    →β  5")
    print(f"  {green('Both paths reach the same normal form: 5')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_eta():
    clear()
    print(box("Eta Expansion and Reduction"))
    print()
    print(wrap(
        "Eta (η) says that a function is equal to its eta-expansion — the "
        "lambda that takes an argument and applies the function to it."
    ))
    print()
    print(f"  {bold(green('Rule (η-reduction):'))}")
    print(f"  {cyan('λx. f x  →η  f   (if x not free in f)')}")
    print()
    print(f"  {bold(green('Rule (η-expansion):'))}")
    print(f"  {cyan('f  →η  λx. f x')}")
    print()
    print(wrap(
        "Eta makes functions EXTENSIONAL in the definitional equality: "
        "f = (λx. f x) definitionally. This is the definitional version "
        "of function extensionality for one input."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Eta for products:'))}\n")
    print(f"  {cyan('η-pair:  p ≡ (fst p, snd p)')}")
    print()
    print(wrap(
        "Any pair p is definitionally equal to the pair of its projections. "
        "Together with β for pairs (fst(a,b) ≡ a, snd(a,b) ≡ b), this makes "
        "the product type extensional: two pairs with the same projections are equal."
    ))
    print()
    print(f"\n  {bold(green('Eta for the identity type:'))}\n")
    print(f"  {cyan('η-path:  p ≡ J(C, c, p)   (when C a a refl ≡ c a)')}")
    print()
    print(wrap(
        "The J eliminator has an η-rule: any proof p of a=b equals the "
        "result of eliminating it with the identity motive. This makes "
        "path induction a definitional computation rule."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('η in practice — the asymmetry:'))}\n")
    print(wrap(
        "Most type theories include β but are more cautious about η. "
        "In Book HoTT, λ-types have both β and η definitionally. "
        "But the identity type does NOT have a useful η definitionally — "
        "because J requires matching the exact normal form of paths."
    ))
    print()
    print(f"  {'β for λ':25}  {green('definit. (always)')}")
    print(f"  {'η for λ':25}  {green('definit. (most systems)')}")
    print(f"  {'β for ℕ (rec)':25}  {green('definit. (always)')}")
    print(f"  {'β for identity (J)':25}  {green('definit. on refl only')}")
    print(f"  {'η for identity':25}  {red('propositional in Book HoTT')}")
    print(f"  {'η for identity':25}  {green('definit. in Cubical HoTT')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_stuck():
    clear()
    print(box("Stuck Terms: Where Computation Stops"))
    print()
    print(wrap(
        "A term is STUCK if it cannot reduce further but is not in normal form "
        "for semantic reasons. In Book HoTT, the univalence axiom introduces "
        "stuck terms — closed terms that neither reduce nor are values."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Normal terms that cannot reduce:'))}\n")

    normal = [
        ("λx. x",          "a lambda — irreducible, but a value"),
        ("0",               "zero — a constructor, irreducible"),
        ("succ (succ 0)",   "2 — constructor applied to constructor"),
        ("λx. λy. x",      "const function — value"),
    ]
    for term, note in normal:
        print(f"  {bold(green('✓'))} {cyan(term):<30}  {dim(note)}")
    print()

    print(f"\n  {bold(red('Stuck terms under the univalence axiom:'))}\n")

    stuck = [
        ("transport id (ua e) a",
         "Book HoTT: cannot reduce because ua has no computation rule.",
         "Cubical HoTT: reduces to  e a  definitionally."),
        ("funext h",
         "Book HoTT: funext is an axiom; happly(funext h) x is stuck.",
         "Cubical HoTT: funext h = λi. λx. h x i — computes!"),
        ("J (λ_ _ _. ℕ) 0 p",
         "For p : a=b (non-refl), J applied to a non-refl path is stuck.",
         "In cubical: p is λi. ... and J computes by squeezing the interval."),
        ("transport (λX. X) (ua id_A) a",
         "Should equal a, but ua id_A is stuck without computation rule.",
         "Cubical: ua id_A = refl (definitionally), so transport is id."),
    ]

    for term, problem, cubical in stuck:
        print(f"  {bold(red('✗'))} {bold(term)}")
        print(f"    {red('Book HoTT:')} {dim(problem)}")
        print(f"    {green('Cubical:  ')} {dim(cubical)}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('Why this matters:'))}\n")
    print(wrap(
        "Stuck terms mean type checking can succeed but COMPUTATION is "
        "incomplete. A proof of ∀n. transport ... n = n may typecheck but "
        "the computation 'transport ... 5 = 5' cannot be verified by "
        "normalization — the proof term is stuck. In Cubical HoTT, "
        "all these terms compute, making the system CANONICAL."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_canonicity():
    clear()
    print(box("Canonicity: Every Closed Term Has a Value"))
    print()
    print(wrap(
        "A type theory is CANONICAL if every closed term of a base type "
        "reduces to a constructor. For ℕ: every closed term of type ℕ "
        "reduces to 0 or succ(succ(...(0)...))."
    ))
    print()
    print(f"  {cyan('Canonicity: ∀(n : ℕ closed). n ≡ 0  or  n ≡ succ m  for some m')}")
    print()
    print(rule())
    print(f"\n  {bold(green('MLTT (no univalence) is canonical:'))}\n")
    print(wrap(
        "Without univalence, every closed term of type ℕ reduces to a numeral. "
        "This follows from normalization: terms have normal forms, and the only "
        "normal forms of type ℕ are numerals. Type checking is decidable."
    ))
    print()
    print(f"\n  {bold(red('Book HoTT with univalence axiom is NOT canonical:'))}\n")
    print(wrap(
        "The univalence axiom adds a 'constant' ua with no computation rule. "
        "One can construct closed terms of type ℕ that are stuck — they "
        "do not reduce to any numeral. This breaks canonicity."
    ))
    print()
    print(f"  {dim('Sketch: define n = transport (code) (ua flip) 0')}")
    print(f"  {dim('This should equal 1 (flip 0 = 1 under the code family)')}")
    print(f"  {dim('But transport (code) (ua flip) is stuck — ua has no rule')}")
    print(f"  {dim('So n is a closed term of type ℕ that is stuck.')}")
    print()
    print(f"\n  {bold(green('Cubical HoTT IS canonical:'))}\n")
    print(wrap(
        "Cubical Type Theory restores canonicity: every closed term normalizes. "
        "The Glue type gives ua a computation rule, so transport along ua "
        "actually reduces. Cohen-Coquand-Huber-Mörtberg proved this in 2018."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Summary: the computation landscape'))}\n")

    rows = [
        ("Simply Typed LC",      "β (+ η)",        "Yes",   "Decidable"),
        ("System F",             "β (+ η)",        "Yes",   "Decidable"),
        ("MLTT",                 "β + ι (rec)",    "Yes",   "Decidable"),
        ("Book HoTT",            "β + ι + J-refl", "No",    "Semi-dec."),
        ("Cubical HoTT",         "β + ι + hcomp",  "Yes",   "Decidable"),
    ]

    print(f"  {'System':22}  {'Rules':20}  {'Canonical':12}  {'Type checking'}")
    print(f"  {dim('─'*68)}")
    for system, rules, canon, tc in rows:
        c = green(canon) if canon == "Yes" else red(canon)
        print(f"  {bold(system):30}  {rules:20}  {c:20}  {dim(tc)}")
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("defn",      "Definitional vs. propositional equality",     _section_definitional),
    ("beta",      "Beta reduction and Church-Rosser",            _section_beta),
    ("eta",       "Eta expansion and reduction",                 _section_eta),
    ("stuck",     "Stuck terms: where computation stops",        _section_stuck),
    ("canonicity","Canonicity and the computation landscape",    _section_canonicity),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Normal Forms: Beta, Eta, and Definitional Equality", width=70))
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
