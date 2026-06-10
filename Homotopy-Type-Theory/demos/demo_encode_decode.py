#!/usr/bin/env python3
"""
demo_encode_decode.py — The Encode-Decode Method

The encode-decode method is the central proof technique for computing path
spaces of higher inductive types in HoTT.

Pattern:
  Given a HIT A with basepoint a₀, to compute (a₀ = a₁):
    1. Define a type family  code : A → Type
    2. Define  encode : (a₀ = x) → code(x)   by transport
    3. Define  decode : code(x) → (a₀ = x)   by induction on code
    4. Prove   encode ∘ decode = id  and  decode ∘ encode = id

This demo applies the pattern to three types:
  • Bool    — prove true ≠ false
  • ℕ       — prove 0 ≠ succ n and succ is injective
  • S¹      — compute π₁(S¹) = ℤ  (winding numbers)
  • S²      — explain why the code for S² is S¹ (not ℤ), and what that means

Commands
  1-4   select an example
  n     next example
  p     previous example
  s     step through the encode-decode construction
  t     the pattern (abstract template)
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

def box(title: str, lines: list[str], width: int = 60) -> str:
    inner = width - 2
    top    = "  ┌" + "─" * inner + "┐"
    bottom = "  └" + "─" * inner + "┘"
    ttl    = f"  │ {bold(title):<{inner + 8}} │"
    sep    = "  ├" + "─" * inner + "┤"
    rows   = [f"  │ {l:<{inner - 1}} │" for l in lines]
    return "\n".join([top, ttl, sep] + rows + [bottom])

# ── Examples ──────────────────────────────────────────────────────────────────

EXAMPLES = [
    {
        "name": "Bool — true ≠ false",
        "type": "Bool",
        "type_desc": "data Bool : Type where  true false : Bool",
        "goal": "prove  true ≠ false,  i.e., (true = false) → ⊥",
        "steps": [
            (
                "1. Define code : Bool → Type",
                "code(true)  = ⊤   (the unit type — one element)\n"
                "  code(false) = ⊥   (the empty type — no elements)",
                "The code type captures the 'essence' of each element.\n"
                "  true gets a populated type; false gets an empty type."
            ),
            (
                "2. Define encode : (true = b) → code(b)",
                "encode(refl_true) = tt   (the unique element of ⊤)",
                "Transport refl_true along code gives tt : code(true) = ⊤.\n"
                "  encode(p) = transport(cong code p)(tt)."
            ),
            (
                "3. Observe: encode(p) : code(b)",
                "If b = false, then encode(p) : code(false) = ⊥\n"
                "  But ⊥ has no elements — contradiction!",
                "So no path p : true = false can exist.\n"
                "  The type (true = false) is equivalent to ⊥."
            ),
            (
                "4. Conclusion",
                "(true = false) ≃ ⊥\n"
                "  In particular: true ≠ false  ✓",
                "The encode function would map a proof of true=false\n"
                "  to an element of ⊥ — which doesn't exist."
            ),
        ],
        "program": "not_equal : true ≠ false\nnot_equal p = encode p   -- : ⊥  (absurd!)",
        "punchline": (
            "The code type acts as a 'discriminant': it collapses\n"
            "  to ⊤ for one value and ⊥ for another. A path true = false\n"
            "  would transport the element tt : ⊤ to an element of ⊥,\n"
            "  which is impossible. This is the constructive proof of\n"
            "  decidable equality for Bool."
        ),
        "failure": None,
    },
    {
        "name": "ℕ — zero ≠ succ n, succ is injective",
        "type": "ℕ",
        "type_desc": "data ℕ : Type where  zero : ℕ  |  succ : ℕ → ℕ",
        "goal": "prove  zero ≠ succ n   and   succ m = succ n → m = n",
        "steps": [
            (
                "1. Define code : ℕ → ℕ → Type   (two-variable version)",
                "code(zero,  zero)   = ⊤\n"
                "  code(zero,  succ n) = ⊥\n"
                "  code(succ m, zero)  = ⊥\n"
                "  code(succ m, succ n) = code(m, n)   ← recursive!",
                "code(m, n) = ⊤ iff m = n (as naturals).\n"
                "  The recursive case mirrors the structure of ℕ."
            ),
            (
                "2. Define encode : (m = n) → code(m, n)",
                "encode(refl_m) = tt : code(m, m)   (diagonal = ⊤)",
                "Reflexivity witnesses that m = m, so code(m, m) = ⊤."
            ),
            (
                "3. zero ≠ succ n",
                "A path p : zero = succ n would give\n"
                "  encode(p) : code(zero, succ n) = ⊥.\n"
                "  Impossible — no element of ⊥ exists.",
                "This is the No-Confusion property for ℕ."
            ),
            (
                "4. succ is injective",
                "A path p : succ m = succ n gives\n"
                "  encode(p) : code(succ m, succ n) = code(m, n).\n"
                "  Decode this to get a path m = n.",
                "The decode direction: decode(c : code(m, n)) : m = n\n"
                "  is proved by induction on m and n simultaneously."
            ),
        ],
        "program": (
            "zero-ne-succ : zero ≠ succ n\n"
            "zero-ne-succ p = encode p   -- : ⊥\n\n"
            "succ-inj : succ m = succ n → m = n\n"
            "succ-inj p = decode (encode p)"
        ),
        "punchline": (
            "The two-variable code type captures definitional injectivity\n"
            "  and disjointness of constructors. This is the constructive\n"
            "  proof that ℕ has decidable equality — a fact that in ZFC\n"
            "  is 'obvious' but here requires an explicit construction."
        ),
        "failure": None,
    },
    {
        "name": "S¹ — compute π₁(S¹) = ℤ",
        "type": "S¹",
        "type_desc": "data S¹ : Type where\n  base : S¹\n  loop : base = base",
        "goal": "prove  (base = base) ≃ ℤ   (the winding number isomorphism)",
        "steps": [
            (
                "1. Define code : S¹ → Type   (the 'helix')",
                "code(base) = ℤ\n"
                "  code(loop i) = ua(succEquiv) i\n"
                "  where succEquiv : ℤ ≃ ℤ  is n ↦ n + 1",
                "As you traverse the loop, the fiber ℤ shifts by +1.\n"
                "  This is the universal cover of S¹: the helix ℝ → S¹."
            ),
            (
                "2. Define encode : (base = x) → code(x)",
                "encode(p) = transport(cong code p)(0 : ℤ)",
                "Start at 0 in the fiber over base.\n"
                "  Transport along p lifts p to the helix, ending at an integer.\n"
                "  That integer is the winding number of p."
            ),
            (
                "3. Define decode : code(x) → (base = x)   by S¹-elim",
                "decode_base : ℤ → (base = base)\n"
                "  decode_base(n) = loop^n   (loop composed n times)\n"
                "  For n < 0: loop⁻¹ composed |n| times",
                "By univalence, the action on code(loop i)\n"
                "  is compatible with the successor equivalence."
            ),
            (
                "4. Prove the roundtrips",
                "encode(decode(n)) = n   ✓\n"
                "  decode(encode(p)) = p   ✓   (harder!)",
                "The first is: transporting n steps along loop^n returns n.\n"
                "  The second requires the flattening lemma / contractibility of\n"
                "  the total space of the helix (which is contractible, like ℝ)."
            ),
        ],
        "program": (
            "encode : (base = base) → ℤ\n"
            "encode p = transport (cong helix p) 0\n\n"
            "decode : ℤ → (base = base)\n"
            "decode n = loop ^ n   -- loop composed n times\n\n"
            "π₁S¹≅ℤ : (base = base) ≃ ℤ\n"
            "π₁S¹≅ℤ = encode , decode , roundtrip₁ , roundtrip₂"
        ),
        "punchline": (
            "The helix is the universal cover of S¹. Encode 'lifts' a\n"
            "  loop to the cover, landing at an integer (the winding number).\n"
            "  Decode 'winds' n times to recover the loop.\n"
            "  This gives the group isomorphism π₁(S¹) ≅ (ℤ, +):\n"
            "  loop composition = integer addition."
        ),
        "failure": None,
    },
    {
        "name": "S² — why π₁(S²) = 0 but π₂(S²) = ℤ",
        "type": "S²",
        "type_desc": "data S² : Type where\n  base : S²\n  surf : refl_{base} = refl_{base}",
        "goal": "explain code(S²) and why it is NOT ℤ",
        "steps": [
            (
                "1. The naive approach fails",
                "Try: code(base) = ℤ,  code(surf i j) = ua(succ) i  ...\n"
                "  Problem: surf is a 2-cell (path between paths),\n"
                "  so code must also vary over surf as a 2-cell of types.",
                "For S¹, loop is a 1-path and code needs to vary over it.\n"
                "  For S², surf is a 2-path and code needs to vary over\n"
                "  a square of types. The code type must be a 1-groupoid, not a set."
            ),
            (
                "2. What IS the correct code for π₁(S²)?",
                "code(base) = ⊤   (the trivial type)\n"
                "  Transport around any loop at base lands back at ⊤\n"
                "  (because any loop on S² is contractible).",
                "This gives: (base = base) ≃ ⊤,  so π₁(S²) = 1.  ✓\n"
                "  S² is simply connected — no winding numbers."
            ),
            (
                "3. What is the code for π₂(S²)?",
                "The path space (base = base) in S² is itself S¹!\n"
                "  So the code for π₂ must be: code₂(base) = ℤ\n"
                "  but the encode/decode now involves maps S² → S² ...",
                "This is the Hopf invariant: a 2-loop is classified by\n"
                "  an integer, but the proof goes through the Hopf fibration\n"
                "  S¹ → S³ → S², not a simple code-type argument."
            ),
            (
                "4. The lesson: higher spheres require higher codes",
                "S¹: code is ℤ (a set, h-level 0)  — π₁ = ℤ\n"
                "  S²: code for π₁ is ⊤, for π₂ needs S¹ (a groupoid!)\n"
                "  S³: code for π₃ involves S² (a 2-groupoid)  ...\n"
                "  Pattern: code for πₙ(Sⁿ) involves Sⁿ⁻¹ recursively.",
                "This is why higher homotopy groups of spheres are hard:\n"
                "  the encode-decode method requires codes at each dimension,\n"
                "  and those codes are themselves complex types."
            ),
        ],
        "program": (
            "-- Simple connectivity:\n"
            "π₁S²=1 : (base =_{S²} base) ≃ ⊤\n"
            "π₁S²=1 = ... (encode-decode with code = const ⊤)\n\n"
            "-- Brunerie's π₄(S³) = ℤ/2ℤ required a 100-page proof\n"
            "-- because the code type is not elementary."
        ),
        "punchline": (
            "The encode-decode method is powerful but not magic.\n"
            "  Its difficulty scales with the dimension of the sphere:\n"
            "  π₁(S¹) = ℤ is a one-page proof;\n"
            "  π₄(S³) = ℤ/2ℤ (Brunerie, 2016) was 100 pages.\n"
            "  The method is the same — the code types get harder."
        ),
        "failure": (
            "There is no 'simple' code type for π₂(S²).\n"
            "  The Hopf fibration is needed. See demo_hopf.py."
        ),
    },
]

# ── Abstract template ─────────────────────────────────────────────────────────

PATTERN = """
  ╔══════════════════════════════════════════════════════════════╗
  ║              The Encode-Decode Template                      ║
  ╠══════════════════════════════════════════════════════════════╣
  ║                                                              ║
  ║  Goal: characterise the path space  (a₀ = x)  for a HIT A.  ║
  ║                                                              ║
  ║  Step 1.  code : A → Type                                    ║
  ║    Define the 'code type' — what a path should encode as.    ║
  ║    Must satisfy: code(a₀) = the expected path space.         ║
  ║                                                              ║
  ║  Step 2.  encode : (a₀ = x) → code(x)                       ║
  ║    encode(p) = transport(cong code p)(r)                     ║
  ║    where r : code(a₀) is the 'initial code' (e.g. 0 : ℤ).   ║
  ║                                                              ║
  ║  Step 3.  decode : code(x) → (a₀ = x)                       ║
  ║    Defined by induction on the HIT (S¹-elim, ℕ-elim, etc.). ║
  ║                                                              ║
  ║  Step 4.  Roundtrips                                         ║
  ║    (i)  encode(decode(c)) = c    for all c : code(x)         ║
  ║    (ii) decode(encode(p)) = p    for all p : a₀ = x          ║
  ║                                                              ║
  ║  Result:  (a₀ = x) ≃ code(x)                                ║
  ║                                                              ║
  ╚══════════════════════════════════════════════════════════════╝
"""

# ── Display ───────────────────────────────────────────────────────────────────

def _show_example(idx: int, step: int = -1):
    ex = EXAMPLES[idx]
    clear()
    print(bold(f"\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(f"  ║  Encode-Decode: {cyan(ex['name']):<40}║"))
    print(bold(f"  ╚═══════════════════════════════════════════════════════╝\n"))

    print(f"  {bold('Type')}: {dim(ex['type_desc'])}")
    print(f"  {bold('Goal')}: {yellow(ex['goal'])}")
    print()

    if step < 0:
        # Show all steps summary
        print(f"  {bold('Construction overview')} ({len(ex['steps'])} steps):\n")
        for i, (title, _, _) in enumerate(ex['steps'], 1):
            print(f"  {bold(cyan(str(i)))}. {title}")
        print()
        if ex['failure']:
            print(f"  {bold(red('Note'))}:")
            for line in ex['failure'].split('\n'):
                print(f"    {dim(line)}")
            print()
        print(f"  {dim('[s] step through   [1-4] jump to step   [n/p] prev/next example')}")
    else:
        # Show specific step
        title, code_text, explanation = ex['steps'][step]
        print(f"  {bold(f'Step {step+1}')} of {len(ex['steps'])}: {bold(yellow(title))}\n")
        print(f"  {bold('Construction')}:")
        for line in code_text.split('\n'):
            print(f"    {cyan(line)}")
        print()
        print(f"  {bold('Explanation')}:")
        for line in explanation.split('\n'):
            print(f"  {dim(line)}")
        print()
        if step == len(ex['steps']) - 1:
            print(f"  {bold('Resulting program')}:")
            for line in ex['program'].split('\n'):
                print(f"    {green(line)}")
            print()
            print(f"  {bold('Why this works')}:")
            for line in ex['punchline'].split('\n'):
                print(f"  {line}")
        print()
        nav = []
        if step > 0: nav.append("[b] back")
        if step < len(ex['steps']) - 1: nav.append("[f] forward")
        nav.append("[r] restart")
        nav.append("[n/p] next/prev example")
        print(f"  {dim('  '.join(nav))}")

def _step_through(idx: int):
    step = 0
    while True:
        _show_example(idx, step)
        print()
        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break
        if cmd in ("f", "", "next"):
            if step < len(EXAMPLES[idx]['steps']) - 1:
                step += 1
        elif cmd == "b":
            if step > 0:
                step -= 1
        elif cmd == "r":
            step = 0
        elif cmd in ("n",):
            break
        elif cmd == "q":
            return "quit"

# ── Main ─────────────────────────────────────────────────────────────────────

def main():
    idx = 0
    while True:
        _show_example(idx)
        print()
        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break

        if cmd in ("q", "quit", "exit"):
            break
        elif cmd == "n":
            idx = (idx + 1) % len(EXAMPLES)
        elif cmd == "p":
            idx = (idx - 1) % len(EXAMPLES)
        elif cmd in ("1", "2", "3", "4"):
            idx = int(cmd) - 1
        elif cmd == "s":
            result = _step_through(idx)
            if result == "quit":
                break
        elif cmd == "t":
            clear()
            print(PATTERN)
            input(dim("  Press Enter to return…"))
        elif cmd in ("h", "help", "?"):
            clear()
            print(f"""
  {bold('Encode-Decode Method — Commands')}

  {cyan('1-4')}   jump to example 1-4
  {cyan('n/p')}   next / previous example
  {cyan('s')}     step through the construction interactively
  {cyan('t')}     show the abstract template
  {cyan('h')}     this help
  {cyan('q')}     quit
""")
            input(dim("  Press Enter…"))


if __name__ == "__main__":
    clear()
    print(bold("""
  ╔════════════════════════════════════════════════════════╗
  ║       The Encode-Decode Method in HoTT                 ║
  ╚════════════════════════════════════════════════════════╝
"""))
    print("""  To compute the path space  (a₀ = x)  of a HIT A:

    code  : A → Type          the 'fiber' over each point
    encode: (a₀ = x) → code(x)   lift a path to its code
    decode: code(x) → (a₀ = x)   reconstruct a path from its code

  The roundtrip laws then give:  (a₀ = x) ≃ code(x)

  Applied to:
    Bool  → prove true ≠ false
    ℕ     → prove 0 ≠ succ n and succ is injective
    S¹    → compute π₁(S¹) = ℤ  (the winding number)
    S²    → why π₁(S²) = 0 but π₂(S²) = ℤ is harder

  Type 'n' to navigate, 's' to step through, 'q' to quit.
""")
    input(dim("  Press Enter to start…"))
    main()
    print(f"\n  {dim('Encode-decode: the proof technique of synthetic homotopy theory.')}\n")
