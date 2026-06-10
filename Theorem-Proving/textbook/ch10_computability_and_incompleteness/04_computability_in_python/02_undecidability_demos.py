"""Demonstrations of undecidability using Python.
These are not proofs (Python is not a proof assistant) but executable illustrations
of the key ideas in computability theory.
"""
import inspect
import types

# =====================================================
# The Halting Problem: No General Halt Detector Exists
# =====================================================

# Suppose halts(f, x) were a function that returns True iff f(x) terminates.
# We can derive a contradiction:

def diagonal(f):
    """If halts(f, f) is True, loop forever. If False, return.

    This is the diagonalization argument made executable.
    (In reality we cannot implement halts -- this just shows the logical structure.)
    """
    if hypothetical_halts(f, f):
        while True:  # loop forever
            pass
    else:
        return  # terminate

# If halts existed and worked:
# - diagonal(diagonal) must either halt or not halt.
# - If diagonal(diagonal) halts: halts(diagonal, diagonal) = True
#   so diagonal runs forever -- contradiction.
# - If diagonal(diagonal) doesn't halt: halts(diagonal, diagonal) = False
#   so diagonal returns -- contradiction.
# Therefore hypothetical_halts cannot exist.

# =====================================================
# Busy Beaver: A Non-Computable Function
# =====================================================

# BB(n) = the maximum number of steps an n-state Turing machine can make before halting.
# Known values: BB(1)=1, BB(2)=6, BB(3)=21, BB(4)=107, BB(5)>47,000,000
# BB grows faster than any computable function.
# It is not computable (if it were, we could solve the halting problem).

KNOWN_BUSY_BEAVER = {1: 1, 2: 6, 3: 21, 4: 107}

def busy_beaver_lower_bound(n: int) -> int:
    """Returns a lower bound on BB(n) from known results."""
    return KNOWN_BUSY_BEAVER.get(n, -1)

# =====================================================
# Rice's Theorem: No Non-Trivial Semantic Property is Decidable
# =====================================================

# Rice's theorem: For any non-trivial property P of partial computable functions,
# there is no algorithm that decides whether a given program computes a function with P.
#
# "Non-trivial" means: some programs have P, some don't.
# "Semantic" means: depends only on the function computed, not the code.

# Examples of undecidable properties (by Rice's theorem):
# - Does program P halt on all inputs?
# - Does program P compute the identity function?
# - Does program P ever output 42?
# - Does program P have any fixed points?

# The proof works by reduction to the halting problem.
# Suppose decide_P(code) decides property P.
# Use decide_P to build halts_on_empty(code) (whether code halts on input 0).
# Then decide_P decides the halting problem -- contradiction.

# =====================================================
# Reduction: Halting -> Acceptance
# =====================================================

# Many undecidable problems reduce to each other.
# The acceptance problem (does M accept w?) is Turing-complete.

# Reduction from halting to acceptance:
# Given (M, w) asking "does M halt on w?",
# build M' that: runs M on w; if M halts (accepts or rejects), M' accepts.
# Then M halts on w ⟺ M' accepts w.
# Since acceptance is undecidable, so is halting. (And vice versa.)

# =====================================================
# Decidable vs Undecidable: Examples
# =====================================================

DECIDABLE = [
    "Is this string a palindrome?",
    "Is this number prime?",
    "Does this regular expression match this string?",
    "Is this propositional formula satisfiable? (SAT -- decidable, just NP-hard)",
    "Does this context-free grammar generate the empty language?",
    "Is this linear arithmetic formula valid? (Presburger arithmetic)",
]

UNDECIDABLE = [
    "Does this Python program halt on input 0?",
    "Does this program ever print 'hello'?",
    "Are these two programs equivalent?",
    "Does this grammar generate all strings? (Post Correspondence Problem reduces here)",
    "Is this Diophantine equation solvable? (Hilbert's 10th problem -- Matiyasevich 1970)",
    "Is this first-order formula valid? (FOL validity -- Gödel's incompleteness shadow)",
]

# =====================================================
# The Arithmetical Hierarchy
# =====================================================

# Σ₁ sets: enumerable by a Turing machine (r.e.)
# Π₁ sets: co-r.e. (complement is Σ₁)
# Δ₁ = Σ₁ ∩ Π₁: decidable

# The halting set K = {(M, w) | M halts on w} is Σ₁-complete.
# Its complement K̄ is Π₁ but not Σ₁.
# K is r.e. (enumerate halting computations) but not decidable.

# At each level Σₙ, Πₙ, Δₙ the hierarchy is strict --
# problems exist at every level that are not at lower levels.
# This mirrors the polynomial hierarchy in computational complexity.
