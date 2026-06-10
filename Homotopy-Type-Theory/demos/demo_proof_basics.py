#!/usr/bin/env python3
"""
Logic and Proof Foundations
============================
Propositional logic, proof techniques, mathematical induction,
and predicate logic — the bedrock beneath type theory and HoTT.
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


def _section_propositional():
    clear()
    print(box("Propositional Logic: Connectives and Truth"))
    print()
    print(wrap(
        "Propositional logic deals with propositions — statements that are "
        "true or false — connected by logical operators. These are the "
        "building blocks of all mathematical reasoning."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The basic connectives:'))}\n")
    connectives = [
        ("P ∧ Q",  "conjunction",    "P AND Q — true when both are true"),
        ("P ∨ Q",  "disjunction",    "P OR Q — true when at least one is true"),
        ("P → Q",  "implication",    "P IMPLIES Q — false only when P true, Q false"),
        ("¬P",     "negation",       "NOT P — flips truth value"),
        ("P ↔ Q",  "biconditional",  "P IFF Q — true when same truth value"),
        ("⊤",      "verum",          "always true"),
        ("⊥",      "absurdum",       "always false"),
    ]
    for symbol, name, desc in connectives:
        print(f"  {bold(cyan(symbol)):20} {yellow(name):18} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Truth table for implication P → Q:'))}\n")
    print(f"  {'P':6}  {'Q':6}  {'P → Q':10}  {'Note'}")
    print(f"  {dim('─'*55)}")
    rows = [
        ("T", "T", "T", "hypothesis satisfied, conclusion holds"),
        ("T", "F", "F", "the only failing case"),
        ("F", "T", "T", "vacuously true — hypothesis never fires"),
        ("F", "F", "T", "vacuously true — hypothesis never fires"),
    ]
    for p, q, imp, note in rows:
        color = green if imp == "T" else red
        print(f"  {p:6}  {q:6}  {color(imp):18}  {dim(note)}")
    print()
    print(wrap(
        "Key insight for HoTT: P → Q is the type of functions from P to Q. "
        "A proof of P → Q is a function converting any proof of P into a proof of Q. "
        "Implication IS function type."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_proof_techniques():
    clear()
    print(box("Proof Techniques"))
    print()
    print(wrap(
        "Mathematical proofs use several standard strategies. The right "
        "technique depends on the structure of what you are trying to prove."
    ))
    print()
    techniques = [
        ("Direct proof",
         "Assume hypotheses; chain logical steps to the conclusion.",
         "Prove n even → n² even: let n=2k, then n²=4k²=2(2k²). Done."),
        ("Contrapositive",
         "P→Q is equivalent to ¬Q→¬P. Often the contrapositive is easier.",
         "Prove n² odd → n odd. Contrapositive: n even → n² even (direct above)."),
        ("Contradiction",
         "Assume ¬P, derive ⊥; conclude P. Classical (uses LEM).",
         "Prove √2 irrational: assume √2=p/q reduced; 2q²=p², so both even. ↯"),
        ("Cases",
         "Split the goal into exhaustive cases; prove each.",
         "Prove |n|≥0: case n≥0 direct; case n<0 then |n|=-n>0."),
        ("Induction",
         "Prove P(0); prove P(n)→P(n+1); conclude ∀n.P(n).",
         "Prove Σᵢ₌₁ⁿ i = n(n+1)/2 for all n:ℕ."),
        ("Constructive",
         "Exhibit a witness — no excluded middle needed.",
         "Prove ∃n>100 prime: exhibit 101."),
    ]
    for name, strategy, example in techniques:
        print(f"  {bold(cyan(name))}")
        print(f"    {strategy}")
        print(f"    {dim(example)}")
        print()
    print(rule())
    print(f"\n  {bold(yellow('Classical vs constructive:'))}\n")
    print(wrap(
        "Classical logic allows proof by contradiction and double-negation "
        "elimination freely. Constructive (intuitionistic) logic requires "
        "a witness for existence claims — you must construct the object. "
        "HoTT is constructive; LEM is neither provable nor refutable."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_induction():
    clear()
    print(box("Mathematical Induction"))
    print()
    print(wrap(
        "Induction is the fundamental technique for statements about natural "
        "numbers. It works because ℕ is the LEAST set closed under zero and "
        "successor — there is nothing else."
    ))
    print()
    print(f"  {cyan('Induction principle:')}")
    print(f"  {cyan('  P(0)  →  (∀n. P(n) → P(n+1))  →  ∀n. P(n)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Live demo: Σᵢ₌₁ⁿ i = n(n+1)/2'))}\n")

    def partial_sum(n):
        return sum(range(1, n+1))

    def formula(n):
        return n * (n + 1) // 2

    print(f"  {'n':5}  {'Σᵢ₌₁ⁿ i':12}  {'n(n+1)/2':12}  {'Match?'}")
    print(f"  {dim('─'*45)}")
    for n in range(1, 12):
        s = partial_sum(n)
        f = formula(n)
        match = green("✓") if s == f else red("✗")
        print(f"  {n:5}  {s:12}  {f:12}  {match}")

    print()
    print(rule())
    print(f"\n  {bold(green('Strong induction:'))}\n")
    print(f"  {cyan('(∀m<n. P(m)) → P(n)  →  ∀n. P(n)')}")
    print()
    print(wrap(
        "Strong induction assumes P for ALL smaller values (not just n-1). "
        "Used to prove: every n>1 factors into primes; the Fibonacci sequence "
        "grows at most exponentially; etc."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('In type theory (the recursor):'))}\n")
    print(f"  {cyan('rec_ℕ : C(0) → (Πn. C(n) → C(n+1)) → Πn. C(n)')}")
    print()
    print(wrap(
        "When C is a proposition (Prop), rec_ℕ IS the induction principle. "
        "When C is a type family, rec_ℕ computes a value. "
        "Same principle — proof and computation unified."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_predicate():
    clear()
    print(box("Predicate Logic: Quantifiers"))
    print()
    print(wrap(
        "Predicate logic extends propositional logic with variables, "
        "predicates (P(x) is true/false depending on x), and quantifiers "
        "that range over a domain."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The quantifiers and their type-theoretic meanings:'))}\n")
    print(f"  {bold(cyan('∀x:A. P(x)'))}")
    print(f"    Classical: for all x in A, P(x) holds")
    print(f"    Type theory: {cyan('Π(x:A). P x')}   (dependent function type)")
    print(f"    Proof: a function f with f(x):P(x) for each x")
    print()
    print(f"  {bold(cyan('∃x:A. P(x)'))}")
    print(f"    Classical: there exists some x in A such that P(x) holds")
    print(f"    Type theory: {cyan('Σ(x:A). P x')}   (dependent pair type)")
    print(f"    Proof: a pair (a, p) where a:A and p:P(a)")
    print()
    print(rule())
    print(f"\n  {bold(green('Key inference rules:'))}\n")
    rules_list = [
        ("∀-intro",  "To prove ∀x.P(x): let x be arbitrary, prove P(x)"),
        ("∀-elim",   "From ∀x.P(x) and term t: derive P(t)  (function application)"),
        ("∃-intro",  "From P(t) for specific t: derive ∃x.P(x)  (provide witness)"),
        ("∃-elim",   "From ∃x.P(x): if P(x)→Q for any x, conclude Q"),
        ("→-intro",  "To prove P→Q: assume P, prove Q  (lambda abstraction)"),
        ("→-elim",   "From P→Q and P: derive Q  (modus ponens = function application)"),
    ]
    for name, desc in rules_list:
        print(f"  {bold(yellow(name)):20} {desc}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Classical vs intuitionistic logic:'))}\n")
    classical = [
        ("LEM",  "P ∨ ¬P",              "provable classically, NOT in HoTT"),
        ("DNE",  "¬¬P → P",             "follows from LEM; not constructive"),
        ("Peirce","((P→Q)→P)→P",        "equivalent to LEM"),
        ("DM",   "¬(P∧Q) ↔ ¬P∨¬Q",     "weaker form holds constructively"),
    ]
    for law, formula, status in classical:
        print(f"  {bold(yellow(law)):8} {cyan(formula):30} {dim(status)}")
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("prop",       "Propositional logic: connectives and truth",   _section_propositional),
    ("techniques", "Proof techniques: direct, contradiction, ...", _section_proof_techniques),
    ("induction",  "Mathematical induction",                       _section_induction),
    ("predicate",  "Predicate logic and quantifiers",              _section_predicate),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Logic and Proof Foundations", width=70))
        print()
        for i, (_, title, _fn) in enumerate(SECTIONS):
            marker = bold(cyan("▶")) if i == idx else " "
            print(f"  {marker} {bold(str(i+1))}   {title}")
        print()
        print(rule())
        print(f"  {dim('1-4  jump   n  next   p  prev   q  quit')}")
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
