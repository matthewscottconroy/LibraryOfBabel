#!/usr/bin/env python3
"""
Abstract Algebra: Groups, Rings, Fields, and Algebra in HoTT
=============================================================
The algebraic structures that appear throughout mathematics and
their formalization in homotopy type theory.
"""

import textwrap
from math import gcd
from itertools import product as iproduct

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


def _section_groups():
    clear()
    print(box("Groups: Definition and Examples"))
    print()
    print(wrap(
        "A GROUP is a set G with a binary operation · satisfying four axioms. "
        "Groups capture the abstract structure of symmetry — every symmetry "
        "of every object forms a group."
    ))
    print()
    print(f"  {cyan('Group axioms for (G, ·, e):')}")
    print(f"  {cyan('  Closure:      ∀a b:G.  a·b : G')}")
    print(f"  {cyan('  Associativity: ∀a b c. (a·b)·c = a·(b·c)')}")
    print(f"  {cyan('  Identity:      ∃e:G. ∀a. e·a = a·e = a')}")
    print(f"  {cyan('  Inverses:      ∀a. ∃a⁻¹. a·a⁻¹ = a⁻¹·a = e')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The zoo of groups:'))}\n")

    examples = [
        ("(ℤ, +, 0)",      "integers under addition — the free abelian group on 1 generator"),
        ("(ℝ×, ·, 1)",     "nonzero reals under multiplication"),
        ("ℤ/nℤ",           "integers mod n — cyclic group of order n"),
        ("(ℤ/2ℤ)²",        "Klein four-group — abelian, order 4, every element order 2"),
        ("Sₙ",             "symmetric group — all permutations of {1,...,n}"),
        ("Aₙ",             "alternating group — even permutations of {1,...,n}"),
        ("Dₙ",             "dihedral group — symmetries of regular n-gon, order 2n"),
        ("GL(n,ℝ)",        "invertible n×n real matrices under multiplication"),
        ("SO(3)",          "rotation matrices in 3D — symmetry group of the sphere"),
        ("π₁(X,x₀)",       "fundamental group of a space — the key HoTT example"),
    ]
    for name, desc in examples:
        print(f"  {bold(cyan(name)):22} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Live demo: ℤ/6ℤ'))}\n")
    n = 6
    G = list(range(n))
    print(f"  Elements: {G}")
    print(f"  Operation: addition mod {n}")
    print()
    print(f"  {'·':4}", end="")
    for g in G:
        print(f"  {g}", end="")
    print()
    print(f"  {dim('─'*30)}")
    for a in G:
        print(f"  {a:4}", end="")
        for b in G:
            val = (a + b) % n
            color = bold if val == 0 else dim
            print(f"  {color(str(val))}", end="")
        print()
    print()
    input(bold("  Press Enter to continue... "))


def _section_homomorphisms():
    clear()
    print(box("Subgroups and Homomorphisms"))
    print()
    print(wrap(
        "A SUBGROUP is a subset H ≤ G that is itself a group under the "
        "restriction of the operation. A HOMOMORPHISM is a structure-preserving "
        "map between groups."
    ))
    print()
    print(f"  {cyan('H ≤ G iff:  e∈H,  a,b∈H → a·b∈H,  a∈H → a⁻¹∈H')}")
    print()
    print(f"  {cyan('f:G→H is a homomorphism iff f(a·b) = f(a)·f(b)')}")
    print(f"  {cyan('  (implies f(e_G) = e_H and f(a⁻¹) = f(a)⁻¹)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key constructions:'))}\n")
    constructions = [
        ("Kernel",  "ker(f) = {g:G | f(g)=e_H}  — always a normal subgroup"),
        ("Image",   "im(f) = {f(g) | g:G}  — a subgroup of H"),
        ("Cosets",  "gH = {g·h | h∈H}  — partition G into equal-sized pieces"),
        ("Quotient","G/N = G mod a normal subgroup N"),
        ("Product", "G×H — group on pairs, componentwise operation"),
        ("IsoBij",  "f isomorphism iff f bijective homomorphism"),
    ]
    for name, desc in constructions:
        print(f"  {bold(cyan(name)):14} {desc}")
    print()
    print(rule())
    lagrange_str = "Lagrange: |H| divides |G| for finite G"
    isomorphism_str = "First isomorphism: G/ker(f) ≅ im(f)"
    print(f"\n  {bold(green('Fundamental theorems:'))}\n")
    theorems = [
        (lagrange_str,
         "Order of any subgroup divides order of group"),
        (isomorphism_str,
         "The image captures the group mod its kernel"),
        ("Cayley: G embeds in S_{|G|}",
         "Every group is a permutation group"),
    ]
    for thm, note in theorems:
        print(f"  {bold(yellow(thm))}")
        print(f"    {dim(note)}")
        print()
    input(bold("  Press Enter to continue... "))


def _section_free_groups():
    clear()
    print(box("Free Groups and Presentations"))
    print()
    print(wrap(
        "The FREE GROUP F(S) on a set S consists of all WORDS (finite sequences) "
        "over S and formal inverses S⁻¹, reduced by cancellation. Every group "
        "is a quotient of a free group."
    ))
    print()
    print(f"  {cyan('F({{a}}) = ℤ  (powers of a: ..., a⁻², a⁻¹, e, a, a², ...)')}")
    print(f"  {cyan('F({{a,b}}) = free group on two generators (non-abelian)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Word reduction in F({a,b}):'))}\n")

    def reduce_word(word):
        stack = []
        for c in word:
            if stack and ((stack[-1].islower() and c == stack[-1].upper()) or
                          (stack[-1].isupper() and c == stack[-1].lower())):
                stack.pop()
            else:
                stack.append(c)
        return ''.join(stack) if stack else 'e'

    words = ['ab', 'aAbB', 'aBaB', 'abBA', 'aabbAA', 'abAB', 'AaBb']
    print(f"  {'Word':15}  {'Reduced'}")
    print(f"  {dim('─'*30)}")
    for w in words:
        r = reduce_word(w)
        color = green if r == 'e' else cyan
        print(f"  {dim(w):20}  {color(r)}")
    print()
    print(f"  {dim('(uppercase = inverse: A = a⁻¹, B = b⁻¹)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Group presentations:'))}\n")
    presentations = [
        ("ℤ",        "<a | >",            "free group on one generator"),
        ("ℤ/nℤ",     "<a | aⁿ=e>",        "cyclic group of order n"),
        ("ℤ²",       "<a,b | ab=ba>",      "free abelian group"),
        ("Dₙ",       "<r,s | rⁿ,s²,(rs)²>","dihedral group"),
        ("Q₈",       "<i,j | i⁴,i²=j²,ij=ji⁻¹>","quaternion group"),
        ("π₁(T²)",   "<a,b | ab=ba>",      "torus fundamental group = ℤ²"),
        ("π₁(RP²)",  "<a | a²=e>",         "ℤ/2ℤ"),
    ]
    for group, pres, note in presentations:
        print(f"  {bold(cyan(group)):12} = {yellow(pres):30} {dim(note)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_rings_fields():
    clear()
    print(box("Rings, Fields, and Beyond"))
    print()
    print(wrap(
        "A RING has two operations (+,·) where (R,+) is an abelian group, "
        "· is associative, and · distributes over +. A FIELD is a "
        "commutative ring where every nonzero element has a multiplicative inverse."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The algebraic hierarchy:'))}\n")
    hierarchy = [
        ("Semigroup",    "(S, ·)",         "associativity only"),
        ("Monoid",       "(M, ·, e)",      "semigroup + identity"),
        ("Group",        "(G, ·, e, ⁻¹)", "monoid + inverses"),
        ("Abelian group","(A, +, 0, -)",   "group + commutativity"),
        ("Ring",         "(R, +, ·, 0, 1)","abelian group + ring mult"),
        ("Comm. ring",   "(R, +, ·)",      "ring + ab = ba"),
        ("Domain",       "no zero divisors","a·b=0 → a=0 or b=0"),
        ("Field",        "(F, +, ·, 0, 1)","comm domain + all nonzero invertible"),
    ]
    for name, sig, note in hierarchy:
        print(f"  {bold(cyan(name)):18} {yellow(sig):26} {dim(note)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Key examples:'))}\n")
    examples = [
        ("ℤ",        "ring",   "integers — NOT a field (no 1/2 in ℤ)"),
        ("ℚ, ℝ, ℂ",  "field",  "the classical number fields"),
        ("ℤ/pℤ",     "field",  "p prime → field of p elements"),
        ("ℤ[x]",     "ring",   "polynomials with integer coefficients"),
        ("ℝ[x]/(x²+1)","field", "isomorphic to ℂ"),
        ("M_n(ℝ)",   "ring",   "n×n real matrices — non-commutative"),
        ("𝔽₂={0,1}", "field",  "the field with 2 elements"),
    ]
    for name, kind, note in examples:
        print(f"  {bold(cyan(name)):18} {yellow(kind):10} {dim(note)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_algebra_hott():
    clear()
    print(box("Algebra in Homotopy Type Theory"))
    print()
    print(wrap(
        "In HoTT, algebraic structures are defined as TYPES with structure. "
        "A group is a type G with operations, and the group laws are PROOFS "
        "living in identity types. This changes the nature of equality."
    ))
    print()
    print(f"  {cyan('Group : 𝒰₁')}")
    print(f"  {cyan('Group = Σ(G:𝒰₀). Σ(·:G→G→G). Σ(e:G).')}")
    print(f"  {cyan('         Σ(inv:G→G). GroupLaws(G,·,e,inv)')}")
    print()
    print(f"  {cyan('GroupLaws(G,·,e,inv) = ')}")
    print(f"  {cyan('  (∀a b c. (a·b)·c = a·(b·c))')}")
    print(f"  {cyan('  × (∀a. e·a = a)   × (∀a. a·e = a)')}")
    print(f"  {cyan('  × (∀a. inv(a)·a = e)   × (∀a. a·inv(a) = e)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Univalence and isomorphism:'))}\n")
    print(wrap(
        "Univalence says: two groups G and H are EQUAL as types iff they "
        "are isomorphic as groups (assuming the laws are propositions). "
        "This makes the standard mathematical practice — treating isomorphic "
        "objects as equal — LITERAL in HoTT."
    ))
    print()
    print(f"  {cyan('G =_Group H  ≃  G ≅_Group H   (by univalence)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Higher group theory:'))}\n")
    print(wrap(
        "Every loop space ΩX is a group (with path concatenation). "
        "Higher loop spaces Ω²X are abelian groups (Eckmann-Hilton). "
        "This means homotopy groups πₙ(X) are built into HoTT for free."
    ))
    print()
    higher = [
        ("Group",          "ΩX  (loop space)",       "has associative composition"),
        ("Abelian group",  "Ω²X = Ω(ΩX)",           "Eckmann-Hilton: composition commutes"),
        ("B(G) delooping", "K(G,1)",                  "classifying space; G = π₁(BG)"),
        ("Group cohomology","H^n(BG ; A)",            "via Eilenberg-MacLane spaces"),
    ]
    for concept, example, note in higher:
        print(f"  {bold(cyan(concept)):20} {yellow(example):28} {dim(note)}")
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("groups",  "Groups: definition and examples",          _section_groups),
    ("hom",     "Subgroups and homomorphisms",              _section_homomorphisms),
    ("free",    "Free groups and presentations",            _section_free_groups),
    ("rings",   "Rings, fields, and the algebraic ladder",  _section_rings_fields),
    ("hott",    "Algebra in homotopy type theory",          _section_algebra_hott),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Abstract Algebra: Groups, Rings, Fields", width=70))
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
