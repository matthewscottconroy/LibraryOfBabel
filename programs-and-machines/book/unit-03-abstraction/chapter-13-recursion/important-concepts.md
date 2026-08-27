# Key Concepts

**Recursion.** A method that calls itself. Unremarkable mechanically: Chapter 12
established that frames belong to *executions*, so two executions of one method
are as separate as two of different methods.

**Base case.** The input answered directly, without recursing. Write it first: it
settles the method's domain and its termination argument before you need either.

**Recursive case.** The answer expressed in terms of a *smaller* instance of the
same problem.

**The termination requirement.** Every recursive call must move strictly closer to
a base case, and a base case must be reachable from every accepted input. This is
Chapter 9's variant argument — the shrinking argument is the decreasing
non-negative quantity.

**Missing or unreachable base case** produces `StackOverflowError`. A base case
that exists but is never approached fails the same way.

**Trust the recursive call.** Do not trace. Assume the call returns the correct
answer for its smaller input, and check only that you build the right answer for
this one from it. Two sentences, regardless of input size.

**The leap.** The discomfort is that you call a method that does not yet work. The
framing that helps: pretend it is a competent colleague's method, then notice the
framing was accurate.

**Why the trust is legitimate.** It is mathematical induction. The base case is the
base case, the recursive case is the inductive step, and the assumption is the
induction hypothesis. Writing a correct recursive method is constructing a proof —
which is why the argument is two steps long.

**Loops and recursion are the same principle.** Invariant ↔ contract;
establishment ↔ base case; preservation ↔ recursive case; variant ↔ shrinking
argument. Either converts to the other; the choice is about which makes your
problem's structure visible.

**Structural recursion.** When a data structure is defined recursively — a tree is
empty, or a value with two trees — a method over it has the same shape and the
correctness argument writes itself. The iterative version means maintaining by
hand the stack the machine already provides.

**Strong induction.** Assuming the claim for *all* smaller values, not just the
previous one. Corresponds to recursions that shrink irregularly, like `gcd` or
binary search.

**Linear recursion.** One recursive call per invocation. *n* calls, depth *n*.

**Tree recursion.** Several calls per invocation. Fine when the branches are
disjoint — `size(tree)` visits each node once. Catastrophic when they overlap:
`fib(10)` takes 177 calls, `fib(20)` 21,891, `fib(30)` 2,692,537. Growth is
exponential.

**The real problem is recomputation**, not branching. Ask of any tree recursion:
*do the branches overlap?* If they do, memoize or work upwards.

**Memoization.** Record answers already computed and consult the table first.
Turns the exponential Fibonacci into a linear one.

**Divide and conquer.** Split, solve the parts, combine. Branches are disjoint so
nothing is recomputed, and halving gives depth about $\log_{2} n$ — a million
elements in twenty levels.

**Recursive procedure versus recursive process.** The procedure is syntactic — the
text calls itself. The process is about pending work: whether the machine must
remember something at each level and finish it on the way back.

**Tail recursion.** The recursive call is the last thing the method does, with its
result returned unchanged. Nothing is pending, so the process is iterative and the
frame could be reused.

**Java does not eliminate tail calls.** The accumulator version still uses *n*
frames. The distinction remains worth knowing — it tells you the space cost in
principle and how mechanical the loop rewrite would be — but it buys no runtime
benefit in Java.

**Where the state lives.** Recursion keeps pending state implicitly on the stack;
iteration keeps it explicitly in variables. That is the whole difference.

**When a loop is better.** Straight walks through sequences; large or
user-controlled depth; when the recursion needs a bookkeeping parameter the caller
should not see; when branches overlap.

**When recursion is better.** Recursive data; problems that divide; backtracking,
where the stack handles undo for free; and when the recursive definition *is* the
specification.

**Elegance that helps** reveals the problem's structure. Elegance that does not is
cleverness admired for itself — the naive Fibonacci being the standard example:
beautiful, faithful to the definition, and unusable.
