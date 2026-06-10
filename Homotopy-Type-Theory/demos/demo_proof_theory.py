#!/usr/bin/env python3
"""
Proof Theory: Judgments, Natural Deduction, and Sequent Calculus
=================================================================
The formal study of PROOFS as mathematical objects — their structure,
transformations, and computational content.
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


def _section_judgments():
    clear()
    print(box("Judgments in Type Theory"))
    print()
    print(wrap(
        "A JUDGMENT is a basic assertion in a formal system — what we "
        "claim to be true. In Martin-Löf Type Theory, judgments are the "
        "atomic building blocks from which proofs are constructed."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The four basic judgments of MLTT:'))}\n")

    judgments = [
        ("A type",        "A is a well-formed type",
         "ℕ type,  Bool type,  A→B type"),
        ("A = B type",    "A and B are definitionally equal types",
         "ℕ = ℕ type,  (A→B→C) = (A→(B→C)) type"),
        ("a : A",         "a is a term of type A  (a is a proof of A)",
         "0 : ℕ,  true : Bool,  λx.x : A→A"),
        ("a = b : A",     "a and b are definitionally equal terms of type A",
         "2+3 = 5 : ℕ,  (λx.x)(0) = 0 : ℕ"),
    ]

    for form, meaning, examples in judgments:
        print(f"  {bold(cyan(form))}")
        print(f"    Meaning:  {meaning}")
        print(f"    Examples: {dim(examples)}")
        print()

    print(rule())
    print(f"\n  {bold(green('Judgment forms with context:'))}\n")
    print(f"  {cyan('Γ ⊢ A type')}")
    print(f"  {cyan('Γ ⊢ a : A')}")
    print(f"  {cyan('Γ ⊢ a = b : A')}")
    print()
    print(f"  {dim('where Γ = x₁:A₁, x₂:A₂(x₁), ..., xₙ:Aₙ(x₁,...,xₙ₋₁)')}")
    print(f"  {dim('is a CONTEXT — a telescope of typed variables')}")
    print()
    print(wrap(
        "The context records which variables are in scope and their types. "
        "Dependent types arise because later types can depend on earlier terms. "
        "The context is what makes dependent type theory dependent."
    ))
    print()
    input(bold("  Press Enter to continue... "))


def _section_natural_deduction():
    clear()
    print(box("Natural Deduction"))
    print()
    print(wrap(
        "Natural deduction (Gentzen 1935) is a proof system that mirrors "
        "the natural way mathematicians argue. Each connective has "
        "INTRODUCTION rules (how to prove it) and ELIMINATION rules "
        "(how to use it)."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('Rules for conjunction (∧):'))}\n")
    print(f"  {cyan('  ∧-intro:    A    B')}")
    print(f"  {cyan('             ─────     (pair up two proofs)')}")
    print(f"  {cyan('              A∧B')}")
    print()
    print(f"  {cyan('  ∧-elim-L:  A∧B')}")
    print(f"  {cyan('             ───        ∧-elim-R:  A∧B')}")
    print(f"  {cyan('              A                     ───')}")
    print(f"  {cyan('                                    B')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Rules for implication (→):'))}\n")
    print(f"  {cyan('  →-intro:   [A]')}")
    print(f"  {cyan('              ⋮')}")
    print(f"  {cyan('              B          (discharge assumption A)')}")
    print(f"  {cyan('             ─────')}")
    print(f"  {cyan('             A → B')}")
    print()
    print(f"  {cyan('  →-elim (modus ponens):  A→B    A')}")
    print(f"  {cyan('                           ─────────')}")
    print(f"  {cyan('                               B')}")
    print()
    print(rule())
    print(f"\n  {bold(green('All connectives — intro and elim:'))}\n")
    rules = [
        ("⊤-intro",  "────",    "I",         "(the trivial proof)"),
        ("⊥-elim",   "⊥",       "────",      "(ex falso quodlibet — anything from False)"),
        ("∧-intro",  "A  B",    "A∧B",       "(pair)"),
        ("∧-elim",   "A∧B",     "A or B",    "(project)"),
        ("∨-intro",  "A",       "A∨B or B∨A","(inject)"),
        ("∨-elim",   "A∨B  [A]→C  [B]→C", "C", "(case split)"),
        ("→-intro",  "[A]...B", "A→B",       "(lambda)"),
        ("→-elim",   "A→B  A",  "B",         "(apply)"),
        ("∀-intro",  "[x:A]...P(x)", "∀x.P(x)", "(generalize)"),
        ("∀-elim",   "∀x.P(x)  t:A", "P(t)", "(specialize)"),
        ("∃-intro",  "t:A  P(t)", "∃x.P(x)", "(witness)"),
        ("∃-elim",   "∃x.P(x)  [x,P(x)]→C", "C", "(extract)"),
    ]
    print(f"  {'Rule':12}  {'Premises':25}  {'Conclusion':12}  {'Note'}")
    print(f"  {dim('─'*68)}")
    for rule_name, premises, conclusion, note in rules:
        print(f"  {bold(yellow(rule_name)):20} {dim(premises):30} {cyan(conclusion):18} {dim(note)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_sequent():
    clear()
    print(box("Sequent Calculus"))
    print()
    print(wrap(
        "Sequent calculus (Gentzen 1935) is an alternative to natural deduction "
        "that is symmetric between hypotheses and conclusions. A SEQUENT is "
        "written Γ ⊢ Δ meaning: assuming all of Γ, at least one of Δ holds."
    ))
    print()
    print(f"  {cyan('Γ ⊢ Δ  means:  if all of Γ hold, then some of Δ holds')}")
    print(f"  {dim('(In intuitionistic LJ: Δ has at most one formula)')}")
    print(f"  {dim('(In classical LK: Δ can have multiple — disjunction)')}")
    print()
    print(rule())
    print(f"\n  {bold(green('The structural rules:'))}\n")

    structural = [
        ("Weakening",   "Γ ⊢ Δ  →  Γ,A ⊢ Δ   or   Γ ⊢ A,Δ",
         "add unused hypotheses/conclusions"),
        ("Contraction", "Γ,A,A ⊢ Δ  →  Γ,A ⊢ Δ",
         "merge duplicate hypotheses"),
        ("Exchange",    "...,A,B,... ⊢ Δ  →  ...,B,A,... ⊢ Δ",
         "reorder — handled by multisets"),
        ("Cut",         "Γ ⊢ A,Δ   Γ,A ⊢ Δ   →  Γ ⊢ Δ",
         "USE a lemma — the central structural rule"),
    ]
    for name, form, note in structural:
        print(f"  {bold(cyan(name)):16} {form}")
        print(f"    {dim(note)}")
        print()

    print(rule())
    print(f"\n  {bold(green('Logical rules (examples):'))}\n")
    print(f"  {cyan('∧-L:  Γ,A,B ⊢ Δ       ∧-R:  Γ ⊢ A,Δ   Γ ⊢ B,Δ')}")
    print(f"  {cyan('      ───────────             ─────────────────────')}")
    print(f"  {cyan('      Γ,A∧B ⊢ Δ               Γ ⊢ A∧B,Δ')}")
    print()
    print(f"  {cyan('→-L:  Γ ⊢ A,Δ   Γ,B ⊢ Δ   →-R:  Γ,A ⊢ B,Δ')}")
    print(f"  {cyan('      ─────────────────────        ──────────────')}")
    print(f"  {cyan('      Γ,A→B ⊢ Δ                   Γ ⊢ A→B,Δ')}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_cut_elimination():
    clear()
    print(box("Cut Elimination"))
    print()
    print(wrap(
        "The CUT RULE is like using a lemma: prove A, then use A to prove C. "
        "Gentzen's HAUPTSATZ (main theorem) says: any proof using cut can be "
        "transformed into a cut-free proof. This is one of the deepest "
        "theorems in proof theory."
    ))
    print()
    print(f"  {cyan('Cut rule: if Γ ⊢ A  and  Γ,A ⊢ C  then  Γ ⊢ C')}")
    print()
    print(rule())
    print(f"\n  {bold(green('Why cut elimination matters:'))}\n")
    consequences = [
        ("Subformula property",
         "Cut-free proofs only contain subformulas of the conclusion.",
         "This gives PROOF SEARCH — we know what to look for."),
        ("Consistency",
         "A cut-free proof of ⊥ would require ⊥ as a subformula.",
         "Since ⊥ has no introduction rule, no cut-free proof of ⊥ exists."),
        ("Decidability",
         "For many systems, cut-free proof search is decidable.",
         "This gives decision procedures for provability."),
        ("Normalization",
         "In natural deduction, corresponds to beta-reduction.",
         "Curry-Howard: cut = redex, cut-elimination = reduction."),
    ]
    for name, what, why in consequences:
        print(f"  {bold(cyan(name))}")
        print(f"    {what}")
        print(f"    {dim(why)}")
        print()
    print(rule())
    print(f"\n  {bold(yellow('Proof-theoretic ordinals:'))}\n")
    print(wrap(
        "The strength of a formal system can be measured by how long "
        "cut-elimination can take (termination ordinal) or which ordinals "
        "the system can prove well-founded:"
    ))
    print()
    ordinals = [
        ("PA",               "ε₀",        "Peano arithmetic"),
        ("ACA₀",            "ε_0",        "arithmetic comprehension"),
        ("ATR₀",            "Γ₀",         "arithmetical transfinite recursion"),
        ("Π¹₁-CA₀",         "ψ(Ωω)",      "projective comprehension"),
        ("ZFC",              "much larger","much stronger"),
    ]
    print(f"  {'System':20}  {'Ordinal':15}  {'Notes'}")
    print(f"  {dim('─'*55)}")
    for sys, ordinal, note in ordinals:
        print(f"  {bold(yellow(sys)):28}  {cyan(ordinal):23}  {dim(note)}")
    print()
    input(bold("  Press Enter to continue... "))


def _section_curry_howard():
    clear()
    print(box("Curry-Howard: Proofs Are Programs"))
    print()
    print(wrap(
        "The Curry-Howard correspondence is the observation that PROOF SYSTEMS "
        "and TYPE SYSTEMS for programming languages are the SAME THING. "
        "A proof is a program; its type is the proposition it proves; "
        "proof normalization is program execution."
    ))
    print()
    print(rule())
    print(f"\n  {bold(green('The dictionary:'))}\n")
    print(f"  {'Logic':35}  {'Type Theory / Programming'}")
    print(f"  {dim('─'*65)}")
    dictionary = [
        ("Proposition A",          "Type A"),
        ("Proof of A",             "Term (program) of type A"),
        ("A ∧ B",                  "Product type A × B"),
        ("A ∨ B",                  "Sum type A + B"),
        ("A → B",                  "Function type A → B"),
        ("⊤ (True)",               "Unit type 𝟙"),
        ("⊥ (False)",              "Empty type 𝟘"),
        ("∀x:A. P(x)",             "Π(x:A). P x  (dependent function)"),
        ("∃x:A. P(x)",             "Σ(x:A). P x  (dependent pair)"),
        ("Cut rule",               "Let binding / substitution"),
        ("Cut elimination",        "Beta reduction (computation)"),
        ("Normal proof",           "Normal form (value)"),
        ("Natural deduction",      "Simply-typed lambda calculus"),
        ("Sequent calculus",       "Continuation-passing style"),
        ("Classical logic",        "Call/cc (control operators)"),
        ("Linear logic",           "Linear types (no duplication)"),
        ("Modal logic",            "Staged / modal type theory"),
    ]
    for logic, type_th in dictionary:
        print(f"  {cyan(logic):35}  {yellow(type_th)}")
    print()
    input(bold("  Press Enter to return to menu... "))


SECTIONS = [
    ("judgments", "Judgments in type theory",                 _section_judgments),
    ("nd",        "Natural deduction",                        _section_natural_deduction),
    ("sequent",   "Sequent calculus",                         _section_sequent),
    ("cut",       "Cut elimination and its consequences",     _section_cut_elimination),
    ("ch",        "Curry-Howard: proofs are programs",        _section_curry_howard),
]

def main():
    idx = 0
    while True:
        clear()
        print(box("Proof Theory: Judgments and Formal Proofs", width=70))
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
