# Blocking and Waiting

You send a request. Nothing comes back.

Did it arrive? Did it arrive, get processed, and lose its reply on the way home?
Is the server dead, or is it merely busy?

You cannot tell. Not with more effort, not with a better library, not with a
cleverer protocol — the information is not available to you at any price. Almost
everything in distributed systems is a technique for living with that sentence.

It follows from two facts about networks.

**Everything is slow.** Line the numbers up and let them land. A memory read takes
about a nanosecond. A disk read takes tens of microseconds. A network round trip
inside one data center takes half a millisecond, and a round trip across the world
takes about 150.

That last one is a hundred million times the memory read. And it is bounded below
by the speed of light, so nobody is going to improve it.

**Anything can fail, at any point, without telling you.** A cable is cut, a
machine reboots, a router drops packets. And the failure is often
*indistinguishable from slowness*: a request with no reply might mean the server
is dead, or busy, or the reply is on its way.

That second fact is the hard one and it is worth stating starkly: **you cannot
tell a slow machine from a dead one.** Every distributed system is built on
guesses about which it is.

## Blocking

```java
String line = in.readLine();       // returns when a line arrives. Or never.
Socket c = server.accept();        // returns when a client connects. Or never.
```

Blocking calls are easy to read — the code says what it does, in order — and each
one occupies a thread for as long as it waits.

For thirty years that was the central constraint on server design. Platform
threads are expensive, so a server could hold a few thousand at most, so a
thread-per-connection server could not scale, so the industry built elaborate
asynchronous machinery to avoid blocking.

Java 21's virtual threads mostly dissolve this. Section 31.1.1 measured ten
thousand blocked threads finishing in 146 milliseconds; a virtual thread that
blocks unmounts from its carrier and costs nothing while waiting.

So the modern advice is the older advice: **write blocking code, one thread per
task**, and let the runtime handle it. The asynchronous styles below are still
worth recognizing, and are less often necessary than they were.

## The alternatives, briefly

**Non-blocking I/O.** `java.nio`: a `Selector` watches many channels and reports
which are ready, so one thread serves thousands of connections. Efficient, and the
code inverts — Chapter 30's event loop, with sockets instead of clicks — and every
operation becomes a state machine. This is how servers were written between about
2002 and 2021.

**Futures and callbacks.** `CompletableFuture`, reactive streams. Composable, and
they produce the nesting Chapter 30 mentioned; stack traces stop naming the
logical sequence.

**async/await.** What C#, JavaScript, Rust and Python offer: asynchronous
execution written in sequential syntax, with the compiler transforming it. Java
chose virtual threads instead, on the argument that if blocking is cheap you do not
need new syntax.

## Timeouts

The single most important practical point in this lesson.

**Every network operation must have a timeout.** Without one, a failure that
should be an error becomes a hang — and a hang is worse, because it consumes a
resource forever and produces no diagnostic.

```java
socket.setSoTimeout(5000);                         // read timeout
socket.connect(address, 3000);                     // connect timeout
```

Two separate timeouts, and both are needed: connecting to an unreachable host and
waiting for a reply from a connected one are different failures.

Choosing values is genuinely hard. Too short and you fail requests that would have
succeeded, and — worse — you retry them, adding load to a system that is already
struggling. Too long and a stuck request holds a resource for minutes. The honest
answer is to measure your own latency distribution and set the timeout well above
the high percentile, then revisit it.

## The at-least-once problem

Now back to the question this lesson opened with, because it has a consequence we
have not drawn out yet.

The request may have been lost on the way out, or it may have been processed with
the reply lost on the way back. Both possibilities look identical from where you
are standing, and you have to do something anyway.

So you retry — and Section 28.2.1's warning arrives with real money attached. If
the operation is not **idempotent**, your retry may do it a second time. Retrying a
read costs nothing. Retrying "charge this card" charges the card again.

The standard fix is an **idempotency key**: the client generates a unique
identifier per logical operation, the server records which identifiers it has
processed, and a repeat is recognized and answered with the original result. Every
payment API you will meet works this way, and now you know why the parameter is
there.

The general statement: **a network gives you at-least-once or at-most-once, and
never exactly-once.** Exactly-once is achieved, when it is achieved, by making
at-least-once safe — which is idempotency, not a delivery guarantee.

## Failure modes to design for

The list is short and it is the checklist worth carrying.

**Slow.** Timeouts, and a bound on concurrent requests so that slowness does not
become memory exhaustion.

**Down.** Retry with backoff — bounded, exponential, and with jitter so that a
thousand clients do not retry in unison. And a **circuit breaker**: after enough
consecutive failures, stop trying for a while, so that a struggling service is not
finished off by its own clients.

**Wrong.** A reply that is malformed, truncated, or from a version you do not
recognize. Validate what arrives; never trust the far end, which may be running
older code or may not be who you think.

**Partially applied.** The operation half happened. This is Chapter 29's
interrupted write across a network, and it has no clean local fix — which is why
distributed transactions are hard and why most systems choose idempotency and
reconciliation instead.

## The theory, briefly

Two results worth knowing by name, because they explain why this area has no
tidy answers.

**The CAP theorem** — Brewer, 1998. When the network partitions, a distributed
system may preserve consistency or availability, not both. Since partitions happen
whether you plan for them or not, the real choice is which to sacrifice when they
do. The theorem is frequently overstated; what it rules out is narrower than the
popular version, and reading the actual statement is worthwhile.

**The Byzantine Generals problem** — Lamport, Shostak and Pease, 1982. Agreement
among parties that communicate over an unreliable network, some of whom may lie.
The result is a bound on how many faulty parties can be tolerated, and it is the
foundation of both fault-tolerant systems and blockchain consensus.

Neither is something you need in order to write a socket program. Both are worth
knowing exist, because they mark the point at which "just retry" stops being an
adequate answer.

## The end of the unit

Unit VII asked what happens when a program stops being a closed system.

Chapter 28: the world produces conditions your code did not, and a failure is a
value that travels. Chapter 29: what you write outlives you, so a format is a
promise. Chapter 30: the user acts in an order you did not choose, so control
inverts. Chapter 31: several things happen genuinely at once, so the assumption
that a value stays put is gone.

One thread runs through all four. **The program's own guarantees end at its
boundary**, and everything past that boundary is an assumption you are making —
that the file exists, that the encoding matches, that the reply will come, that
nobody else is writing.

The skill is not eliminating those assumptions, which is impossible. It is knowing
which ones you are making, checking the ones you can, and failing usefully on the
rest.

Unit VIII closes the book by asking what a program costs and what no program can
do at all.
