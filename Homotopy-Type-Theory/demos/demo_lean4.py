#!/usr/bin/env python3
"""
Lean 4: Interactive Theorem Proving
====================================
Lean 4 is a dependent type theory proof assistant and programming
language. It is the primary tool for modern formalized mathematics.
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


def _section_basics():
    clear()
    print(box("Lean 4: Basics"))
    print()
    print(wrap(
        "Lean 4 is a functional programming language and interactive proof "
        "assistant based on dependent type theory. It is the successor to "
        "Lean 3 and is designed for large-scale formalization."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Types and terms:'))}\n")
    code = [
        ("-- Basic types",                   ""),
        ("def n : Nat := 42",                "-- a natural number"),
        ("def s : String := \"hello\"",      "-- a string"),
        ("def p : Prop := 1 + 1 = 2",        "-- a proposition (type)"),
        ("",                                  ""),
        ("-- Functions",                      ""),
        ("def double : Nat → Nat := fun n => n * 2", ""),
        ("def add : Nat → Nat → Nat := fun a b => a + b", ""),
        ("",                                  ""),
        ("-- Dependent types",                ""),
        ("def Vec (α : Type) (n : Nat) : Type := ...", ""),
        ("def head : Vec α (n+1) → α := ...", "-- length-indexed!"),
        ("",                                  ""),
        ("-- Propositions as types",          ""),
        ("theorem one_plus_one : 1 + 1 = 2 := rfl", "-- rfl = reflexivity"),
        ("example : ∀ n : Nat, n + 0 = n := fun n => ...", ""),
    ]
    for line, comment in code:
        if line == "":
            print()
        elif comment:
            print(f"  {cyan(line):50} {dim(comment)}")
        else:
            print(f"  {cyan(line)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_type_system():
    clear()
    print(box("The Lean 4 Type System"))
    print()
    print(wrap(
        "Lean 4 uses the Calculus of Constructions with universe polymorphism "
        "and inductive types. It is based on dependent type theory with "
        "a hierarchy of type universes."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Universe hierarchy:'))}\n")
    print(f"  {cyan('Type 0  (= Type)  -- the universe of ordinary types')}")
    print(f"  {cyan('Type 1            -- the universe of Type 0')}")
    print(f"  {cyan('Type 2            -- the universe of Type 1')}")
    print(f"  {cyan('Sort 0  (= Prop)  -- the impredicative universe of propositions')}")
    print()
    print(f"  {dim('Lean uses Prop for propositions (proof-irrelevant by default)')}")
    print(f"  {dim('Type u is the universe at universe level u')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Inductive types:'))}\n")
    print(f"  {cyan('inductive Nat : Type where')}")
    print(f"  {cyan('  | zero : Nat')}")
    print(f"  {cyan('  | succ : Nat → Nat')}")
    print()
    print(f"  {cyan('inductive List (α : Type u) : Type u where')}")
    print(f"  {cyan('  | nil  : List α')}")
    print(f"  {cyan('  | cons : α → List α → List α')}")
    print()
    print(f"  {cyan('inductive Fin : Nat → Type where')}")
    print(f"  {cyan('  | zero : Fin (n+1)')}")
    print(f"  {cyan('  | succ : Fin n → Fin (n+1)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Structures (product types):'))}\n")
    print(f"  {cyan('structure Point where')}")
    print(f"  {cyan('  x : Float')}")
    print(f"  {cyan('  y : Float')}")
    print()
    print(f"  {cyan('structure Group where')}")
    print(f"  {cyan('  carrier : Type')}")
    print(f"  {cyan('  mul     : carrier → carrier → carrier')}")
    print(f"  {cyan('  one     : carrier')}")
    print(f"  {cyan('  inv     : carrier → carrier')}")
    print(f"  {cyan('  ...     -- laws')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_tactics():
    clear()
    print(box("Tactic Proofs in Lean 4"))
    print()
    print(wrap(
        "Lean 4 has two proof modes: TERM mode (write the proof term directly) "
        "and TACTIC mode (use tactics to interactively build the proof). "
        "Tactics are the most practical approach for complex proofs."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Basic tactics:'))}\n")
    tactics = [
        ("rfl",        "prove a = a  (reflexivity)"),
        ("intro h",    "introduce hypothesis h (for ∀ or →)"),
        ("apply f",    "apply a function/theorem f to the goal"),
        ("exact e",    "close the goal with term e"),
        ("simp",       "simplify using simp lemmas"),
        ("ring",       "prove ring equalities automatically"),
        ("omega",      "decide linear arithmetic over ℤ/ℕ"),
        ("linarith",   "decide linear arithmetic with hypotheses"),
        ("cases h",    "case split on inductive hypothesis h"),
        ("induction n","induct on n"),
        ("constructor","split a conjunction/exists goal"),
        ("left/right", "choose which side of a disjunction to prove"),
        ("use t",      "provide witness t for an exists goal"),
        ("have h:P",   "assert intermediate claim P (opens new goal)"),
        ("calc",       "chain equalities/inequalities"),
        ("decide",     "evaluate a decidable proposition"),
    ]
    print(f"  {'Tactic':16}  {'Purpose'}")
    print(f"  {dim('─'*55)}")
    for tactic, purpose in tactics:
        print(f"  {bold(cyan(tactic)):24} {dim(purpose)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Example proof:'))}\n")
    print(f"  {cyan('theorem add_comm (m n : Nat) : m + n = n + m := by')}")
    print(f"  {cyan('  induction m with')}")
    print(f"  {cyan('  | zero => simp')}")
    print(f"  {cyan('  | succ k ih =>')}")
    print(f"  {cyan('    rw [Nat.succ_add, ih, Nat.add_succ]')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_mathlib():
    clear()
    print(box("Mathlib: The Mathematical Library"))
    print()
    print(wrap(
        "MATHLIB is the main mathematics library for Lean 4. It contains "
        "formalized mathematics across many fields and is the largest "
        "formal mathematics library in existence."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('What is in Mathlib (selected):'))}\n")
    content = [
        ("Algebra",      "Groups, rings, fields, modules, algebras, categories"),
        ("Analysis",     "Metric spaces, topology, measure theory, Fourier"),
        ("Number theory","Primes, modular arithmetic, Diophantine equations"),
        ("Geometry",     "Euclidean geometry, differential geometry"),
        ("Combinatorics","Graphs, matroids, probabilistic method"),
        ("Logic",        "Set theory, model theory, computability"),
        ("Topology",     "Topological spaces, manifolds, fibrations"),
        ("Category theory","Functors, adjunctions, monoidal categories"),
        ("Group cohomology","Group homology, spectral sequences"),
        ("Linear algebra","Matrices, determinants, eigenvalues, tensors"),
    ]
    for area, topics in content:
        print(f"  {bold(cyan(area)):18} {dim(topics)}")
    print()
    print(rule())
    print(f"\n  {bold(green('Using Mathlib:'))}\n")
    print(f"  {cyan('-- In lakefile.lean:')}")
    print(f"  {cyan('require mathlib from git \"https://github.com/leanprover-community/mathlib4\"')}")
    print()
    print(f"  {cyan('-- In your file:')}")
    print(f"  {cyan('import Mathlib')}")
    print(f"  {cyan('-- or import specific modules:')}")
    print(f"  {cyan('import Mathlib.Topology.Basic')}")
    print(f"  {cyan('import Mathlib.GroupTheory.Subgroup.Basic')}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('Lean vs other provers:'))}\n")
    comparison = [
        ("Lean 4",   "general purpose, Mathlib, good automation"),
        ("Coq",      "CIC, SSReflect, strong dependently typed extraction"),
        ("Agda",     "cubical mode, HoTT, interactive type theory"),
        ("Isabelle", "Isar proof language, Archive of Formal Proofs"),
        ("HOL Light","high-confidence, proved Kepler conjecture with Flyspeck"),
    ]
    for name, desc in comparison:
        print(f"  {bold(yellow(name)):12} {dim(desc)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_hott_lean():
    clear()
    print(box("HoTT-Style Constructions in Lean 4"))
    print()
    print(wrap(
        "Lean 4 is based on the Calculus of Inductive Constructions — "
        "a different foundation than HoTT (no univalence). However, one "
        "can still do homotopy-type-theory-inspired mathematics in Lean 4."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('What Lean 4 has natively:'))}\n")
    native = [
        ("Dependent types",     "Π and Σ types fully supported"),
        ("Identity types",      "a = b  with rfl and J (Eq.rec)"),
        ("Propositional trunc.","Quot type: quotient by any relation"),
        ("Universe levels",     "Type u for all levels u"),
        ("Inductive types",     "full recursive datatypes with eliminators"),
        ("HITs (partial)",      "via Quot; can simulate some HITs"),
    ]
    for feature, desc in native:
        print(f"  {bold(cyan(feature)):25} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(green('What requires work or axioms:'))}\n")
    needs_work = [
        ("Univalence",          "NOT provable; can add as axiom"),
        ("Function ext.",       "NOT provable; available as funext axiom"),
        ("Propositional resiz.","NOT in Lean 4 by default"),
        ("Higher HITs",         "only quotients built in; S¹, Torus require hacks"),
        ("Synthetic homotopy",  "limited without full HoTT axioms"),
    ]
    for feature, desc in needs_work:
        print(f"  {bold(yellow(feature)):25} {dim(desc)}")
    print()
    print(rule())
    print(f"\n  {bold(yellow('The HoTT library for Lean 4:'))}\n")
    print(wrap(
        "There are ongoing projects to formalize HoTT in Lean 4 by adding "
        "univalence as an axiom. See 'lean4-hott' and related projects. "
        "For native HoTT formalization, Agda with cubical mode is currently "
        "the most mature option."
    ))
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("basics",  "Lean 4 basics: types and terms",    _section_basics),
    ("types",   "The type system and universes",      _section_type_system),
    ("tactics", "Tactic proofs",                      _section_tactics),
    ("mathlib", "Mathlib: the mathematical library",  _section_mathlib),
    ("hott",    "HoTT-style constructions in Lean 4", _section_hott_lean),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Lean 4: Interactive Theorem Proving", width=70))
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
