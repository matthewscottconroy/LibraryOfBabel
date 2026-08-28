# Shared State and Races

Eight threads. Each adds one to a shared counter, two hundred thousand times.

The right answer is 1,600,000. Three runs of the same program gave 414,984, then
649,224, then 799,209 — none of them right, none of them the same, and no error
reported anywhere.

The cause is one line of code that looks like a single action and is not.

Here are those three runs in full:

```
expected 1600000, got  414984
expected 1600000, got  649224
expected 1600000, got  799209
```

Three runs, three answers, none of them right, and one of them lost three quarters
of the increments.

## Why

`plain++` looks like one thing. It is three:

```
read  the current value from memory into a register
add   one
write the register back to memory
```

Two threads can interleave between any of those steps:

```
thread A: read  plain      -> 100
thread B: read  plain      -> 100
thread A: add   1          -> 101
thread B: add   1          -> 101
thread A: write 101
thread B: write 101
```

Two increments, one recorded. The other is lost, silently, with no error and no
way to detect it after the fact.

This is a **race condition**: the result depends on the relative timing of
operations whose order nobody specified. The operation is not **atomic** — it
cannot be observed half-done from the outside, and `plain++` can.

The scale of the loss in the measurement above is not exaggeration. With eight
threads on 24 cores all hammering one memory location, most increments collide.

## Why it is worse than it looks

Three things make this harder than the interleaving story suggests.

**It is not deterministic.** Three runs gave three answers. A test that passes
proves nothing, and a bug that appears once a week cannot be reproduced on demand.

**A debugger changes it.** Stepping through slows one thread enormously and the
interleaving disappears. Print statements do the same. The observation changes the
behaviour, which is why these bugs are diagnosed by reasoning rather than by
watching.

**The processor and compiler reorder.** Both are permitted to execute your
statements out of order, and to keep values in registers or per-core caches rather
than writing them to main memory. So a value one thread writes may not be visible
to another for an unbounded time — and this is a separate problem from the
interleaving, called **visibility**.

## Visibility, and why volatile is not enough

```java
static volatile int volatileOnly = 0;
```

`volatile` addresses visibility: a write is immediately visible to other threads,
and reordering around it is restricted.

It does **not** make an operation atomic. Verified:

```
expected 1600000, got 533401
```

Still wrong, and by about as much. `volatileOnly++` is still read-add-write; each
step now sees fresh memory, and the interleaving is unchanged.

The rule: **`volatile` is for visibility of a single write, never for
read-modify-write.** Its legitimate use is a flag one thread sets and another
polls:

```java
private volatile boolean stopped = false;     // correct use
```

Without `volatile` there, the reading thread may never see the change — the JIT is
entitled to hoist the read out of the loop, and a loop checking a non-volatile flag
can spin forever.

## The three fixes

**A lock**, so that only one thread is inside at a time:

```java
synchronized (lock) { guarded++; }
```

Verified: `expected 1600000, got 1600000`.

**An atomic**, where the hardware performs read-modify-write as one instruction:

```java
atomic.incrementAndGet();
```

Verified: `expected 1600000, got 1600000`.

**Do not share it.** Each thread counts into a local variable and the totals are
combined at the end. Locals are on the thread's own stack, so no synchronization is
needed at all, and this is both the fastest and the simplest answer.

The third is the one to reach for. `reduce` from Chapter 26 is exactly this shape:
an accumulator per worker, combined by an associative operator, which is why the
pattern parallelizes and why the counter lambda did not.

## What counts as shared

Anything reachable from more than one thread:

- **static fields** — reachable from everywhere, which is why Section 23.1.3 called
  global mutable state the strongest coupling there is
- **object fields**, if two threads hold the reference
- **collections**, if the reference is shared
- **arrays**, likewise
- anything a lambda **captured** — Chapter 26's array escape hatch, and precisely
  what broke the parallel counter there

And what is not shared:

- **local variables of primitive type**, always
- **immutable objects** — no writes, so no race. Chapter 20's argument arriving as
  a correctness requirement rather than a preference
- objects reachable from only one thread — **confinement**

## The three ways to be safe

Everything in this chapter reduces to one of these.

**Do not share.** Confinement: give each thread its own data. The event dispatch
thread of Chapter 30 is this — components are confined to one thread, which is why
the toolkit needs no locks.

**Share only immutable data.** A `record`, a `String`, a `List.of`. No writes means
no races, and this requires no discipline from readers because there is nothing to
get wrong.

**Synchronize.** Locks, atomics, concurrent collections. Necessary sometimes,
costly always, and the source of every deadlock.

They are in order of preference and the ordering is not close. Most concurrency
bugs are the result of reaching for the third when the first or second would have
worked.

## A word about testing

You cannot test a race away. A concurrency bug that appears in one run in ten
thousand will pass any test suite you have the patience to run, and the absence of
a failure is not evidence.

What does work: reason about which state is shared, keep the amount of it small
enough to enumerate, and use tools built for the job — Java's `jcstress` harness,
thread sanitizers, and static analysers that find unsynchronized access.

And the strongest technique remains the structural one. **A program with no shared
mutable state cannot have a race**, and that property can be established by
reading rather than by testing.

Next: what synchronization costs and how it fails.
