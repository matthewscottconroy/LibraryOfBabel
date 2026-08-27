# Important Concepts

**An exception is a value that travels** — constructed at the point of failure,
carrying information about it, moving up the call stack until something handles
it. Chapter 12's stack is what it travels along.

**The sentinel problem** — a failure code steals a value from the result space, so
`parse("-1")` and `parse("oops")` become indistinguishable. Structural, not a
defect of one example.

**The four non-exception alternatives** — sentinel value, `null`, a status flag,
and a result object. The first three make failure easy to ignore; the fourth is
genuinely good and is what Rust and Go do.

**The propagation problem** — with return-based failure, every method between the
detection and the handling must check and forward. Exceptions decouple where a
failure is detected from where it is handled, which is the argument for them in
one sentence.

**What exceptions cost** — control flow becomes invisible at the call site, and
`catch (Exception e)` makes it easy to swallow things you did not anticipate.

**When not to throw** — for an expected outcome, for flow control, or for
something the caller could cheaply have checked. The test: could a careful caller
have avoided this?

**throw and catch** — clauses are tried in order and the first match wins, so the
most specific must come first. Everything between the throw and the catch is
abandoned.

**Exceptions should carry data** — a handler that parses the message string to
learn the shortfall is a handler you have failed. Messages are for humans, fields
are for code.

**The hierarchy** — `Throwable` splits into `Error` (the JVM is in trouble; do not
catch) and `Exception`, which splits into `RuntimeException` (unchecked) and
everything else (checked).

**finally** — runs on the way out however the block is left. Never `return` from
it; doing so discards an in-flight exception.

**try-with-resources** — closes automatically, in reverse order, before the
handler runs, with the body's exception winning and the close failure attached as
suppressed.

**The stack trace** — captured when the exception is *constructed*, by walking the
live frames. Read top-down for where it broke, downward for how you got there;
the most useful line is the topmost one in your own code.

**Chaining** — pass the original as the cause when rethrowing. Its absence is the
reason for a great many unproductive afternoons.

**Checked versus unchecked** — checked exceptions must be declared or handled, and
Java is the only mainstream language with them. Intended rule: checked for
recoverable conditions, unchecked for programming errors.

**Why checked exceptions failed** — callers often cannot act usefully, the
declaration becomes part of an interface's permanent contract, and the pressure on
a programmer who cannot handle one is toward catching and ignoring.

**The verdict** — no language designed after Java adopted them, Java's own newer
APIs use unchecked exceptions, and lambdas cannot carry checked ones at all. The
diagnosis was right and the remedy too rigid.

**UncheckedIOException** — exists in the JDK specifically to wrap `IOException`,
which is itself a comment on the feature.

**Catch where you can do something meaningful** — retry, fall back, report to a
user, record and continue, or add context and rethrow. If none applies, do not
catch.

**The three wrong places** — too low (destroying the caller's ability to
distinguish cases), too broad (collapsing distinct failures and swallowing bugs),
and everywhere (error handling outweighing logic).

**Boundaries** — error handling concentrated at the top of a request, a task, a
user action, or `main`. Between them, most code should not mention exceptions.

**Catch to add context** — a handler that adds the line number and file name and
rethrows with the cause is legitimate and undervalued.

**Retry loops need three things** — a bound, a backoff, and a rethrow of the final
failure. And one check before writing one: is the operation safe to repeat?

**Garbage collection does not release handles** — it reclaims memory, eventually,
and knows nothing about open files, sockets, or locks.

**Prefer a scope to a lifetime** — a resource acquired, used and released inside
one block needs no reasoning about control flow. One held in a field does.

**finalize and Cleaner** — the first is deprecated for removal; the second is a
backstop, not a mechanism. Nothing that must happen promptly should depend on
collection.

**Fail fast** — check assumptions at the earliest point where they can be checked.
Validating in the constructor means an invalid object never exists, and the
failure arrives with the supplier still on the stack.

**The empty catch block** — the failure happened, nothing recorded it, and the
evidence was destroyed at the moment it existed.

**Loud means three things** — something durable records it, someone can find out,
and the failure is not converted into a plausible wrong answer.

**The cost of throwing** — measured at about 355 nanoseconds, roughly a thousand
times a returned sentinel; and almost all of it is capturing the stack trace, not
the throw or the catch.

**Assertions are disabled by default** — for internal invariants during
development, never for validating input. Writing them and assuming they run is the
indefensible position.
