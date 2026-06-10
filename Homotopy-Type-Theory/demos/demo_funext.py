#!/usr/bin/env python3
"""
Function Extensionality: When Are Functions Equal?
===================================================
funext: (∀x. f x = g x) → f = g

In intensional type theory, two functions are definitionally equal only if
they reduce to the same normal form. Functions that agree on all inputs but
are defined differently are NOT automatically equal. Function extensionality
(funext) asserts they ARE equal — and this follows from univalence.
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


# ── Section 1: The problem ────────────────────────────────────────────────────

def _section_problem():
    clear()
    print(box("The Problem: When Are Two Functions Equal?"))
    print()
    print(wrap(
        "Consider two functions f, g : ℕ → ℕ that compute the same values "
        "on every input — but by different algorithms. Are they equal as functions?"
    ))
    print()
    print(f"  {bold(green('In mathematics (extensional view):'))}")
    print(f"  {cyan('  f = g  iff  ∀n. f n = g n')}")
    print(f"  {'':4}If they agree on all inputs, they are the same function. Period.")
    print()
    print(f"  {bold(yellow('In intensional type theory (MLTT):'))}")
    print(f"  {cyan('  f ≡ g  (definitional equality) requires syntactic reduction')}")
    print(f"  {'':4}f and g must reduce to the same normal form to be DEFINITIONALLY equal.")
    print(f"  {'':4}Propositional equality f = g may or may not hold.")
    print()
    print(rule())
    print(f"\n  {bold(red('Concrete example: two additions'))}\n")
    print(f"  {cyan('f : ℕ → ℕ → ℕ')}")
    print(f"  {cyan('f n m = rec_ℕ m (λ_ r. succ r) n    -- fold succ m times into n')}")
    print()
    print(f"  {cyan('g : ℕ → ℕ → ℕ')}")
    print(f"  {cyan('g n m = rec_ℕ n (λ_ r. succ r) m    -- fold succ n times into m')}")
    print()
    print(wrap(
        "Both f and g compute addition, and they agree on every input. But "
        "f(0)(m) ≡ m definitionally while g(0)(m) requires induction on m. "
        "They are propositionally equal (∀n m. f n m = g n m is provable), "
        "but they are NOT definitionally equal."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('In Python, this distinction vanishes:'))}\n")

    def f(n, m):
        result = m
        for _ in range(n):
            result += 1
        return result

    def g(n, m):
        result = n
        for _ in range(m):
            result += 1
        return result

    print(f"  f = lambda n, m: fold n times starting from m")
    print(f"  g = lambda n, m: fold m times starting from n")
    print()
    for n, m in [(0, 5), (3, 4), (7, 0), (2, 2)]:
        same = "✓" if f(n,m) == g(n,m) else "✗"
        print(f"  f({n},{m}) = {f(n,m)},  g({n},{m}) = {g(n,m)}  {green(same)}")
    print()
    print(wrap(
        "Python's == tests VALUES, not definitions. In HoTT, propositional "
        "equality (=) also tests values (pointwise), but definitional equality (≡) "
        "tests REDUCTION BEHAVIOR. They are different concepts."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 2: happly — the easy direction ────────────────────────────────────

def _section_happly():
    clear()
    print(box("happly: The Easy Direction"))
    print()
    print(wrap(
        "Given f = g (a path between functions), we can derive pointwise equality. "
        "This direction is always provable by path induction."
    ))
    print()
    print(f"  {cyan('happly : f = g → ∀(x:A). f x = g x')}")
    print(f"  {cyan('happly refl x = refl')}")
    print()
    print(wrap(
        "Proof: by path induction on f = g. When the path is refl (f = f), "
        "happly refl x = refl : f x = f x. The general case follows. □"
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('happly is always available in any type theory:'))}\n")
    print(wrap(
        "No axioms needed. If you have a path f = g in a function type, "
        "you can always apply both sides to any argument and get a path. "
        "The function type respects path structure — it is a π-type."
    ))
    print()
    print(f"  {bold('For dependent functions:')}")
    print(f"  {cyan('happlyD : f = g → ∀(x:A). f x =_{{P x}} g x')}")
    print()
    print(wrap(
        "Even for dependent functions f g : Π(x:A). P(x), if f = g then "
        "f x and g x are equal — but the equality lives in P(x), the fiber "
        "over x. (They might be in the same type even though P varies.)"
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('What we CANNOT do without funext:'))}\n")
    print(wrap(
        "The converse — going from pointwise equality (∀x. f x = g x) back "
        "to function equality (f = g) — requires an AXIOM in intensional MLTT. "
        "Without it, you might have:"
    ))
    print()
    print(f"  {cyan('h : ∀x. f x = g x')}")
    print(f"  {dim('-- h gives you pointwise proofs of equality')}")
    print(f"  {dim('-- but you CANNOT in general construct a path f = g')}")
    print()
    print(wrap(
        "This is the gap: knowing f and g agree everywhere does not, by itself, "
        "give you a path between f and g in the function type."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 3: funext as an axiom ────────────────────────────────────────────

def _section_funext_axiom():
    clear()
    print(box("Function Extensionality as an Axiom"))
    print()
    print(wrap(
        "Function extensionality (funext) postulates that happly is an equivalence:"
    ))
    print()
    print(f"  {cyan('funext : (∀x. f x = g x) → f = g')}")
    print()
    print(f"  {cyan('happly-funext : happly (funext h) x = h x')}")
    print(f"  {cyan('funext-happly : funext (happly p) = p')}")
    print()
    print(wrap(
        "These two laws say funext and happly are INVERSES — they form an "
        "equivalence between (f = g) and (∀x. f x = g x)."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('What funext buys you:'))}\n")

    consequences = [
        ("Identity functions are equal",
         "id ∘ f = f  (provable; funext makes  id ∘ f = f  a path)"),
        ("Composition is associative",
         "(f ∘ g) ∘ h = f ∘ (g ∘ h)  (propositional, using funext)"),
        ("η-expansion holds",
         "f = (λx. f x)  — the η-rule holds propositionally"),
        ("Pointwise = implies global =",
         "If you prove ∀x. map f x = map g x, conclude map f = map g"),
        ("Functional programs are proofs",
         "Two programs with the same spec are equal — enables quotient types on programs"),
    ]

    for title, note in consequences:
        print(f"  {bold(cyan('•'))} {bold(title)}")
        print(wrap(note, width=66, indent="    "))
        print()

    print(rule())
    print(f"\n  {bold(yellow('Without funext: pathological models'))}\n")
    print(wrap(
        "There are models of MLTT where funext FAILS. In these models, two "
        "functions can agree on all inputs yet be distinguishable. These are "
        "exotic 'operational' models where type theory is computing rather than "
        "doing mathematics. For doing mathematics, funext is essential."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 4: funext from univalence ────────────────────────────────────────

def _section_from_univalence():
    clear()
    print(box("Deriving funext from Univalence"))
    print()
    print(wrap(
        "One of the key features of HoTT: function extensionality is a THEOREM, "
        "not an axiom. It follows from univalence."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The proof sketch (Voevodsky):'))}\n")

    steps = [
        ("Step 1", "The path space of a product",
         "For A × B, we have (a,b) = (a',b') ≃ (a=a') × (b=b'). "
         "Paths in a product are PAIRS of paths in the components."),
        ("Step 2", "Functions as sections",
         "A function f : A → B can be viewed as a section of the projection "
         "π₁ : A × B → A. Two functions f, g correspond to two sections."),
        ("Step 3", "Paths between sections",
         "A path between sections corresponds to a pointwise path in the fibers. "
         "For f, g : A → B, a path f = g in Π(x:A). B corresponds to "
         "a section of the path fibration over A."),
        ("Step 4", "Univalence provides the equivalence",
         "By univalence, paths in Type are equivalences. The universe's path "
         "space carries enough structure to convert between f=g and ∀x. fx=gx. "
         "The key is that the type of sections is itself computed fiberwise."),
        ("Step 5", "Conclusion",
         "happly is an equivalence, so funext = happly⁻¹ exists. □"),
    ]

    for label, title, note in steps:
        print(f"  {bold(green(label))}: {bold(title)}")
        print(wrap(note, width=66, indent="    "))
        print()

    print(rule())
    print(f"\n  {bold(yellow('A cleaner proof via interval types (Cubical):'))}\n")
    print(wrap(
        "In Cubical HoTT, there is an interval type 𝕀 with endpoints 0, 1. "
        "A path f = g is literally a function 𝕀 → (A → B). Given h : ∀x. fx=gx, "
        "define p : 𝕀 → (A → B) by p(i) = λx. h(x)(i). Then p(0) = f and p(1) = g, "
        "so p is a path from f to g. This makes funext definitionally valid — "
        "no axiom needed at all."
    ))
    print()
    print(wrap(
        "This is one of the main advantages of Cubical Type Theory: results "
        "that require axioms in Book HoTT become definitional equalities, "
        "making computation inside proofs tractable."
    ))
    print()
    input(bold("  Press Enter to continue... "))


# ── Section 5: Dependent funext and consequences ─────────────────────────────

def _section_dep_funext():
    clear()
    print(box("Dependent funext and Practical Consequences"))
    print()
    print(wrap(
        "The dependent version of funext applies to dependent functions "
        "f g : Π(x:A). P(x):"
    ))
    print()
    print(f"  {cyan('dfunext : (∀x. f x =_{{P x}} g x) → f = g')}")
    print()
    print(wrap(
        "Here the pointwise equalities live in the FIBERS P(x), which may "
        "differ for each x. The result is still a path f = g in the function type."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The type of paths in a Π-type:'))}\n")
    print(f"  {cyan('(f = g) ≃ ∀(x:A). f x = g x')}")
    print()
    print(wrap(
        "The path space of a Π-type is again a Π-type. This is a general "
        "principle: paths in 'structured' types are 'structured paths'. "
        "For products it's pairs of paths; for functions it's functions of paths."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Practical consequences in formal verification:'))}\n")

    examples = [
        ("Equality of natural number functions",
         "add_comm : ∀n m. n+m = m+n → add = flip add",
         "Given pointwise commutativity, funext gives add = flip add as functions."),
        ("Equality of list operations",
         "map_id : ∀xs. map id xs = xs → map id = id",
         "map id and id agree on all lists; funext concludes they are equal."),
        ("Algebra laws",
         "monoid laws η: e·f = f = f·e → e is unit for all f",
         "η-conversion rules become equalities of functions, not just pointwise equalities."),
        ("Program correctness",
         "Two sorting algorithms agree → equal as functions Sort → List",
         "If proved correct (same output on all inputs), they are propositionally equal."),
    ]

    for title, form, note in examples:
        print(f"  {bold(cyan(title))}")
        print(f"  {dim(form)}")
        print(wrap(note, width=66, indent="    "))
        print()

    print(rule())
    print(f"\n  {bold(green('Live Python illustration:'))}\n")

    import functools

    def add_left(n, m): return n + m
    def add_right(n, m): return m + n

    print(f"  add_left  = lambda n, m: n + m")
    print(f"  add_right = lambda n, m: m + n")
    print()
    print(f"  Pointwise agreement:")
    for n, m in [(0,5),(3,3),(7,2)]:
        eq = add_left(n,m) == add_right(n,m)
        print(f"    add_left({n},{m}) = {add_left(n,m)} = add_right({n},{m})  {green('✓') if eq else red('✗')}")
    print()
    print(f"  {dim('In Python: add_left == add_right is False (different objects)')}")
    print(f"  {dim('In HoTT:   funext(∀n m. add_left n m = add_right n m) proves them equal')}")
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("problem",    "The problem: intensional vs. extensional equality", _section_problem),
    ("happly",     "happly: the easy direction",                        _section_happly),
    ("axiom",      "funext as an axiom and its consequences",           _section_funext_axiom),
    ("univalence", "Deriving funext from univalence",                   _section_from_univalence),
    ("dependent",  "Dependent funext and practical consequences",       _section_dep_funext),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Function Extensionality: When Are Functions Equal?", width=70))
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
