#!/usr/bin/env python3
"""
demo_groupoid_laws.py — The Groupoid Laws as Higher Paths

In HoTT, every type is an ∞-groupoid. Path composition is NOT strictly
associative — it is associative only up to a higher path (a 2-cell).

The groupoid laws:
  • Left unit:    refl · p = p       (up to a path lu(p) : refl · p = p)
  • Right unit:   p · refl = p       (up to a path ru(p) : p · refl = p)
  • Associativity: (p · q) · r = p · (q · r)    (up to assoc(p,q,r))
  • Inverse left:  p⁻¹ · p = refl    (up to a path)
  • Inverse right: p · p⁻¹ = refl    (up to a path)

These 'up to' witnesses are themselves paths between paths — 2-cells.
This is what makes types ∞-groupoids rather than ordinary groups:
the laws hold only up to coherent higher structure.

The Eckmann-Hilton argument shows that π₂ (loops of loops) is ABELIAN:
  p · q = q · p  for 2-loops p, q at the same basepoint.
This does NOT hold for π₁ in general (e.g., π₁(S¹ ∨ S¹) is free = non-abelian).

Commands
  1   left unit law and its witness 2-cell
  2   right unit law and its witness 2-cell
  3   associativity and the associator square
  4   inverse laws
  5   Eckmann-Hilton: why π₂ is abelian
  c   compose paths interactively
  p   the pentagon coherence (highest level)
  h   help
  q   quit
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

# ── Path model ────────────────────────────────────────────────────────────────

class Path:
    """A path represented as a symbolic expression and a list of nodes."""
    def __init__(self, label: str, start: str, end: str):
        self.label = label
        self.start = start
        self.end   = end

    def compose(self, other: "Path") -> "Path":
        assert self.end == other.start, f"Can't compose: {self.end} ≠ {other.start}"
        if self.label == "refl":
            # refl · q = q  (definitionally? No! Only up to a 2-cell.)
            return Path(f"(refl · {other.label})", self.start, other.end)
        if other.label == "refl":
            return Path(f"({self.label} · refl)", self.start, other.end)
        return Path(f"({self.label} · {other.label})", self.start, other.end)

    def invert(self) -> "Path":
        return Path(f"{self.label}⁻¹", self.end, self.start)

    def __repr__(self) -> str:
        return f"{self.label} : {self.start} = {self.end}"

REFL = lambda x: Path("refl", x, x)

# ── Drawing squares ───────────────────────────────────────────────────────────

def _square(top: str, right: str, bottom: str, left: str,
            tl: str = "a", tr: str = "b", bl: str = "c", br: str = "d") -> str:
    """Draw a square with labeled edges and corners."""
    w = 40
    lines = []
    lines.append(f"  {tl} {'─' * max(1, (w - len(top))//2)}{bold(cyan(top))}{'─' * max(1, (w - len(top) + 1)//2)} {tr}")
    lines.append(f"  {'│':<{w//2+2}}{bold(cyan(right)):>{w//2}}")
    for _ in range(2):
        lines.append(f"  {bold(yellow(left)):<{w//2+10}}  {bold(yellow(right)):<{w//2+5}}")
    lines.append(f"  {'│':<{w//2+2}}{'│':>{w//2+1}}")
    lines.append(f"  {bl} {'─' * max(1, (w - len(bottom))//2)}{bold(cyan(bottom))}{'─' * max(1, (w - len(bottom) + 1)//2)} {br}")
    return "\n".join(lines)

# ── Sections ──────────────────────────────────────────────────────────────────

def _left_unit():
    clear()
    print(bold("\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(  "  ║  Left Unit Law: refl · p = p                         ║"))
    print(bold(  "  ╚═══════════════════════════════════════════════════════╝\n"))

    p = Path("p", "a", "b")
    refl_a = REFL("a")
    comp = refl_a.compose(p)

    print(f"  {bold('Paths')}:")
    print(f"    refl : {cyan('a = a')}")
    print(f"    p    : {cyan('a = b')}")
    print(f"    refl · p  =  {yellow(comp.label)} : a = b")
    print()
    print(wrap(
        "The term 'refl · p' is NOT definitionally equal to 'p' in MLTT "
        "(path concatenation is defined by induction, and 'refl · p' reduces "
        "to 'p' definitionally in some formulations, but not all). "
        "In the HoTT Book formulation, there is a PATH lu(p) : refl · p = p "
        "that is itself a 2-cell — a path between paths.", width=72
    ))
    print()

    print(f"  {bold('The witness 2-cell')} lu(p) : (refl · p) = p")
    print()
    print(f"  {'a':>6}  ──── refl ────  {'a':>6}")
    print(f"  {'|':>6}                 {'|':>6}")
    print(f"  {'p':>4}  {dim('refl·p = p?')}  {'p':>4}")
    print(f"  {'|':>6}                 {'|':>6}")
    print(f"  {'b':>6}  ────────────  {'b':>6}")
    print()
    print(wrap(
        "lu(p) is the left unit 2-cell: a path from 'refl · p' to 'p'. "
        "It is constructed by path induction on p: when p = refl, "
        "lu(refl) = refl (the trivial 2-cell). "
        "The existence of lu shows types form a 'weak' groupoid, not a strict one.", width=72
    ))
    print()
    print(f"  {bold('Proof sketch')}:")
    print(f"    lu : (p : a = b) → refl · p = p")
    print(f"    lu refl = refl   -- base case by path induction")
    print(f"    -- (J rule: reduce to the case p = refl, then check trivially)")
    print()
    input(dim("  Press Enter to return…"))

def _right_unit():
    clear()
    print(bold("\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(  "  ║  Right Unit Law: p · refl = p                        ║"))
    print(bold(  "  ╚═══════════════════════════════════════════════════════╝\n"))

    print(wrap(
        "The right unit law p · refl = p requires a witness 2-cell ru(p). "
        "Unlike the left unit, this does NOT hold definitionally in standard MLTT. "
        "When path concatenation p · q is defined by induction on p "
        "(the left argument), then: "
        "  refl · q = q   holds definitionally (left-induction makes it compute), "
        "  p · refl = p   does NOT — it only holds propositionally (up to a path).",
        width=72
    ))
    print()

    print(f"  {bold('Computation')}: try p · refl for a symbolic path p")
    print()
    print(f"    p : a = b")
    print(f"    p · refl = {yellow('p · refl')}   -- does NOT reduce to p!")
    print()
    print(f"    ru(p) : p · refl = p    -- the 2-cell witness")
    print()
    print(wrap(
        "The witness ru is proved by path induction on p: "
        "when p = refl, ru(refl) : refl · refl = refl — which holds since "
        "both sides reduce to refl. The full ru follows by J.", width=72
    ))
    print()

    print(f"  {bold('This asymmetry is fundamental')}:")
    print(wrap(
        "Left and right units behave differently definitionally, "
        "depending on which argument path concatenation recurses on. "
        "In Cubical Type Theory (CCHM), the De Morgan structure makes "
        "both lu and ru hold definitionally — one advantage of cubical over book HoTT.",
        width=72
    ))
    print()
    print(f"  {bold('Proof')}:")
    print(f"    ru : (p : a = b) → p · refl = p")
    print(f"    ru refl = refl   -- by path induction (J rule)")
    print()
    input(dim("  Press Enter to return…"))

def _associativity():
    clear()
    print(bold("\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(  "  ║  Associativity: (p · q) · r = p · (q · r)           ║"))
    print(bold(  "  ╚═══════════════════════════════════════════════════════╝\n"))

    print(f"  {bold('Three paths')}:")
    print(f"    p : a = b")
    print(f"    q : b = c")
    print(f"    r : c = d")
    print()
    print(f"  {bold('Two bracketings')}:")
    print(f"    Left:  {yellow('(p · q) · r')} : a = d")
    print(f"    Right: {yellow('p · (q · r)')} : a = d")
    print()
    print(wrap(
        "These have the same type (a = d) but are NOT definitionally equal. "
        "There is a 2-cell assoc(p,q,r) : (p · q) · r = p · (q · r). "
        "This 2-cell is an element of the identity type between two paths.",
        width=72
    ))
    print()

    print(f"  {bold('The associator as a square')}:")
    print()
    print(f"        a ──── p ────► b")
    print(f"        │               │")
    print(f"     (p·q)·r       p·(q·r)")
    print(f"        │               │")
    print(f"        ▼               ▼")
    print(f"        d ◄──── ... ──── d")
    print()
    print(f"  {dim('The interior of the square is filled by assoc(p,q,r) — a 2-cell.')}")
    print()
    print(f"  {bold('Proof')}:")
    print(f"    assoc : (p : a = b) → (q : b = c) → (r : c = d)")
    print(f"          → (p · q) · r = p · (q · r)")
    print(f"    assoc refl q r = refl   -- left induction: refl simplifies both sides")
    print()
    print(wrap(
        "By induction on p: when p = refl, both (refl · q) · r "
        "and refl · (q · r) compute to q · r (using lu definitionally). "
        "So assoc(refl, q, r) = refl. The full assoc follows by J.",
        width=72
    ))
    print()
    input(dim("  Press Enter to return…"))

def _inverse_laws():
    clear()
    print(bold("\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(  "  ║  Inverse Laws: p⁻¹ · p = refl  and  p · p⁻¹ = refl ║"))
    print(bold(  "  ╚═══════════════════════════════════════════════════════╝\n"))

    print(f"  {bold('Given')} p : a = b,  {bold('then')} p⁻¹ : b = a\n")
    print(f"  {bold('Law 1')}: p⁻¹ · p = refl_b")
    print(f"    The composed path {yellow('p⁻¹ · p')} : b = b")
    print(f"    is homotopic to the constant path {yellow('refl_b')} : b = b")
    print()
    print(f"  {bold('Law 2')}: p · p⁻¹ = refl_a")
    print(f"    The composed path {yellow('p · p⁻¹')} : a = a")
    print(f"    is homotopic to the constant path {yellow('refl_a')} : a = a")
    print()
    print(wrap(
        "Neither law holds definitionally in MLTT. "
        "Each requires a 2-cell witness. "
        "For law 1, inv-left(p) : p⁻¹ · p = refl_b. "
        "Proved by induction on p: inv-left(refl) is trivial since "
        "refl⁻¹ = refl and refl · refl = refl definitionally.", width=72
    ))
    print()

    print(f"  {bold('Geometric intuition')}:")
    print(f"    p⁻¹ · p: go from b to a (along p backwards), then a to b (along p).")
    print(f"    The round trip is homotopic to staying at b.")
    print(f"    But it's not IDENTICAL to refl — it has a specific shape (a 'lollipop').")
    print()

    print(f"  {bold('Why this matters for HoTT')}:")
    print(wrap(
        "In a strict groupoid (e.g., a group), p⁻¹ · p = 1 is an EQUATION. "
        "In a weak groupoid (every type in HoTT), p⁻¹ · p = refl holds "
        "up to a 2-cell — the cancellation is a path, not a definitional rule. "
        "This weakening is not a defect: it's what makes every type an ∞-groupoid "
        "rather than a strict 1-groupoid.", width=72
    ))
    print()

    print(f"  {bold('Proofs')}:")
    print(f"    inv-left  : (p : a = b) → p⁻¹ · p = refl")
    print(f"    inv-left refl = refl")
    print()
    print(f"    inv-right : (p : a = b) → p · p⁻¹ = refl")
    print(f"    inv-right refl = refl")
    print()
    input(dim("  Press Enter to return…"))

def _eckmann_hilton():
    clear()
    print(bold("\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(  "  ║  The Eckmann-Hilton Argument: π₂ is Abelian          ║"))
    print(bold(  "  ╚═══════════════════════════════════════════════════════╝\n"))

    print(wrap(
        "For 1-loops (paths in the fundamental group π₁), composition "
        "is generally non-commutative. The free group F₂ = π₁(S¹ ∨ S¹) "
        "has elements like aba⁻¹b⁻¹ ≠ refl.", width=72
    ))
    print()
    print(bold("  Theorem (Eckmann-Hilton):"))
    print(f"    For any type A and basepoint a : A,")
    print(f"    π₂(A, a) = π₁(Ω(A, a))  is {bold(green('abelian'))} (commutative).")
    print()
    print(wrap(
        "Where Ω(A, a) = (a =_A a) is the loop space. "
        "π₂(A, a) consists of 2-loops: loops in the loop space.", width=72
    ))
    print()

    print(bold("  Proof sketch (the Eckmann-Hilton argument):"))
    print()
    print(f"  Let p, q : refl = refl  (2-loops at base)\n")
    print(f"  Horizontal composition: p ★ q")
    print(f"  Vertical   composition: p · q")
    print()
    print(f"  The square showing p ★ q:")
    print(f"    a ── refl ──► a ── refl ──► a")
    print(f"    │                           │")
    print(f"    p                           q")
    print(f"    │                           │")
    print(f"    a ── refl ──► a ── refl ──► a")
    print()
    print(f"  Reading the square: p ★ q = (refl · q) · (p · refl) = q · p")
    print(f"  But also:           p ★ q = (p · refl) · (refl · q) = p · q")
    print()
    print(f"  Therefore: {bold(yellow('p · q = p ★ q = q · p'))}")
    print()
    print(wrap(
        "The two ways of composing the square (horizontally vs vertically) "
        "are equal by the interchange law, and both compose to p·q and q·p. "
        "Therefore p·q = q·p — the 2-loop composition is commutative.", width=72
    ))
    print()
    print(wrap(
        "Consequence: π₂(S²) = ℤ is a trivially abelian group (ℤ is abelian). "
        "π₃(S²) = ℤ is abelian (all πₙ for n ≥ 2 are abelian). "
        "This is a general theorem of algebraic topology, proved here synthetically.", width=72
    ))
    print()
    input(dim("  Press Enter to return…"))

def _pentagon():
    clear()
    print(bold("\n  ╔═══════════════════════════════════════════════════════╗"))
    print(bold(  "  ║  The Pentagon Coherence: Associativity in 3D          ║"))
    print(bold(  "  ╚═══════════════════════════════════════════════════════╝\n"))

    print(wrap(
        "Given four composable paths p, q, r, s, there are five ways to bracket "
        "the composition. The associator 2-cell assoc(p,q,r) gives 5 different "
        "paths between the five bracketings. These 5 paths form the boundary of "
        "a pentagon, and the pentagon must commute — meaning the five paths "
        "around the boundary are all equal as 3-cells.", width=72
    ))
    print()
    print(f"  {bold('The five bracketings of p · q · r · s')}:")
    print()
    print(f"    1.  ((p · q) · r) · s")
    print(f"    2.  (p · (q · r)) · s")
    print(f"    3.  p · ((q · r) · s)")
    print(f"    4.  p · (q · (r · s))")
    print(f"    5.  (p · q) · (r · s)")
    print()
    print(f"  {bold('The pentagon diagram')} (each edge is an assoc 2-cell):")
    print()
    print(f"              (1)")
    print(f"            /     \\")
    print(f"          (5)     (2)")
    print(f"          |         |")
    print(f"          (4) ─── (3)")
    print()
    print(wrap(
        "The pentagon says: no matter which sequence of assoc moves you apply "
        "to go from bracketing (1) to bracketing (4), you get the same result. "
        "This is the pentagon identity of a monoidal category — or in HoTT, "
        "it is automatically satisfied because all these 3-cells are paths "
        "that are provably equal by path induction.", width=72
    ))
    print()
    print(wrap(
        "In ordinary (strict) category theory, the pentagon is an AXIOM. "
        "In HoTT, it is a THEOREM — it follows from path induction. "
        "You never have to check coherence conditions manually: "
        "the type theory enforces them automatically.", width=72
    ))
    print()
    input(dim("  Press Enter to return…"))

# ── Main ─────────────────────────────────────────────────────────────────────

def main():
    while True:
        clear()
        print(bold("\n  ╔════════════════════════════════════════════════════════╗"))
        print(bold(  "  ║      Groupoid Laws as Higher Paths                     ║"))
        print(bold(  "  ╚════════════════════════════════════════════════════════╝\n"))
        print(f"  Every type is an {bold('∞-groupoid')}. The groupoid laws hold")
        print(f"  {bold('up to higher paths')} — not definitionally, but propositionally.\n")
        print(f"  {cyan('1')}   Left unit law:        refl · p = p    (witness lu(p))")
        print(f"  {cyan('2')}   Right unit law:       p · refl = p    (witness ru(p))")
        print(f"  {cyan('3')}   Associativity:   (p·q)·r = p·(q·r)   (associator square)")
        print(f"  {cyan('4')}   Inverse laws:    p⁻¹·p = refl        (cancellation 2-cell)")
        print(f"  {cyan('5')}   Eckmann-Hilton:  π₂ is abelian       (2-loops commute)")
        print(f"  {cyan('p')}   Pentagon:        coherence of assoc   (3-cells are automatic)")
        print(f"  {cyan('h')}   help    {cyan('q')}  quit\n")

        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break

        if cmd in ("q", "quit"):
            break
        elif cmd == "1":
            _left_unit()
        elif cmd == "2":
            _right_unit()
        elif cmd == "3":
            _associativity()
        elif cmd == "4":
            _inverse_laws()
        elif cmd == "5":
            _eckmann_hilton()
        elif cmd == "p":
            _pentagon()
        elif cmd in ("h", "help", "?"):
            clear()
            print(f"""
  {bold('Groupoid Laws — Commands')}

  {cyan('1')}   left unit:   refl · p = p  (witness 2-cell lu)
  {cyan('2')}   right unit:  p · refl = p  (witness 2-cell ru)
  {cyan('3')}   associativity: (p·q)·r = p·(q·r)  (assoc square)
  {cyan('4')}   inverse laws: p⁻¹·p = refl  (cancellation)
  {cyan('5')}   Eckmann-Hilton: 2-loops commute  (π₂ is abelian)
  {cyan('p')}   pentagon coherence  (the 3D version of associativity)
  {cyan('h')}   this help
  {cyan('q')}   quit
""")
            input(dim("  Press Enter…"))


if __name__ == "__main__":
    clear()
    print(bold("""
  ╔════════════════════════════════════════════════════════╗
  ║     Groupoid Laws as Higher Paths                      ║
  ╚════════════════════════════════════════════════════════╝
"""))
    print("""  Every type in HoTT is an ∞-groupoid. But the groupoid laws
  don't hold by definition — they hold up to HIGHER PATHS.

    refl · p = p        (only up to a 2-cell lu(p))
    p · refl = p        (only up to a 2-cell ru(p))
    (p·q)·r = p·(q·r)  (only up to the associator square)

  These witnesses are themselves paths between paths — 2-cells.
  And the coherences between those witnesses are 3-cells. And so on.
  This infinite tower of coherence is what '∞-groupoid' means.

  Type '1'-'5' to explore the laws, 'p' for the pentagon, 'q' to quit.
""")
    input(dim("  Press Enter to start…"))
    main()
    print(f"\n  {dim('Every type is an ∞-groupoid: laws hold up to coherent higher paths.')}\n")
