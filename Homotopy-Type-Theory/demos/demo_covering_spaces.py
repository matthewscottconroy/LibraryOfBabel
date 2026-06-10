#!/usr/bin/env python3
"""
Covering Spaces and the Galois Correspondence
==============================================
Covering spaces of X ↔ subgroups of π₁(X).

A covering space is a map p:E→X that is locally homeomorphic to a product
X×F. The fundamental theorem of covering space theory establishes a
Galois-like correspondence: connected covers of X biject with subgroups
of π₁(X). In HoTT, this is simply the theory of set-valued type families.
"""

import textwrap
import math

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


def _section_definition():
    clear()
    print(box("Covering Spaces: The Definition"))
    print()
    print(wrap(
        "A covering space of X is a map p : E → X such that every point "
        "x : X has an open neighborhood U such that p⁻¹(U) is a disjoint "
        "union of open sets, each mapping homeomorphically onto U."
    ))
    print()
    print(f"  {cyan('p : E → X  is a covering iff')}")
    print(f"  {cyan('  ∀x:X. ∃U∋x. p⁻¹(U) ≃ U × F  (for some discrete set F)')}")
    print()
    print(wrap(
        "The set F = p⁻¹(x) is called the FIBER over x. For a connected cover, "
        "the fiber has the same cardinality everywhere — the DEGREE of the cover."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Examples:'))}\n")

    examples = [
        ("ℝ → S¹",   "exp(2πi·): ℝ→S¹",   "∞-sheeted", "π₁(S¹)=ℤ, fiber=ℤ"),
        ("S¹ → S¹",  "z ↦ zⁿ (degree n)",  "n-sheeted",  "fiber = ℤ/nℤ, subgroup nℤ≤ℤ"),
        ("S² → RP²", "antipodal quotient",   "2-sheeted",  "π₁(RP²)=ℤ/2ℤ, fiber={±1}"),
        ("S³ → SO(3)","spin double cover",    "2-sheeted",  "π₁(SO(3))=ℤ/2ℤ"),
        ("X → X",    "identity",             "1-sheeted",  "trivial cover, full group"),
    ]

    print(f"  {'Cover':20}  {'Description':22}  {'Sheets':12}  {'Subgroup'}")
    print(f"  {dim('─'*70)}")
    for cover, desc, sheets, sub in examples:
        print(f"  {bold(cyan(cover)):28}  {desc:22}  {sheets:12}  {dim(sub)}")

    print()
    print(rule())
    print(f"\n  {bold(yellow('The key property: path lifting'))}\n")
    print(wrap(
        "Given a path γ : I → X starting at x₀, and a lift e₀ ∈ p⁻¹(x₀), "
        "there is a UNIQUE path γ̃ : I → E with p ∘ γ̃ = γ and γ̃(0) = e₀. "
        "Paths lift uniquely. This is what makes covering spaces so rigid."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_circle_covers():
    clear()
    print(box("Covers of S¹: The n-Sheeted Family"))
    print()
    print(wrap(
        "The circle S¹ is the simplest example with a rich covering theory. "
        "Its fundamental group is π₁(S¹) = ℤ, and the subgroups of ℤ are "
        "exactly nℤ = {0, ±n, ±2n, ...} for n ≥ 0."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The n-sheeted cover:  pₙ : S¹ → S¹,  z ↦ zⁿ'))}\n")

    for n in [1, 2, 3, 4]:
        subgroup = f"{n}ℤ" if n > 1 else "ℤ"
        index = f"[ℤ : {n}ℤ] = {n}" if n > 1 else "index 1"
        fiber = f"{{e^(2πik/{n}) | k=0..{n-1}}}" if n <= 3 else f"{{n-th roots of unity}}"
        print(f"  {bold(cyan(f'p_{n}'))}: z ↦ z^{n}")
        print(f"    Fiber over 1:   {fiber}")
        print(f"    Subgroup:       {subgroup}")
        print(f"    Index:          {index}")
        print()

    print(f"  {bold(cyan('p_∞'))}: ℝ → S¹,  t ↦ e^(2πit)")
    print(f"    Fiber over 1:   ℤ  (all integers)")
    print(f"    Subgroup:       {{0}} = trivial")
    print(f"    Index:          ∞ (universal cover)")
    print()

    print(rule())
    print(f"\n  {bold(green('Visualizing p₃: z ↦ z³'))}\n")
    n = 3
    print(f"  The circle wraps around 3 times. The fiber over any point")
    print(f"  consists of 3 equally spaced points:")
    print()
    for k in range(n):
        angle = 2 * math.pi * k / n
        x = math.cos(angle)
        y = math.sin(angle)
        print(f"    e^(2πi·{k}/{n}) = ({x:.3f}, {y:.3f})")
    print()
    print(wrap(
        "A loop going once around S¹ lifts to a path going 1/3 of the way "
        "around the cover. Only after going around 3 times does the lift "
        "return to its starting point — reflecting that the loop generates "
        "3ℤ in π₁, not all of ℤ."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_galois():
    clear()
    print(box("The Galois Correspondence"))
    print()
    print(wrap(
        "The fundamental theorem of covering space theory establishes a "
        "bijection (for nice spaces X with chosen basepoint):"
    ))
    print()
    print(f"  {bold(cyan('{connected covers of X up to isomorphism}'))}")
    print(f"  {'':10}↕   {bold('one-to-one')}")
    print(f"  {bold(cyan('{subgroups of π₁(X,x₀) up to conjugacy}'))}")
    print()
    print(wrap(
        "The cover E corresponding to subgroup H ≤ π₁(X) is constructed as: "
        "E = paths in X from x₀, modulo the relation 'same endpoint and the "
        "concatenation loop is in H'. In HoTT this is a quotient type."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The correspondence for S¹  (π₁(S¹) = ℤ):'))}\n")

    print(f"  {'Subgroup H':20}  {'Index [ℤ:H]':14}  {'Corresponding cover'}")
    print(f"  {dim('─'*60)}")
    subgroups = [
        ("ℤ  (full group)",     "1",    "S¹ itself (trivial cover)"),
        ("2ℤ",                  "2",    "2-sheeted: S¹→S¹, z↦z²"),
        ("3ℤ",                  "3",    "3-sheeted: S¹→S¹, z↦z³"),
        ("nℤ",                  "n",    "n-sheeted: S¹→S¹, z↦zⁿ"),
        ("{0} (trivial)",        "∞",   "ℝ→S¹ (universal cover)"),
    ]
    for H, idx, cover in subgroups:
        print(f"  {H:28}  {idx:14}  {dim(cover)}")

    print()
    print(rule())
    print(f"\n  {bold(green('The general statement:'))}\n")
    print(f"  {cyan('Cov(X)  ≃  Sub(π₁(X))/conj')}")
    print()
    print(wrap(
        "Conjugate subgroups correspond to isomorphic covers (just with a "
        "different basepoint). For abelian fundamental groups (like ℤ), "
        "conjugacy is trivial and subgroups biject with covers exactly."
    ))
    print()
    print(wrap(
        "This is completely analogous to the Galois correspondence in field "
        "theory: subfields of a Galois extension biject with subgroups of the "
        "Galois group. Covering space theory IS Galois theory, categorically."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_hott_view():
    clear()
    print(box("Covering Spaces in HoTT: Type Families Over the Base"))
    print()
    print(wrap(
        "In HoTT, a covering space of X with fiber F is simply a type family "
        "P : X → Set (a set-valued dependent type). The covering space E is the "
        "total space Σ(x:X). P(x), and the projection p is π₁ : Σx.Px → X."
    ))
    print()
    print(f"  {cyan('cover P : X → Set')}")
    print(f"  {cyan('total(P) = Σ(x:X). P x')}")
    print(f"  {cyan('p = fst  : Σx.Px → X     (the covering map)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The circle covers in HoTT:'))}\n")
    print(f"  {bold('Universal cover (ℝ):')}")
    print(f"  {cyan('univ : S¹ → Set')}")
    print(f"  {cyan('univ base = ℤ')}")
    print(f"  {cyan('univ (loop i) = ua(+1)(i)   -- transport adds 1')}")
    print()
    print(f"  {dim('Total space = Σ(x:S¹). univ x  ≃  ℝ  (contractible)')}")
    print()
    print(f"  {bold('n-sheeted cover:')}")
    print(f"  {cyan('cover_n : S¹ → Set')}")
    print(f"  {cyan('cover_n base = ℤ/nℤ')}")
    print(f"  {cyan('cover_n (loop i) = ua(+1 mod n)(i)  -- transport adds 1 mod n')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Why Set-valued? Discrete fibers.'))}\n")
    print(wrap(
        "A covering space has DISCRETE fibers — the fiber is just a set, "
        "with no continuous structure. In HoTT, this means P(x) must be a SET "
        "(h-level 0). The fiber has no interesting path structure; paths "
        "in E come only from paths in X (by lifting)."
    ))
    print()
    print(f"  {cyan('covering = Σ(P : X → Set). ...  (Set-valued family)')}")
    print(f"  {dim('vs.')}")
    print(f"  {cyan('fibration = Σ(P : X → Type). ...  (arbitrary family)')}")
    print()
    print(wrap(
        "A general fibration (Σ(x:X). P(x) → X) can have fiber with higher "
        "homotopy. Covering spaces are the 0-truncated case — the fibers are "
        "discrete. This is the precise HoTT analogue of 'locally homeomorphic "
        "to a product with a discrete set'."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_universal_cover():
    clear()
    print(box("The Universal Cover: Σ(x:X). base = x"))
    print()
    print(wrap(
        "The universal cover of X (based at x₀) is the type of based paths "
        "in X — paths starting at the basepoint:"
    ))
    print()
    print(f"  {cyan('Ẋ = Σ(x:X). (base = x)   (path space based at base)')}")
    print(f"  {cyan('p : Ẋ → X,   p(x, γ) = x   (forget the path)')}")
    print()
    print(wrap(
        "This is the HoTT universal cover because Ẋ is CONTRACTIBLE "
        "(center of contraction: (base, refl)). The fiber over x is "
        "the loop space based at x, which equals π₁(X) as a set (after truncation)."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('For S¹:'))}\n")
    print(f"  {cyan('Ṡ¹ = Σ(x:S¹). (base = x)')}")
    print()
    print(wrap(
        "The fiber of the universal cover over base is (base = base) = ℤ "
        "(by π₁(S¹)=ℤ). The total space Ṡ¹ is contractible — it is the 'real "
        "line' ℝ that wraps around S¹."
    ))
    print()
    print(f"  {bold('The covering map:')}")
    print(f"  {cyan('p : Ṡ¹ → S¹')}")
    print(f"  {cyan('p (x, γ) = x   -- just project to the endpoint')}")
    print()
    print(f"  {bold('Deck transformations:')}")
    print(f"  {cyan('τₙ : Ṡ¹ → Ṡ¹')}")
    print(f"  {cyan('τₙ (x, γ) = (x, loopⁿ · γ)  -- prepend n loops')}")
    print(f"  {dim('  This is the ℤ action: τₙ shifts the fiber by n.')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The general pattern:'))}\n")
    print(wrap(
        "For any connected X, the universal cover is Σ(x:X). ||base = x||₀ "
        "(the 0-truncated path space, to get a set-valued family). "
        "The fundamental group π₁(X) acts as deck transformations. "
        "Subcovers correspond to orbits of subgroups."
    ))
    print()
    print(wrap(
        "This is a beautiful example of HoTT making topology transparent: "
        "the universal cover is just the PATH SPACE, and covering theory "
        "is the theory of actions on path spaces."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("def",      "Covering spaces: the definition",                _section_definition),
    ("circle",   "Covers of S¹: the n-sheeted family",            _section_circle_covers),
    ("galois",   "The Galois correspondence",                      _section_galois),
    ("hott",     "Covering spaces in HoTT: set-valued families",  _section_hott_view),
    ("universal","The universal cover: Σ(x:X). base = x",         _section_universal_cover),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Covering Spaces and the Galois Correspondence", width=70))
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
