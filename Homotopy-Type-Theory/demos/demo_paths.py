#!/usr/bin/env python3
"""
demo_paths.py — Path Algebra in a User-Defined Space

Build a finite topological space as a directed graph, then:
  • Compose paths (concatenation)
  • Invert paths
  • Detect loops (elements of the fundamental group)
  • Check contractibility
  • Compute connected components
  • Explore the Seifert–van Kampen theorem informally

Concepts illustrated
  • Paths as sequences of edges
  • Path composition (·) and inversion (⁻¹)
  • Homotopy: two paths are homotopic if one can be deformed to the other
    (in a discrete space, homotopy = same endpoints + same sequence after
     canceling a · a⁻¹ subpaths)
  • Fundamental group at a base point
  • Contractible spaces (fundamental group = trivial)
  • Simply connected spaces

Commands
  p    pick a preset space
  a    add an edge
  d    delete an edge
  s    show current space
  c    compose two paths interactively
  i    invert a path
  l    list all loops at a base point
  k    check contractibility
  g    compute fundamental group generators (informal)
  e    explain a concept
  r    reset to empty space
  h    help
  q    quit
"""

from __future__ import annotations
import sys
import textwrap
from collections import defaultdict, deque

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

# ── Space (undirected graph, bidirectional edges) ────────────────────────────

class Space:
    def __init__(self, name: str = "Custom"):
        self.name = name
        self.edges: set[tuple[str, str]] = set()
        self.nodes: set[str] = set()

    def add_edge(self, a: str, b: str):
        self.nodes.add(a)
        self.nodes.add(b)
        key = tuple(sorted([a, b]))
        self.edges.add(key)

    def remove_edge(self, a: str, b: str):
        key = tuple(sorted([a, b]))
        self.edges.discard(key)
        # Remove isolated nodes
        connected = {x for e in self.edges for x in e}
        self.nodes = connected

    def neighbors(self, node: str) -> list[str]:
        result = []
        for e in self.edges:
            if e[0] == node:
                result.append(e[1])
            elif e[1] == node:
                result.append(e[0])
        return sorted(result)

    def has_edge(self, a: str, b: str) -> bool:
        return tuple(sorted([a, b])) in self.edges

    def all_paths(self, start: str, end: str, max_len: int = 6) -> list[list[str]]:
        """BFS/DFS for all simple paths from start to end."""
        result = []
        queue = deque([[start]])
        while queue:
            path = queue.popleft()
            if len(path) > max_len + 1:
                continue
            cur = path[-1]
            if cur == end and len(path) > 1:
                result.append(path)
            for nb in self.neighbors(cur):
                if nb not in path[1:]:  # allow revisiting start for loops
                    queue.append(path + [nb])
        return result

    def all_loops(self, base: str, max_len: int = 8) -> list[list[str]]:
        """All loops at base (start and end at base)."""
        result = []
        queue = deque([[base]])
        while queue:
            path = queue.popleft()
            if len(path) > max_len + 1:
                continue
            cur = path[-1]
            if cur == base and len(path) > 1:
                result.append(path)
                continue  # don't extend loops further
            for nb in self.neighbors(cur):
                if nb not in path[1:] or (nb == base and len(path) > 2):
                    queue.append(path + [nb])
        # Deduplicate (normalise)
        seen = set()
        unique = []
        for p in result:
            key = tuple(p)
            if key not in seen:
                seen.add(key)
                unique.append(p)
        return unique[:20]  # cap display

    def components(self) -> list[set[str]]:
        """Connected components via BFS."""
        remaining = set(self.nodes)
        comps = []
        while remaining:
            start = min(remaining)
            comp = set()
            q = deque([start])
            while q:
                n = q.popleft()
                if n in comp:
                    continue
                comp.add(n)
                for nb in self.neighbors(n):
                    if nb not in comp:
                        q.append(nb)
            comps.append(comp)
            remaining -= comp
        return comps

    def is_connected(self) -> bool:
        if not self.nodes:
            return True
        return len(self.components()) == 1

    def is_contractible(self) -> bool:
        """A finite graph is contractible iff it's a tree (connected, |E|=|V|-1)."""
        if not self.nodes:
            return True
        if not self.is_connected():
            return False
        return len(self.edges) == len(self.nodes) - 1

    def euler_char(self) -> int:
        """χ = V − E for a graph."""
        return len(self.nodes) - len(self.edges)

    def fundamental_group_description(self, base: str) -> str:
        """Informal description of π₁."""
        if not self.is_connected():
            return "undefined (space not connected)"
        if self.is_contractible():
            return "trivial (1)  — the space is contractible"
        # Count independent cycles: β₁ = E − V + 1
        beta1 = len(self.edges) - len(self.nodes) + 1
        if beta1 == 0:
            return "trivial (1)"
        elif beta1 == 1:
            return "ℤ  — one independent loop (like S¹)"
        elif beta1 == 2:
            return "ℤ * ℤ  — two independent loops (free group of rank 2)"
        else:
            return f"free group of rank {beta1}  (F_{beta1})"

# ── Preset spaces ──────────────────────────────────────────────────────────

PRESETS: list[dict] = [
    {
        "name": "Point (⊤)",
        "desc": "A single point. Contractible. π₁ = 1.",
        "edges": [],
        "nodes": ["*"],
    },
    {
        "name": "Interval [0,1]",
        "desc": "Two points connected by a path. Contractible. π₁ = 1.",
        "edges": [("0","1")],
    },
    {
        "name": "Triangle (S¹ approximation)",
        "desc": "Three vertices in a cycle. π₁ = ℤ  (like S¹).",
        "edges": [("A","B"), ("B","C"), ("C","A")],
    },
    {
        "name": "Square",
        "desc": "Four vertices in a cycle. π₁ = ℤ.",
        "edges": [("A","B"), ("B","C"), ("C","D"), ("D","A")],
    },
    {
        "name": "Filled triangle (disk D²)",
        "desc": "Triangle + diagonal. Contractible. π₁ = 1.",
        "edges": [("A","B"), ("B","C"), ("C","A"), ("A","C")],
    },
    {
        "name": "Figure 8 (bouquet of 2 circles)",
        "desc": "Two loops sharing a base point. π₁ = ℤ * ℤ (free group rank 2).",
        "edges": [("*","A"), ("A","*"), ("*","B"), ("B","C"), ("C","*")],
    },
    {
        "name": "Path graph (1-2-3-4)",
        "desc": "Four nodes in a line. Contractible (a tree). π₁ = 1.",
        "edges": [("1","2"), ("2","3"), ("3","4")],
    },
    {
        "name": "Star graph (K₁,₄)",
        "desc": "Center connected to 4 leaves. Contractible tree. π₁ = 1.",
        "edges": [("C","A"), ("C","B"), ("C","D"), ("C","E")],
    },
    {
        "name": "Theta graph (Θ)",
        "desc": "Two vertices with 3 parallel edges. π₁ = ℤ * ℤ (but abelianised ≅ ℤ²).",
        "edges": [("A","B"), ("A","B-2"), ("A","B-3"),
                  ("B","B-2"), ("B","B-3"), ("B-2","B-3")],
        # Actually theta is just two nodes with parallel paths:
    },
]

# Simpler theta:
PRESETS[8] = {
    "name": "Theta graph (Θ)",
    "desc": "Two endpoints A,B with 3 vertex-disjoint paths between them. π₁ = ℤ².",
    "edges": [("A","M1"), ("M1","B"), ("A","M2"), ("M2","B"), ("A","B")],
}

def _load_preset(space: Space, preset: dict):
    space.edges.clear()
    space.nodes.clear()
    space.name = preset["name"]
    if "nodes" in preset:
        for n in preset["nodes"]:
            space.nodes.add(n)
    for a, b in preset.get("edges", []):
        space.add_edge(a, b)

# ── ASCII art of the space ─────────────────────────────────────────────────

def _draw_space(space: Space) -> str:
    """Simple edge list drawing (no spatial layout)."""
    lines = []
    lines.append(bold(f"  Space: {cyan(space.name)}"))
    lines.append(f"  Nodes ({len(space.nodes)}): {', '.join(bold(n) for n in sorted(space.nodes))}")
    lines.append(f"  Edges ({len(space.edges)}): {', '.join(f'{yellow(a)}─{yellow(b)}' for a, b in sorted(space.edges))}")
    lines.append(f"  Connected: {green('yes') if space.is_connected() else red('no')}")
    lines.append(f"  Contractible: {green('yes') if space.is_contractible() else red('no')}")
    lines.append(f"  Euler char χ: {bold(str(space.euler_char()))}")
    if space.nodes:
        base = min(space.nodes)
        lines.append(f"  π₁({base}): {bold(yellow(space.fundamental_group_description(base)))}")
    return "\n".join(lines)

# ── Path operations ────────────────────────────────────────────────────────

def _path_str(path: list[str]) -> str:
    return " → ".join(bold(cyan(v)) for v in path)

def _compose_paths(p: list[str], q: list[str]) -> list[str] | None:
    """Compose p then q; requires p[-1] == q[0]."""
    if not p or not q:
        return None
    if p[-1] != q[0]:
        return None
    return p + q[1:]

def _invert_path(p: list[str]) -> list[str]:
    return list(reversed(p))

def _simplify_path(p: list[str]) -> list[str]:
    """Cancel a · a⁻¹ subpaths (backtracking)."""
    changed = True
    while changed:
        changed = False
        for i in range(len(p) - 1):
            if p[i] == p[i + 1]:  # same node twice in a row = stay put
                continue
            if i + 2 < len(p) and p[i] == p[i + 2]:
                # a → b → a: remove the backtrack
                p = p[:i + 1] + p[i + 3:]
                changed = True
                break
    return p

# ── Interactive modules ────────────────────────────────────────────────────

def _do_compose(space: Space):
    print()
    print(bold("  Path composition"))
    print(dim("  Enter two paths as space-separated node sequences."))
    print(dim(f"  Nodes: {sorted(space.nodes)}"))
    print()
    try:
        raw1 = input("  Path 1 (e.g. A B C): ").strip().split()
        raw2 = input("  Path 2 (e.g. C D E): ").strip().split()
    except (EOFError, KeyboardInterrupt):
        return

    # Validate nodes
    for n in raw1 + raw2:
        if n not in space.nodes:
            print(red(f"  Node '{n}' not in space."))
            input(dim("  Press Enter…"))
            return

    # Validate edges in path 1
    for i in range(len(raw1) - 1):
        if not space.has_edge(raw1[i], raw1[i + 1]):
            print(red(f"  No edge {raw1[i]}─{raw1[i+1]} in path 1."))
            input(dim("  Press Enter…"))
            return
    for i in range(len(raw2) - 1):
        if not space.has_edge(raw2[i], raw2[i + 1]):
            print(red(f"  No edge {raw2[i]}─{raw2[i+1]} in path 2."))
            input(dim("  Press Enter…"))
            return

    comp = _compose_paths(raw1, raw2)
    if comp is None:
        print(red(f"  Cannot compose: path 1 ends at {raw1[-1]}, path 2 starts at {raw2[0]}."))
        print(dim("  Paths must meet at a common vertex."))
    else:
        simp = _simplify_path(list(comp))
        print()
        print(f"  p = {_path_str(raw1)}")
        print(f"  q = {_path_str(raw2)}")
        print(f"  p · q = {_path_str(comp)}")
        if simp != comp:
            print(f"  simplified: {_path_str(simp)}")
        if simp[0] == simp[-1]:
            print(green(f"  ↑ This is a loop at {simp[0]}! An element of π₁."))
        if len(simp) == 1:
            print(cyan(f"  ↑ This simplifies to the constant path refl_{simp[0]}."))
    print()
    input(dim("  Press Enter…"))

def _do_invert(space: Space):
    print()
    print(bold("  Path inversion"))
    print(dim(f"  Nodes: {sorted(space.nodes)}"))
    try:
        raw = input("  Path (e.g. A B C): ").strip().split()
    except (EOFError, KeyboardInterrupt):
        return

    inv = _invert_path(raw)
    print(f"  p   = {_path_str(raw)}")
    print(f"  p⁻¹ = {_path_str(inv)}")
    comp = _compose_paths(raw, inv)
    if comp:
        simp = _simplify_path(comp)
        print(f"  p · p⁻¹ = {_path_str(comp)}")
        if len(simp) == 1:
            print(cyan(f"  Simplifies to refl_{simp[0]} ✓  (as expected in HoTT)"))
    print()
    input(dim("  Press Enter…"))

def _do_list_loops(space: Space):
    if not space.nodes:
        print(red("  No nodes in space."))
        input(dim("  Press Enter…"))
        return

    print()
    print(bold("  Loops at base point"))
    print(dim(f"  Nodes: {sorted(space.nodes)}"))
    try:
        base = input(f"  Base point (default: {min(space.nodes)}): ").strip()
    except (EOFError, KeyboardInterrupt):
        return
    if not base:
        base = min(space.nodes)
    if base not in space.nodes:
        print(red(f"  Node '{base}' not in space."))
        input(dim("  Press Enter…"))
        return

    loops = space.all_loops(base, max_len=6)
    print()
    if not loops:
        print(green(f"  No non-trivial loops at {base}."))
        print(dim("  The trivial loop refl is always present."))
        print(cyan(f"  π₁({base}) = 1  (trivial group)"))
    else:
        print(f"  Loops at {bold(cyan(base))} (up to length 6):\n")
        for i, loop in enumerate(loops[:15], 1):
            print(f"  {i:2d}.  {_path_str(loop)}")
        if len(loops) > 15:
            print(dim(f"  … and {len(loops)-15} more"))

    pi1 = space.fundamental_group_description(base)
    print()
    print(f"  {bold('π₁')}({bold(cyan(base))}) ≅ {bold(yellow(pi1))}")
    print()
    input(dim("  Press Enter…"))

def _do_contractibility(space: Space):
    print()
    print(bold("  Contractibility check"))
    print()

    if not space.nodes:
        print(dim("  Empty space is contractible (vacuously)."))
        input(dim("  Press Enter…"))
        return

    contr = space.is_contractible()
    conn  = space.is_connected()
    chi   = space.euler_char()
    beta1 = max(0, len(space.edges) - len(space.nodes) + 1)

    print(f"  Nodes (V): {len(space.nodes)}")
    print(f"  Edges (E): {len(space.edges)}")
    print(f"  Euler char χ = V − E = {chi}")
    print(f"  First Betti number β₁ = E − V + 1 = {beta1}")
    print()
    print(f"  Connected: {green('yes') if conn else red('no')}")
    print(f"  β₁ = 0 (tree-like): {green('yes') if beta1 == 0 else red('no')}")
    print()

    if contr:
        print(green("  ✓ CONTRACTIBLE"))
        print(dim("  This space can be continuously contracted to a point."))
        print(dim("  π₁ = 1, all πₙ = 0. It behaves like a single point."))
    else:
        print(red("  ✗ NOT CONTRACTIBLE"))
        if not conn:
            print(dim("  Reason: not connected (multiple components)."))
        elif beta1 > 0:
            print(dim(f"  Reason: {beta1} independent cycle(s) — non-trivial π₁."))
    print()
    print(wrap(
        "In HoTT, contractibility means: ∃ c : A, ∀ x : A, c = x. "
        "For a graph, this holds exactly when it's a tree "
        "(connected, acyclic = β₁ = 0).", width=72))
    print()
    input(dim("  Press Enter…"))

def _do_fundamental_group(space: Space):
    print()
    if not space.nodes:
        print(red("  Space is empty."))
        input(dim("  Press Enter…"))
        return

    base = min(space.nodes)
    pi1 = space.fundamental_group_description(base)
    beta1 = max(0, len(space.edges) - len(space.nodes) + 1)

    print(bold("  Fundamental Group (informal)"))
    print()
    print(f"  Space: {bold(cyan(space.name))}")
    print(f"  Base point: {bold(cyan(base))}")
    print()
    print(f"  {bold('π₁')}({space.name}, {base})  ≅  {bold(yellow(pi1))}")
    print()

    if beta1 == 0:
        print(wrap(
            "No independent cycles → fundamental group is trivial. "
            "The space is simply connected.", width=72))
    elif beta1 == 1:
        print(wrap(
            "One independent cycle → fundamental group is ℤ. "
            "This is the fundamental group of the circle S¹! "
            "Every loop at the base is classified by an integer "
            "(how many times it winds around the cycle).", width=72))
    elif beta1 == 2:
        print(wrap(
            "Two independent cycles → free group F₂ = ℤ * ℤ. "
            "Elements are words in a, b, a⁻¹, b⁻¹. "
            "This is the fundamental group of the figure-8 space.", width=72))
    else:
        print(wrap(
            f"β₁ = {beta1} independent cycles → free group F_{beta1}. "
            "By the Seifert–van Kampen theorem, the fundamental group of "
            "a graph is always a free group of rank β₁.", width=72))
    print()

    if space.is_connected() and beta1 > 0:
        print(bold("  Spanning tree generators:"))
        print(dim("  Choose a spanning tree T ⊂ space."))
        print(dim("  Each non-tree edge e gives one generator of π₁."))
        # Find spanning tree edges via BFS
        tree_edges = set()
        visited = {base}
        q = deque([base])
        while q:
            n = q.popleft()
            for nb in space.neighbors(n):
                if nb not in visited:
                    visited.add(nb)
                    tree_edges.add(tuple(sorted([n, nb])))
                    q.append(nb)
        non_tree = [e for e in space.edges if e not in tree_edges]
        for i, e in enumerate(non_tree, 1):
            print(f"  Generator {i}: loop through edge {yellow(e[0])}─{yellow(e[1])}")
    print()
    input(dim("  Press Enter…"))

EXPLANATIONS = {
    "path": (
        "Path",
        "A path in a space A from point a to point b is a term of type "
        "a =_A b (the identity type). In a graph, this is a sequence of "
        "adjacent vertices. In HoTT, paths are the fundamental morphisms "
        "of the ∞-groupoid structure on every type."
    ),
    "composition": (
        "Path Composition (·)",
        "If p : a = b and q : b = c, then p · q : a = c. "
        "Composition is associative up to homotopy (not definitionally). "
        "The constant path refl is the unit: p · refl = p = refl · p. "
        "In a graph, composition is concatenation of vertex sequences."
    ),
    "inversion": (
        "Path Inversion (⁻¹)",
        "If p : a = b, then p⁻¹ : b = a. "
        "p · p⁻¹ is homotopic to refl_a, and p⁻¹ · p to refl_b. "
        "In a graph, inversion reverses the sequence of vertices."
    ),
    "loop": (
        "Loop / Fundamental Group",
        "A loop at base b is a path p : b = b. The set of loops "
        "forms a group under composition: the fundamental group π₁(A, b). "
        "For the circle S¹, π₁(S¹) = ℤ. For a tree, π₁ = 1. "
        "For the figure-8, π₁ = ℤ * ℤ (the free group of rank 2)."
    ),
    "contractible": (
        "Contractible / Simply Connected",
        "A space is contractible if it can be continuously deformed to a point: "
        "∃ c, ∀ x, c = x. For graphs: contractible ↔ it's a tree (connected + no cycles). "
        "Simply connected means π₁ = 1 but not necessarily contractible "
        "(e.g., S² is simply connected but not contractible)."
    ),
    "homotopy": (
        "Homotopy of Paths",
        "Two paths p, q : a = b are homotopic (p ~ q) if there is a "
        "2-cell H : p = q (a path between paths). In a discrete graph "
        "(no 2-cells), two paths are homotopic iff one can be obtained "
        "from the other by canceling adjacent-backtrack pairs x → y → x. "
        "The fundamental group π₁ is paths modulo homotopy."
    ),
    "svan kampen": (
        "Seifert–van Kampen Theorem",
        "If a connected space X = U ∪ V where U, V are open connected "
        "subsets and U ∩ V is connected, then "
        "π₁(X) = π₁(U) *_{π₁(U∩V)} π₁(V)  (pushout of groups). "
        "In HoTT this is proved using the higher inductive type formulation. "
        "For graphs, it implies π₁(graph) is always a free group."
    ),
}

def _do_explain():
    print()
    print(bold("  Explain a concept:"))
    print(dim("  Type one of: path, composition, inversion, loop, contractible,"))
    print(dim("  homotopy, svan kampen"))
    print()
    try:
        key = input("  > ").strip().lower()
    except (EOFError, KeyboardInterrupt):
        return

    for k, (title, body) in EXPLANATIONS.items():
        if k in key or all(w in key for w in k.split()):
            clear()
            print(f"\n  {bold(title)}\n")
            print(wrap(body, width=72))
            print()
            input(dim("  Press Enter…"))
            return
    print(red("  Not found. Try: path, loop, contractible, homotopy, …"))
    input(dim("  Press Enter…"))

def _do_pick_preset(space: Space):
    clear()
    print(bold("\n  Preset Spaces\n"))
    for i, p in enumerate(PRESETS, 1):
        print(f"  {bold(str(i))}.  {cyan(p['name'])}")
        print(f"      {dim(p['desc'])}")
    print(f"\n  {dim('0. Cancel')}")
    print()
    try:
        choice = int(input("  > ").strip())
    except (ValueError, EOFError):
        return
    if 1 <= choice <= len(PRESETS):
        _load_preset(space, PRESETS[choice - 1])
        print(green(f"  Loaded: {PRESETS[choice-1]['name']}"))
        import time; time.sleep(0.5)

def _do_add_edge(space: Space):
    print()
    print(bold("  Add edge"))
    print(dim("  Enter two node names separated by space (e.g. 'A B')."))
    try:
        raw = input("  > ").strip().split()
    except (EOFError, KeyboardInterrupt):
        return
    if len(raw) != 2:
        print(red("  Need exactly two node names."))
        input(dim("  Press Enter…"))
        return
    a, b = raw
    space.add_edge(a, b)
    print(green(f"  Added edge {a}─{b}"))
    import time; time.sleep(0.3)

def _do_remove_edge(space: Space):
    print()
    print(bold("  Remove edge"))
    print(f"  Edges: {', '.join(f'{a}─{b}' for a,b in sorted(space.edges))}")
    try:
        raw = input("  > ").strip().split()
    except (EOFError, KeyboardInterrupt):
        return
    if len(raw) != 2:
        print(red("  Need exactly two node names."))
        input(dim("  Press Enter…"))
        return
    a, b = raw
    space.remove_edge(a, b)
    print(yellow(f"  Removed edge {a}─{b}"))
    import time; time.sleep(0.3)

def _help():
    clear()
    print(f"""
  {bold('Path Algebra Explorer — Commands')}

  {cyan('p')}   pick a preset space
  {cyan('a')}   add an edge (and nodes)
  {cyan('d')}   delete an edge
  {cyan('s')}   show current space
  {cyan('c')}   compose two paths interactively
  {cyan('i')}   invert a path
  {cyan('l')}   list all loops at a base point
  {cyan('k')}   check contractibility
  {cyan('g')}   compute fundamental group (informal)
  {cyan('e')}   explain a concept (path, loop, homotopy, …)
  {cyan('r')}   reset to empty space
  {cyan('h')}   this help
  {cyan('q')}   quit
""")
    input(dim("  Press Enter…"))

def main():
    space = Space("Custom (empty)")
    while True:
        clear()
        print(bold("\n  ╔════════════════════════════════════════════════════════╗"))
        print(bold(  "  ║            Path Algebra Explorer                       ║"))
        print(bold(  "  ╚════════════════════════════════════════════════════════╝\n"))
        print(_draw_space(space))
        print()
        print(dim("  [p] preset  [a] add edge  [d] del edge  [c] compose  [i] invert"))
        print(dim("  [l] loops   [k] contract  [g] π₁ group  [e] explain  [r] reset  [h] help  [q] quit"))
        print()

        try:
            cmd = input(bold("  > ")).strip().lower()
        except (EOFError, KeyboardInterrupt):
            break

        if cmd in ("q", "quit", "exit"):
            break
        elif cmd == "p":
            _do_pick_preset(space)
        elif cmd == "a":
            _do_add_edge(space)
        elif cmd == "d":
            _do_remove_edge(space)
        elif cmd == "s":
            print()
            print(_draw_space(space))
            input(dim("  Press Enter…"))
        elif cmd == "c":
            _do_compose(space)
        elif cmd == "i":
            _do_invert(space)
        elif cmd == "l":
            _do_list_loops(space)
        elif cmd == "k":
            _do_contractibility(space)
        elif cmd == "g":
            _do_fundamental_group(space)
        elif cmd == "e":
            _do_explain()
        elif cmd == "r":
            space = Space("Custom (empty)")
        elif cmd in ("h", "help", "?"):
            _help()


if __name__ == "__main__":
    clear()
    print(bold("""
  ╔════════════════════════════════════════════════════════╗
  ║           Path Algebra Explorer                        ║
  ╚════════════════════════════════════════════════════════╝
"""))
    print("""  Build a finite space as a graph, then explore its paths.

  Core operations from HoTT:
    p · q   — path composition  (needs p[-1] = q[0])
    p⁻¹     — path inversion    (reverses direction)
    refl    — constant path     (unit of composition)

  Fundamental group π₁(X, b):
    All loops at base point b, modulo homotopy.
    For a graph: always a free group of rank β₁ = E − V + 1.

  Try preset spaces to see familiar examples,
  or build your own by adding edges!

  Type 'p' for presets, 'h' for help, 'q' to quit.
""")
    input(dim("  Press Enter to start…"))
    main()
    print(f"\n  {dim('Paths are the fundamental morphisms of the ∞-groupoid of types.')}\n")
