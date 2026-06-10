#!/usr/bin/env python3
"""
demo_equiv.py — Type Equivalences and the Univalence Axiom

Experiment with equivalences between finite types and see
what the univalence axiom ua : (A ≃ B) → (A = B) means concretely.

Concepts
  • Equivalence:  f : A → B with quasi-inverse g  (f∘g ~ id, g∘f ~ id)
  • Univalence:   ua(e) : A = B  from e : A ≃ B
  • Transport:    transport(p) : A → B  for  p : A = B
  • Automorphisms of Bool:  exactly 2  (id and not)
  • Fin n equivalences:  n! possible bijections

Commands
  1    Bool automorphisms (Bool ≃ Bool has exactly 2 elements)
  2    Fin 3 bijections (explore all 6 permutations)
  3    transport demonstration (move structure along ua)
  4    equivalence checker (enter a function, verify it's an equiv)
  5    ua composition law  (ua(e) · ua(f) = ua(e ∘ f))
  h    help
  q    quit
"""

from __future__ import annotations
import sys
import itertools
import textwrap

# ── ANSI ──────────────────────────────────────────────────────────────────────

def _c(code, t): return f"\033[{code}m{t}\033[0m"
def bold(t):     return _c("1", t)
def green(t):    return _c("32", t)
def yellow(t):   return _c("33", t)
def cyan(t):     return _c("36", t)
def red(t):      return _c("31", t)
def magenta(t):  return _c("35", t)
def dim(t):      return _c("2", t)
def blue(t):     return _c("34", t)

def clear(): print("\033[2J\033[H", end="")

def wrap(s, width=70, indent=2):
    prefix = " " * indent
    return textwrap.fill(s, width=width, initial_indent=prefix,
                         subsequent_indent=prefix)

# ── Equivalence representation ────────────────────────────────────────────────

class Equiv:
    """A bijection between finite sets represented as lists."""
    def __init__(self, name: str, domain: list, codomain: list,
                 forward: dict, backward: dict, description: str = ""):
        self.name = name
        self.domain = domain
        self.codomain = codomain
        self.forward = forward   # element → element
        self.backward = backward
        self.description = description

    def apply(self, x):
        return self.forward[x]

    def apply_inv(self, y):
        return self.backward[y]

    def compose(self, other: "Equiv") -> "Equiv":
        """self then other: A → B → C."""
        fwd = {x: other.forward[self.forward[x]] for x in self.domain}
        bwd = {z: self.backward[other.backward[z]] for z in other.codomain}
        return Equiv(
            name=f"({self.name} ∘ {other.name})",
            domain=self.domain,
            codomain=other.codomain,
            forward=fwd, backward=bwd,
            description=f"Composite of {self.name} and {other.name}",
        )

    def inverse(self) -> "Equiv":
        return Equiv(
            name=f"({self.name})⁻¹",
            domain=self.codomain,
            codomain=self.domain,
            forward=self.backward, backward=self.forward,
            description=f"Inverse of {self.name}",
        )

    def is_identity(self) -> bool:
        return all(self.forward[x] == x for x in self.domain)

    def __repr__(self) -> str:
        pairs = "  ".join(f"{k}↦{v}" for k, v in self.forward.items())
        return f"Equiv[{self.name}]: {pairs}"

def _make_permutation(elements: list, perm: list) -> Equiv:
    fwd = {elements[i]: perm[i] for i in range(len(elements))}
    bwd = {v: k for k, v in fwd.items()}
    name = "(" + " ".join(str(p) for p in perm) + ")"
    return Equiv(name=name, domain=elements, codomain=elements,
                 forward=fwd, backward=bwd)

BOOL = ["true", "false"]
FIN3 = [0, 1, 2]

BOOL_AUTOS = [
    Equiv("id",   BOOL, BOOL, {"true": "true", "false": "false"},
                              {"true": "true", "false": "false"},
          "Identity: true ↦ true, false ↦ false"),
    Equiv("not",  BOOL, BOOL, {"true": "false", "false": "true"},
                              {"true": "false", "false": "true"},
          "Negation: true ↦ false, false ↦ true"),
]

FIN3_PERMS = [
    _make_permutation(FIN3, list(p))
    for p in itertools.permutations(FIN3)
]

# ── Module 1: Bool automorphisms ─────────────────────────────────────────────

def _show_bool_autos():
    clear()
    print(bold("\n  ┌──────────────────────────────────────────────────────┐"))
    print(bold(  "  │  Bool ≃ Bool  —  The Automorphism Group              │"))
    print(bold(  "  └──────────────────────────────────────────────────────┘\n"))
    print(wrap(
        "The univalence axiom tells us that (Bool = Bool) ≃ (Bool ≃ Bool). "
        "The type of self-equivalences of Bool has exactly 2 elements: "
        "id and not. There are no others.", width=72))
    print()

    for i, e in enumerate(BOOL_AUTOS):
        marker = bold(cyan(f"  e{i+1}. "))
        print(f"{marker}{bold(e.name)}: {e.description}")
        for x in BOOL:
            print(f"       {yellow(str(x))} ↦ {green(str(e.apply(x)))}")
        print()

    print(f"  {bold('Cayley table')} (composition e_i ∘ e_j):\n")
    labels = ["id ", "not"]
    header = "         " + "  ".join(bold(f"∘ {l}") for l in labels)
    print(header)
    for i, ei in enumerate(BOOL_AUTOS):
        row = f"  {bold(labels[i])}   "
        for ej in BOOL_AUTOS:
            comp = ei.compose(ej)
            is_id = comp.is_identity()
            cell = green("id ") if is_id else yellow("not")
            row += f"  {cell} "
        print(row)

    print()
    print(wrap(
        "This is ℤ/2ℤ = {0, 1} under addition! "
        "The group of automorphisms of Bool is exactly the cyclic group of order 2. "
        "By univalence, the type (Bool = Bool) is also ℤ/2ℤ — "
        "one path for each automorphism.", width=72))
    print()

    print(f"  {bold('Why only 2?')}  Any equivalence e : Bool ≃ Bool must satisfy:")
    print(f"    e(true) ∈ {{true, false}}  — 2 choices")
    print(f"    e(false) = the other one  — forced by bijectivity")
    print(f"    → exactly 2! = 2 automorphisms  ✓")
    print()
    print(wrap(
        "More generally: |Aut(Fin n)| = n!  — this is why (Fin n = Fin n) ≃ Sym(n).", width=72))
    print()
    input(dim("  Press Enter to return…"))

# ── Module 2: Fin 3 bijections ─────────────────────────────────────────────

def _show_fin3(current_idx: int = 0):
    while True:
        clear()
        e = FIN3_PERMS[current_idx]
        total = len(FIN3_PERMS)

        print(bold("\n  ┌──────────────────────────────────────────────────────┐"))
        print(bold(  "  │  Fin 3 Bijections  —  Sym(3)                         │"))
        print(bold(  "  └──────────────────────────────────────────────────────┘\n"))
        print(f"  Permutation {bold(str(current_idx + 1))} of {bold(str(total))}  {dim('(3! = 6 total)')}\n")

        # Draw the mapping as arrows
        for x in FIN3:
            arrow = "─→"
            print(f"    {bold(cyan(str(x)))}  {arrow}  {bold(green(str(e.apply(x))))}")
        print()

        # Composition with itself
        e2 = e.compose(e)
        e3 = e2.compose(e)
        print(f"  {bold('Powers')}:")
        print(f"    e¹ : {yellow(repr(e))}")
        print(f"    e² : {yellow(repr(e2))}")
        print(f"    e³ : {green(repr(e3))} {'← identity!' if e3.is_identity() else ''}")
        print()

        # Order
        order = None
        for k in range(1, 7):
            ex = e
            for _ in range(k - 1):
                ex = ex.compose(e)
            if ex.is_identity():
                order = k
                break
        print(f"  {bold('Order')}: {bold(yellow(str(order)))}  (eᵒʳᵈᵉʳ = id)")
        print()

        # Fixed points
        fixed = [x for x in FIN3 if e.apply(x) == x]
        if fixed:
            print(f"  {bold('Fixed points')}: {', '.join(str(x) for x in fixed)}")
        else:
            print(f"  {bold('Fixed points')}: {dim('none')}")
        print()

        # Is this the identity?
        if e.is_identity():
            print(f"  {green('◎ This is the identity permutation (unit of Sym(3))')}")
        print()

        print(dim(f"  [n] next  [p] prev  [c] compose two  [i] inverse  [b] back"))
        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break

        if cmd in ("b", "q", ""):
            break
        elif cmd in ("n",):
            current_idx = (current_idx + 1) % total
        elif cmd in ("p",):
            current_idx = (current_idx - 1) % total
        elif cmd == "c":
            try:
                print(f"\n  Choose second permutation (1–{total}):")
                j = int(input("  > ").strip()) - 1
                if 0 <= j < total:
                    comp = e.compose(FIN3_PERMS[j])
                    print(f"\n  {bold('Composition result')}:")
                    print(f"  {repr(comp)}")
                    is_id = comp.is_identity()
                    print(f"  {'→ identity!' if is_id else '→ non-trivial permutation'}")
            except (ValueError, KeyError):
                print(red("  Invalid input."))
            input(dim("  Press Enter…"))
        elif cmd == "i":
            inv = e.inverse()
            print(f"\n  {bold('Inverse')}: {repr(inv)}")
            input(dim("  Press Enter…"))

# ── Module 3: Transport demonstration ────────────────────────────────────────

def _show_transport():
    clear()
    print(bold("\n  ┌──────────────────────────────────────────────────────┐"))
    print(bold(  "  │  Transport along ua(e)                               │"))
    print(bold(  "  └──────────────────────────────────────────────────────┘\n"))

    print(wrap(
        "If e : A ≃ B, then ua(e) : A = B is a path in the universe. "
        "Transporting along this path converts A-structures to B-structures. "
        "Concretely: transport(ua(e)) = e.forward.", width=72))
    print()

    # Demo: Bool has a structure — a function f : Bool → Bool
    # Transport along 'not' equivalence: what happens to f?
    print(bold("  Example: transport a predicate along the 'not' equivalence\n"))
    print(dim("  Let P : Bool → Prop  be  P(x) = (x = true)  ('is x true?')"))
    print(dim("  Transport P along ua(not) gives P' : Bool → Prop\n"))

    pred_name = "is-true"
    pred = {"true": True, "false": False}

    not_equiv = BOOL_AUTOS[1]  # 'not'
    # transport(ua(not))(P) at x = P(not(x))
    # i.e., P'(x) = P(not⁻¹(x)) = P(not(x)) since not = not⁻¹
    transported_pred = {x: pred[not_equiv.apply_inv(x)] for x in BOOL}

    col1 = "Original  P"
    col2 = "Transported P'"
    print(f"  {col1:20s}   {col2:20s}")
    print(f"  {'─'*20}   {'─'*20}")
    for x in BOOL:
        orig = green("true") if pred[x] else red("false")
        trans = green("true") if transported_pred[x] else red("false")
        print(f"  P({cyan(x):14s}) = {orig:16s}   P'({cyan(x):14s}) = {trans}")

    print()
    print(wrap(
        "Observation: P'(x) = P(not(x)). The transported predicate "
        "'is-true' after swapping Bool becomes 'is-false'. "
        "This is the substitution principle in action: "
        "once Bool = Bool (via ua(not)), any Bool-predicate can be "
        "transported to a predicate on the 'other' Bool.", width=72))
    print()

    # Interactive: let user pick an automorphism and a value
    print(bold("  Try it yourself: transport a value along an automorphism\n"))
    print(dim("  Which equivalence?  1=id  2=not"))
    try:
        choice = input("  > ").strip()
        eq = BOOL_AUTOS[0] if choice == "1" else BOOL_AUTOS[1]
        print(dim(f"  Value to transport (true / false):"))
        val = input("  > ").strip().lower()
        if val in ("true", "false"):
            result = eq.apply(val)
            print()
            print(f"  transport(ua({eq.name}))({cyan(val)}) = {green(result)}")
            print()
            print(dim(f"  (transport along ua(e) acts as e.forward on elements)"))
    except (EOFError, KeyboardInterrupt):
        pass

    print()
    input(dim("  Press Enter to return…"))

# ── Module 4: Equivalence checker ────────────────────────────────────────────

def _show_equiv_checker():
    clear()
    print(bold("\n  ┌──────────────────────────────────────────────────────┐"))
    print(bold(  "  │  Equivalence Checker  (Fin 3 → Fin 3)                │"))
    print(bold(  "  └──────────────────────────────────────────────────────┘\n"))
    print(wrap(
        "Enter a function Fin3 → Fin3 by specifying where 0, 1, 2 go. "
        "We check if it is an equivalence (bijection).", width=72))
    print()

    try:
        print(dim("  Where does 0 go?"))
        a = int(input("  0 ↦ ").strip())
        print(dim("  Where does 1 go?"))
        b = int(input("  1 ↦ ").strip())
        print(dim("  Where does 2 go?"))
        c = int(input("  2 ↦ ").strip())
    except (ValueError, EOFError):
        print(red("  Invalid input."))
        input(dim("  Press Enter…"))
        return

    mapping = {0: a, 1: b, 2: c}
    values = list(mapping.values())
    print()

    # Check: all values in {0,1,2}?
    if not all(v in FIN3 for v in values):
        print(red(f"  ✗ Values {values} are not all in Fin 3 = {{0,1,2}}"))
        print(dim("  A function between Fin 3 must stay in Fin 3."))
        input(dim("  Press Enter…"))
        return

    # Check injectivity
    if len(set(values)) < 3:
        dups = [v for v in FIN3 if values.count(v) > 1]
        print(red(f"  ✗ NOT injective: {dups} appear more than once"))
        print(dim("  An equivalence must be injective (no two inputs map to the same output)."))
        input(dim("  Press Enter…"))
        return

    # It's a bijection
    inv_map = {v: k for k, v in mapping.items()}
    print(green(f"  ✓ This IS an equivalence (bijection)!"))
    print()
    print(f"  {bold('Forward')}: " + "  ".join(f"{k}↦{mapping[k]}" for k in FIN3))
    print(f"  {bold('Inverse')}: " + "  ".join(f"{k}↦{inv_map[k]}" for k in FIN3))
    print()

    # Find which permutation this is
    for i, p in enumerate(FIN3_PERMS):
        if all(p.apply(x) == mapping[x] for x in FIN3):
            print(f"  This is permutation #{i+1} in the Sym(3) table.")
            break

    # Check if it's the identity
    if all(mapping[x] == x for x in FIN3):
        print(cyan("  This is the identity equivalence (refl after ua)."))

    print()
    print(wrap(
        "By univalence, this bijection corresponds to a path "
        "ua(e) : Fin 3 = Fin 3 in the universe. "
        "Transporting along this path permutes the elements.", width=72))
    print()
    input(dim("  Press Enter to return…"))

# ── Module 5: ua composition law ────────────────────────────────────────────

def _show_ua_composition():
    clear()
    print(bold("\n  ┌──────────────────────────────────────────────────────┐"))
    print(bold(  "  │  ua Composition Law                                  │"))
    print(bold(  "  └──────────────────────────────────────────────────────┘\n"))
    print(wrap(
        "The univalence axiom ua : (A ≃ B) → (A = B) is itself "
        "an equivalence. It satisfies the composition law:", width=72))
    print()
    print(f"  {bold('ua(e) · ua(f)')} = {bold('ua(f ∘ e)')}")
    print()
    print(wrap(
        "where · is path concatenation and ∘ is equivalence composition. "
        "This means the group structure on Aut(A) matches the loop space "
        "structure on (A = A) — univalence is a group isomorphism.", width=72))
    print()

    # Demonstrate with Bool
    id_e   = BOOL_AUTOS[0]  # id
    not_e  = BOOL_AUTOS[1]  # not

    print(f"  {bold('Bool example')} (two automorphisms: id, not)\n")

    pairs = [
        (id_e,  id_e,  "id ∘ id  = id"),
        (id_e,  not_e, "id ∘ not = not"),
        (not_e, id_e,  "not ∘ id = not"),
        (not_e, not_e, "not ∘ not = id  ← two negations cancel!"),
    ]

    for e, f, note in pairs:
        comp = e.compose(f)
        comp_name = "id" if comp.is_identity() else "not"
        col = green if comp.is_identity() else yellow
        print(f"  ua({e.name}) · ua({f.name})  =  ua({e.name} ∘ {f.name})  =  ua({col(comp_name)})")

    print()
    print(wrap(
        "Notice not ∘ not = id: applying 'not' twice returns to the identity. "
        "In the loop space language: the loop ua(not) traversed twice is "
        "homotopic to refl (the trivial loop ua(id)).", width=72))
    print()
    print(wrap(
        "The inverse law: ua(e)⁻¹ = ua(e⁻¹). "
        "Path inversion corresponds to taking the inverse equivalence.", width=72))
    print()

    print(f"  {bold('Proof sketch')} (from the univalence axiom):")
    print(dim("  1. ua is an equivalence: equiv-to-path is its quasi-inverse"))
    print(dim("  2. Groupoid laws in the universe follow from path laws"))
    print(dim("  3. ua is a group homomorphism (Aut, ∘) → (Ω(U, A), ·)"))
    print(dim("  4. univalence makes it an isomorphism"))
    print()
    input(dim("  Press Enter to return…"))

# ── Help ─────────────────────────────────────────────────────────────────────

def _help():
    clear()
    print(f"""
  {bold('Type Equivalence & Univalence — Commands')}

  {cyan('1')}   Bool automorphisms  (why |Aut(Bool)| = 2)
  {cyan('2')}   Fin 3 bijections    (explore all 6 permutations of Sym(3))
  {cyan('3')}   Transport along ua  (move structure across an equivalence path)
  {cyan('4')}   Equivalence checker (enter a map, verify it's a bijection)
  {cyan('5')}   ua composition law  (ua(e) · ua(f) = ua(f ∘ e))
  {cyan('h')}   this help
  {cyan('q')}   quit
""")
    input(dim("  Press Enter…"))

def main():
    dispatch = {
        "1": _show_bool_autos,
        "2": lambda: _show_fin3(0),
        "3": _show_transport,
        "4": _show_equiv_checker,
        "5": _show_ua_composition,
        "h": _help,
    }
    while True:
        clear()
        print(bold("\n  ╔════════════════════════════════════════════════════════╗"))
        print(bold(  "  ║   Type Equivalences & Univalence                       ║"))
        print(bold(  "  ╚════════════════════════════════════════════════════════╝\n"))
        print(f"  {bold('The Univalence Axiom')}  (Voevodsky, HoTT):  {cyan('(A ≃ B) ≃ (A = B)')}\n")
        print(wrap(
            "An equivalence e : A ≃ B gives a path ua(e) : A = B in the "
            "type-universe. Transport along this path is the equivalence itself. "
            "This makes equality of types as rich as equivalence.", width=72))
        print()
        print(f"  {bold('Choose an experiment')}:")
        print(f"  {cyan('1')}  Bool ≃ Bool  —  automorphism group (2 elements)")
        print(f"  {cyan('2')}  Fin 3 bijections  —  Sym(3) with 6 permutations")
        print(f"  {cyan('3')}  Transport demonstration")
        print(f"  {cyan('4')}  Check if your function is an equivalence")
        print(f"  {cyan('5')}  ua composition law")
        print(f"  {cyan('h')}  help    {cyan('q')}  quit")
        print()
        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break
        if cmd in ("q", "quit", "exit", "0"):
            break
        fn = dispatch.get(cmd)
        if fn:
            fn()


if __name__ == "__main__":
    clear()
    print(bold("""
  ╔════════════════════════════════════════════════════════╗
  ║       Type Equivalences & Univalence Axiom             ║
  ╚════════════════════════════════════════════════════════╝
"""))
    print("""  The Univalence Axiom says:

    (A ≃ B) ≃ (A = B)

  An equivalence between types IS a path between types.
  Transporting along ua(e) acts exactly as e.

  This demo lets you experiment with:
    • All bijections between finite types (Fin n)
    • What transport does to structures
    • How ua respects composition and inversion
""")
    input(dim("  Press Enter to start…"))
    main()
    print(f"\n  {dim('Univalence: equivalence and identity are the same concept.')}\n")
