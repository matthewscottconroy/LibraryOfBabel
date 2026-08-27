# Key Concepts

**Decomposition has no algorithm.** Two competent programmers will divide the same
problem differently and both may be right. What exists is heuristics, vocabulary,
and experience about which divisions people regret.

**Refactoring is normal.** You usually write something that works, notice its
structure, and reorganize. Waiting to see the right division before writing
anything is not how it goes.

**Seam signals.** Blank lines (the most reliable — you put them there to separate
ideas); comments labelling a section (a name looking for a method); repetition;
indentation depth; and difficulty naming without "and". A comment explaining
*what* is a method name; a comment explaining *why* is information and stays.

**What decomposition buys.** The caller reads names rather than executing steps
mentally; the pieces become independently testable; each has one place to be
fixed; and reuse, which is the least important.

**What it costs.** More things to find; relationships between the pieces become
implied rather than visible; and occasionally a lost opportunity to combine
passes.

**The judgment.** Would a reader of the *calling* code rather see a name or the
steps? Under-decomposed code is a wall of statements; over-decomposed code is a
maze where every question requires following a chain. Both are real failures.

**One job, properly stated.** A method does one job when it operates at a single
level of abstraction and a caller would think of it as one action. `mean` sums and
divides and still does one job; `saveAndEmail` does two.

**Single level of abstraction.** A method's body should read as steps at
comparable altitude. When one step is markedly more detailed than its neighbours,
it wants to be a method.

**Command–query separation.** A method should either *do* something or *answer*
something. A query can be called freely, twice or not at all; a command cannot.
Methods that do both force every call site to think about both.

**Length is a symptom.** The real questions are whether you can name it, whether
you can hold it in your head, and whether it is at one level. Fifty lines is worth
a pause, not a rule.

**Parameter count as signal.** Many parameters usually means many jobs — or a
group that always travels together and wants to become an object.

**Cohesion.** A unit is cohesive when its parts all serve one purpose; the tell for
its absence is that you can describe the parts separately without loss. Paired with
**coupling**; the aim is high cohesion, low coupling.

**A test is an executable claim.** Run the code, check the result. No framework is
needed to start; JUnit supplies the plumbing and changes nothing about what a test
is.

**A test is the contract, executed.** The comment says what should be true; the
test checks it. When the code changes, a comment stays silently wrong and a test
fails loudly.

**Why test.** It catches bugs now; it catches bugs *later*, which is the real
value; it makes change affordable, so improvement happens instead of workarounds;
and it forces you to call your own interface as a client.

**What to test.** Normal cases, boundaries, the contract's edges, and — most
valuable over time — a case for every bug you have already fixed.

**Equivalence classes.** Inputs fall into groups handled the same way. Test one
per group; more from the same group adds little. A suite's size matters less than
the number of distinct classes it covers.

**Boundary values.** Empty, one, two, duplicates, already-sorted, reverse-sorted,
type extremes, and null. Bugs cluster here because this is where the code's
decisions change.

**Test the failures too.** If the contract promises an exception, check that it is
thrown. Quietly returning garbage breaks the promise as surely as a wrong answer.

**Coverage is a diagnostic, not a target.** It tells you which lines ran, not
whether the assertions were meaningful. `divide(6,2)` gives full coverage and
misses division by zero. Teams that target coverage get tests that execute
everything and check nothing.

**How many tests.** Enough that you would be surprised if a change broke something
without a test failing.

**Tests as documentation.** The only documentation that cannot become wrong
silently. To learn an unfamiliar method, read its tests: they show the accepted
inputs, the results, the edge cases someone thought worth writing, and the failure
behavior.

**Test names are failure reports.** Name the case and the expected behavior; the
name is the first thing you see when it breaks.

**Arrange, act, assert.** Setup, the call under test, the check. Keeping them
separate is what stops tests becoming as hard to read as the code.

**One claim per test**, for the same diagnostic reason as one job per method — and
because a combined test stops at its first failure.

**Difficulty testing is a design signal.** Hard to test usually means: too many
jobs, an uncontrollable external dependency, elaborate setup implying tight
coupling, or no observable result. The test is the method's first client, and if
the first client finds it awkward, so will the rest.
