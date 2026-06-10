#!/usr/bin/env python3
"""
demo_curry_howard.py — The Curry-Howard Correspondence

Propositions are types. Proofs are programs. The same thing.

  Proposition          Type               Proof term
  ─────────────────────────────────────────────────────
  A ∧ B           →   A × B          →   (a, b)
  A ∨ B           →   A + B          →   inl a  or  inr b
  A → B           →   A → B          →   λ x → f x
  ¬A              →   A → ⊥          →   λ x → absurd
  ⊤               →   Unit           →   tt
  ⊥               →   Empty          →   (no term)
  ∀ x:A. P(x)    →   Π (x:A). P(x)  →   λ x → p(x)
  ∃ x:A. P(x)    →   Σ (x:A). P(x)  →   (a, proof)

This demo lets you explore 10 propositions:
  • See the proposition as a type
  • See the proof term (the program that inhabits the type)
  • Run the program to see what it computes
  • Understand why classical tautologies FAIL intuitionistically

Commands
  1-0   jump to a proposition
  n/p   next / previous
  r     run the proof term as a program
  x     show the extractable program
  c     classical failures (LEM, DNE, Peirce)
  h     help
  q     quit
"""

from __future__ import annotations
import textwrap

# ── ANSI ──────────────────────────────────────────────────────────────────────

def _c(code, t): return f"\033[{code}m{t}\033[0m"
def bold(t):    return _c("1", t)
def green(t):   return _c("32", t)
def yellow(t):  return _c("33", t)
def cyan(t):    return _c("36", t)
def red(t):     return _c("31", t)
def dim(t):     return _c("2", t)
def magenta(t): return _c("35", t)

def clear(): print("\033[2J\033[H", end="")

def wrap(s, width=70, indent=2):
    prefix = " " * indent
    return textwrap.fill(s, width=width, initial_indent=prefix,
                         subsequent_indent=prefix)

# ── Propositions ──────────────────────────────────────────────────────────────

PROPS = [
    {
        "name": "Identity",
        "logic": "A → A",
        "type":  "A → A",
        "proof": "λ x → x",
        "haskell": "id :: a -> a\nid x = x",
        "python_fn": lambda x: x,
        "python_sig": "id(x) = x",
        "example": "id(42) = 42  |  id(True) = True  |  id('hello') = 'hello'",
        "explanation": (
            "The identity proposition 'A implies A' is inhabited by the identity "
            "function. This is both the simplest proof (a single introduction rule) "
            "and the simplest program (return your input unchanged). "
            "In Haskell this is 'id'; in Python 'lambda x: x'. "
            "There is exactly one proof of A → A up to definitional equality."
        ),
        "rule": "To prove A → A, assume A (intro rule) and immediately prove A (the assumption).",
    },
    {
        "name": "Composition (hypothetical syllogism)",
        "logic": "(A → B) → (B → C) → (A → C)",
        "type":  "(A → B) → (B → C) → A → C",
        "proof": "λ f → λ g → λ x → g (f x)",
        "haskell": "(.) :: (a -> b) -> (b -> c) -> a -> c\n(.) f g x = g (f x)",
        "python_fn": lambda f: lambda g: lambda x: g(f(x)),
        "python_sig": "compose(f)(g)(x) = g(f(x))",
        "example": "compose(str)(len)('hello') = len(str('hello'))  ... wait, try:\ncompose(lambda x: x+1)(lambda x: x*2)(3) = 8",
        "explanation": (
            "If A implies B and B implies C, then A implies C. "
            "The proof term is function composition: given f : A → B and g : B → C, "
            "construct g ∘ f : A → C by λ x → g(f x). "
            "This is exactly the (.) operator in Haskell. "
            "Logic's chain rule = programming's function composition. "
            "No other proof term has this type (up to eta-expansion)."
        ),
        "rule": "Intro rule for →: assume A, derive C. The derivation goes A →(f) B →(g) C.",
    },
    {
        "name": "Conjunction introduction (and-intro)",
        "logic": "A → B → A ∧ B",
        "type":  "A → B → A × B",
        "proof": "λ a → λ b → (a, b)",
        "haskell": "pair :: a -> b -> (a, b)\npair a b = (a, b)",
        "python_fn": lambda a: lambda b: (a, b),
        "python_sig": "pair(a)(b) = (a, b)",
        "example": "pair(3)(True) = (3, True)  |  pair('x')([1,2]) = ('x', [1,2])",
        "explanation": (
            "To prove A ∧ B, provide a proof of A and a proof of B. "
            "The proof term is a pair (a, b). "
            "This is the tuple constructor. "
            "Conjunction = product type. "
            "The two elimination rules for ∧ are fst and snd (projections)."
        ),
        "rule": "∧-intro: from a : A and b : B, derive (a, b) : A × B.",
    },
    {
        "name": "Conjunction elimination (and-elim)",
        "logic": "A ∧ B → A   and   A ∧ B → B",
        "type":  "A × B → A        A × B → B",
        "proof": "fst = λ (a, b) → a        snd = λ (a, b) → b",
        "haskell": "fst :: (a, b) -> a\nfst (a, _) = a\n\nsnd :: (a, b) -> b\nsnd (_, b) = b",
        "python_fn": lambda p: (lambda q: q[0])(p),
        "python_sig": "fst(a, b) = a   |   snd(a, b) = b",
        "example": "fst((3, True)) = 3  |  snd(('x', [1,2])) = [1,2]",
        "explanation": (
            "From a proof of A ∧ B, extract a proof of A (or B). "
            "The proof terms are the projection functions fst and snd. "
            "Conjunction = product type, so ∧-elim = tuple projection. "
            "These are the two canonical programs you can write with a pair: "
            "take the first component or take the second."
        ),
        "rule": "∧-elim₁: from (a, b) : A × B, derive a : A (project left).",
    },
    {
        "name": "Commutativity of conjunction",
        "logic": "A ∧ B → B ∧ A",
        "type":  "A × B → B × A",
        "proof": "λ (a, b) → (b, a)",
        "haskell": "swap :: (a, b) -> (b, a)\nswap (a, b) = (b, a)",
        "python_fn": lambda p: (p[1], p[0]),
        "python_sig": "swap(a, b) = (b, a)",
        "example": "swap((3, True)) = (True, 3)  |  swap(('x', 42)) = (42, 'x')",
        "explanation": (
            "Commutativity of conjunction corresponds to the swap function on pairs. "
            "There is exactly one proof of A ∧ B → B ∧ A, and it is swap. "
            "This illustrates parametric polymorphism: the proof term works "
            "for any types A and B without knowing what they are. "
            "In Haskell, the type 'swap :: (a, b) -> (b, a)' has exactly one "
            "implementation up to identity: Reynolds's parametricity theorem."
        ),
        "rule": "Apply ∧-elim to get a : A and b : B, then ∧-intro to build (b, a) : B × A.",
    },
    {
        "name": "Curry (currying)",
        "logic": "(A ∧ B → C) → A → B → C",
        "type":  "(A × B → C) → A → B → C",
        "proof": "λ f → λ a → λ b → f (a, b)",
        "haskell": "curry :: ((a, b) -> c) -> a -> b -> c\ncurry f a b = f (a, b)",
        "python_fn": lambda f: lambda a: lambda b: f((a, b)),
        "python_sig": "curry(f)(a)(b) = f((a, b))",
        "example": "curry(lambda p: p[0] + p[1])(3)(4) = 7",
        "explanation": (
            "Currying converts a function that takes a pair into a function "
            "that takes its arguments one at a time. "
            "In logic: if A ∧ B implies C, then A implies (B implies C). "
            "This is the adjunction hom(A×B, C) ≅ hom(A, B→C). "
            "Currying is the computational manifestation of this adjunction. "
            "Its inverse is uncurrying: λ f → λ (a,b) → f a b."
        ),
        "rule": "The adjunction between × and →: (A×B → C) ≅ (A → B → C).",
    },
    {
        "name": "Disjunction introduction",
        "logic": "A → A ∨ B   and   B → A ∨ B",
        "type":  "A → A + B        B → A + B",
        "proof": "inl = λ a → Left a        inr = λ b → Right b",
        "haskell": "-- Haskell Either:\nleft  :: a -> Either a b\nleft = Left\n\nright :: b -> Either a b\nright = Right",
        "python_fn": lambda a: ("Left", a),
        "python_sig": "inl(a) = ('Left', a)   |   inr(b) = ('Right', b)",
        "example": "inl(3) = ('Left', 3)  |  inr(True) = ('Right', True)",
        "explanation": (
            "To prove A ∨ B, prove A (and tag it 'left') or prove B (and tag it 'right'). "
            "The proof terms are the two constructors of the sum type: inl and inr. "
            "Disjunction = sum type = Either in Haskell. "
            "Note: classical logic says A ∨ ¬A. This would require us to say "
            "which one (A or ¬A) we can prove — but for arbitrary A, we cannot. "
            "So A ∨ ¬A has no proof term in intuitionistic logic."
        ),
        "rule": "∨-intro₁: from a : A, derive Left(a) : A + B. Similarly for inr.",
    },
    {
        "name": "Ex falso quodlibet (explosion)",
        "logic": "⊥ → A",
        "type":  "Empty → A",
        "proof": "λ x → absurd x   -- by elimination on the empty type",
        "haskell": "absurd :: Void -> a\nabsurd x = case x of {}  -- no cases!",
        "python_fn": lambda _: (_ for _ in ()).throw(Exception("absurd")),
        "python_sig": "absurd(x) = ⊥  (never called — no element of Empty exists)",
        "example": "absurd has no input to give it. You can never call it.",
        "explanation": (
            "From a proof of ⊥ (falsehood), you can derive anything. "
            "The proof term is the 'absurd' eliminator: given x : Empty, "
            "produce a term of any type by case analysis — there are no cases! "
            "In Haskell: 'case x of {}' has type 'a' for any 'a'. "
            "This is vacuously true: if you had an element of the empty type, "
            "you could derive anything — but you never have such an element."
        ),
        "rule": "⊥-elim: given x : Empty, derive any type A by case analysis (no branches needed).",
    },
    {
        "name": "Modus ponens (function application)",
        "logic": "(A → B) → A → B",
        "type":  "(A → B) → A → B",
        "proof": "λ f → λ x → f x",
        "haskell": "($) :: (a -> b) -> a -> b\n($) f x = f x",
        "python_fn": lambda f: lambda x: f(x),
        "python_sig": "apply(f)(x) = f(x)",
        "example": "apply(lambda x: x*2)(21) = 42",
        "explanation": (
            "Modus ponens: if A implies B, and A holds, then B holds. "
            "The proof term is function application: given f : A → B and x : A, "
            "apply f to x to get f(x) : B. "
            "This is the ($) operator in Haskell. "
            "Logic's modus ponens = programming's function application. "
            "The entire eval/apply loop of an interpreter is modus ponens."
        ),
        "rule": "→-elim: from f : A → B and x : A, derive f(x) : B.",
    },
    {
        "name": "Distribution of → over ∧",
        "logic": "(A → B ∧ C) → (A → B) ∧ (A → C)",
        "type":  "(A → B × C) → (A → B) × (A → C)",
        "proof": "λ f → (λ x → fst (f x),  λ x → snd (f x))",
        "haskell": "dist :: (a -> (b, c)) -> (a -> b, a -> c)\ndist f = (fst . f, snd . f)",
        "python_fn": lambda f: (lambda x: f(x)[0], lambda x: f(x)[1]),
        "python_sig": "dist(f) = (lambda x: fst(f(x)), lambda x: snd(f(x)))",
        "example": "f = lambda x: (x+1, x*2)\ndist(f)[0](3) = 4   dist(f)[1](3) = 6",
        "explanation": (
            "If a function f produces a pair (B, C), you can split it into "
            "two functions — one producing B, one producing C. "
            "The proof term pairs the two projections of f. "
            "This is the universal property of the product: "
            "hom(A, B × C) ≅ hom(A, B) × hom(A, C). "
            "Every map into a product is the same as a pair of maps."
        ),
        "rule": "Split f : A → B × C into (fst∘f : A→B, snd∘f : A→C).",
    },
]

# ── Classical failures ─────────────────────────────────────────────────────────

CLASSICAL = [
    {
        "name": "Law of Excluded Middle (LEM)",
        "logic": "A ∨ ¬A",
        "type":  "A + (A → ⊥)",
        "why_fails": (
            "To inhabit A + (A → ⊥), we must provide either:\n"
            "  • Left(a) with a : A, or\n"
            "  • Right(f) with f : A → ⊥\n\n"
            "For arbitrary A, we have no idea which one to give.\n"
            "Example: let A = 'there are infinitely many twin primes'.\n"
            "  We don't know if A holds or ¬A holds.\n"
            "  LEM asserts we CAN decide — but constructively we cannot."
        ),
        "consequence": (
            "Without LEM, we cannot decide arbitrary propositions.\n"
            "  This is not a bug but a feature: intuitionistic logic is\n"
            "  the logic of constructive computation. A 'proof' of A ∨ ¬A\n"
            "  must explicitly exhibit a proof of one disjunct."
        ),
        "in_hott": (
            "In HoTT, LEM is consistent if A is a mere proposition (h-level -1).\n"
            "  The propositional axiom of choice (for sets) also holds.\n"
            "  But LEM for arbitrary types would collapse the h-level hierarchy."
        ),
    },
    {
        "name": "Double Negation Elimination (DNE)",
        "logic": "¬¬A → A",
        "type":  "((A → ⊥) → ⊥) → A",
        "why_fails": (
            "¬¬A says: 'the assumption ¬A leads to contradiction'.\n"
            "  Classically, this means A must hold.\n"
            "  Constructively, ¬¬A says: 'if A were false, we'd have a contradiction'\n"
            "  But this doesn't give us an element of A — just a refutation of ¬A.\n\n"
            "The missing piece: we can't 'extract' an A from a proof that ¬A is false\n"
            "  without knowing WHICH element of A to produce."
        ),
        "consequence": (
            "Classical logic is the logic of truth values {true, false}.\n"
            "  Intuitionistic logic is the logic of evidence and construction.\n"
            "  DNE translates: 'if I can't disprove A, then A is true'.\n"
            "  Constructively: 'I can't disprove A' ≠ 'I can prove A'."
        ),
        "in_hott": (
            "¬¬A → A can be added as an axiom (for mere props) without inconsistency.\n"
            "  In Cubical Agda: 'em : (P : Prop) → P ∨ ¬ P' can be postulated.\n"
            "  The double negation monad ¬¬ is a modality (the classical modality).\n"
            "  ¬¬-stable propositions are exactly the 'classical' propositions."
        ),
    },
    {
        "name": "Peirce's Law",
        "logic": "((A → B) → A) → A",
        "type":  "((A → B) → A) → A",
        "why_fails": (
            "Peirce's law is equivalent to LEM (over minimal logic).\n"
            "  To inhabit it, given f : (A → B) → A, we need a : A.\n"
            "  We could try applying f, but f needs (A → B).\n"
            "  To build (A → B), we need to assume A — but we're trying to prove A!\n\n"
            "This circularity has no constructive resolution for arbitrary A, B.\n"
            "  The only escape is a classical case split: either A holds (done)\n"
            "  or A doesn't hold (so A → B is vacuous, apply f, get A — contradiction)."
        ),
        "consequence": (
            "Peirce's law is equivalent to LEM and DNE over minimal logic.\n"
            "  Adding any one of LEM, DNE, Peirce's law as an axiom gives classical logic.\n"
            "  The call/cc (call-with-current-continuation) combinator in Scheme\n"
            "  inhabits Peirce's law — but only in a language with first-class continuations.\n"
            "  Call/cc gives you a classical escape hatch in an intuitionistic language."
        ),
        "in_hott": (
            "The correspondence: Peirce's law ↔ call/cc ↔ classical logic.\n"
            "  In Haskell/ML (no call/cc by default), Peirce's law has no inhabitant.\n"
            "  In Scheme/SML with call/cc: callCC :: ((a -> b) -> a) -> a inhabits it.\n"
            "  This is the computational content of classical reasoning."
        ),
    },
]

# ── Display ───────────────────────────────────────────────────────────────────

def _show_prop(idx: int):
    p = PROPS[idx]
    clear()
    print(bold(f"\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(f"  ║  Curry-Howard  ({idx+1}/{len(PROPS)})                              ║"))
    print(bold(f"  ╚═══════════════════════════════════════════════════════╝\n"))

    print(f"  {bold('Proposition')}: {bold(yellow(p['logic']))}")
    print(f"  {bold('Type')}       : {bold(cyan(p['type']))}")
    print(f"  {bold('Proof term')} : {bold(green(p['proof']))}")
    print()

    print(f"  {bold('As a Haskell program')}:")
    for line in p['haskell'].split('\n'):
        print(f"    {cyan(line)}")
    print()

    print(f"  {bold('Example')}:")
    for line in p['example'].split('\n'):
        print(f"    {dim(line)}")
    print()

    print(f"  {bold('Introduction rule')}: {dim(p['rule'])}")
    print()
    print(wrap(p['explanation'], width=72))
    print()
    print(f"  {dim('[n] next  [p] prev  [1-0] jump  [r] run  [c] classical failures  [h] help')}")

def _show_classical():
    idx = 0
    while True:
        c = CLASSICAL[idx]
        clear()
        print(bold(f"\n  ╔═══════════════════════════════════════════════════════╗"))
        print(bold(f"  ║  Classical Failures  ({idx+1}/{len(CLASSICAL)})                       ║"))
        print(bold(f"  ╚═══════════════════════════════════════════════════════╝\n"))

        print(f"  {bold(red('UNPROVABLE INTUITIONISTICALLY'))}: {bold(yellow(c['logic']))}")
        print(f"  {bold('Type')}: {bold(cyan(c['type']))}")
        print()
        print(f"  {bold('Why it fails')}:")
        for line in c['why_fails'].split('\n'):
            print(f"  {line}")
        print()
        print(f"  {bold('Consequence')}:")
        for line in c['consequence'].split('\n'):
            print(f"  {dim(line)}")
        print()
        print(f"  {bold('In HoTT')}:")
        for line in c['in_hott'].split('\n'):
            print(f"  {magenta(line)}")
        print()
        print(f"  {dim('[n] next  [p] prev  [b] back to main')}")
        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break
        if cmd in ("b", "q"):
            break
        elif cmd == "n":
            idx = (idx + 1) % len(CLASSICAL)
        elif cmd == "p":
            idx = (idx - 1) % len(CLASSICAL)

def _run_prop(idx: int):
    p = PROPS[idx]
    clear()
    print(bold(f"\n  Running: {yellow(p['logic'])}\n"))
    print(f"  {bold('Proof term')}: {green(p['proof'])}")
    print(f"  {bold('As function')}: {cyan(p['python_sig'])}")
    print()
    print(f"  {bold('Example output')}:")
    for line in p['example'].split('\n'):
        print(f"    {dim(line)}")
    print()
    print(wrap(
        "The proof term IS the program. Running it demonstrates that the "
        "program's behavior corresponds to the logical content of the proof. "
        "The type signature IS the proposition. A proof IS an implementation.",
        width=72
    ))
    print()
    input(dim("  Press Enter to return…"))

# ── Main ─────────────────────────────────────────────────────────────────────

def main():
    idx = 0
    key_map = {str(i): i - 1 for i in range(1, 10)}
    key_map["0"] = 9

    while True:
        _show_prop(idx)
        print()
        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break

        if cmd in ("q", "quit", "exit"):
            break
        elif cmd == "n":
            idx = (idx + 1) % len(PROPS)
        elif cmd == "p":
            idx = (idx - 1) % len(PROPS)
        elif cmd in key_map:
            idx = key_map[cmd]
        elif cmd == "r":
            _run_prop(idx)
        elif cmd == "c":
            _show_classical()
        elif cmd in ("h", "help", "?"):
            clear()
            print(f"""
  {bold('Curry-Howard Correspondence — Commands')}

  {cyan('n/p')}     next / previous proposition
  {cyan('1-0')}     jump to proposition 1-10
  {cyan('r')}       run the proof term as a program
  {cyan('c')}       classical failures: LEM, DNE, Peirce
  {cyan('h')}       this help
  {cyan('q')}       quit

  {bold('The correspondence (one line')}):
  {dim('Proposition = Type')}  |  {dim('Proof = Program')}  |  {dim('Hypothesis = Variable')}
""")
            input(dim("  Press Enter…"))


if __name__ == "__main__":
    clear()
    print(bold("""
  ╔════════════════════════════════════════════════════════╗
  ║     The Curry-Howard Correspondence                    ║
  ╚════════════════════════════════════════════════════════╝
"""))
    print("""  Propositions are types. Proofs are programs. Exactly.

    A → B   =   function type   =   implication
    A × B   =   product type    =   conjunction
    A + B   =   sum type        =   disjunction
    ⊤       =   Unit            =   truth
    ⊥       =   Empty           =   falsehood

  Every proof of a proposition IS a program.
  Every program of a type IS a proof.
  The same thing, viewed differently.

  Type 'n' to navigate, 'c' for classical failures, 'q' to quit.
""")
    input(dim("  Press Enter to start…"))
    main()
    print(f"\n  {dim('Propositions are types. Proofs are programs.')}\n")
