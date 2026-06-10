#!/usr/bin/env python3
"""
Kripke Semantics for Intuitionistic Logic
==========================================
Worlds, forcing relations, and countermodels for classical tautologies.

A Kripke frame is a set of "possible worlds" with an accessibility relation.
Truth is not absolute but relative to a world — and monotone: once true,
always true in accessible worlds. This models the intuitionistic idea that
knowledge only grows over time.
"""

import textwrap
from itertools import product as iproduct

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


# ── Kripke model engine ───────────────────────────────────────────────────────

class KripkeFrame:
    """
    worlds: list of world names
    access: dict world -> set of accessible worlds (reflexive by default)
    val:    dict (world, atom) -> bool  (monotone by construction if we check)
    """
    def __init__(self, worlds, access, val):
        self.worlds = worlds
        self.access = access  # w -> {w, w1, w2, ...}
        self.val = val        # (w, 'P') -> bool

    def forces(self, w, formula):
        """Evaluate w ⊩ formula where formula is a nested tuple."""
        if isinstance(formula, str):                     # atom
            return self.val.get((w, formula), False)
        op = formula[0]
        if op == 'T':   return True
        if op == 'F':   return False
        if op == 'not':
            phi = formula[1]
            # w ⊩ ¬φ iff for all w' ≥ w, w' ⊬ φ
            return all(not self.forces(wp, phi) for wp in self.access[w])
        if op == 'and':
            return self.forces(w, formula[1]) and self.forces(w, formula[2])
        if op == 'or':
            return self.forces(w, formula[1]) or self.forces(w, formula[2])
        if op == 'imp':
            phi, psi = formula[1], formula[2]
            # w ⊩ φ→ψ iff for all w' ≥ w, w' ⊩ φ implies w' ⊩ ψ
            return all((not self.forces(wp, phi)) or self.forces(wp, psi)
                       for wp in self.access[w])
        raise ValueError(f"Unknown op: {op}")

    def valid(self, formula):
        """True iff forced at all worlds."""
        return all(self.forces(w, formula) for w in self.worlds)

    def counterworld(self, formula):
        """Return a world where formula fails, or None."""
        for w in self.worlds:
            if not self.forces(w, formula):
                return w
        return None

def fmt_formula(f):
    """Pretty-print a formula tuple."""
    if isinstance(f, str): return f
    op = f[0]
    if op == 'T':   return '⊤'
    if op == 'F':   return '⊥'
    if op == 'not': return f'¬{fmt_formula(f[1])}'
    if op == 'and': return f'({fmt_formula(f[1])} ∧ {fmt_formula(f[2])})'
    if op == 'or':  return f'({fmt_formula(f[1])} ∨ {fmt_formula(f[2])})'
    if op == 'imp': return f'({fmt_formula(f[1])} → {fmt_formula(f[2])})'
    return str(f)


# ── Pre-built frames ──────────────────────────────────────────────────────────

def _two_world_frame(p_at_1=False, q_at_1=False):
    """Two worlds 0 < 1.  0 can access 0 and 1.  1 accesses only 1."""
    worlds = [0, 1]
    access = {0: [0, 1], 1: [1]}
    val = {(0,'P'): False, (1,'P'): p_at_1,
           (0,'Q'): False, (1,'Q'): q_at_1}
    return KripkeFrame(worlds, access, val)

def _three_world_frame():
    """
    Three worlds: 0 < 1, 0 < 2 (diamond / branching).
    P is true at 1, Q is true at 2.
    """
    worlds = [0, 1, 2]
    access = {0: [0,1,2], 1: [1], 2: [2]}
    val = {(0,'P'): False, (1,'P'): True,  (2,'P'): False,
           (0,'Q'): False, (1,'Q'): False, (2,'Q'): True}
    return KripkeFrame(worlds, access, val)

def _chain_frame(n=3):
    """Linear chain 0 < 1 < 2 < ... < n-1, P becomes true at n-1."""
    worlds = list(range(n))
    access = {i: list(range(i, n)) for i in range(n)}
    val = {}
    for i in range(n):
        val[(i,'P')] = (i == n-1)
        val[(i,'Q')] = False
    return KripkeFrame(worlds, access, val)


# ── Sections ──────────────────────────────────────────────────────────────────

def _section_frames():
    clear()
    print(box("Kripke Frames: Worlds and Accessibility"))
    print()
    print(wrap(
        "A Kripke frame for intuitionistic logic is a pair (W, ≤) where W is "
        "a set of 'possible worlds' and ≤ is a preorder (reflexive, transitive "
        "accessibility relation). Intuitively: worlds represent stages of "
        "knowledge, and w ≤ w′ means w′ is a 'later' or 'more informed' world."
    ))
    print()
    print(f"  {cyan('Frame = (W : Set, ≤ : W → W → Prop, refl, trans)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('A valuation'))}")
    print()
    print(f"  {cyan('V : AtomicProp → W → Prop')}")
    print(f"  {cyan('  monotone: w ≤ w′ → V(p)(w) → V(p)(w′)')}")
    print()
    print(wrap(
        "The monotonicity condition captures the intuitionistic idea: once you "
        "KNOW something is true, you cannot un-know it at a later world. "
        "Truth only grows; it never shrinks as you move forward."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Example: the two-world frame'))}\n")
    print(f"  {bold('Worlds:')} 0, 1   {bold('Accessibility:')} 0 ≤ 0, 0 ≤ 1, 1 ≤ 1")
    print()
    print(f"    0 ──→ 1")
    print(f"    {dim('(0 can see 1, but 1 cannot see 0)')}")
    print()
    print(f"  P is false at world 0, true at world 1.")
    print(f"  {dim('(We learn P at some future moment)')}")
    print()
    print(f"\n  {bold(green('Example: the diamond frame'))}\n")
    print(f"         0")
    print(f"        {dim('/ \\')}")
    print(f"       1   2")
    print(f"  {dim('P true at 1, Q true at 2, neither at 0.')}")
    print(f"  {dim('Worlds 1 and 2 are incompatible futures.')}")
    print()
    print(wrap(
        "The diamond frame is crucial for refuting disjunction: at world 0, "
        "we cannot assert P ∨ Q even though it becomes true in both futures — "
        "because we don't yet know WHICH disjunct will hold."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_forcing():
    clear()
    print(box("The Forcing Relation: w ⊩ φ"))
    print()
    print(wrap(
        "The forcing relation w ⊩ φ ('world w forces formula φ') is defined "
        "by induction on the formula:"
    ))
    print()
    rules = [
        ("w ⊩ p",     "(atom)",   "V(p)(w) = true"),
        ("w ⊩ ⊤",     "",         "always"),
        ("w ⊩ ⊥",     "",         "never"),
        ("w ⊩ φ∧ψ",   "",         "w ⊩ φ  and  w ⊩ ψ"),
        ("w ⊩ φ∨ψ",   "",         "w ⊩ φ  or   w ⊩ ψ"),
        ("w ⊩ φ→ψ",   "",         "for all w′ ≥ w:  w′ ⊩ φ  implies  w′ ⊩ ψ"),
        ("w ⊩ ¬φ",    "(= φ→⊥)", "for all w′ ≥ w:  w′ ⊬ φ"),
    ]
    for form, note, meaning in rules:
        nd = f" {dim(note)}" if note else ""
        print(f"  {bold(cyan(form))}{nd}")
        print(f"    iff  {meaning}")
        print()

    print(rule())
    print(f"\n  {bold(yellow('Key observations:'))}\n")
    obs = [
        ("Monotonicity",  "If w ⊩ φ and w ≤ w′, then w′ ⊩ φ.  (Provable by induction.)"),
        ("Disjunction",   "w ⊩ φ∨ψ requires knowing NOW which holds — not just 'eventually'."),
        ("Implication",   "w ⊩ φ→ψ checks ALL future worlds, not just w."),
        ("Negation",      "w ⊩ ¬φ means φ is PERMANENTLY refuted from w onward."),
    ]
    for name, note in obs:
        print(f"  {bold(green(name))}")
        print(wrap(note, width=66, indent="    "))
        print()

    print(rule())
    print(f"\n  {bold(green('Live: forcing in the two-world frame'))}\n")
    frame = _two_world_frame(p_at_1=True)
    formulas = [
        ('P',                           'P'),
        ('not', 'P'),                   # ¬P
        ('or', 'P', ('not', 'P')),      # P ∨ ¬P
        ('imp', 'P', 'P'),              # P → P
        ('not', ('not', 'P')),          # ¬¬P
    ]
    fmt_list = [
        'P', '¬P', 'P ∨ ¬P', 'P → P', '¬¬P'
    ]
    actual = [
        'P',
        ('not', 'P'),
        ('or', 'P', ('not', 'P')),
        ('imp', 'P', 'P'),
        ('not', ('not', 'P')),
    ]
    for fstr, f in zip(fmt_list, actual):
        w0 = green("✓") if frame.forces(0, f) else red("✗")
        w1 = green("✓") if frame.forces(1, f) else red("✗")
        print(f"  {fstr:<20}  world 0: {w0}   world 1: {w1}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_lem():
    clear()
    print(box("LEM Fails: A Countermodel for P ∨ ¬P"))
    print()
    print(wrap(
        "The Law of Excluded Middle (LEM: φ ∨ ¬φ) is valid classically but "
        "NOT intuitionistically. We construct an explicit Kripke countermodel."
    ))
    print()
    print(f"  {bold('Frame:')} two worlds, 0 ≤ 1")
    print(f"  {bold('Valuation:')} P is false at 0, true at 1")
    print()
    print(f"    world 0 ──→ world 1")
    print(f"    P: {red('false')}         P: {green('true')}")
    print()
    print(rule())

    frame = _two_world_frame(p_at_1=True)
    phi = ('or', 'P', ('not', 'P'))

    print(f"\n  {bold(green('Checking world 0 ⊩ P ∨ ¬P:'))}\n")
    print(f"  We need:  world 0 ⊩ P  OR  world 0 ⊩ ¬P")
    print()
    p_at_0 = frame.forces(0, 'P')
    neg_p_at_0 = frame.forces(0, ('not', 'P'))
    print(f"  world 0 ⊩ P    ?  {green('yes') if p_at_0 else red('no')}  (P is false at 0)")
    print()
    print(f"  world 0 ⊩ ¬P   ?  {green('yes') if neg_p_at_0 else red('no')}")
    print(f"  {dim('(¬P means: for all w′ ≥ 0, w′ ⊬ P.  But world 1 ≥ 0 and 1 ⊩ P.  So ¬P fails.)')}")
    print()
    result = frame.forces(0, phi)
    print(f"  {bold('Therefore:')} world 0 ⊩ P ∨ ¬P  ?  {green('yes') if result else bold(red('NO'))}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Intuitionistic meaning:'))}\n")
    print(wrap(
        "At world 0, we have no information about P. We cannot assert P (it might "
        "be false) and we cannot assert ¬P (it will become true at world 1). "
        "The disjunction P ∨ ¬P would require us to CHOOSE a disjunct now, "
        "before we have enough information."
    ))
    print()
    print(wrap(
        "This is why LEM is not provable constructively: there is no algorithm "
        "that, given an arbitrary proposition P, decides whether P or ¬P holds. "
        "The Kripke model makes this failure precise and geometric."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('What IS valid: ¬¬(P ∨ ¬P)'))}\n")
    dnem = ('not', ('not', ('or', 'P', ('not', 'P'))))
    valid = frame.valid(dnem)
    print(f"  ¬¬(P ∨ ¬P) valid in this frame? {green('yes') if valid else red('no')}")
    print(wrap(
        "The double negation of LEM IS intuitionistically provable — you cannot "
        "refute LEM, but that is weaker than proving it. This reflects the "
        "double-negation translation of classical into intuitionistic logic."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_countermodels():
    clear()
    print(box("More Countermodels: Classical Tautologies That Fail"))
    print()
    print(wrap(
        "Many classical tautologies fail intuitionistically. Here we find "
        "explicit Kripke countermodels for each."
    ))
    print()

    # Diamond frame: P true at 1, Q true at 2, neither at 0
    diamond = _three_world_frame()

    cases = [
        ("LEM",           "P ∨ ¬P",
         ('or', 'P', ('not','P')),
         _two_world_frame(p_at_1=True),
         "At world 0: P unknown (true at 1, not yet)."),

        ("DNE",           "¬¬P → P",
         ('imp', ('not',('not','P')), 'P'),
         _two_world_frame(p_at_1=True),
         "At world 0: ¬¬P holds (P will be true), but P is not true yet."),

        ("Peirce",        "((P→Q)→P)→P",
         ('imp',('imp',('imp','P','Q'),'P'),'P'),
         _two_world_frame(p_at_1=True),
         "Classical: if (P→Q)→P then P. Fails in two-world frame."),

        ("Weak LEM",      "¬P ∨ ¬¬P",
         ('or', ('not','P'), ('not',('not','P'))),
         diamond,
         "Diamond frame: at 0, ¬P fails (P true at 1) and ¬¬P fails (P false at 2)."),

        ("de Morgan",     "¬(P∧Q)→(¬P∨¬Q)",
         ('imp', ('not',('and','P','Q')), ('or',('not','P'),('not','Q'))),
         diamond,
         "At world 0 in diamond: ¬(P∧Q) holds but neither ¬P nor ¬Q does."),
    ]

    for name, fmtstr, formula, frame, note in cases:
        cw = frame.counterworld(formula)
        valid_str = green("valid") if cw is None else red(f"fails at world {cw}")
        print(f"  {bold(cyan(name)):<20} {fmtstr}")
        print(f"    Status:  {valid_str}")
        if cw is not None:
            print(f"    Reason:  {dim(note)}")
        print()

    print(rule())
    print(f"\n  {bold(green('What IS valid in all Kripke frames:'))}\n")
    frame2 = _two_world_frame(p_at_1=True)
    frame3 = _three_world_frame()
    intuit = [
        ("P → P",              ('imp','P','P')),
        ("P → ¬¬P",            ('imp','P',('not',('not','P')))),
        ("¬¬¬P → ¬P",          ('imp',('not',('not',('not','P'))),('not','P'))),
        ("(P→Q)→(P→Q)",       ('imp',('imp','P','Q'),('imp','P','Q'))),
        ("P∧Q → Q∧P",          ('imp',('and','P','Q'),('and','Q','P'))),
    ]
    for fmtstr, formula in intuit:
        v2 = frame2.valid(formula)
        v3 = frame3.valid(formula)
        ok = green("✓") if (v2 and v3) else red("✗")
        print(f"  {ok} {fmtstr}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_soundness():
    clear()
    print(box("Soundness and Completeness"))
    print()
    print(wrap(
        "The Kripke semantics is SOUND and COMPLETE for intuitionistic "
        "propositional logic (IPC):"
    ))
    print()
    print(f"  {cyan('IPC ⊢ φ   iff   all Kripke frames force φ')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Soundness (⊢ implies ⊩):'))}\n")
    print(wrap(
        "Every intuitionistic proof is valid in every Kripke frame. "
        "Proven by induction on proof structure: the rules of natural deduction "
        "correspond exactly to the forcing clauses."
    ))
    print()
    print(f"  {bold('modus ponens:')} if w ⊩ φ→ψ and w ⊩ φ, then w ⊩ ψ")
    print(f"  {dim('  (direct from the forcing clause for →)')}")
    print()
    print(f"  {bold('introduction:')}")
    print(f"  {dim('  ∧I: w ⊩ φ and w ⊩ ψ implies w ⊩ φ∧ψ')}")
    print(f"  {dim('  ∨I: w ⊩ φ implies w ⊩ φ∨ψ')}")
    print(f"  {dim('  →I: (for all w′≥w: if w′⊩φ then w′⊩ψ) implies w⊩φ→ψ')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Completeness (⊩ implies ⊢):'))}\n")
    print(wrap(
        "If φ is valid in all Kripke frames, then φ is provable in IPC. "
        "The proof uses the CANONICAL MODEL: the frame whose worlds are "
        "the theories (deductively closed sets of formulas) of IPC."
    ))
    print()
    print(f"  {dim('Canonical frame:')}")
    print(f"  {cyan('  W = {Γ | Γ is a theory of IPC}')}")
    print(f"  {cyan('  Γ ≤ Δ  iff  Γ ⊆ Δ')}")
    print(f"  {cyan('  V(p)(Γ) = (p ∈ Γ)')}")
    print()
    print(wrap(
        "The canonical model satisfies: Γ ⊩ φ iff φ ∈ Γ. So if φ is forced "
        "at all worlds (all theories), then φ belongs to the empty theory — "
        "but the empty theory is what IPC proves, so ⊢ φ."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Connection to HoTT:'))}\n")
    print(wrap(
        "In HoTT, Kripke semantics corresponds to PRESHEAF MODELS. The worlds "
        "are objects of a category C, accessibility is the morphism structure, "
        "and types/propositions are presheaves (functors C^op → Set). "
        "The forcing relation is the presheaf evaluation. This is one reason "
        "HoTT has a presheaf model — it is a Kripke model in disguise."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_explore():
    clear()
    print(box("Interactive: Explore Forcing in the Diamond Frame"))
    print()
    print(wrap(
        "The diamond frame has three worlds: 0 (bottom), 1 (left), 2 (right). "
        "P is true only at 1. Q is true only at 2."
    ))
    print()
    print(f"         {bold('0')}")
    print(f"        {dim('/ \\')}")
    print(f"       {bold('1')}   {bold('2')}")
    print(f"  P: {green('T')} at 1, {red('F')} elsewhere")
    print(f"  Q: {green('T')} at 2, {red('F')} elsewhere")
    print()
    frame = _three_world_frame()

    # Predefined interesting formulas
    tests = [
        ("P",                  'P'),
        ("Q",                  'Q'),
        ("P ∨ Q",              ('or','P','Q')),
        ("¬(P ∨ Q)",           ('not',('or','P','Q'))),
        ("¬P ∨ ¬Q",            ('or',('not','P'),('not','Q'))),
        ("P → Q",              ('imp','P','Q')),
        ("Q → P",              ('imp','Q','P')),
        ("¬P ∧ ¬Q",            ('and',('not','P'),('not','Q'))),
        ("¬(P ∧ Q)",           ('not',('and','P','Q'))),
        ("(P→Q)∨(Q→P)",        ('or',('imp','P','Q'),('imp','Q','P'))),
        ("¬¬(P ∨ Q)",          ('not',('not',('or','P','Q')))),
    ]

    print(f"  {bold('Formula'):<28} {'w=0':>6} {'w=1':>6} {'w=2':>6} {'valid':>6}")
    print(f"  {dim('─'*52)}")
    for fmtstr, formula in tests:
        results = [frame.forces(w, formula) for w in [0,1,2]]
        valid = all(results)
        cells = [green("T") if r else red("F") for r in results]
        vstr = green("✓") if valid else dim("·")
        print(f"  {fmtstr:<28} {cells[0]:>14} {cells[1]:>14} {cells[2]:>14} {vstr:>14}")
    print()
    print(wrap(
        "Notice: (P→Q)∨(Q→P) — Dummett's formula — is classically valid. "
        "It fails here at world 0: P→Q fails (P true at 1, Q false at 1) and "
        "Q→P fails (Q true at 2, P false at 2). This formula axiomatizes "
        "the 'linear' Kripke frames — Dummett's intermediate logic LC."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("frames",   "Kripke frames: worlds and accessibility",         _section_frames),
    ("forcing",  "The forcing relation w ⊩ φ",                     _section_forcing),
    ("lem",      "LEM fails: a countermodel for P ∨ ¬P",           _section_lem),
    ("counter",  "More countermodels: classical tautologies that fail", _section_countermodels),
    ("complete", "Soundness and completeness",                      _section_soundness),
    ("explore",  "Interactive: forcing in the diamond frame",       _section_explore),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Kripke Semantics for Intuitionistic Logic", width=70))
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
