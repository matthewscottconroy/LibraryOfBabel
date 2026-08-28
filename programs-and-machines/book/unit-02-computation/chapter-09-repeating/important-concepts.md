# Key Concepts

**A loop is a backward jump.** A transition that consults state and, on one
outcome, sets the program counter to a place it has already been. The single
capability that separates a program whose running time is bounded by its length
from one that can run for a billion steps or forever.

**The three moving parts.** Initialization puts the state where the loop expects
it; the condition decides whether to continue; progress is what eventually makes
the condition false. Omit progress and the loop runs forever — with the machine
working perfectly.

**`while` is the primitive.** `for` and `do`-`while` both reduce to it. Every
repetition in Java is a conditional backward jump.

**`for` as abbreviation.** Gathers the three parts into one header, so they can be
checked at a glance, and scopes the counter to the loop. Use `for` when counting,
`while` when waiting for a condition — if the header would have blanks, use
`while`.

**`do`-`while`.** Tests at the bottom, so the body always runs at least once.
Uncommon; when you meet one, check the behavior was intended.

**The enhanced `for`.** `for (String s : items)`. No counter, no condition, no
progress — and therefore no possible off-by-one. When you do not need the index,
do not have one.

**`break` and `continue`.** Exit the loop, or skip to the next iteration. Both add
exits, so the reader can no longer assume the loop ends only when its condition
fails. `break` leaves only the innermost loop; for nests, prefer extracting a
method and using `return`.

**Nesting multiplies.** A loop over *n* inside a loop over *n* runs $n^{2}$ times.
Ten thousand items becomes a hundred million iterations. Whenever you nest, notice
that you have multiplied.

**Loop invariant.** A claim about the state that is true at the top of every
iteration. Almost always a statement about *the part already processed*, expressed
in terms of the loop variable — which is why `a[0..i-1]` recurs.

**The three obligations.** *Establishment*: the invariant is true when the loop is
first reached. *Preservation*: if true at the top of an iteration, still true at
the top of the next. *Termination*: combined with the exit condition, the
invariant gives the result you wanted.

**This is induction.** Establishment is the base case; preservation is the
inductive step; the loop supplies "for all *n*". A loop and a recursion are the
same idea justified the same way.

**Proof beats testing here.** The three obligations establish correctness for
*every* input. Testing samples a state space that Chapter 6 showed is far too
large to cover.

**Deriving from the invariant.** Decide the invariant first, then write
initialization that establishes it, a body that preserves it, and a condition
whose failure yields the answer. The loop comes out correct by construction.

**Termination is separate.** An invariant proves that *if* the loop stops the
answer is right. A loop can satisfy its invariant forever.

**Variant.** A non-negative whole number that strictly decreases each iteration.
Its existence proves termination and bounds the iteration count. Invariant plus
variant is **total correctness**.

**Termination can be genuinely hard.** The Collatz loop is six lines and whether
it always halts is an open problem posed in 1937. This previews Chapter 34: no
method decides halting for arbitrary programs.

**Off-by-one is a failed boundary claim.** Not carelessness — the code looks
right, which is why it was written. A range has two ends and four plausible
conventions, and the difference is one character.

**The fencepost.** 100 meters with a post every 10 meters needs eleven posts. Gaps
and posts differ by one.

**Half-open ranges.** Java includes the start and excludes the end. The count is
the difference; adjacent ranges join with no gap or overlap; the empty range is
natural. Follow the convention, because `length`, `size()`, and `substring`
already do.

**Test the edges.** Empty, one element, and the boundaries — that is where the
four range conventions differ, and therefore where bugs live.
