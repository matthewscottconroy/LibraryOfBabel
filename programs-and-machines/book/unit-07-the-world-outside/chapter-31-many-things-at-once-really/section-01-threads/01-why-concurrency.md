# Why Concurrency

For about thirty years, a program written today ran faster next year without anyone
touching it. Clock speeds doubled roughly every two years and everyone planned
around it.

That stopped, around 2005, and it stopped for a reason from physics rather than
engineering. What the manufacturers did instead is why this chapter exists — and it
moved a problem that used to belong to hardware onto your desk.

There are two quite different reasons to want concurrency, and most of the confused
advice you will read about it comes from somebody running them together.

## Responsiveness

The first is the one Chapter 30 was built on. A program doing something slow must
remain able to do something else — respond to a click, accept a request, redraw a
window.

Here the work is not necessarily large; it is *blocking*. A thread waiting for a
network response is doing nothing at all, and while it waits another thread can
proceed. Nothing goes faster; the program merely stops being stuck.

This reason has existed since the 1960s, and it is why operating systems have
threads.

## Throughput

The second reason is newer, and it is the one that changed how programs are
written.

That free lunch had a physical cause, and it had a physical end. Power dissipation
rises sharply with clock frequency, and the heat became unmanageable — a chip at
10 GHz would need cooling that no consumer device can carry. Frequencies plateaued
around 3 to 5 GHz and have stayed there for twenty years.

Manufacturers had transistors to spend and no way to spend them on speed, so they
spent them on **cores**. Verified on this machine:

```
availableProcessors = 24
```

Twenty-four. A single-threaded program on this machine uses one twenty-fourth of
it.

That is the change in one sentence: **a sequential program no longer gets faster,
and using more of the machine is now the programmer's problem.**

## Concurrency and parallelism

Two words, often used interchangeably, and the distinction is worth having.

**Concurrency** is *structuring* a program as several independent activities. They
may or may not run simultaneously. A single-core machine running four threads is
concurrent and not parallel — the operating system interleaves them.

**Parallelism** is *executing* several things simultaneously, which requires
several processors.

Rob Pike's formulation, and it is the one to remember: **concurrency is about
dealing with many things at once; parallelism is about doing many things at
once.** Concurrency is a program structure; parallelism is a runtime property.

The distinction matters because it separates the two reasons above. Responsiveness
needs concurrency and not parallelism — one core suffices. Throughput needs
parallelism, and gets it only if the work genuinely divides.

## Threads

A **thread** is an independent sequence of execution within a process. Each has
its own call stack — Chapter 12's — and its own program counter. All threads in a
process share the heap, which is where every problem in this chapter comes from.

```java
Thread t = new Thread(() -> System.out.println("hello from another thread"));
t.start();
t.join();     // wait for it to finish
```

`start()` begins execution and returns immediately. `join()` waits.

Note what is shared and what is not. Two threads have separate stacks, so their
**local variables are private** — which is the single most useful fact in this
chapter, and the reason a pure function from Chapter 26 is automatically safe.
They share the heap, so **objects and static fields are common**, and that is
where the trouble lives.

## What it costs

A platform thread — the traditional kind, mapped one-to-one onto an operating
system thread — is not cheap. Each has a stack, typically reserving a megabyte of
address space, and switching between them costs a trip through the kernel.

So you cannot have a hundred thousand of them, which for decades shaped how
servers were written: a thread pool of a few hundred, each handling a request at a
time, with elaborate machinery to avoid blocking them.

Java 21 changed this. **Virtual threads** are scheduled by the JVM rather than the
operating system, cost a few hundred bytes, and unmount from their carrier thread
when they block.

Verified:

```
10,000 virtual threads each sleeping 100ms: 146 ms
```

Ten thousand threads, each blocking for a tenth of a second, finished in about a
seventh of a second in total. With platform threads that would need ten thousand
megabyte stacks; here it is a rounding error.

That changes the advice materially. The elaborate asynchronous programming models
built to avoid blocking a scarce thread — callbacks, futures, reactive streams —
were solving a problem that virtual threads mostly remove. **A blocking call on a
virtual thread is now fine**, and simple sequential code is once again the right
default.

What virtual threads do *not* change is anything in the next lesson. They are
cheaper threads, not safer ones.

## Amdahl's law

Before assuming that more cores means more speed, one piece of arithmetic.

If a fraction $p$ of a program can be parallelized and the rest cannot, the best
possible speedup on $n$ processors is

$$S(n) = \frac{1}{(1 - p) + \frac{p}{n}}$$

Gene Amdahl, 1967. The consequence is unforgiving. With $p = 0.95$ — ninety-five
percent parallel, which is very good — the limit as $n$ grows is $1/0.05 = 20$.
Twenty times, no matter how many processors you buy. On this machine's 24 cores
you would get about 13.

And with $p = 0.5$, the ceiling is 2.

**The sequential part dominates.** Which gives the practical rule: before
parallelizing, find out what fraction of the time is actually in the part you can
divide. Chapter 18's insistence on measuring first applies with particular force
here, because concurrency also costs correctness, and paying that for a 1.3-times
speedup is a bad trade.

Next: what goes wrong.
