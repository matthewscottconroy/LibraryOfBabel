#!/usr/bin/env python3
"""
Quotient Types: Doing Mathematics with HITs
============================================
Quotient types A/~ as higher inductive types.

In classical mathematics, the quotient A/~ is constructed set-theoretically
as the set of equivalence classes. In HoTT, it is a HIT where we ADD PATHS
between equivalent elements — the equivalence relation becomes path structure.
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


# ── Section 1: The quotient HIT ───────────────────────────────────────────────

def _section_quotient_hit():
    clear()
    print(box("The Set-Quotient as a Higher Inductive Type"))
    print()
    print(wrap(
        "Given a type A and a relation R : A → A → Prop, the quotient A/R is:"
    ))
    print()
    print(f"  {cyan('data A/R : Type where')}")
    print(f"  {cyan('  [_] : A → A/R                          -- inject elements')}")
    print(f"  {cyan('  quot : ∀(x y : A). R x y → [x] = [y]  -- related → equal')}")
    print(f"  {cyan('  squash : isSet (A/R)                   -- result is a set')}")
    print()
    print(wrap(
        "The crucial move: instead of forming equivalence CLASSES (sets of elements), "
        "we ADD PATHS between elements that are related. The quotient is not 'subsets "
        "of A' but 'A with new paths added', then truncated to a set."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The recursion principle:'))}\n")
    print(f"  {cyan('rec : isSet B → (f : A → B)')}")
    print(f"  {cyan('    → (∀ x y. R x y → f x = f y)')}")
    print(f"  {cyan('    → A/R → B')}")
    print()
    print(wrap(
        "To define a function A/R → B into a set B, you need: a function f on "
        "elements, plus a proof that f RESPECTS the relation (related elements "
        "map to equal elements). The quotient handles the coherence for you."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Compare: set-theoretic quotient'))}\n")
    print(wrap(
        "Set theory: A/~ = { [a] | a ∈ A } where [a] = { b ∈ A | a ~ b }. "
        "You need to check functions are WELL-DEFINED on equivalence classes. "
        "HoTT: the type system enforces well-definedness through the quot path "
        "constructor. A non-respecting function simply does not typecheck."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 2: Building ℤ ─────────────────────────────────────────────────────

def _section_integers():
    clear()
    print(box("The Integers: ℤ = (ℕ × ℕ) / ~"))
    print()
    print(wrap(
        "One elegant construction of the integers: pairs (a, b) : ℕ × ℕ "
        "represent the 'difference' a - b. Two pairs are equivalent when "
        "they represent the same difference:"
    ))
    print()
    relation_str = "(a, b) ~ (a'', b'') :≡ a + b'' = a'' + b"
    print(f"  {cyan(relation_str)}")
    print()
    print(f"  {cyan('ℤ :≡ (ℕ × ℕ) / ~')}")
    print()

    class IntPair:
        def __init__(self, a, b): self.a, self.b = a, b
        def __repr__(self): return f"({self.a}, {self.b})"
        def equiv(self, other): return self.a + other.b == other.a + self.b
        def to_int(self): return self.a - self.b
        def canonical(self):
            if self.a >= self.b: return IntPair(self.a - self.b, 0)
            else:                return IntPair(0, self.b - self.a)

    print(f"  {bold(green('Representatives of small integers:'))}\n")
    pairs = [
        (IntPair(0,2), "−2"),
        (IntPair(0,1), "−1"),
        (IntPair(0,0), "0"),
        (IntPair(1,0), "1"),
        (IntPair(2,0), "2"),
        (IntPair(3,1), "2 (alternate)"),
        (IntPair(5,3), "2 (alternate)"),
    ]
    for p, label in pairs:
        eq_class = f"[(a,b) : a−b = {p.to_int()}]"
        print(f"  {str(p):>10}  →  ℤ element {bold(yellow(label)):30}  {dim(eq_class)}")

    print()
    print(rule())
    print(f"\n  {bold(green('Equivalence check:'))}\n")
    checks = [(IntPair(3,1), IntPair(5,3)), (IntPair(2,0), IntPair(0,2)),
              (IntPair(4,2), IntPair(7,5)), (IntPair(1,0), IntPair(0,1))]
    for p, q in checks:
        rel = p.equiv(q)
        sym = green("~") if rel else red("≁")
        note = f"since {p.a}+{q.b}={'='if rel else '≠'}{q.a}+{p.b}"
        print(f"  {p}  {sym}  {q}   {dim(note)}")

    print()
    print(rule())
    print(f"\n  {bold(yellow('Addition: (a,b) + (c,d) = (a+c, b+d)'))}\n")
    print(wrap(
        "Addition is well-defined on equivalence classes: if (a,b)~(a',b') and "
        "(c,d)~(c',d'), then (a+c, b+d) ~ (a'+c', b'+d'). "
        "The quot path constructor guarantees that [p] + [q] is unambiguous."
    ))
    p1, p2 = IntPair(3, 1), IntPair(2, 4)
    result = IntPair(p1.a + p2.a, p1.b + p2.b)
    print(f"  {p1} + {p2} = {result}  represents {p1.to_int()} + {p2.to_int()} = {result.to_int()}")
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 3: Building ℚ ─────────────────────────────────────────────────────

def _section_rationals():
    clear()
    print(box("The Rationals: ℚ = (ℤ × ℕ₊) / ~"))
    print()
    print(wrap(
        "Similarly, rational numbers are pairs (numerator, denominator) where "
        "the denominator is a positive natural number, quotiented by:"
    ))
    print()
    rat_rel = "(p, q) ~ (p', q') :≡ p · q' = p' · q"
    print(f"  {cyan(rat_rel)}")
    print()
    print(f"  {cyan('ℚ :≡ (ℤ × ℕ₊) / ~')}")
    print()

    from math import gcd

    class RatPair:
        def __init__(self, p, q):
            assert q > 0, "denominator must be positive"
            self.p, self.q = p, q
        def __repr__(self): return f"({self.p}/{self.q})"
        def equiv(self, other): return self.p * other.q == other.p * self.q
        def canonical(self):
            g = gcd(abs(self.p), self.q)
            return RatPair(self.p // g, self.q // g)
        def to_float(self): return self.p / self.q

    print(f"  {bold(green('Representatives of small rationals:'))}\n")
    rats = [
        (RatPair(1,2),  "1/2"),
        (RatPair(2,4),  "1/2 (alternate)"),
        (RatPair(3,6),  "1/2 (alternate)"),
        (RatPair(2,3),  "2/3"),
        (RatPair(4,6),  "2/3 (alternate)"),
        (RatPair(-1,3), "-1/3"),
        (RatPair(-2,6), "-1/3 (alternate)"),
    ]
    for r, label in rats:
        can = r.canonical()
        print(f"  {str(r):>8}  →  ℚ element {bold(yellow(label)):30}  canonical: {can}")

    print()
    print(rule())
    print(f"\n  {bold(green('Equivalence check:'))}\n")
    checks = [(RatPair(1,2), RatPair(2,4)), (RatPair(2,3), RatPair(4,6)),
              (RatPair(1,3), RatPair(2,3)), (RatPair(3,4), RatPair(6,8))]
    for p, q in checks:
        rel = p.equiv(q)
        sym = green("~") if rel else red("≁")
        note = f"{p.p}·{q.q} {'=' if rel else '≠'} {q.p}·{p.q}"
        print(f"  {p}  {sym}  {q}   {dim(note)}")

    print()
    print(rule())
    print(f"\n  {bold(yellow('Addition: (p,q) + (r,s) = (p·s + r·q, q·s)'))}\n")
    a, b = RatPair(1,3), RatPair(1,6)
    result = RatPair(a.p * b.q + b.p * a.q, a.q * b.q)
    print(f"  {a} + {b} = {result}  =  {result.canonical()}  ({a.to_float():.4f} + {b.to_float():.4f} = {result.to_float():.4f})")
    print()
    print(wrap(
        "Multiplication, comparison, and all other operations are similarly "
        "well-defined on equivalence classes. The quot path constructor guarantees "
        "that no choice of representative matters — by type theory."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 4: The circle as a quotient ───────────────────────────────────────

def _section_circle_quotient():
    clear()
    print(box("S¹ as a Quotient: ℝ/ℤ"))
    print()
    print(wrap(
        "The unit circle S¹ can be constructed as the quotient of the real "
        "line ℝ by the integer translation action:"
    ))
    print()
    print(f"  {cyan('x ~ y :≡ ∃(n:ℤ). x = y + n')}")
    print(f"  {cyan('S¹ :≡ ℝ/~  ≃  ℝ/ℤ')}")
    print()
    print(wrap(
        "The real line 'wraps around' by identifying each point with all "
        "its integer translates. The result is a circle."
    ))
    print()
    print(f"  {bold(green('Visualizing the quotient:'))}")
    print()
    print(f"  ℝ:  {dim('...')} −2  −1   0   1   2   3  {dim('...')}")
    print(f"       {dim('|')}    {dim('|')}    {dim('|')}   {dim('|')}   {dim('|')}   {dim('|')}")
    print(f"       {cyan('↓')}    {cyan('↓')}    {cyan('↓')}   {cyan('↓')}   {cyan('↓')}   {cyan('↓')}  (quotient map)")
    print(f"  S¹:  {green('*')}    {green('*')}    {green('*')}   {green('*')}   {green('*')}   {green('*')}  all same point!")
    print(f"          {dim('(all integers map to base point)')}")
    print()
    print(wrap(
        "The interval [0,1] maps ONTO S¹, with 0 and 1 identified "
        "(they differ by 1, an integer). This is why the circle has π₁ = ℤ: "
        "the universal cover is ℝ, and deck transformations are integer translations."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Compare to the HIT definition of S¹:'))}\n")
    print(f"  {cyan('data S¹ where')}")
    print(f"  {cyan('  base : S¹')}")
    print(f"  {cyan('  loop : base = base')}")
    print()
    print(wrap(
        "The HIT definition adds one loop at base. The quotient definition "
        "adds a path between every integer pair. These are equivalent — "
        "but the HIT is simpler to reason about in type theory."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('More quotient constructions of familiar spaces:'))}\n")
    spaces = [
        ("T² = ℝ²/ℤ²",  "Torus: identify (x,y) ~ (x+m, y+n) for m,n:ℤ"),
        ("RP² = S²/~",   "Real projective plane: identify antipodal points x ~ -x on S²"),
        ("Kl  = [0,1]²/~","Klein bottle: identify (x,0)~(x,1) and (0,y)~(1,1-y)"),
        ("CP¹ = ℂ²*/~",  "Riemann sphere = ℂP¹: complex lines through origin"),
    ]
    for name, desc in spaces:
        print(f"  {bold(cyan(name)):<20}  {desc}")
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 5: Quotient groups ────────────────────────────────────────────────

def _section_quotient_groups():
    clear()
    print(box("Quotient Groups: G/N in HoTT"))
    print()
    print(wrap(
        "If G is a group and N ⊆ G is a normal subgroup, the quotient G/N "
        "is the group obtained by 'collapsing N to the identity'. In HoTT:"
    ))
    print()
    print(f"  {cyan('g ~ g′ :≡ g′ · g⁻¹ ∈ N  (or equivalently: g⁻¹ · g′ ∈ N for normal N)')}")
    print(f"  {cyan('G/N :≡ G / ~')}")
    print()
    print(f"  {bold(green('Key examples:'))}\n")

    examples = [
        ("ℤ/nℤ",   "ℤ", "nℤ = {kn | k:ℤ}", "a ~ b iff n | (a-b)",   "Integers mod n"),
        ("ℤ/2ℤ",   "ℤ", "2ℤ",               "a ~ b iff 2 | (a-b)",   "Two elements: [0] and [1]"),
        ("ℚ/ℤ",    "ℚ", "ℤ",                "x ~ y iff x-y ∈ ℤ",    "Rationals mod integers (circle!)"),
        ("G/G",    "G", "all of G",          "everything ~",          "Trivial group"),
        ("G/{e}",  "G", "trivial",           "only e ~ e",            "G itself"),
    ]

    for name, G, N, rel, desc in examples:
        print(f"  {bold(cyan(name)):<14} G={G}, N={N}")
        print(f"    {dim(rel)}")
        print(f"    {desc}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('Computing ℤ/5ℤ interactively:'))}\n")

    n = 5
    print(f"  Representatives of ℤ/{n}ℤ: " + ", ".join(f"[{i}]" for i in range(n)))
    print()

    ops = [(2, 4), (3, 3), (4, 4), (1, 4)]
    print(f"  {'Addition mod ' + str(n):}")
    for a, b in ops:
        result = (a + b) % n
        print(f"  [{a}] + [{b}] = [{result}]   ({a} + {b} = {a+b} ≡ {result} mod {n})")

    print()
    print(wrap(
        "In HoTT, [a] + [b] is well-defined by the recursion principle for "
        "quotients: the function (a,b) ↦ [(a+b) mod n] respects the relation "
        "because if a ~ a' and b ~ b', then (a+b) ~ (a'+b') (adding multiples "
        "of n gives a multiple of n)."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 6: Why quotients matter for HoTT ──────────────────────────────────

def _section_why():
    clear()
    print(box("Why Quotient Types Matter for Foundations"))
    print()
    print(wrap(
        "Quotient types are essential for doing algebra in HoTT. Without them, "
        "you cannot construct ℤ, ℚ, or any algebraic structure defined by "
        "a quotient. Here is the full picture:"
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The number tower via quotients:'))}\n")

    tower = [
        ("𝟙", "Unit type",         "Single element"),
        ("ℕ",  "Natural numbers",   "Inductive type with zero and successor"),
        ("ℤ",  "Integers",          "(ℕ×ℕ)/~ where (a,b)~(c,d) iff a+d=c+b"),
        ("ℚ",  "Rationals",         "(ℤ×ℕ₊)/~ where (p,q)~(r,s) iff p·s=r·q"),
        ("ℝ",  "Reals",             "Cauchy sequences of ℚ modulo convergence, or Dedekind cuts"),
        ("ℂ",  "Complex numbers",   "ℝ × ℝ (no quotient needed — direct product)"),
    ]

    for name, full, how in tower:
        print(f"  {bold(cyan(name)):10}  {bold(full)}")
        print(f"    {dim(how)}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('The HoTT perspective:'))}\n")
    print(wrap(
        "In set theory, quotients are fundamental but ad hoc — they require "
        "working with equivalence classes as sets-of-sets. In HoTT, quotients "
        "are instances of HITs: the relation becomes a PATH CONSTRUCTOR, and "
        "the type system automatically enforces well-definedness of functions "
        "out of quotients. There is no separate 'you must check this is "
        "well-defined' step — it is built into the type of the eliminator."
    ))
    print()
    print(wrap(
        "Moreover, univalence makes quotients transparent: if two constructions "
        "give equivalent types (e.g., two constructions of ℤ), then they are "
        "literally equal as types. The 'same up to isomorphism' of classical "
        "algebra becomes 'literally the same type' in HoTT."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("hit",     "The set-quotient as a HIT",              _section_quotient_hit),
    ("int",     "Building ℤ = (ℕ×ℕ)/~",                  _section_integers),
    ("rat",     "Building ℚ = (ℤ×ℕ₊)/~",                 _section_rationals),
    ("circle",  "S¹ as a quotient: ℝ/ℤ",                 _section_circle_quotient),
    ("groups",  "Quotient groups G/N",                    _section_quotient_groups),
    ("why",     "Why quotient types matter for foundations",_section_why),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Quotient Types: Doing Mathematics with HITs", width=70))
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
