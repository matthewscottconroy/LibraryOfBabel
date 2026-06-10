#!/usr/bin/env python3
"""
demo_suspension.py — The Suspension Functor and Iterated Spheres

Every sphere Sⁿ is built by iterating a single construction: suspension.

  S⁰ = {north, south}   (two points, no paths between them)
  S¹ = Σ(S⁰)            (add a north, south, and a meridian for each point of S⁰)
  S² = Σ(S¹)            (add a north, south, and a meridian for each point of S¹)
  Sⁿ⁺¹ = Σ(Sⁿ)          (in general)

The suspension ΣA is defined as the HIT:
  data ΣA : Type where
    north : ΣA
    south : ΣA
    merid : A → north = south

Each point a : A gives a path (meridian) from north to south.

The Freudenthal Suspension Theorem says (in the stable range):
  πₖ(ΣA) ≅ πₖ₋₁(A)   for k ≤ 2·conn(A)

So homotopy groups SHIFT UP by 1 under suspension (in the stable range).
This is why πₙ(Sⁿ) = ℤ for all n: it's just π₁(S¹) = ℤ suspended (n-1) times.

Commands
  0-4   navigate from S⁰ to S⁴
  n/p   next / previous sphere
  m     show meridians of the current sphere
  f     Freudenthal: see the stable range and group shifts
  g     known homotopy groups table
  e     explain the HIT construction
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
def blue(t):    return _c("34", t)

def clear(): print("\033[2J\033[H", end="")

def wrap(s, width=70, indent=2):
    prefix = " " * indent
    return textwrap.fill(s, width=width, initial_indent=prefix,
                         subsequent_indent=prefix)

# ── Sphere data ───────────────────────────────────────────────────────────────

SPHERES = [
    {
        "n": 0,
        "name": "S⁰",
        "full_name": "S⁰  (the 0-sphere)",
        "description": "Two isolated points: north and south. No paths between them.",
        "hit_def": "data S⁰ : Type where\n  north : S⁰\n  south : S⁰",
        "points": ["north", "south"],
        "meridians": [],
        "meridian_desc": "None — S⁰ has no paths between its two points.",
        "connectivity": -1,
        "pi_groups": {
            0: "ℤ/2ℤ  (two components: {north}, {south})",
            1: "0",
            2: "0",
            3: "0",
            4: "0",
        },
        "ascii": """
      ●  north

      ●  south
""",
        "construction": "S⁰ is the base case. It has two points and no paths.",
        "freudenthal_stability": "S⁰ is (−1)-connected, so Freudenthal applies for k ≤ 2·(−1)+2 = 0.",
    },
    {
        "n": 1,
        "name": "S¹",
        "full_name": "S¹  (the circle)",
        "description": "Suspension of S⁰. Two meridians (one for each point of S⁰) form a circle.",
        "hit_def": ("data S¹ : Type where\n"
                    "  base : S¹\n"
                    "  loop : base = base\n\n"
                    "-- As suspension of S⁰:\n"
                    "data ΣS⁰ : Type where\n"
                    "  north : ΣS⁰\n"
                    "  south : ΣS⁰\n"
                    "  merid : S⁰ → north = south\n"
                    "-- merid(north-S⁰) and merid(south-S⁰) are the two meridians"),
        "points": ["north = base", "south"],
        "meridians": ["merid(north-of-S⁰)", "merid(south-of-S⁰)"],
        "meridian_desc": ("S⁰ has 2 points, so ΣS⁰ has 2 meridians from north to south.\n"
                          "  Going 1st meridian then (2nd)⁻¹ gives the loop at north = the base loop of S¹."),
        "connectivity": 0,
        "pi_groups": {
            0: "0  (S¹ is connected)",
            1: "ℤ  (winding numbers — the main result!)",
            2: "0",
            3: "0",
            4: "0",
        },
        "ascii": """
         N (north/base)
        /|\\
       / | \\
      /  |  \\
     S (south)
    Two meridians → one circle
""",
        "construction": "ΣS⁰ = S¹: two meridians from north to south, their union forms a circle.",
        "freudenthal_stability": "S¹ is 0-connected. Freudenthal: πₖ(ΣS¹) ≅ πₖ₋₁(S¹) for k ≤ 2.",
    },
    {
        "n": 2,
        "name": "S²",
        "full_name": "S²  (the 2-sphere)",
        "description": "Suspension of S¹. One meridian for each point of S¹ (uncountably many) form a 2-sphere.",
        "hit_def": ("data S² : Type where\n"
                    "  base : S²\n"
                    "  surf : refl_{base} = refl_{base}\n\n"
                    "-- As suspension of S¹:\n"
                    "data ΣS¹ : Type where\n"
                    "  north : ΣS¹\n"
                    "  south : ΣS¹\n"
                    "  merid : S¹ → north = south\n"
                    "-- One meridian for each point of S¹ (a whole circle's worth)"),
        "points": ["north", "south"],
        "meridians": ["merid(x)  for each x : S¹", "(uncountably many, parametrized by S¹)"],
        "meridian_desc": ("S¹ has uncountably many points, so ΣS¹ has a meridian for each.\n"
                          "  The 'equator' is the image of S¹ under merid.\n"
                          "  A loop in S¹ becomes a 2-cell in S²."),
        "connectivity": 1,
        "pi_groups": {
            0: "0  (S² is connected)",
            1: "0  (S² is simply connected — no winding numbers)",
            2: "ℤ  (Hopf invariant — a 2-cell wrapping the sphere)",
            3: "ℤ  (the Hopf fibration! π₃(S²) = ℤ — surprising)",
            4: "ℤ/2ℤ  (Brunerie's theorem, 2016 — 100-page HoTT proof)",
        },
        "ascii": """
           N (north)
          /|\\
         / | \\
        /  |  \\
       ●   |   ●
        \\  |  /
         \\ | /
          \\|/
           S (south)
    ∞-many meridians parametrized by S¹
""",
        "construction": "ΣS¹ = S²: for each point of S¹, a meridian. Together they trace out the 2-sphere.",
        "freudenthal_stability": "S² is 1-connected. Freudenthal: πₖ(ΣS²) ≅ πₖ₋₁(S²) for k ≤ 3.",
    },
    {
        "n": 3,
        "name": "S³",
        "full_name": "S³  (the 3-sphere)",
        "description": "Suspension of S². Appears in the Hopf fibration S¹ → S³ → S².",
        "hit_def": ("data ΣS² : Type where\n"
                    "  north : ΣS²\n"
                    "  south : ΣS²\n"
                    "  merid : S² → north = south\n"
                    "-- One meridian for each point of S²"),
        "points": ["north", "south"],
        "meridians": ["merid(x)  for each x : S²", "(parametrized by S² — a sphere's worth)"],
        "meridian_desc": ("S² parametrizes the meridians of S³.\n"
                          "  S³ appears in the Hopf fibration: S¹ → S³ → S².\n"
                          "  The Brunerie number lives in π₄(S³)."),
        "connectivity": 2,
        "pi_groups": {
            0: "0",
            1: "0",
            2: "0",
            3: "ℤ  (every map S³ → S³ has a degree)",
            4: "ℤ/2ℤ  (Brunerie's number! Proved in HoTT 2016)",
            5: "ℤ/2ℤ",
        },
        "ascii": """
      S³ = ΣS²
      (too high-dimensional to draw,
       but think: every point of S²
       gives a path from north to south)

      The Hopf bundle: S¹ → S³ → S²
      Every point of S² has an S¹ above it.
""",
        "construction": "ΣS² = S³: for each point of S², a meridian arc. This builds the 3-sphere.",
        "freudenthal_stability": "S³ is 2-connected. Freudenthal: πₖ(ΣS³) ≅ πₖ₋₁(S³) for k ≤ 5.",
    },
    {
        "n": 4,
        "name": "S⁴",
        "full_name": "S⁴  (the 4-sphere)",
        "description": "Suspension of S³. The stable range πₙ(Sⁿ) = ℤ reaches here via Freudenthal.",
        "hit_def": ("data ΣS³ : Type where\n"
                    "  north : ΣS³\n"
                    "  south : ΣS³\n"
                    "  merid : S³ → north = south"),
        "points": ["north", "south"],
        "meridians": ["merid(x)  for each x : S³"],
        "meridian_desc": "S³ parametrizes the meridians of S⁴.",
        "connectivity": 3,
        "pi_groups": {
            0: "0",
            1: "0",
            2: "0",
            3: "0",
            4: "ℤ  (Freudenthal: π₄(S⁴) ≅ π₃(S³) = ℤ  ✓)",
            5: "ℤ/2ℤ",
            6: "ℤ/2ℤ",
        },
        "ascii": """
      S⁴ = ΣS³

      Freudenthal: π₄(S⁴) ≅ π₃(S³) ≅ ℤ  ✓

      The pattern πₙ(Sⁿ) = ℤ for all n ≥ 1
      follows by applying Freudenthal repeatedly,
      starting from π₁(S¹) = ℤ.
""",
        "construction": "ΣS³ = S⁴. Freudenthal confirms π₄(S⁴) = ℤ, matching the general pattern.",
        "freudenthal_stability": "S⁴ is 3-connected. The stable range begins: for k ≤ 2n−1, πₖ(Sⁿ) is stable.",
    },
]

FREUDENTHAL_EXPLANATION = """
  ╔══════════════════════════════════════════════════════════════╗
  ║         The Freudenthal Suspension Theorem                   ║
  ╠══════════════════════════════════════════════════════════════╣
  ║                                                              ║
  ║  If A is n-connected (πₖ(A) = 0 for all k ≤ n), then:      ║
  ║                                                              ║
  ║    the suspension map  E : πₖ(A) → πₖ₊₁(ΣA)               ║
  ║    is an isomorphism for k ≤ 2n                              ║
  ║    and a surjection for k = 2n+1                             ║
  ║                                                              ║
  ║  Consequence: πₙ(Sⁿ) = ℤ for all n ≥ 1                     ║
  ║    • π₁(S¹) = ℤ  (proved directly by encode-decode)         ║
  ║    • π₂(S²) = ℤ  (Freudenthal from π₁(S¹), k=1 ≤ 2·0=0?)  ║
  ║      → Actually: S¹ is 0-connected, stable for k ≤ 0.       ║
  ║      Direct proof needed for π₂(S²)—but Freudenthal works.  ║
  ║    • π₃(S³) = ℤ  (from π₂(S²) = ℤ by Freudenthal)          ║
  ║    • π₄(S⁴) = ℤ  (from π₃(S³) = ℤ by Freudenthal)          ║
  ║    • πₙ(Sⁿ) = ℤ  for all n ≥ 1  (by induction)             ║
  ║                                                              ║
  ║  In HoTT:                                                    ║
  ║    Freudenthal was formalized in Cubical Agda.               ║
  ║    πₙ(Sⁿ) = ℤ for all n is still being formalized (2024).  ║
  ║    The base case (n=1) is done; the inductive step needs     ║
  ║    careful management of connectivity conditions.            ║
  ║                                                              ║
  ╚══════════════════════════════════════════════════════════════╝
"""

PI_TABLE = """
  Homotopy groups πₖ(Sⁿ):  rows = k, columns = n
  (? = unknown or difficult; stable range below the diagonal)

       S⁰     S¹     S²     S³     S⁴     S⁵
  π₀  ℤ/2    0      0      0      0      0
  π₁  0      ℤ      0      0      0      0
  π₂  0      0      ℤ      0      0      0
  π₃  0      0      ℤ      ℤ      0      0      ← Hopf!
  π₄  0      0    ℤ/2   ℤ/2     ℤ      0
  π₅  0      0    ℤ/2   ℤ/2   ℤ/2     ℤ
  π₆  0      0   ℤ/12   ℤ/12  ℤ/2   ℤ/2

  Key:
    πₙ(Sⁿ) = ℤ  (the diagonal — proved by Freudenthal from π₁(S¹)=ℤ)
    π₃(S²) = ℤ  (the Hopf fibration — see demo_hopf.py)
    π₄(S³) = ℤ/2ℤ  (Brunerie's theorem — 100-page HoTT proof!)

  In classical algebraic topology, these groups are computable
  but grow increasingly complex. In HoTT, computing them requires
  synthetic proofs that are significant research results.
"""

# ── Display ───────────────────────────────────────────────────────────────────

def _show_sphere(idx: int):
    s = SPHERES[idx]
    clear()
    print(bold(f"\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(f"  ║  Suspension Functor: {cyan(s['full_name']):<36}║"))
    print(bold(f"  ╚═══════════════════════════════════════════════════════╝\n"))

    print(f"  {bold('Description')}: {dim(s['description'])}")
    print()

    # ASCII art
    for line in s['ascii'].split('\n'):
        print(f"  {cyan(line)}")

    print(f"  {bold('HIT definition')}:")
    for line in s['hit_def'].split('\n'):
        print(f"    {dim(line)}")
    print()

    print(f"  {bold('Connectivity')}: {yellow(str(s['connectivity']))}-connected")
    print()

    print(f"  {bold('Key homotopy groups')}:")
    for k, v in sorted(s['pi_groups'].items()):
        if k <= s['n'] + 2:
            marker = cyan("← main") if (k == s['n'] and k > 0) else ""
            print(f"    π_{k}({s['name']}) = {green(v)}  {marker}")
    print()

    print(f"  {dim(s['freudenthal_stability'])}")
    print()
    print(f"  {dim('[n/p] next/prev sphere  [0-4] jump  [m] meridians  [f] Freudenthal  [g] table  [h] help')}")

def _show_meridians(idx: int):
    s = SPHERES[idx]
    clear()
    print(bold(f"\n  Meridians of {cyan(s['name'])}\n"))
    print(f"  {bold('Definition')}: merid : {s['name'].replace('Σ','')[0] if idx > 0 else 'S⁰'} → north = south")
    print()
    for line in s['meridian_desc'].split('\n'):
        print(f"  {line}")
    print()
    print(f"  {bold('Why this gives {s[\"name\"]}:')}")
    print(f"  {wrap(s['construction'], indent=2)}")
    print()
    if idx == 1:
        print(f"  {bold('Geometric intuition for S¹ = ΣS⁰')}:")
        print(f"  {dim('  Take two points (N and S of S⁰).')}")
        print(f"  {dim('  Connect each to a new north and south:')}")
        print(f"  {dim('    merid(north-of-S⁰) : N(ΣS⁰) = S(ΣS⁰)')}")
        print(f"  {dim('    merid(south-of-S⁰) : N(ΣS⁰) = S(ΣS⁰)')}")
        print(f"  {dim('  Together they form a closed loop ≅ S¹.  ✓')}")
    elif idx == 2:
        print(f"  {bold('Geometric intuition for S² = ΣS¹')}:")
        print(f"  {dim('  For each point x on the equator circle S¹,')}")
        print(f"  {dim('  draw a path from north pole to south pole.')}")
        print(f"  {dim('  These meridians tile the surface of a 2-sphere.')}")
        print(f"  {dim('  A loop in S¹ (at equator) becomes a 2-cell in S².  ✓')}")
    print()
    input(dim("  Press Enter to return…"))

# ── Main ─────────────────────────────────────────────────────────────────────

def main():
    idx = 1  # start at S¹
    while True:
        _show_sphere(idx)
        print()
        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break

        if cmd in ("q", "quit"):
            break
        elif cmd == "n":
            idx = min(idx + 1, len(SPHERES) - 1)
        elif cmd == "p":
            idx = max(idx - 1, 0)
        elif cmd in ("0", "1", "2", "3", "4"):
            idx = int(cmd)
        elif cmd == "m":
            _show_meridians(idx)
        elif cmd == "f":
            clear()
            print(FREUDENTHAL_EXPLANATION)
            input(dim("  Press Enter to return…"))
        elif cmd == "g":
            clear()
            print(bold("\n  Homotopy Groups of Spheres (known values)\n"))
            print(PI_TABLE)
            input(dim("  Press Enter to return…"))
        elif cmd == "e":
            clear()
            s = SPHERES[idx]
            print(bold(f"\n  Construction: {s['full_name']}\n"))
            print(wrap(
                f"The suspension ΣA of a type A is the HIT with two points "
                f"(north and south) and a path (meridian) from north to south "
                f"for each point of A. "
                f"For {s['name']}, the previous sphere {SPHERES[max(0,idx-1)]['name']} "
                f"is being suspended. "
                f"Each point of {SPHERES[max(0,idx-1)]['name']} contributes one meridian. "
                f"The resulting space is exactly {s['name']}.",
                width=72
            ))
            print()
            print(f"  {bold('HIT definition')}:")
            for line in s['hit_def'].split('\n'):
                print(f"    {cyan(line)}")
            print()
            input(dim("  Press Enter to return…"))
        elif cmd in ("h", "help", "?"):
            clear()
            print(f"""
  {bold('Suspension Functor — Commands')}

  {cyan('0-4')}   jump to S⁰, S¹, S², S³, S⁴
  {cyan('n/p')}   next / previous sphere
  {cyan('m')}     meridians: how each sphere is built from the previous
  {cyan('f')}     Freudenthal suspension theorem
  {cyan('g')}     table of homotopy groups πₖ(Sⁿ)
  {cyan('e')}     explain the HIT construction
  {cyan('h')}     this help
  {cyan('q')}     quit

  {bold('Key idea')}: {dim('Sⁿ = Σ(Sⁿ⁻¹) = Σ²(Sⁿ⁻²) = … = Σⁿ(S⁰)')}
  {bold('Key theorem')}: {dim('πₙ(Sⁿ) = ℤ for all n ≥ 1  (Freudenthal from π₁(S¹)=ℤ)')}
""")
            input(dim("  Press Enter…"))


if __name__ == "__main__":
    clear()
    print(bold("""
  ╔════════════════════════════════════════════════════════╗
  ║     The Suspension Functor: Σ(Sⁿ) = Sⁿ⁺¹             ║
  ╚════════════════════════════════════════════════════════╝
"""))
    print("""  Every sphere is built by suspending the previous one:

    S⁰ = {north, south}   (two points)
    S¹ = ΣS⁰  = circle    (two meridians → one loop)
    S² = ΣS¹  = 2-sphere  (circle-worth of meridians)
    S³ = ΣS²  = 3-sphere  (appears in Hopf fibration)
    Sⁿ = ΣSⁿ⁻¹ in general

  The Freudenthal Suspension Theorem tells us that
  suspension is an isomorphism on homotopy groups in a
  stable range — explaining why πₙ(Sⁿ) = ℤ for all n.

  Type '0'-'4' to navigate, 'f' for Freudenthal, 'q' to quit.
""")
    input(dim("  Press Enter to start…"))
    main()
    print(f"\n  {dim('Sⁿ⁺¹ = ΣSⁿ: all spheres from one construction, iterated.')}\n")
