#!/usr/bin/env python3
"""
demo_hlevels.py — The H-level hierarchy

Explore the truncation ladder:
  h-level −2  contractible   (one point up to unique path)
  h-level −1  mere prop      (truth value)
  h-level  0  set            (no higher structure)
  h-level  1  1-groupoid     (paths form a set)
  h-level  2  2-groupoid     (paths between paths form a set)
  h-level  ∞  ∞-groupoid     (all of HoTT)

Commands
  l / r   navigate examples left / right
  p       inspect the path space of the current type
  t       truncate current type and see what collapses
  q       query: "is X a prop?  is X a set?"
  e       explanations and theorems
  h       help
  0       quit
"""

from __future__ import annotations
import sys
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

# ── Type definitions ──────────────────────────────────────────────────────────

# Each entry: (hlevel, name, agda_type, elements, path_space_desc, explanation)
TYPES: list[dict] = [
    {
        "hlevel": -2,
        "name": "Unit  (⊤)",
        "agda": "⊤ : Type\n⊤ = Lift ⊤",
        "elements": ["tt"],
        "element_note": "one element: tt",
        "path_space": "tt = tt  ≃  ⊤  (contractible: exactly one path = refl)",
        "path_hlevel": -2,
        "explanation": (
            "Unit is the prototypical contractible type. "
            "There is exactly one element (tt), and the only path between "
            "any two elements is refl — there is no choice. "
            "A type A is contractible (h-level −2) iff "
            "there exists a : A such that all x : A satisfy a = x. "
            "The center of contraction here is tt."
        ),
        "is_prop": True, "is_set": True,
    },
    {
        "hlevel": -1,
        "name": "Bool as Prop  (∥ Bool ∥₋₁)",
        "agda": "∥ Bool ∥₋₁ : Prop",
        "elements": ["∣ true ∣", "∣ false ∣  (equal!)"],
        "element_note": "propositionally: one truth value",
        "path_space": "p = q  for all p q : ∥Bool∥₋₁  (unique paths)",
        "path_hlevel": -2,
        "explanation": (
            "Propositional truncation ∥A∥₋₁ collapses A to a mere proposition: "
            "we only remember 'A is inhabited' or not. Even though Bool has two "
            "elements, ∥Bool∥₋₁ has only one truth value — 'Bool is inhabited'. "
            "A mere prop is a type where all elements are equal: ∀ x y, x = y. "
            "Examples: isProp, isEquiv, hProp itself."
        ),
        "is_prop": True, "is_set": True,
    },
    {
        "hlevel": -1,
        "name": "Empty  (⊥)",
        "agda": "⊥ : Type\n⊥ = Empty",
        "elements": [],
        "element_note": "no elements (vacuously a prop)",
        "path_space": "(vacuous — no elements to form paths)",
        "path_hlevel": -2,
        "explanation": (
            "The empty type ⊥ is a mere proposition because the statement "
            "'all elements of ⊥ are equal' is vacuously true — there are no "
            "elements to compare. Both ⊤ (one element) and ⊥ (no elements) "
            "sit at h-level −1 as mere propositions."
        ),
        "is_prop": True, "is_set": True,
    },
    {
        "hlevel": 0,
        "name": "ℕ  (natural numbers)",
        "agda": "ℕ : Set\nℕ = zero | suc ℕ",
        "elements": ["0", "1", "2", "3", "…"],
        "element_note": "infinitely many, all distinct",
        "path_space": "m = n  in ℕ  ≃  isEq m n  (decidable equality)",
        "path_hlevel": -1,
        "explanation": (
            "ℕ is a set (h-level 0): its path spaces are all mere propositions "
            "(either m = n is contractible if m = n, or empty if m ≠ n). "
            "Sets are types where equality is 'proof-irrelevant': there is at most "
            "one proof that m = n. All countable discrete types (Bool, ℤ, Fin n) "
            "are sets. So is every type defined by pattern matching that produces a "
            "decidable equality."
        ),
        "is_prop": False, "is_set": True,
    },
    {
        "hlevel": 0,
        "name": "Bool",
        "agda": "Bool : Set\nBool = true | false",
        "elements": ["true", "false"],
        "element_note": "two elements, true ≠ false",
        "path_space": "true = true ≃ ⊤,   true = false ≃ ⊥",
        "path_hlevel": -1,
        "explanation": (
            "Bool is a set: every identity type Bool-element = Bool-element "
            "is either contractible (refl) or empty. The proof uses 'encode' "
            "(map b = c to a code type, then analyse). Bool is NOT a proposition "
            "because true ≠ false — they are distinct elements."
        ),
        "is_prop": False, "is_set": True,
    },
    {
        "hlevel": 1,
        "name": "S¹  (the circle)",
        "agda": "data S¹ : Type where\n  base : S¹\n  loop : base = base",
        "elements": ["base"],
        "element_note": "one point, but non-trivial loops",
        "path_space": "base = base  ≃  ℤ  (the winding number!)",
        "path_hlevel": 0,
        "explanation": (
            "S¹ is a 1-groupoid (h-level 1): its path spaces are sets (h-level 0), "
            "but S¹ itself is not a set because there are interesting loops at base. "
            "Specifically base = base ≃ ℤ — we can wind around any integer number "
            "of times. But paths BETWEEN paths (2-cells) are all trivial: π₂(S¹) = 0. "
            "This means S¹ is an Eilenberg–MacLane space K(ℤ,1)."
        ),
        "is_prop": False, "is_set": False,
    },
    {
        "hlevel": 2,
        "name": "S²  (the 2-sphere)",
        "agda": "data S² : Type where\n  base : S²\n  surf : refl = refl",
        "elements": ["base"],
        "element_note": "one point, non-trivial 2-cells",
        "path_space": "base = base  ≃  S¹  (path space has S¹ shape!)",
        "path_hlevel": 1,
        "explanation": (
            "S² is a 2-groupoid (h-level 2). Its path space at base is S¹ "
            "(π₁(S²) = 0, but π₂(S²) = ℤ — there are interesting 2-spherical shells). "
            "Brunerie's theorem (proved in HoTT) showed π₃(S²) = ℤ, with the "
            "Brunerie number β = ±1 as a key witness. "
            "Higher spheres Sⁿ live at h-level n."
        ),
        "is_prop": False, "is_set": False,
    },
    {
        "hlevel": -2,
        "name": "Σ (A : ⊤), B(a)  (fiber over ⊤)",
        "agda": "Σ ⊤ B  ≃  B(tt)",
        "elements": ["(tt, b)  for each b : B(tt)"],
        "element_note": "same structure as B(tt)",
        "path_space": "inherits h-level from B",
        "path_hlevel": None,
        "explanation": (
            "Sigma types (dependent pairs) inherit h-level from their components: "
            "if A is contractible (h-level −2) and B(a) is contractible for all a, "
            "then Σ A B is contractible. More generally: "
            "hlevel(Σ A B) = max(hlevel(A), sup_a hlevel(B(a))). "
            "This is why 'total spaces of fibrations over contractible bases' "
            "have the same h-level as the fiber."
        ),
        "is_prop": None, "is_set": None,
    },
    {
        "hlevel": 0,
        "name": "A → B  (function type, A B : Set)",
        "agda": "A → B : Set  when A B : Set",
        "elements": ["f, g : A → B"],
        "element_note": "functions; extensionally equal iff pointwise",
        "path_space": "f = g  ≃  ∀ x, f(x) = g(x)  (funext!)",
        "path_hlevel": -1,
        "explanation": (
            "Function extensionality (funext) is an axiom in book HoTT but a "
            "theorem in Cubical Type Theory. It says: two functions are equal iff "
            "they are pointwise equal. When B : Set, this makes A → B a set too. "
            "When B is a mere prop, A → B is a mere prop. "
            "The h-level of A → B = max(hlevel(A), hlevel(B)) for A : Set."
        ),
        "is_prop": None, "is_set": True,
    },
]

HLEVEL_NAMES = {
    -2: "contractible",
    -1: "mere proposition",
     0: "set",
     1: "1-groupoid",
     2: "2-groupoid",
}

HLEVEL_COLORS = {
    -2: green,
    -1: cyan,
     0: blue,
     1: yellow,
     2: magenta,
}

def _hlevel_str(n):
    name = HLEVEL_NAMES.get(n, f"{n}-groupoid")
    color = HLEVEL_COLORS.get(n, lambda x: x)
    return bold(color(f"h-level {n}  ({name})"))

# ── Displays ──────────────────────────────────────────────────────────────────

def _draw_ladder(current_idx: int):
    """Vertical h-level ladder on the right."""
    levels = [
        (-2, "contractible"),
        (-1, "mere prop   "),
        ( 0, "set         "),
        ( 1, "1-groupoid  "),
        ( 2, "2-groupoid  "),
        ( None, "∞-groupoid  "),
    ]
    lines = []
    cur_level = TYPES[current_idx]["hlevel"]
    for lvl, name in levels:
        marker = "▶ " if lvl == cur_level else "  "
        color = HLEVEL_COLORS.get(lvl, dim)
        if lvl == cur_level:
            line = bold(color(f"  {marker}h-lv {lvl if lvl is not None else '∞'}  {name}"))
        else:
            line = dim(f"  {marker}h-lv {lvl if lvl is not None else '∞'}  {name}")
        lines.append(line)
        if lvl is not None:
            lines.append(dim("  │"))
    return "\n".join(lines)

def _show_type(idx: int):
    t = TYPES[idx]
    clear()

    print(bold("\n  ╔══════════════════════════════════════════════════════╗"))
    print(bold(  "  ║           H-Level Hierarchy Explorer                ║"))
    print(bold(  "  ╚══════════════════════════════════════════════════════╝\n"))

    # H-level ladder
    print(_draw_ladder(idx))
    print()

    # Type info
    hl = t["hlevel"]
    print(f"  {bold('Type')}: {bold(cyan(t['name']))}")
    print(f"  {bold('H-level')}: {_hlevel_str(hl)}")
    print()

    # Agda/HoTT definition
    print(f"  {bold('Definition (Agda)')}:")
    for line in t["agda"].split("\n"):
        print(f"    {dim(line)}")
    print()

    # Elements
    print(f"  {bold('Elements')}: {t['element_note']}")
    if t["elements"]:
        for el in t["elements"][:5]:
            print(f"    • {cyan(el)}")
        if len(t["elements"]) > 5:
            print(f"    • {dim('…')}")
    else:
        print(f"    {red('(none)')}")
    print()

    # Path space
    print(f"  {bold('Path space')} (a = b):")
    print(f"    {yellow(t['path_space'])}")
    if t["path_hlevel"] is not None:
        print(f"    → paths are at {_hlevel_str(t['path_hlevel'])}")
    print()

    # Is-prop / is-set summary
    if t["is_prop"] is not None:
        prop_sym = green("✓ YES") if t["is_prop"] else red("✗ NO")
        set_sym  = green("✓ YES") if t["is_set"]  else red("✗ NO")
        print(f"  Is a mere prop? {prop_sym}    Is a set? {set_sym}")
        print()

    nav = f"  {dim('[l] prev   [r] next')}  ({idx+1}/{len(TYPES)})"
    print(nav)
    print(f"  {dim('[p] path space   [t] truncation   [e] explain   [q] query   [h] help')}")

def _show_explanation(idx: int):
    t = TYPES[idx]
    clear()
    print(f"\n  {bold(t['name'])}  —  Explanation\n")
    print(wrap(t["explanation"], width=72))
    print()
    input(dim("  Press Enter to return…"))

def _show_path_space(idx: int):
    t = TYPES[idx]
    clear()
    print(f"\n  {bold('Path space of')} {cyan(t['name'])}\n")
    print(f"  For a, b : {t['name'].split()[0]},  the identity type  a = b  is:\n")
    print(f"  {yellow('  ' + t['path_space'])}\n")

    ph = t["path_hlevel"]
    if ph is not None:
        print(f"  The path space itself lives at {_hlevel_str(ph)}.\n")
        if ph == -2:
            print(wrap("Contractible path spaces mean there is at most one path "
                       "between any two points — this is exactly what it means for "
                       "the ambient type to be a mere prop (h-level −1).", width=72))
        elif ph == -1:
            print(wrap("Propositional path spaces mean equality is proof-irrelevant: "
                       "if a = b at all, there is exactly one proof of it. "
                       "This is what it means to be a set (h-level 0).", width=72))
        elif ph == 0:
            print(wrap("Set-valued path spaces mean that paths between paths (2-cells) "
                       "are trivial. The type is a 1-groupoid (h-level 1): paths are "
                       "interesting, but higher homotopy is discrete.", width=72))
    print()
    input(dim("  Press Enter to return…"))

def _show_truncation(idx: int):
    t = TYPES[idx]
    clear()
    hl = t["hlevel"]
    name = t["name"].split()[0]
    print(f"\n  {bold('Truncation of')} {cyan(t['name'])}\n")

    trunc_table = [
        (-2, f"∥{name}∥₋₂", "Same as contractibilisation — collapses to ⊤ if inhabited"),
        (-1, f"∥{name}∥₋₁", "Mere proposition: only retains 'is A inhabited?'"),
        ( 0, f"∥{name}∥₀",  "Set-truncation: quotient by all higher paths"),
        ( 1, f"∥{name}∥₁",  "Groupoid truncation: quotient by 2-cells and higher"),
    ]

    print(f"  h-level of original: {_hlevel_str(hl)}\n")
    print(f"  Truncation at h-level n collapses all cells above dimension n.\n")

    for n, trunc_name, effect in trunc_table:
        if n >= hl:
            marker = dim("(no change — already that level)") if n == hl else ""
            print(f"  {bold(trunc_name)} (n={n}): {yellow(effect)} {marker}")
        else:
            print(f"  {dim(trunc_name)} (n={n}): {dim('(losing information — truncating higher structure)')}")
    print()

    if hl >= 1:
        print(wrap(
            f"For S¹ (h-level 1): ∥S¹∥₀ = ⊤  (the set-truncation is contractible, "
            "since S¹ is connected). ∥S¹∥₋₁ = ⊤  (it is merely inhabited).", width=72))
    elif hl == 0:
        print(wrap(
            f"For a set like ℕ: all truncations ∥ℕ∥ₙ for n ≥ 0 are just ℕ again "
            "(no higher structure to collapse). ∥ℕ∥₋₁ = ⊤ (merely inhabited).", width=72))

    print()
    input(dim("  Press Enter to return…"))

def _query_mode(idx: int):
    clear()
    t = TYPES[idx]
    print(f"\n  {bold('Query mode')} — {cyan(t['name'])}\n")
    print(dim("  Ask yes/no questions about this type. Type 'done' to exit.\n"))

    qa = {
        "is this a prop":         (t["is_prop"],  "A prop (h-level −1) means all elements are equal."),
        "is this a set":          (t["is_set"],   "A set (h-level 0) has proof-irrelevant equality."),
        "is this contractible":   (t["hlevel"] == -2, "Contractible = h-level −2 = ∃ center of contraction."),
        "is this a 1-groupoid":   (t["hlevel"] == 1,  "1-groupoid: path spaces are sets."),
        "is this a groupoid":     (t["hlevel"] <= 1,  "Groupoid = h-level ≤ 1."),
        "does this have a path space":  (True,    "Every type has an identity type / path space."),
        "is pi1 trivial":         (t["hlevel"] <= 0,  "π₁ is trivial for sets and below (no interesting loops)."),
        "can i truncate this":    (True,    "Any type can be truncated to any h-level ≥ its current level."),
    }

    while True:
        try:
            q = input(bold("  ? ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break
        if q in ("done", "quit", "q", ""):
            break

        matched = None
        for key, (answer, note) in qa.items():
            if all(w in q for w in key.split()):
                matched = (answer, note)
                break

        if matched is None:
            print(dim("  (try: 'is this a prop', 'is this a set', 'is pi1 trivial', …)"))
        else:
            answer, note = matched
            if answer is None:
                sym = yellow("? depends")
            elif answer:
                sym = green("✓ YES")
            else:
                sym = red("✗ NO")
            print(f"  {sym}  —  {dim(note)}")
        print()

EXPLANATIONS = """
  ╔══════════════════════════════════════════════════════════════╗
  ║                  H-Level Theorems                            ║
  ╠══════════════════════════════════════════════════════════════╣
  ║                                                              ║
  ║  Whitehead's principle (HoTT):                               ║
  ║    A map f : A → B is an equiv iff                           ║
  ║    it induces bijections on all πₙ.                          ║
  ║                                                              ║
  ║  Closure under Σ:                                            ║
  ║    If A : Prop and B : A → Prop then Σ A B : Prop.           ║
  ║    If A : Set  and B : A → Set  then Σ A B : Set.            ║
  ║                                                              ║
  ║  Function types:                                             ║
  ║    If B : Prop then A → B : Prop.                            ║
  ║    If B : Set  then A → B : Set.                             ║
  ║                                                              ║
  ║  Identity types go down by one:                              ║
  ║    If A : n-type then (a = b) : (n−1)-type.                  ║
  ║    e.g. if A : Set then (a = b) : Prop.                      ║
  ║                                                              ║
  ║  HITs can land anywhere:                                      ║
  ║    S¹ is a 1-type,  S² is a 2-type,  Sⁿ is an n-type.       ║
  ║                                                              ║
  ╚══════════════════════════════════════════════════════════════╝
"""

def _show_theory():
    clear()
    print(EXPLANATIONS)
    input(dim("  Press Enter to return…"))

def _help():
    clear()
    print(f"""
  {bold('H-Level Hierarchy Explorer — Commands')}

  {cyan('l')} / {cyan('r')}   navigate types (prev / next)
  {cyan('p')}       inspect path space of current type
  {cyan('t')}       truncation: what collapses at each level
  {cyan('q')}       query mode (ask yes/no questions)
  {cyan('e')}       explanation of current type
  {cyan('T')}       general h-level theorems
  {cyan('h')}       this help
  {cyan('0')}       quit
""")
    input(dim("  Press Enter…"))

def main():
    idx = 0
    while True:
        _show_type(idx)
        print()
        try:
            cmd = input(bold("  > ")).strip()
        except (EOFError, KeyboardInterrupt):
            break

        if cmd in ("0", "q", "quit", "exit"):
            break
        elif cmd in ("r", "n"):
            idx = (idx + 1) % len(TYPES)
        elif cmd in ("l", "p"):
            if cmd == "l":
                idx = (idx - 1) % len(TYPES)
            else:
                _show_path_space(idx)
        elif cmd == "t":
            _show_truncation(idx)
        elif cmd == "e":
            _show_explanation(idx)
        elif cmd == "q":
            _query_mode(idx)
        elif cmd == "T":
            _show_theory()
        elif cmd in ("h", "help", "?"):
            _help()


if __name__ == "__main__":
    clear()
    print(bold("""
  ╔════════════════════════════════════════════════════════╗
  ║          H-Level Hierarchy Explorer                    ║
  ╚════════════════════════════════════════════════════════╝
"""))
    print("""  Every type in HoTT lives at some rung of the h-level ladder:

    −2  contractible   (one element, one path, unique everything)
    −1  mere prop      (at most one element up to equality)
     0  set            (equality is proof-irrelevant)
     1  1-groupoid     (loops form a set)
     2  2-groupoid     (loops-of-loops form a set)
     ∞  ∞-groupoid     (unrestricted higher homotopy)

  Navigate through examples and probe their path spaces.

  Type 'r'/'l' to move, 'h' for help, '0' to quit.
""")
    input(dim("  Press Enter to start…"))
    main()
    print(f"\n  {dim('Every type lives somewhere on the h-level ladder.')}\n")
