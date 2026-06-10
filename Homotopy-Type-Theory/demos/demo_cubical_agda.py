#!/usr/bin/env python3
"""
Cubical Agda: Computational HoTT
=================================
Agda with the cubical library gives a COMPUTATIONAL version of HoTT
where univalence is not an axiom but a theorem with a normal form.
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


def _section_agda_basics():
    clear()
    print(box("Agda Basics"))
    print()
    print(wrap(
        "Agda is a dependently typed functional programming language and "
        "proof assistant developed at Chalmers. It is based on "
        "Martin-Löf Type Theory and is particularly popular for HoTT "
        "formalization."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Basic Agda syntax:'))}\n")
    code = [
        ("-- Types and terms",                           ""),
        ("data ℕ : Set where",                          ""),
        ("  zero : ℕ",                                  ""),
        ("  suc  : ℕ → ℕ",                              ""),
        ("",                                             ""),
        ("-- Functions by pattern matching",             ""),
        ("_+_ : ℕ → ℕ → ℕ",                            ""),
        ("zero  + m = m",                               ""),
        ("suc n + m = suc (n + m)",                     ""),
        ("",                                             ""),
        ("-- Dependent types",                           ""),
        ("Vec : Set → ℕ → Set",                         ""),
        ("Vec A zero    = ⊤",                           "-- empty vector"),
        ("Vec A (suc n) = A × Vec A n",                 ""),
        ("",                                             ""),
        ("-- Identity types",                            ""),
        ("data _≡_ {A : Set} (a : A) : A → Set where", ""),
        ("  refl : a ≡ a",                              "-- the only constructor"),
        ("",                                             ""),
        ("-- Propositions (in classic HoTT Book style)", ""),
        ("sym : a ≡ b → b ≡ a",                        ""),
        ("sym refl = refl",                              ""),
    ]
    for line, comment in code:
        if line == "":
            print()
        elif comment:
            print(f"  {cyan(line):50} {dim(comment)}")
        else:
            print(f"  {cyan(line)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_cubical_mode():
    clear()
    print(box("Cubical Agda: The Interval Type"))
    print()
    print(wrap(
        "CUBICAL AGDA extends Agda with primitives for CUBICAL TYPE THEORY. "
        "The key addition is the INTERVAL type 𝕀 with two endpoints i0, i1 "
        "and a connection algebra."
    ))
    print()
    print(f"  {cyan('{-# OPTIONS --cubical #-}')}")
    print(f"  {cyan('open import Cubical.Core.Everything')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The interval 𝕀 and path types:'))}\n")
    print(f"  {cyan('𝕀 : Set  -- the abstract interval')}")
    print(f"  {cyan('i0 : 𝕀   -- left endpoint (= 0)')}")
    print(f"  {cyan('i1 : 𝕀   -- right endpoint (= 1)')}")
    print()
    print(f"  {cyan('-- Path type: a PRIMITIVE in cubical Agda')}")
    print(f"  {cyan('PathP : (A : 𝕀 → Set) → A i0 → A i1 → Set')}")
    print(f"  {cyan('Path  : (A : Set) → A → A → Set')}")
    print(f"  {cyan('Path A a b = PathP (λ _ → A) a b')}")
    print()
    print(f"  {cyan('-- refl is a lambda over 𝕀')}")
    print(f"  {cyan('refl : {A : Set} {a : A} → Path A a a')}")
    print(f"  {cyan('refl = λ i → a   -- constant path, ignores i')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Path algebra is definitional:'))}\n")
    print(f"  {cyan('-- sym by flipping the interval')}")
    print(f"  {cyan('sym : Path A x y → Path A y x')}")
    print(f"  {cyan('sym p = λ i → p (~ i)   -- ~ is reversal')}")
    print()
    print(f"  {cyan('-- transport along a path is comp')}")
    print(f"  {cyan('transport : Path Set A B → A → B')}")
    print(f"  {cyan('transport p a = transp (λ i → p i) i0 a')}")
    print()
    print(f"  {dim('In Book HoTT: sym, transport are axioms or definitions via J')}")
    print(f"  {dim('In Cubical Agda: they COMPUTE by definition')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_path_types():
    clear()
    print(box("Paths as Functions Out of 𝕀"))
    print()
    print(wrap(
        "In cubical Agda, a PATH p:a=b is literally a function p:𝕀→A "
        "with p(i0)=a and p(i1)=b. This means path operations "
        "reduce definitionally — no axioms needed."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Path operations that compute:'))}\n")

    operations = [
        ("refl a",
         "λ i → a",
         "constant function — endpoint condition trivially satisfied"),
        ("sym p",
         "λ i → p (~ i)",
         "reverse the interval — ~ i = 1 - i"),
        ("p ∙ q  (concat)",
         "hcomp ...",
         "uses hcomp (homogeneous composition)"),
        ("ap f p",
         "λ i → f (p i)",
         "map a function over a path — f∘p"),
        ("transport P p a",
         "transp (λ i → P (p i)) i0 a",
         "move a along a path in a family"),
        ("funext f",
         "λ i x → f x i",
         "extensionality: swap the arguments"),
    ]

    for name, impl, note in operations:
        print(f"  {bold(cyan(name))}")
        print(f"    = {yellow(impl)}")
        print(f"    {dim(note)}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('Why this matters:'))}\n")
    print(wrap(
        "In BOOK HoTT, all path operations are axioms or defined via J — "
        "they do not compute. In CUBICAL AGDA, they all reduce. This means:"
    ))
    print()
    benefits = [
        ("Canonicity",        "every closed term of type ℕ computes to a numeral"),
        ("No stuck terms",    "funext(f) transported along ua can reduce"),
        ("Decidable eq.",     "type checking remains decidable"),
        ("Proof extraction",  "proofs compile to programs that run"),
    ]
    for name, desc in benefits:
        print(f"  {bold(green(name)):20} {dim(desc)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_hits_agda():
    clear()
    print(box("HITs in Cubical Agda"))
    print()
    print(wrap(
        "Cubical Agda supports HIGHER INDUCTIVE TYPES natively. HITs have "
        "both point constructors (like normal inductive types) and "
        "PATH constructors (whose result type is a path)."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The circle S¹ as a HIT:'))}\n")
    print(f"  {cyan('data S1 : Set where')}")
    print(f"  {cyan('  base : S1')}")
    print(f"  {cyan('  loop : base ≡ base')}")
    print()
    print(f"  {dim('  -- loop is a path (1-path constructor)')}")
    print()
    print(f"  {cyan('-- Eliminator:')}")
    print(f"  {cyan('S1-elim : (P : S1 → Set) → (b : P base) → (l : PathP (ap P loop) b b) → (x : S1) → P x')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The interval 𝕀 as a HIT:'))}\n")
    print(f"  {cyan('data Interval : Set where')}")
    print(f"  {cyan('  left  : Interval')}")
    print(f"  {cyan('  right : Interval')}")
    print(f"  {cyan('  seg   : left ≡ right')}")
    print()
    print(f"  {dim('  -- contractible: used to prove function extensionality')}")
    print()
    print(rule())
    print(f"\n  {bold(green('More HITs:'))}\n")
    hits = [
        ("Susp A",    "suspension — S¹ = Susp Bool, S² = Susp S¹"),
        ("Pushout",   "colimit over a span A ← C → B"),
        ("Trunc n A", "n-truncation: ||A||_n"),
        ("Set A",     "set-truncation: quotient by all paths"),
        ("BG",        "delooping / classifying space of group G"),
    ]
    for name, desc in hits:
        print(f"  {bold(cyan(name)):16} {dim(desc)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_univalence_computational():
    clear()
    print(box("Univalence is Computational in Cubical Agda"))
    print()
    print(wrap(
        "The key achievement of cubical type theory: UNIVALENCE IS A THEOREM "
        "with a normal form. It does not need to be an axiom — it can be "
        "proved from the cubical primitives, and it computes."
    ))
    print()
    print(f"  {cyan('-- ua: equivalence → path in the universe')}")
    print(f"  {cyan('ua : A ≃ B → A ≡ B')}")
    print(f"  {cyan('ua e i = Glue B (λ {{ (i = i0) → A , e ; (i = i1) → B , idEquiv B }})')}")
    print()
    print(f"  {dim('  Glue is a primitive that glues types along equivalences')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Transport along ua:'))}\n")
    print(f"  {cyan('transport (ua e) a = e .fst a')}")
    print()
    print(wrap(
        "This is the key computation: transporting along the path (ua e) "
        "gives EXACTLY applying the underlying function of e. In Book HoTT "
        "this is an axiom; in Cubical Agda it is a definitional equality."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Book HoTT vs Cubical Agda:'))}\n")
    print(f"  {'Property':30}  {'Book HoTT':20}  {'Cubical Agda'}")
    print(f"  {dim('─'*68)}")
    comparison = [
        ("Univalence",           "axiom",         "theorem (provable)"),
        ("funext",               "axiom",         "theorem (λ i x → f x i)"),
        ("transport(ua e)",      "axiom",         "computes to e.fst"),
        ("HITs",                 "by axioms",     "built in"),
        ("Canonicity",           "unknown/fail",  "yes (computable)"),
        ("Decidability",         "unknown",       "yes"),
        ("J eliminator",         "primitive",     "provable from path ops"),
        ("Path = Id type",       "definitional",  "Path A a b primitive"),
    ]
    for prop, book, cub in comparison:
        print(f"  {prop:35} {dim(book):28} {green(cub)}")
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("basics",   "Agda basics",                              _section_agda_basics),
    ("cubical",  "Cubical Agda: the interval type",          _section_cubical_mode),
    ("paths",    "Paths as functions out of 𝕀",              _section_path_types),
    ("hits",     "HITs in Cubical Agda",                     _section_hits_agda),
    ("ua",       "Univalence is computational",              _section_univalence_computational),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Cubical Agda: Computational HoTT", width=70))
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
