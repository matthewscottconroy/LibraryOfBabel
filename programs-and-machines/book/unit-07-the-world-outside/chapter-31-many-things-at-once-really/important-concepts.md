# Important Concepts

**Two reasons for concurrency** — responsiveness, which needs only interleaving,
and throughput, which needs several processors. Conflating them produces confused
advice.

**Clock speeds stopped rising around 2005** for a physical reason: power
dissipation. Transistors were spent on cores instead, so a sequential program no
longer gets faster and using the machine is now the programmer's problem.

**Concurrency versus parallelism** — dealing with many things at once, against
doing many at once. Concurrency is a program structure; parallelism is a runtime
property.

**A thread has its own stack and shares the heap** — so local variables are
private and objects and static fields are common. That division is where every
problem in this chapter comes from, and why a pure function is automatically safe.

**Virtual threads** — scheduled by the JVM, costing a few hundred bytes, unmounting
when they block. Measured: 10,000 blocked virtual threads in 146 ms. They make
blocking code the right default again; they do not make anything safer.

**Amdahl's law** — with a fraction $p$ parallelizable, the best speedup on $n$
processors is $1 / ((1-p) + p/n)$. At $p = 0.95$ the ceiling is 20 regardless of
how many cores you buy.

**A race condition** — the result depends on the relative timing of operations
whose order nobody specified. Measured: eight threads incrementing a shared
counter 1.6 million times produced 414,984, then 649,224, then 799,209.

**`x++` is three operations** — read, add, write — so two threads can interleave
between any of them and one increment is silently lost.

**Atomic** — an operation that cannot be observed half done. `x++` is not one.

**Why races are worse than the interleaving story** — they are not deterministic,
a debugger or a print statement changes them, and the compiler and processor may
reorder instructions and cache values per core.

**Visibility** — a value one thread writes may not be seen by another for an
unbounded time. A separate problem from interleaving.

**`volatile` is for visibility of a single write, never read-modify-write** —
measured still wrong at 533,401. Its legitimate use is a flag one thread sets and
another polls, where without it the JIT may hoist the read and the loop spins
forever.

**What counts as shared** — static fields, object fields with a shared reference,
collections, arrays, and anything a lambda captured. Not local primitives, not
immutable objects, not data reachable from one thread.

**The three ways to be safe, in order** — do not share; share only immutable data;
synchronize. Most concurrency bugs come from reaching for the third.

**A lock guarantees two things** — mutual exclusion and visibility. Which means
all access must be synchronized, reads included; a read outside the lock may see a
stale value.

**Happens-before** — the ordering the Java Memory Model specifies, and what makes
mutual exclusion useful rather than merely turn-taking.

**The cost is contention, not synchronization** — measured: 0 ms unshared, 17 ms
with `AtomicInteger`, 193 ms with `synchronized`, for the same 1.6 million
increments. An uncontended lock is nearly free; a contended one means kernel
transitions.

**Deadlock** — two threads holding one lock each and wanting the other. No error,
no CPU use, no indication. Fixed by acquiring locks in a consistent global order.

**Calling an unknown method while holding a lock is a hazard** — it may take locks
you do not know about, which is what Chapter 30's toolkit deadlocks were.

**Concurrent collections** make each operation atomic, not your sequence of them.
`putIfAbsent`, `merge` and `computeIfAbsent` exist for exactly that reason.

**Message passing** — threads own their data and communicate through a queue,
sharing nothing. The most robust approach available and underused in Java.

**Executors separate what to run from what runs it** — Chapter 23's separation
applied to threads. A `Future` is a result that does not exist yet.

**A socket is a pair of streams** — Chapter 29's abstraction unchanged, which is
why the hard part is not the API.

**TCP gives a stream of bytes, not messages** — ten writes may arrive as one read.
There is no `readMessage`, because TCP does not know what a message is.

**Framing** — how a receiver knows where a message ends: a delimiter, a length
prefix, or closing the connection. Omitting it produces a program that works on
localhost and fails on a real network.

**A protocol is an agreement** needing four things: framing, encoding, grammar,
and a sequence of states. The fourth is what a file format does not need.

**The end-to-end principle** — intelligence at the edges, a dumb network in the
middle, which is why a new protocol needs no permission from any router.

**Everything is slow** — a memory read is a nanosecond, a cross-world round trip is
150 milliseconds, and the latter is bounded below by the speed of light.

**You cannot tell a slow machine from a dead one** — the hardest fact in
distributed systems, and the reason every such system is built on guesses.

**Every network operation must have a timeout** — separate ones for connecting and
reading. Without them a failure becomes a hang, which consumes a resource forever
and produces no diagnostic.

**At-least-once or at-most-once, never exactly-once** — a request with no reply may
have been lost either going or returning. Exactly-once is achieved by making
at-least-once safe, which is idempotency.

**Idempotency key** — a client-generated identifier per logical operation, so the
server recognizes a repeat. Why every payment API has that parameter.

**Failure modes to design for** — slow, down, wrong, and partially applied. Retry
with bounded exponential backoff and jitter, and a circuit breaker so that
struggling services are not finished off by their clients.

**CAP and Byzantine Generals** — worth knowing by name as the point at which
"just retry" stops being adequate.
