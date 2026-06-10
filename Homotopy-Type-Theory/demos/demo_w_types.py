#!/usr/bin/env python3
"""
W-Types: The Universal Inductive Type
======================================
W(A, B): well-founded trees with node labels from A and branching from B.

Every inductive type in HoTT is (up to equivalence) a W-type or a quotient
of one. W-types are the single primitive that generates all recursive structure:
natural numbers, lists, binary trees, ordinals, and more.
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


# ── W-type engine ─────────────────────────────────────────────────────────────

class WTree:
    """
    A term of W(A, B) is sup(a, f) where a:A and f:B(a) → W(A,B).
    We represent this with a label (the 'a') and a dict of subtrees.
    """
    def __init__(self, label, children=None):
        self.label = label
        self.children = children or {}  # branch_key -> WTree

    def __repr__(self):
        if not self.children:
            return f"sup({self.label!r}, ∅)"
        ch = ", ".join(f"{k}↦{v}" for k,v in self.children.items())
        return f"sup({self.label!r}, {{{ch}}})"

    def depth(self):
        if not self.children:
            return 0
        return 1 + max(c.depth() for c in self.children.values())

    def size(self):
        return 1 + sum(c.size() for c in self.children.values())

    def display(self, indent=0, branch=""):
        prefix = "  " * indent
        label_str = bold(cyan(str(self.label)))
        if branch:
            print(f"{prefix}{dim(branch + '─')} {label_str}")
        else:
            print(f"{prefix}{label_str}")
        for key, child in self.children.items():
            child.display(indent + 1, str(key))


# ── ℕ as W(Bool, B) where B(true)=𝟙, B(false)=𝟘 ────────────────────────────

def nat_zero():
    return WTree('zero', {})

def nat_succ(n):
    return WTree('succ', {'*': n})

def nat_of_int(k):
    t = nat_zero()
    for _ in range(k):
        t = nat_succ(t)
    return t

def nat_to_int(t):
    if t.label == 'zero':
        return 0
    return 1 + nat_to_int(t.children['*'])

def nat_add(m, n):
    if m.label == 'zero':
        return n
    return nat_succ(nat_add(m.children['*'], n))

def nat_mul(m, n):
    if m.label == 'zero':
        return nat_zero()
    return nat_add(n, nat_mul(m.children['*'], n))


# ── List A as W(A+𝟙, B) where B(inl a)=𝟙, B(inr ★)=𝟘 ──────────────────────

def list_nil():
    return WTree(('nil',), {})

def list_cons(head, tail):
    return WTree(('cons', head), {'*': tail})

def list_of_python(xs):
    t = list_nil()
    for x in reversed(xs):
        t = list_cons(x, t)
    return t

def list_to_python(t):
    if t.label[0] == 'nil':
        return []
    return [t.label[1]] + list_to_python(t.children['*'])

def list_length(t):
    if t.label[0] == 'nil':
        return 0
    return 1 + list_length(t.children['*'])

def list_append(xs, ys):
    if xs.label[0] == 'nil':
        return ys
    return list_cons(xs.label[1], list_append(xs.children['*'], ys))


# ── Binary tree as W(𝟙+𝟙+A, B) ──────────────────────────────────────────────

def tree_leaf():
    return WTree('leaf', {})

def tree_node(label, left, right):
    return WTree(('node', label), {'L': left, 'R': right})

def tree_depth(t):
    if t.label == 'leaf':
        return 0
    return 1 + max(tree_depth(t.children['L']), tree_depth(t.children['R']))

def tree_labels(t):
    if t.label == 'leaf':
        return []
    return tree_labels(t.children['L']) + [t.label[1]] + tree_labels(t.children['R'])


# ── Sections ──────────────────────────────────────────────────────────────────

def _section_definition():
    clear()
    print(box("W-Types: Well-Founded Trees"))
    print()
    print(wrap(
        "The W-type W(A, B) is parameterized by:"
    ))
    print()
    print(f"  {cyan('A : Type          -- the type of node labels (arities)')}")
    print(f"  {cyan('B : A → Type      -- B(a) is the branching type at node a')}")
    print()
    print(f"  {bold('Constructor:')}")
    print(f"  {cyan('sup : Π(a:A). (B(a) → W(A,B)) → W(A,B)')}")
    print()
    print(wrap(
        "A term sup(a, f) is a tree with root labelled a and children indexed "
        "by B(a): for each branch b:B(a), the subtree f(b) is a recursive "
        "W(A,B) term. Well-foundedness is built in: every tree is finite."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The eliminator (recursion principle):'))}\n")
    print(f"  {cyan('W-rec : (P : W(A,B) → Type)')}")
    print(f"  {cyan('  → (∀a (f:B(a)→W(A,B)). (∀b. P(f b)) → P(sup(a,f)))')}")
    print(f"  {cyan('  → ∀t. P(t)')}")
    print()
    print(wrap(
        "To define a function on W(A,B), you just handle one case: a node "
        "sup(a, f) given the INDUCTIVE HYPOTHESES — the values of the function "
        "on all immediate subtrees f(b). Well-foundedness guarantees termination."
    ))
    print()
    print(rule())
    print(f"\n  {bold(yellow('Why W-types?'))}\n")
    print(wrap(
        "W-types are the SINGLE general principle behind all inductive definitions. "
        "Instead of axiomatizing ℕ, List, and Binary Tree separately, a type "
        "theory with W-types derives them all from one rule. This is conceptually "
        "cleaner and easier to study metatheoretically."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_nat():
    clear()
    print(box("ℕ as a W-Type"))
    print()
    print(wrap(
        "Natural numbers have two constructors: zero (0 children) and succ (1 child). "
        "The branching type captures this:"
    ))
    print()
    print(f"  {cyan('A := Bool   (label: true = succ-node, false = zero-node)')}")
    print(f"  {cyan('B(true)  := 𝟙   (one child: the predecessor)')}")
    print(f"  {cyan('B(false) := 𝟘   (no children: zero is a leaf)')}")
    print()
    print(f"  {cyan('ℕ = W(Bool, B)')}")
    print()
    print(f"  {cyan('zero = sup(false, absurd)    -- no branches (B(false) = 𝟘)')}")
    print(f"  {cyan('succ n = sup(true, const n)  -- one branch * ↦ n')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Live construction:'))}\n")

    for k in range(6):
        t = nat_of_int(k)
        print(f"  {bold(str(k))} = {dim(repr(t)[:60])}")

    print()
    print(rule())
    print(f"\n  {bold(green('Arithmetic by W-recursion:'))}\n")

    pairs = [(3,4), (0,5), (2,2), (7,0)]
    for a, b in pairs:
        ta, tb = nat_of_int(a), nat_of_int(b)
        tsum = nat_add(ta, tb)
        tprod = nat_mul(ta, tb)
        print(f"  {a} + {b} = {nat_to_int(tsum)}    {a} × {b} = {nat_to_int(tprod)}")

    print()
    print(wrap(
        "Addition on W-ℕ is defined by recursion on the first argument: "
        "if it is zero, return the second. If it is sup(true, f), return "
        "sup(true, λ★. add(f(★), b)). This is structural recursion on the W-tree."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_list():
    clear()
    print(box("List A as a W-Type"))
    print()
    print(wrap(
        "Lists over A have two constructors: nil (leaf) and cons h t (one child t). "
        "The head h is part of the label:"
    ))
    print()
    print(f"  {cyan('A_list := A + 𝟙   (label: inl a = cons-node, inr ★ = nil-node)')}")
    print(f"  {cyan('B(inl a) := 𝟙    (one child: the tail)')}")
    print(f"  {cyan('B(inr ★) := 𝟘    (no children)')}")
    print()
    print(f"  {cyan('List A = W(A + 𝟙, B_list)')}")
    print()
    print(f"  {cyan('nil       = sup(inr ★, absurd)')}")
    print(f"  {cyan('cons a t  = sup(inl a, const t)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Live construction:'))}\n")

    examples = [[], [1], [1,2,3], ['a','b','c','d']]
    for xs in examples:
        t = list_of_python(xs)
        back = list_to_python(t)
        length = list_length(t)
        print(f"  {str(xs):25} → length {length}  → {back}")

    print()
    print(rule())
    print(f"\n  {bold(green('append by W-recursion:'))}\n")

    pairs = [([1,2], [3,4]), ([], [5,6,7]), ([10,20,30], [])]
    for xs, ys in pairs:
        txs, tys = list_of_python(xs), list_of_python(ys)
        result = list_to_python(list_append(txs, tys))
        print(f"  {str(xs)} ++ {str(ys)} = {result}")

    print()
    input(bold("  Press Enter to continue... "))


def _section_tree():
    clear()
    print(box("Binary Trees as a W-Type"))
    print()
    print(wrap(
        "Binary trees labelled with A have: leaf (0 children) and node(a, L, R) "
        "(2 children). The branching type uses the two-element type 𝟚:"
    ))
    print()
    print(f"  {cyan('A_tree := A + 𝟙   (label: inl a = node, inr ★ = leaf)')}")
    print(f"  {cyan('B(inl a) := 𝟚     (two children: Left and Right)')}")
    print(f"  {cyan('B(inr ★) := 𝟘     (no children)')}")
    print()
    print(f"  {cyan('BTree A = W(A + 𝟙, B_tree)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Example: BST for [5, 3, 7, 1, 4]'))}\n")

    leaf = tree_leaf()
    t1 = tree_node(1, leaf, leaf)
    t4 = tree_node(4, leaf, leaf)
    t3 = tree_node(3, t1, t4)
    t7 = tree_node(7, leaf, leaf)
    t5 = tree_node(5, t3, t7)

    print(f"  Structure:")
    t5.display()
    print()
    print(f"  In-order traversal: {tree_labels(t5)}")
    print(f"  Depth: {tree_depth(t5)}")
    print(f"  Nodes: {t5.size()}")

    print()
    print(rule())
    print(f"\n  {bold(green('General pattern: shapes and positions'))}\n")
    print(wrap(
        "The pair (A, B) in W(A, B) captures the SHAPE of trees: A is the "
        "type of node shapes (labels/arities), and B(a) is the type of "
        "POSITIONS in a node of shape a — the index type for its children."
    ))
    print()
    print(f"  {'Type':25}  {'A (shapes)':20}  {'B(a) (positions)'}")
    print(f"  {dim('─'*65)}")
    examples = [
        ("ℕ",         "Bool",        "B(false)=𝟘, B(true)=𝟙"),
        ("List A",    "A+𝟙",         "B(inl a)=𝟙, B(inr)=𝟘"),
        ("BTree A",   "A+𝟙",         "B(inl a)=𝟚, B(inr)=𝟘"),
        ("Rose Tree A","A",           "B(a)=Fin(arity(a))"),
        ("Ordinals",  "𝟙+𝟙+𝟙",       "B(zero)=𝟘,B(succ)=𝟙,B(lim)=ℕ"),
    ]
    for ty, shape, pos in examples:
        print(f"  {bold(cyan(ty)):33}  {shape:20}  {dim(pos)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_ordinals():
    clear()
    print(box("Ordinals as a W-Type"))
    print()
    print(wrap(
        "Ordinal numbers are one of the most illuminating examples of W-types. "
        "An ordinal is either 0 (zero), a successor α+1, or a limit ordinal "
        "sup(f) where f:ℕ→Ord is an increasing sequence."
    ))
    print()
    print(f"  {cyan('A := 𝟙 + 𝟙 + 𝟙    (zero, succ, limit)')}")
    print(f"  {cyan('B(zero)  = 𝟘')}")
    print(f"  {cyan('B(succ)  = 𝟙')}")
    print(f"  {cyan('B(limit) = ℕ      (countably many predecessors)')}")
    print()
    print(f"  {cyan('Ord = W(𝟙+𝟙+𝟙, B)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The first few ordinals:'))}\n")

    ordinals = [
        ("0",     "zero constructor, no children"),
        ("1",     "succ(0)"),
        ("2",     "succ(succ(0))"),
        ("ω",     "limit(λn. n)   — sup of all natural numbers"),
        ("ω+1",   "succ(ω)"),
        ("ω+2",   "succ(ω+1)"),
        ("ω·2",   "limit(λn. ω+n)  — sup of ω+0, ω+1, ω+2, ..."),
        ("ω²",    "limit(λn. ω·n)  — sup of ω, ω·2, ω·3, ..."),
        ("ω^ω",   "limit(λn. ω^n)  — sup of 1, ω, ω², ω³, ..."),
        ("ε₀",    "limit(λn. ω^ω^...^ω  n times)  — fixed point of α↦ω^α"),
    ]

    for name, desc in ordinals:
        print(f"  {bold(cyan(name)):10}  {dim(desc)}")

    print()
    print(rule())
    print(f"\n  {bold(yellow('Transfinite induction as W-recursion:'))}\n")
    print(wrap(
        "The W-recursion principle on Ord is TRANSFINITE INDUCTION: to prove "
        "P(α) for all α, it suffices to prove P(0), and given P(β) for all "
        "β < α (the inductive hypotheses f:B(a)→P), prove P(α). "
        "This is exactly the structure of transfinite induction."
    ))
    print()
    print(f"  {cyan('W-rec : (∀α. (∀β<α. P β) → P α) → ∀α. P α')}")
    print()
    print(wrap(
        "Well-foundedness of the ordinals (every decreasing sequence is finite) "
        "is BUILT INTO the W-type structure: there are no infinite descending "
        "chains in W(A,B) because every element has finite depth."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_accessibility():
    clear()
    print(box("Accessibility and Well-Founded Recursion"))
    print()
    print(wrap(
        "W-types are closely related to ACCESSIBILITY PREDICATES. A relation "
        "< on A is well-founded iff every element is accessible:"
    ))
    print()
    print(f"  {cyan('data Acc (<) (a:A) where')}")
    print(f"  {cyan('  acc : (∀b. b < a → Acc (<) b) → Acc (<) a')}")
    print()
    print(wrap(
        "Acc is itself a W-type! A proof of Acc(<)(a) is a well-founded tree "
        "of proofs: to show a is accessible, provide accessibility proofs for "
        "all b < a, which in turn need proofs for all c < b, etc. "
        "Since < is well-founded, this tree is finite."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Well-founded recursion on (ℕ, <):'))}\n")
    print(wrap(
        "Given a proof that < on ℕ is well-founded, we can define functions "
        "by well-founded recursion — not just structural recursion on the "
        "nat constructors, but any recursion where the argument strictly decreases."
    ))
    print()
    print(f"  {bold('Example: Ackermann function')}")
    print(f"  {cyan('ack : ℕ → ℕ → ℕ')}")
    print(f"  {cyan('ack 0 n     = n + 1')}")
    print(f"  {cyan('ack (m+1) 0 = ack m 1')}")
    print(f"  {cyan('ack (m+1) (n+1) = ack m (ack (m+1) n)')}")
    print()

    def ack(m, n):
        if m == 0: return n + 1
        if n == 0: return ack(m-1, 1)
        return ack(m-1, ack(m, n-1))

    print(f"  {bold('Values:')}")
    for m in range(4):
        vals = []
        for n in range(5):
            try:
                v = ack(m, n)
                vals.append(str(v) if v < 10000 else "...")
            except RecursionError:
                vals.append("∞")
        print(f"  ack({m}, ·) = {', '.join(vals)}")

    print()
    print(wrap(
        "The Ackermann function is NOT primitive recursive (it grows faster "
        "than any primitive recursive function) but it IS defined by "
        "well-founded recursion on the lexicographic order on ℕ×ℕ. "
        "W-types capture exactly this generalized recursion."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("def",      "W-types: the definition and eliminator",       _section_definition),
    ("nat",      "ℕ as W(Bool, B): natural numbers",            _section_nat),
    ("list",     "List A as a W-type",                          _section_list),
    ("tree",     "Binary trees and the shape/position pattern", _section_tree),
    ("ordinals", "Ordinals as a W-type: transfinite induction", _section_ordinals),
    ("acc",      "Accessibility and well-founded recursion",    _section_accessibility),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("W-Types: The Universal Inductive Type", width=70))
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
