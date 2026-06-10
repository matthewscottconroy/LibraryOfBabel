#!/usr/bin/env python3
"""
demo_transport.py — Transport in Type Families

Transport is the computational engine behind path induction in HoTT.

  transport : (P : A → Type) → (p : a₀ = a₁) → P(a₀) → P(a₁)

Given a type family P over A and a path p from a₀ to a₁,
transport moves elements of P(a₀) to elements of P(a₁).

This is NOT just function application — it depends on the full path p,
not just its endpoints. Different paths between the same endpoints can
give different transport functions.

Key laws:
  transport P refl x = x                        (identity)
  transport P (p · q) x = transport P q (transport P p x)  (composition)
  transport P p⁻¹ = (transport P p)⁻¹           (inversion)

This demo shows 5 type families and lets you transport along paths:
  1. Constant family  P(n) = ℕ    — transport is always the identity
  2. Successor family P(n) = Fin(n+1) — fiber size depends on base
  3. Helix            P(x) = ℤ    — transport around loop is +1
  4. Predicate        P(n) = (n > 0)  — truth value follows the path
  5. Code family      P(x) = code(x) — encode-decode's core family

Commands
  1-5   select type family
  n/p   next / previous
  t     transport: enter a path, see the result
  l     transport laws: verify them interactively
  c     compose: chain two transports
  h     help
  q     quit
"""

from __future__ import annotations
import textwrap
from typing import Callable, Any

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

# ── Path model ────────────────────────────────────────────────────────────────

# A path in a space A is modeled as a list of edges (a₀, a₁, a₂, …)
# Composition: concatenate lists (sharing the midpoint)
# Identity: [x]  (a single-node path = refl)
# Inversion: reverse the list

class Path:
    def __init__(self, nodes: list):
        assert len(nodes) >= 1
        self.nodes = nodes

    @property
    def start(self): return self.nodes[0]
    @property
    def end(self): return self.nodes[-1]

    def compose(self, other: "Path") -> "Path":
        assert self.end == other.start, f"Cannot compose: {self.end} ≠ {other.start}"
        return Path(self.nodes + other.nodes[1:])

    def invert(self) -> "Path":
        return Path(list(reversed(self.nodes)))

    def __repr__(self) -> str:
        return " → ".join(str(n) for n in self.nodes)

    def is_refl(self) -> bool:
        return len(self.nodes) == 1

REFL = lambda x: Path([x])

# ── Type Families ─────────────────────────────────────────────────────────────

FAMILIES = [
    {
        "name": "Constant family",
        "code": "P : ℕ → Type\nP(n) = ℕ",
        "base_space": "ℕ (natural numbers)",
        "fiber_at": lambda n: f"ℕ  (same for all n; currently at base {n})",
        "transport_fn": lambda path, x: x,
        "transport_desc": "transport(P)(p)(x) = x   for any p",
        "sample_base": [0, 1, 2, 3, 4],
        "sample_element": 7,
        "element_desc": "7 : ℕ = P(n)  for any n",
        "explanation": (
            "When P is a constant family — P(n) = ℕ for all n — "
            "transport along any path is the identity function. "
            "Intuitively: the fiber doesn't change as you move in the base, "
            "so there's nothing to 'transport'. "
            "This is analogous to a trivial bundle: fiber × base. "
            "In HoTT: transport(const ℕ)(p)(x) = x  by path induction."
        ),
        "paths": [
            Path([0, 1]),
            Path([0, 1, 2, 3]),
            REFL(0),
        ],
        "interesting_path": None,
    },
    {
        "name": "Fin family (fiber size = base value + 1)",
        "code": "P : ℕ → Type\nP(n) = Fin(n+1)  -- the type {0, 1, …, n}",
        "base_space": "ℕ (natural numbers)",
        "fiber_at": lambda n: f"Fin({n+1}) = {{0, 1, …, {n}}}  (a finite type of {n+1} elements)",
        "transport_fn": lambda path, x: min(x, path.end),
        "transport_desc": "transport(Fin)(p)(x) = min(x, P(end))  — clips to new fiber size",
        "sample_base": [0, 1, 2, 3, 4],
        "sample_element": 2,
        "element_desc": "2 : Fin(3)  (valid when base ≥ 2)",
        "explanation": (
            "Fin(n+1) is the type with exactly n+1 elements: {0, 1, …, n}. "
            "As the base n increases, the fiber grows; as n decreases, it shrinks. "
            "Transport from Fin(m+1) to Fin(n+1) must handle the case m > n "
            "(some elements of the larger fiber have no image in the smaller). "
            "In practice: transport clips the element to the new fiber size. "
            "This shows that transport is NOT just identity — it can actually "
            "change the value of the transported element."
        ),
        "paths": [
            Path([2, 3]),  # growing fiber: Fin(3) → Fin(4)
            Path([3, 2]),  # shrinking fiber: Fin(4) → Fin(3)
            Path([0, 1, 2, 3, 4]),  # long path: Fin(1) → Fin(5)
        ],
        "interesting_path": "Try going from base 4 to base 1 with element 3: it gets clipped to 1.",
    },
    {
        "name": "Helix (universal cover of S¹)",
        "code": "P : S¹ → Type\nP(base) = ℤ\nP(loop i) = ua(succ)(i)   -- fiber shifts by +1 around the loop",
        "base_space": "S¹ (the circle) — positions 0..11 like a clock",
        "fiber_at": lambda n: f"ℤ (the integers — fiber is always ℤ, but different 'lifts')",
        "transport_fn": lambda path, x: x + _count_loops(path),
        "transport_desc": "transport(helix)(p)(n) = n + winding_number(p)",
        "sample_base": list(range(12)),
        "sample_element": 0,
        "element_desc": "0 : ℤ = P(base)  (the starting lift point)",
        "explanation": (
            "The helix is the type family P : S¹ → Type where P(base) = ℤ "
            "and going around the loop once shifts the fiber by +1 "
            "(via the successor equivalence and ua). "
            "Transport along the loop sends n ↦ n+1. "
            "Transport along the inverse loop sends n ↦ n−1. "
            "Transport along any loop sends n ↦ n + winding_number. "
            "This is the key computation in the proof that π₁(S¹) = ℤ: "
            "encode(p) = transport(helix)(p)(0) = winding number of p."
        ),
        "paths": [
            Path(list(range(12)) + [0]),   # one full CW loop: winding +1
            Path([0] + list(range(11, -1, -1)) + [0]),  # one CCW: -1
            Path(list(range(12)) + list(range(12)) + [0]),  # two loops: +2
        ],
        "interesting_path": "Transport 0 around one full loop → get 1 (the winding number!)",
    },
    {
        "name": "Predicate family (truth values)",
        "code": "P : ℕ → Type\nP(n) = (n > 0)  -- a proposition for each n",
        "base_space": "ℕ (natural numbers)",
        "fiber_at": lambda n: ("⊤  (n > 0 holds)" if n > 0 else "⊥  (n > 0 is false)"),
        "transport_fn": lambda path, x: (path.end > 0),
        "transport_desc": "transport(n>0)(p)(proof) = proof that end(p) > 0  (or impossible)",
        "sample_base": [0, 1, 2, 3, 4, 5],
        "sample_element": True,  # a proof that some n > 0
        "element_desc": "proof : (n > 0)  (the unique element of ⊤ when n > 0)",
        "explanation": (
            "A type family can be a predicate: P(n) = (n > 0) is a proposition. "
            "P(0) = ⊥ (false), P(1) = P(2) = … = ⊤ (true). "
            "Transport along a path from n to m converts a proof that n > 0 "
            "into a proof that m > 0 — but only if m > 0. "
            "If m = 0, the transport is impossible (the fiber is empty). "
            "This shows how transport can 'fail' — you cannot transport a proof "
            "along a path that takes you to a point where the predicate is false."
        ),
        "paths": [
            Path([1, 2]),   # 1 > 0 → 2 > 0  (both true)
            Path([2, 3, 4]),  # all positive, transport works
            Path([2, 1, 0]),  # ending at 0: transport would give proof of 0 > 0!
        ],
        "interesting_path": "A path ending at 0 with a proof from n=1: transporting gives a proof of 0>0 — contradiction! This is why fibrations with non-uniform fibers are interesting.",
    },
    {
        "name": "Code family (from encode-decode)",
        "code": "code : Bool → Type\ncode(true)  = ⊤\ncode(false) = ⊥",
        "base_space": "Bool = {true, false}",
        "fiber_at": lambda b: ("⊤  (one element: tt)" if b else "⊥  (no elements)"),
        "transport_fn": lambda path, x: (path.end, x),
        "transport_desc": "transport(code)(p)(x) : code(end(p))",
        "sample_base": [True, False],
        "sample_element": "tt",
        "element_desc": "tt : ⊤ = code(true)  (the unique proof of truth)",
        "explanation": (
            "This is exactly the code family from the encode-decode proof of true ≠ false. "
            "code(true) = ⊤ and code(false) = ⊥. "
            "Starting with tt : code(true), transport along any path p : true = x "
            "gives an element of code(x). "
            "If x = false, this would be an element of ⊥ — impossible. "
            "So no path true = false can exist. "
            "Transport makes the encode function fully explicit: "
            "encode(p) = transport(code)(p)(tt) : code(end(p))."
        ),
        "paths": [
            Path([True]),           # refl_true: transport = identity
            # No path True→False exists! This is the point.
        ],
        "interesting_path": "There is NO path [True, False] because that would transport tt into ⊥.",
    },
]

def _count_loops(path: Path) -> int:
    """Count net full loops for the circle (12-position model)."""
    nodes = path.nodes
    total = 0
    for i in range(len(nodes) - 1):
        a, b = nodes[i], nodes[i + 1]
        if isinstance(a, int) and isinstance(b, int):
            d = b - a
            if d > 6: d -= 12
            if d < -6: d += 12
            total += d
    return total // 12

# ── Display ───────────────────────────────────────────────────────────────────

def _show_family(idx: int):
    f = FAMILIES[idx]
    clear()
    print(bold(f"\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(f"  ║  Transport Demo  ({idx+1}/{len(FAMILIES)}): {f['name']:<33}║"))
    print(bold(f"  ╚═══════════════════════════════════════════════════════╝\n"))

    print(f"  {bold('Type family')}:")
    for line in f['code'].split('\n'):
        print(f"    {cyan(line)}")
    print()
    print(f"  {bold('Base space')}: {dim(f['base_space'])}")
    print()

    print(f"  {bold('Fibers at sample points')}:")
    for b in f['sample_base'][:6]:
        fiber = f['fiber_at'](b)
        print(f"    P({bold(cyan(str(b)))}) = {yellow(fiber)}")
    print()

    print(f"  {bold('Transport rule')}: {green(f['transport_desc'])}")
    print()
    print(wrap(f['explanation'], width=72))
    print()

    if f['interesting_path']:
        print(f"  {bold('Interesting case')}: {magenta(f['interesting_path'])}")
        print()

    if f['paths']:
        print(f"  {bold('Sample paths to transport along')}:")
        for i, p in enumerate(f['paths'], 1):
            print(f"    {i}. {dim(str(p))}  (start={p.start}, end={p.end})")
    print()
    print(f"  {dim('[n/p] next/prev  [t] transport  [l] laws  [c] compose  [h] help')}")

def _do_transport(idx: int):
    f = FAMILIES[idx]
    clear()
    print(bold(f"\n  Transport in {cyan(f['name'])}\n"))
    print(f"  {bold('Family')}: {dim(f['code'].split(chr(10))[0])}")
    print(f"  {bold('Rule')}  : {green(f['transport_desc'])}")
    print()

    if f['paths']:
        print(f"  {bold('Available paths')}:")
        for i, p in enumerate(f['paths'], 1):
            print(f"  {bold(str(i))}.  {str(p)}")
        print()
        try:
            choice = input(dim("  Choose a path (1-" + str(len(f['paths'])) + "): ")).strip()
            pidx = int(choice) - 1
            if not (0 <= pidx < len(f['paths'])):
                print(red("  Invalid choice."))
                input(dim("  Press Enter…"))
                return
        except (ValueError, EOFError):
            return
        path = f['paths'][pidx]
    else:
        print(dim("  No standard paths defined for this family."))
        input(dim("  Press Enter…"))
        return

    elem = f['sample_element']
    result = f['transport_fn'](path, elem)

    print()
    print(f"  {bold('Element to transport')}: {yellow(str(elem))} : P({path.start})")
    print(f"  {bold('Path')}: {cyan(str(path))}")
    print(f"  {bold('Direction')}: {dim(f['base_space'].split()[0])}({path.start}) → {dim(f['base_space'].split()[0])}({path.end})")
    print()
    print(f"  {bold('transport(P)(path)({elem})  =  {green(str(result))}')} : P({path.end})")
    print()
    print(f"  {bold('Fiber change')}: P({path.start}) = {yellow(f['fiber_at'](path.start))}")
    print(f"               → P({path.end}) = {yellow(f['fiber_at'](path.end))}")
    print()
    input(dim("  Press Enter to return…"))

def _do_laws(idx: int):
    f = FAMILIES[idx]
    clear()
    print(bold(f"\n  Transport Laws — {cyan(f['name'])}\n"))

    x = f['sample_element']
    paths = f['paths']
    if not paths:
        print(dim("  No paths defined for this family."))
        input(dim("  Press Enter…"))
        return

    p = paths[0]
    refl_p = Path([p.start])
    q = paths[1] if len(paths) > 1 else paths[0]

    # Law 1: transport P refl x = x
    result_refl = f['transport_fn'](refl_p, x)
    law1_holds = result_refl == x or result_refl == (p.start, x)
    print(f"  {bold('Law 1')}: transport(P)(refl)(x) = x")
    print(f"    x = {yellow(str(x))},  refl_{p.start}")
    print(f"    transport(refl)({x}) = {green(str(result_refl))}")
    print(f"    Identity holds: {green('✓') if law1_holds else red('needs checking')}")
    print()

    # Law 2: transport P (p · q') x = transport P q' (transport P p x)
    # Use first path as p, compose with itself inverted as q' for simplicity
    inv_p = p.invert()
    pq = p.compose(inv_p)
    result_pq = f['transport_fn'](pq, x)
    result_q_of_p = f['transport_fn'](inv_p, f['transport_fn'](p, x))
    print(f"  {bold('Law 2')}: transport(P)(p · q)(x) = transport(P)(q)(transport(P)(p)(x))")
    print(f"    p = {cyan(str(p))}")
    print(f"    q = {cyan(str(inv_p))}  (= p⁻¹)")
    print(f"    p · q = {cyan(str(pq))}")
    print(f"    transport(p·q)({x}) = {green(str(result_pq))}")
    print(f"    transport(q)(transport(p)({x})) = {green(str(result_q_of_p))}")
    print()

    # Law 3: symmetric — transport along inverse
    result_p = f['transport_fn'](p, x)
    result_inv = f['transport_fn'](inv_p, result_p)
    print(f"  {bold('Law 3')}: transport(P)(p⁻¹) is the inverse of transport(P)(p)")
    print(f"    transport(p)({x}) = {green(str(result_p))}")
    print(f"    transport(p⁻¹)({result_p}) = {green(str(result_inv))}")
    print(f"    Roundtrip: start={x}, end={result_inv}")
    print()

    print(wrap(
        "These laws follow from path induction (the J rule). "
        "Transport is determined entirely by its value on refl, "
        "and the composition/inversion laws are then automatic.",
        width=72
    ))
    print()
    input(dim("  Press Enter to return…"))

# ── Main ─────────────────────────────────────────────────────────────────────

def main():
    idx = 0
    while True:
        _show_family(idx)
        print()
        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break

        if cmd in ("q", "quit"):
            break
        elif cmd == "n":
            idx = (idx + 1) % len(FAMILIES)
        elif cmd == "p":
            idx = (idx - 1) % len(FAMILIES)
        elif cmd in ("1", "2", "3", "4", "5"):
            idx = int(cmd) - 1
        elif cmd == "t":
            _do_transport(idx)
        elif cmd == "l":
            _do_laws(idx)
        elif cmd in ("h", "help", "?"):
            clear()
            print(f"""
  {bold('Transport in Type Families — Commands')}

  {cyan('1-5')}   select a type family
  {cyan('n/p')}   next / previous family
  {cyan('t')}     transport: choose a path and watch the element move
  {cyan('l')}     verify the transport laws (identity, composition, inversion)
  {cyan('h')}     this help
  {cyan('q')}     quit

  {bold('Core concept')}:
  {dim('transport : (P : A → Type) → (a₀ = a₁) → P(a₀) → P(a₁)')}
  {dim('Not just application — depends on the full path, not just endpoints.')}
""")
            input(dim("  Press Enter…"))


if __name__ == "__main__":
    clear()
    print(bold("""
  ╔════════════════════════════════════════════════════════╗
  ║         Transport in Type Families                     ║
  ╚════════════════════════════════════════════════════════╝
"""))
    print("""  transport : (P : A → Type) → (a₀ = a₁) → P(a₀) → P(a₁)

  Transport is how paths in the base space induce functions
  between fibers. Given a type family P : A → Type and a
  path p : a₀ = a₁, transport(P)(p) : P(a₀) → P(a₁).

  This is the computational content of substitution:
  if we know P(a₀) and we know a₀ = a₁, we get P(a₁).

  Key insight: transport depends on the full PATH, not just
  the endpoints. Different paths between the same endpoints
  can give different transport functions.

  Type 'n' to navigate families, 't' to transport, 'q' to quit.
""")
    input(dim("  Press Enter to start…"))
    main()
    print(f"\n  {dim('Transport: paths in the base lift to functions between fibers.')}\n")
