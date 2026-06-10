#!/usr/bin/env python3
"""
Dependent Types: Safety Through Precision
=========================================
Vec n, Fin n, Sigma types — types that carry information about their values.

In dependent type theory, the TYPE of an expression can depend on the VALUE
of another expression. This allows the type system to express invariants that
ordinary types cannot, ruling out entire classes of runtime errors at compile time.
"""

import textwrap, os

# ─── ANSI helpers ────────────────────────────────────────────────────────────
def _c(code, t): return f"\033[{code}m{t}\033[0m"
bold    = lambda t: _c("1",    t)
green   = lambda t: _c("32",   t)
yellow  = lambda t: _c("33",   t)
cyan    = lambda t: _c("36",   t)
red     = lambda t: _c("31",   t)
dim     = lambda t: _c("2",    t)
magenta = lambda t: _c("35",   t)

def clear():
    print("\033[2J\033[H", end="")

def wrap(text, width=70, indent="  "):
    lines = []
    for paragraph in text.strip().split("\n"):
        if paragraph.strip() == "":
            lines.append("")
        else:
            lines.extend(textwrap.wrap(paragraph, width, initial_indent=indent,
                                       subsequent_indent=indent))
    return "\n".join(lines)

def box(title, width=66):
    inner = width - 2
    return (f"  ╔{'═' * inner}╗\n"
            f"  ║  {bold(title):<{inner - 2}}║\n"
            f"  ╚{'═' * inner}╝")

def rule(width=68):
    return "  " + dim("─" * width)

# ─── Vec n implementation ────────────────────────────────────────────────────
class Vec:
    """
    Length-indexed vector. The length n is part of the type.
    Vec(n, xs) asserts len(xs) == n — enforced at construction.
    """
    def __init__(self, n: int, xs: list):
        if len(xs) != n:
            raise TypeError(
                f"Vec {n} constructed with {len(xs)} elements — type mismatch")
        self.n = n
        self.xs = list(xs)

    def __repr__(self):
        return f"Vec {self.n} {self.xs}"

    def head(self):
        """Total: only callable when n > 0 (checked at construction)."""
        if self.n == 0:
            raise TypeError("Vec.head called on Vec 0 — impossible in real HoTT")
        return self.xs[0]

    def tail(self):
        if self.n == 0:
            raise TypeError("Vec.tail called on Vec 0 — impossible in real HoTT")
        return Vec(self.n - 1, self.xs[1:])

    def cons(self, x):
        return Vec(self.n + 1, [x] + self.xs)

    def zip_with(self, f, other):
        """zip_with : (A → B → C) → Vec n A → Vec n B → Vec n C"""
        if self.n != other.n:
            raise TypeError(
                f"zip_with: length mismatch {self.n} ≠ {other.n} — type error")
        return Vec(self.n, [f(a, b) for a, b in zip(self.xs, other.xs)])

    def lookup(self, i: "Fin") -> object:
        """lookup : Fin n → Vec n A → A  (always in bounds)"""
        if not (0 <= i.val < self.n):
            raise TypeError(f"Fin {i.val} out of range for Vec {self.n}")
        return self.xs[i.val]

# ─── Fin n implementation ────────────────────────────────────────────────────
class Fin:
    """
    Bounded natural. Fin(n, k) represents k : Fin n, i.e. k < n.
    """
    def __init__(self, n: int, val: int):
        if not (0 <= val < n):
            raise TypeError(f"Fin {n}: value {val} out of range [0, {n-1}]")
        self.n = n
        self.val = val

    def __repr__(self):
        return f"Fin {self.n} ∋ {self.val}"

    def weaken(self):
        """Fin n → Fin (n+1)"""
        return Fin(self.n + 1, self.val)

    def raise_by(self, k: int):
        """Shift: Fin n → Fin (n+k)"""
        return Fin(self.n + k, self.val)


# ─── Sigma type simulation ───────────────────────────────────────────────────
class Sigma:
    """
    Sigma(fst, snd) represents (fst, snd) : Σ(a:A). P(a).
    The second component is a proof or value that depends on fst.
    """
    def __init__(self, fst, snd, label="proof"):
        self.fst = fst
        self.snd = snd
        self.label = label

    def __repr__(self):
        return f"⟨{self.fst}, {self.label}⟩"

# ─── Section content ─────────────────────────────────────────────────────────

def _section_intro():
    clear()
    print(box("Dependent Types: Safety Through Precision"))
    print()
    print(wrap(
        "In simple type theory, types are fixed independently of values. "
        "In DEPENDENT type theory, the type of one expression can DEPEND ON "
        "the VALUE of another. This is the fundamental innovation that "
        "separates Martin-Löf Type Theory (and HoTT) from simply-typed systems."
    ))
    print()
    print(rule())
    print()
    print(f"  {bold('Three key dependent constructions:')}")
    print()
    print(f"  {bold(cyan('Vec n A'))}     — A list with length n baked into the type")
    print(f"  {bold(cyan('Fin n'))}       — A natural number provably less than n")
    print(f"  {bold(cyan('Σ(x:A). P(x)'))} — A pair where the second component's type depends on the first")
    print()
    print(wrap(
        "Together they enable TOTAL FUNCTIONS: functions that are safe by "
        "construction, not by runtime check. The type checker verifies at "
        "compile time that, for instance, you never call head on an empty "
        "vector or index a list out of bounds."
    ))
    print()
    print(rule())
    print(wrap(
        "Press Enter to continue, or 'q' to quit this section."
    ))
    input(bold("  > "))


def _section_vec():
    clear()
    print(box("Vec n — Length-Indexed Vectors"))
    print()
    print(wrap(
        "In Agda or Coq, Vec is a type family:"
    ))
    print()
    print(f"  {cyan('data Vec (A : Set) : ℕ → Set where')}")
    print(f"  {cyan('  nil  : Vec A 0')}")
    print(f"  {cyan('  cons : A → Vec A n → Vec A (n+1)')}")
    print()
    print(wrap(
        "The n in 'Vec A n' is not a runtime tag — it is the ACTUAL TYPE. "
        "Vec 3 ℕ and Vec 4 ℕ are different types, just as ℕ and Bool are. "
        "This means the compiler rejects any program that confuses them."
    ))
    print()
    print(rule())

    # Demo: construction
    print(f"\n  {bold(green('Construction'))}")
    v3 = Vec(3, [10, 20, 30])
    v3b = Vec(3, [1, 2, 3])
    print(f"  v3  = {v3}")
    print(f"  v3b = {v3b}")

    print(f"\n  {bold(green('Safe head and tail (total functions)'))}")
    print(f"  v3.head()    = {v3.head()}")
    print(f"  v3.tail()    = {v3.tail()}")
    print(f"  v3.tail().head() = {v3.tail().head()}")

    print(f"\n  {bold(green('zip_with (+) v3 v3b'))}")
    vsum = v3.zip_with(lambda a, b: a + b, v3b)
    print(f"  result = {vsum}")

    print(f"\n  {bold(green('cons 0 v3  (Vec 3 → Vec 4)'))}")
    v4 = v3.cons(0)
    print(f"  result = {v4}")

    print()
    print(rule())
    print(f"\n  {bold(yellow('What happens with mismatched lengths?'))}")
    print()
    v2 = Vec(2, [7, 8])
    print(f"  v2 = {v2}")
    print(f"  Attempting zip_with (+) v3 v2 ...")
    try:
        v3.zip_with(lambda a, b: a + b, v2)
    except TypeError as e:
        print(f"  {red('TypeError:')} {e}")
    print()
    print(wrap(
        "In real HoTT, this is not a runtime error — it is a TYPE ERROR "
        "caught before execution. The program simply cannot be written. "
        "Python cannot enforce this at the language level, so we simulate "
        "it with explicit checks at the boundary."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Why not just use lists and check at runtime?'))}")
    print()
    print(wrap(
        "In Python, [10, 20, 30] and [1, 2] are both 'list'. The type "
        "system has no way to distinguish them, so functions must either "
        "check at runtime (failing with exceptions) or trust the caller "
        "(failing silently). Dependent types move these guarantees to "
        "compile time: a function typed Vec n A → Vec n A → Vec n A "
        "CANNOT be called with mismatched lengths."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_fin():
    clear()
    print(box("Fin n — Bounded Naturals / Safe Array Indices"))
    print()
    print(wrap(
        "Fin n is the type of natural numbers strictly less than n. "
        "It has exactly n elements: 0, 1, ..., n-1."
    ))
    print()
    print(f"  {cyan('data Fin : ℕ → Set where')}")
    print(f"  {cyan('  fzero : Fin (n+1)')}")
    print(f"  {cyan('  fsucc : Fin n → Fin (n+1)')}")
    print()
    print(wrap(
        "Notice: Fin 0 is the EMPTY TYPE. There is no term of type Fin 0, "
        "so you can never index an empty vector. The type system makes "
        "the impossible literally impossible."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Fin values'))}")
    f0 = Fin(5, 0)
    f3 = Fin(5, 3)
    f4 = Fin(5, 4)
    print(f"  fzero     = {f0}   (smallest index into Vec 5)")
    print(f"  fsucc³(0) = {f3}")
    print(f"  flast     = {f4}   (largest valid index into Vec 5)")

    print(f"\n  {bold(green('Safe vector lookup: lookup : Fin n → Vec n A → A'))}")
    v5 = Vec(5, ['a', 'b', 'c', 'd', 'e'])
    print(f"  v5 = {v5}")
    print(f"  v5.lookup(Fin 5 ∋ 0) = {v5.lookup(f0)!r}")
    print(f"  v5.lookup(Fin 5 ∋ 3) = {v5.lookup(f3)!r}")
    print(f"  v5.lookup(Fin 5 ∋ 4) = {v5.lookup(f4)!r}")

    print(f"\n  {bold(green('Weakening: Fin n → Fin (n+1)'))}")
    fw = f3.weaken()
    print(f"  f3.weaken() = {fw}  (still valid in the larger type)")

    print()
    print(rule())
    print(f"\n  {bold(yellow('Out-of-bounds: the type-theoretic view'))}")
    print()
    print(wrap(
        "In Python, lst[5] on a length-5 list raises IndexError. In a "
        "dependently typed language, you cannot even FORM the expression "
        "lst[5] when lst : Vec 5 A, because 5 does not inhabit Fin 5. "
        "The out-of-bounds access is ruled out by the grammar of the language."
    ))
    print()
    print(f"  {bold('Python (runtime failure):')}")
    lst = ['a', 'b', 'c', 'd', 'e']
    print(f"  lst = {lst}")
    print(f"  lst[5]  →  ", end="")
    try:
        _ = lst[5]
    except IndexError as e:
        print(red(f"IndexError: {e}"))
    print()
    print(f"  {bold('HoTT (type error, not runtime):')}")
    print(f"  {dim('-- Cannot construct Fin 5 ∋ 5: 5 ≮ 5')}")
    print(f"  {cyan('lookup (fsucc⁵ fzero) v5  -- type error at compile time')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_sigma():
    clear()
    print(box("Σ-Types — Dependent Pairs Carrying Proofs"))
    print()
    print(wrap(
        "The sigma type Σ(x:A). P(x) is a generalization of the Cartesian "
        "product A × B. In A × B, the type B is fixed. In Σ(x:A). P(x), "
        "the TYPE of the second component depends on the VALUE of the first."
    ))
    print()
    print(f"  {cyan('data Σ (A : Set) (P : A → Set) : Set where')}")
    print(f"  {cyan('  _,_ : (a : A) → P a → Σ A P')}")
    print()
    print(wrap(
        "A term of Σ(x:A). P(x) is a PAIR (a, p) where p : P(a). "
        "The second component is a certificate, proof, or data whose "
        "type is determined by the first component."
    ))
    print()
    print(rule())

    print(f"\n  {bold(green('Example 1: Σ(n:ℕ). Vec n ℕ  (a list together with its length)'))}")
    print()
    v = Vec(4, [2, 4, 6, 8])
    s1 = Sigma(4, v, label=str(v))
    print(f"  pair = {s1}")
    print(f"  fst  = {s1.fst}   (the length)")
    print(f"  snd  = {s1.snd}   (a Vec 4 ℕ)")
    print()
    print(wrap(
        "The type of snd depends on fst. If fst = 4, then snd : Vec 4 ℕ. "
        "Changing fst to 3 would require snd to be Vec 3 ℕ — a different type."
    ))

    print(f"\n  {bold(green('Example 2: Σ(n:ℕ). (n > 0)  (a positive natural number)'))}")
    print()
    print(f"  {cyan('IsPositive : ℕ → Prop')}")
    print(f"  {cyan('IsPositive n = n > 0')}")
    print()
    s2 = Sigma(7, True, label="proof(7>0)")
    s3 = Sigma(42, True, label="proof(42>0)")
    print(f"  seven      = {s2}   (7, together with proof 7 > 0)")
    print(f"  forty_two  = {s3}   (42, together with proof 42 > 0)")
    print()
    print(wrap(
        "This is a REFINEMENT TYPE: not just any ℕ, but one that carries "
        "a proof of positivity. Division by such a number is safe — the "
        "type system guarantees the denominator is nonzero."
    ))

    print(f"\n  {bold(green('Example 3: Σ(n:ℕ). Fin n  (a \"safe\" integer range)'))}")
    print()
    print(f"  {cyan('-- (n, i) : Σ ℕ Fin  means  i is a valid index for length n')}")
    s4 = Sigma(10, Fin(10, 7), label=repr(Fin(10, 7)))
    s5 = Sigma(3, Fin(3, 0), label=repr(Fin(3, 0)))
    print(f"  pair1 = ⟨10, {s4.snd}⟩   (index 7 into something of length 10)")
    print(f"  pair2 = ⟨3,  {s5.snd}⟩   (index 0 into something of length 3)")
    print()

    print(rule())

    print(f"\n  {bold(yellow('The fundamental shift: proofs as data'))}")
    print()
    print(wrap(
        "In ordinary programming, a function might take an integer n and "
        "assert 0 ≤ n < len(array) with a runtime check. In dependent "
        "type theory, this assertion IS PART OF THE TYPE. The function "
        "signature"
    ))
    print()
    print(f"  {cyan('safe_div : ℝ → Σ(r:ℝ). (r ≠ 0) → ℝ')}")
    print()
    print(wrap(
        "does not require a runtime check because the caller MUST PROVIDE "
        "a proof of r ≠ 0 as part of the argument. If they cannot construct "
        "this proof, the program does not compile. Correctness is no longer "
        "a property we check — it is a property we prove, once, at definition time."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_comparison():
    clear()
    print(box("Comparison: Simple Types vs. Dependent Types"))
    print()

    rows = [
        ("Operation",          "Simple types",              "Dependent types"),
        ("─" * 22,             "─" * 26,                    "─" * 26),
        ("List head",          "head : [A] → A",            "head : Vec (n+1) A → A"),
        ("",                   dim("runtime: []→exception"),  dim("total: Vec 0 has no head")),
        ("List index",         "(!!) : [A] → Int → A",      "lookup : Fin n → Vec n A → A"),
        ("",                   dim("runtime: out-of-bounds"), dim("total: Fin n rules it out")),
        ("Positive number",    "type: Int",                  "type: Σ(n:ℕ). n>0"),
        ("",                   dim("runtime: assert n>0"),   dim("compile-time proof required")),
        ("Matrix multiply",    "mat_mul :: Mat → Mat → Mat","mat_mul : Mat m n→Mat n p→Mat m p"),
        ("",                   dim("runtime: dim check"),    dim("n must match: type error")),
        ("Sorted list",        "type: [A]",                  "type: Σ xs. IsSorted xs"),
        ("",                   dim("convention only"),       dim("sortedness is a proof term")),
    ]

    col_w = [24, 28, 28]
    header = rows[0]
    sep    = rows[1]

    def fmt_row(r, is_header=False):
        parts = []
        for i, cell in enumerate(r):
            w = col_w[i]
            if is_header:
                parts.append(bold(cell.ljust(w)))
            else:
                parts.append(cell.ljust(w))
        print("  " + "  ".join(parts))

    fmt_row(header, is_header=True)
    fmt_row(sep)
    for row in rows[2:]:
        fmt_row(row)

    print()
    print(rule())
    print()
    print(wrap(
        "The key insight: in dependent type theory, the distinction between "
        "'type' and 'proposition' dissolves. A type IS a proposition (the "
        "Curry-Howard correspondence). To HAVE a value of type P is to HAVE "
        "a proof of proposition P. Dependent types allow types to ENCODE "
        "propositions about their values, making the type checker a proof checker."
    ))
    print()
    print(rule())

    print(f"\n  {bold(yellow('Live Python contrast: head on an empty list'))}")
    print()
    print(f"  {bold('Python (simple types):')}")
    print(f"  {'lst = []':40}  {dim('# type: list')}")
    print(f"  {'lst[0]':40}  →  ", end="")
    try:
        _ = [][0]
    except IndexError as e:
        print(red(f"IndexError: {e}"))
    print()
    print(f"  {bold('HoTT (dependent types):')}")
    print(f"  {cyan('nil : Vec 0 A')}")
    print(f"  {cyan('head nil  -- DOES NOT TYPECHECK')}")
    print(f"  {dim('-- head : Vec (n+1) A → A requires n+1 > 0 in the type')}")
    print(f"  {dim('-- Vec 0 A does not unify with Vec (n+1) A')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_why_hott():
    clear()
    print(box("Why HoTT Goes Further: Dependent Types + Univalence"))
    print()
    print(wrap(
        "Dependent types alone (Martin-Löf Type Theory) give you Vec n, "
        "Fin n, and Sigma types. HoTT adds univalence and higher inductive "
        "types, which interact with dependent types in surprising ways."
    ))
    print()
    print(rule())

    print(f"\n  {bold(green('Transport: paths act on type families'))}")
    print()
    print(wrap(
        "Given a type family P : A → Type and a path p : a₀ = a₁ in A, "
        "transport gives you a function:"
    ))
    print()
    print(f"  {cyan('transport P p : P a₀ → P a₁')}")
    print()
    print(wrap(
        "For Vec: if you have a proof that n = m (a path in ℕ), then "
        "transport (Vec · A) p : Vec n A → Vec m A. Propositional equality "
        "of lengths lets you safely coerce vectors."
    ))

    print(f"\n  {bold(green('Univalence: type equivalences are paths'))}")
    print()
    print(wrap(
        "If you have an equivalence e : A ≃ B, then ua(e) : A = B is a path. "
        "Transporting along ua(e) gives the coercion. For dependent types, "
        "this means: if A and B are 'the same structure' (equivalent), then "
        "any property provable for A automatically holds for B."
    ))

    print(f"\n  {bold(green('HITs: new types with prescribed path structure'))}")
    print()
    print(wrap(
        "Higher Inductive Types let you define types by specifying not just "
        "their POINTS but also their PATHS (and higher paths). The circle S¹ "
        "has one point (base) and one loop (loop : base = base). This is a "
        "dependent type: the type 'loop' depends on 'base'."
    ))
    print()
    print(f"  {cyan('data S¹ : Type where')}")
    print(f"  {cyan('  base : S¹')}")
    print(f"  {cyan('  loop : base = base')}")
    print()
    print(wrap(
        "Functions out of S¹ must specify where base goes AND where loop goes "
        "(a path in the image of base). The DEPENDENT eliminator captures this: "
        "to define f : S¹ → P, you give f(base) : P and a proof that "
        "transport P loop (f base) = f base."
    ))
    print()
    print(rule())
    print()
    print(wrap(
        "The synthesis: dependent types give you precision; HITs give you "
        "geometric structure; univalence gives you transportability of proofs. "
        "Together they form a foundation where mathematics and verified "
        "computation are the same activity."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


# ─── Main loop ───────────────────────────────────────────────────────────────
SECTIONS = [
    ("intro",    "Introduction: What Are Dependent Types?",         _section_intro),
    ("vec",      "Vec n — Length-Indexed Vectors",                  _section_vec),
    ("fin",      "Fin n — Bounded Naturals / Safe Array Indices",   _section_fin),
    ("sigma",    "Σ-Types — Dependent Pairs Carrying Proofs",       _section_sigma),
    ("compare",  "Comparison: Simple Types vs. Dependent Types",    _section_comparison),
    ("hott",     "Why HoTT Goes Further: Dependent Types + UA",     _section_why_hott),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Dependent Types: Safety Through Precision", width=70))
        print()
        for i, (key, title, _) in enumerate(SECTIONS):
            marker = bold(cyan("▶")) if i == idx else " "
            print(f"  {marker} {bold(str(i+1))}   {title}")
        print()
        print(rule())
        print(f"  {dim('1-6  jump to section   n  next   p  prev   q  quit')}")
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
