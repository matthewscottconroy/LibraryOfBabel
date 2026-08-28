# Locks and Their Cost

The last lesson broke something. This one fixes it, three different ways, and then
measures what each fix costs — because the cheapest of the three is free and the
most obvious is eleven times more expensive.

There is also a way for locks to fail that has no error message, no CPU usage, and
no indication that anything is wrong. The program stops, and stays stopped.

Sometimes you cannot avoid sharing. Two threads, one counter, and no way to give
each its own.

What you need then is a promise that only one of them is inside the dangerous part
at a time — **mutual exclusion**, and Java hands it to you in one keyword.

```java
synchronized (lock) {
    guarded++;
}
```

Every object in Java carries a **monitor** — a lock nobody asked for, sitting
unused on every object you have ever made. Entering a `synchronized` block takes
it. Leaving gives it back, including when you leave by throwing. A second thread
arriving while it is held stops at the door.

The method form locks `this`:

```java
public synchronized void deposit(long amount) { ... }   // locks this
```

That is convenient and slightly dangerous, because `this` is a public object and
any other code can lock on it too. A private lock object is safer:

```java
private final Object lock = new Object();
```

## What you are actually being promised

Ask most people what a lock does and you get one answer. There are two, and the
second is the one that makes the first worth anything.

**Mutual exclusion.** One thread at a time.

**Visibility.** Everything a thread wrote before releasing the lock is visible to
the next thread that acquires it. Without this, mutual exclusion alone would be
useless: you would take turns and still read stale values.

That pairing has a name — **happens-before** — and the Java Memory Model is
essentially a document about when it holds.

The practical consequence catches people out, so it is worth stating flatly:
**every access to shared state must be synchronized, reads included.** Not just
the writes. A read taken outside the lock is entitled to see a value from before
somebody else's write, no matter how scrupulous the writers were being.

## The cost

Verified on this machine, eight threads incrementing:

```
1 thread, unshared local : 0 ms
8 threads, AtomicInteger : 17 ms
8 threads, synchronized  : 193 ms
```

Three conclusions.

**Unshared is free.** A local variable in one thread costs nothing measurable,
because there is no coordination at all.

**Atomics are cheap.** `AtomicInteger.incrementAndGet` compiles to a single
hardware compare-and-swap instruction. Seventeen milliseconds for 1.6 million
contended increments.

**Locks are eleven times more expensive than atomics here.** An uncontended lock is
nearly free — the JVM optimizes it heavily — but a *contended* one means threads
blocking, being descheduled by the operating system, and being woken again. Those
are kernel operations and they cost microseconds.

Which gives you the shape of the whole problem: **what costs you is contention,
not synchronization.** A lock nobody is waiting for is cheap. A lock eight threads are
fighting over serializes them, and the program can end up slower than
single-threaded — Amdahl's law arriving through a back door, since the locked
region is by definition sequential.

Which is why the practical advice is: **hold locks for as short a time as
possible**, and prefer designs where threads do not contend.

## The failure with no error message

Two threads. Two locks. Each takes them in the order that seemed natural where it
was written:

```java
// thread 1                 // thread 2
synchronized (a) {          synchronized (b) {
    synchronized (b) { }        synchronized (a) { }
}                           }
```

Thread 1 is holding `a` and waiting for `b`. Thread 2 is holding `b` and waiting
for `a`. Neither will ever move again.

And nothing happens. No exception, no stack trace, no CPU usage — the program is
not spinning, it is politely waiting, forever, and the only symptom is that it has
stopped doing anything at all.

The fix is a **lock ordering**: choose a global order and always acquire in it.
Verified — the same two threads acquiring `a` then `b` in both cases:

```
both acquired in the same order, both finished: true
```

That is the whole technique, and it is reliable if it is followed everywhere. The
difficulty is that "everywhere" includes code you call, which may take locks you
do not know about — which is why calling an unknown method while holding a lock is
a known hazard, and Chapter 30's toolkit deadlocks were exactly this.

Two related failures worth naming. **Livelock**, where threads keep responding to
each other and none progresses — two people stepping aside in a corridor.
And **starvation**, where a thread never gets the lock because others keep taking
it.

## The alternatives, in order

**Immutable data.** No writes, no locks, no deadlock. Chapter 20's records and
`List.of`. This is the first thing to try.

**Confinement.** One thread owns the data. Chapter 30's event dispatch thread, and
`ThreadLocal` for the general case.

**Atomics.** `AtomicInteger`, `AtomicLong`, `AtomicReference`. A single variable
updated atomically without a lock, at the cost measured above. `LongAdder` is
faster still under heavy contention because it keeps per-thread cells and sums
them on demand.

**Concurrent collections.** `ConcurrentHashMap`, `CopyOnWriteArrayList`,
`BlockingQueue`. Verified:

```
expected 80000, got 80000
```

Eight threads calling `merge` on one `ConcurrentHashMap` key, correct. These are
written by specialists, they lock at a fine granularity or not at all, and they
should be preferred to `synchronized (map)` in every case.

Note that a concurrent collection makes each *operation* atomic and not your
*sequence* of operations. `if (!map.containsKey(k)) map.put(k, v)` is still a
race; `putIfAbsent` and `merge` and `computeIfAbsent` exist because of it.

**Message passing.** Threads own their data and communicate through a queue,
sharing nothing. `BlockingQueue` is the mechanism, and this is the model Erlang
and Go are built on. It is the most robust approach available and it is
underused in Java.

## Executors

You rarely create threads directly.

```java
try (ExecutorService pool = Executors.newFixedThreadPool(4)) {
    Future<Integer> f = pool.submit(() -> expensive());
    Integer result = f.get();          // waits
}
```

Verified — five tasks of 100 ms each, on four threads:

```
[1, 4, 9, 16, 25] in 201 ms
```

Two hundred milliseconds rather than five hundred: four tasks ran together, then
the fifth. That is the pool doing exactly what it should, and it is also a small
demonstration of why pool size matters.

An `ExecutorService` separates *what to run* from *what runs it*, which is Chapter
23's separation applied to threads. A `Future` is a result that does not exist
yet; `get()` blocks until it does. `CompletableFuture` composes them without
blocking, which is worth knowing about and less necessary than it was.

Since Java 21, `Executors.newVirtualThreadPerTaskExecutor()` gives one virtual
thread per task, and the pool-size question mostly disappears — Section 31.1.1's
ten thousand blocked threads in 146 milliseconds.

## The summary

**Sharing mutable state between threads is the problem.** Everything else is a
technique for managing it, and every technique costs something.

The order to try:

1. Do not share. Confine, or divide the data.
2. Share only immutable data.
3. Use a concurrent collection or an atomic.
4. Use a lock, held briefly, acquired in a consistent order.
5. Write your own synchronization only if you are certain, which you should not be.

Most concurrency bugs are the result of starting at 4.

Next: the case where nothing is shared because the other program is on another
machine.
