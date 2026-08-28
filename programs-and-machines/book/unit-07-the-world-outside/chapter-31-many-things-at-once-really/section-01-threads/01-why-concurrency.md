# Why Concurrency

For about thirty years, a program written today ran faster next year without
anybody touching a line of it. Clock speeds doubled roughly every two years. You
could ship something slow, do nothing at all, and be vindicated by the hardware.

People called it the free lunch, and planned around it. Then, around 2005, it
stopped.

It did not stop because engineers ran out of ideas. It stopped because of physics,
and what the manufacturers did instead is the reason this chapter exists — it took
a problem that had always belonged to hardware and moved it onto your desk.

Before any of the mechanics, one distinction. There are **two quite different
reasons** to want concurrency, and most of the confused advice you will ever read
about it comes from somebody who has run the two together.

## Reason one: staying responsive

This is the one Chapter 30 was built on. A program doing something slow has to
remain capable of doing something else — answering a click, accepting a request,
redrawing a window.

Notice what the problem is *not* here. The work is not necessarily large. It is
**blocking**. A thread waiting on a network response is doing nothing whatsoever,
and while it sits there another thread could be getting on with something.

Nothing goes faster in this story. The program merely stops being stuck. That is
worth being clear about, because it is a completely different goal from the next
one.

This reason has been with us since the 1960s, and it is why operating systems have
threads at all.

## Reason two: getting more done

This is the newer one, and it is the one that changed how programs get written.

The free lunch had a physical cause and therefore a physical ending. Power
dissipation climbs sharply with clock frequency, and the heat became impossible — a
chip running at 10 GHz would need cooling that no consumer device could carry
around. Frequencies levelled off somewhere between 3 and 5 GHz and have sat there
for twenty years.

But the transistor budget kept growing. Manufacturers had transistors to spend and
no way left to spend them on speed. So they spent them on **cores**.

Here is this machine:

```
availableProcessors = 24
```

Twenty-four of them. Which means a single-threaded program running here is using
one twenty-fourth of the computer, and leaving the other twenty-three idle.

That is the whole change, in one sentence: **a sequential program does not get
faster any more, and using the rest of the machine is now your job.**

## Two words that get used interchangeably and should not

**Concurrency** is *structuring* a program as several independent activities. They
may run simultaneously and they may not. A single-core machine running four threads
is concurrent and not parallel — the operating system is interleaving them.

**Parallelism** is *executing* several things at the same instant, which requires
several processors.

Rob Pike's formulation is the one to carry: **concurrency is about dealing with
many things at once; parallelism is about doing many things at once.** Concurrency
is a property of your program's structure. Parallelism is a property of what
happens when it runs.

The distinction earns its keep because it separates the two reasons above.
Responsiveness needs concurrency and does not need parallelism at all — one core is
plenty. Throughput needs parallelism, and only gets it if the work genuinely comes
apart into independent pieces.

## Threads

A **thread** is an independent sequence of execution inside a process. Each one has
its own call stack — Chapter 12's — and its own program counter.

```java
Thread t = new Thread(() -> System.out.println("hello from another thread"));
t.start();
t.join();     // wait for it to finish
```

`start()` sets it going and returns immediately. `join()` waits for it.

Now, the most important sentence in this lesson, and it is about what is shared and
what is not.

Two threads have **separate stacks**, so their local variables are private. Nobody
else can see them, nobody else can touch them. This is the single most useful fact
in the chapter, and it is exactly why a pure function from Chapter 26 is
automatically safe to run on as many threads as you like.

Two threads **share the heap**, so objects and static fields are common property.

Every single problem in this chapter comes out of that second sentence. Keep both
of them in mind and much of what follows will feel less arbitrary.

## What a thread costs

A platform thread — the traditional kind, mapped one-to-one onto an operating
system thread — is not cheap. Each carries a stack, typically reserving a megabyte
of address space, and switching between them means a trip through the kernel.

So you cannot have a hundred thousand of them. That constraint shaped how servers
were written for decades: a pool of a few hundred threads, each handling one
request at a time, surrounded by elaborate machinery whose entire purpose was
avoiding ever blocking one.

Java 21 changed the arithmetic. **Virtual threads** are scheduled by the JVM rather
than the operating system, cost a few hundred bytes each, and unmount from their
carrier thread whenever they block.

Before you read the measurement, guess: ten thousand threads, each sleeping for a
tenth of a second. How long in total?

```
10,000 virtual threads each sleeping 100ms: 146 ms
```

A seventh of a second. With platform threads you would be asking for ten thousand
megabyte stacks; here the whole thing is a rounding error.

That changes the advice, and it changes it materially. All those asynchronous
programming models built to avoid blocking a scarce thread — callbacks, futures,
reactive streams — were solving a problem that virtual threads largely dissolve. **A
blocking call on a virtual thread is fine now**, and plain sequential code is once
again the right default.

What virtual threads do *not* change is anything at all in the next lesson. They
are cheaper threads. They are not safer ones.

## One piece of arithmetic before you get excited

Twenty-four cores suggests twenty-four times the speed. Let us find out.

If a fraction $p$ of a program can be parallelized and the rest cannot, the best
possible speedup on $n$ processors is

$$S(n) = \frac{1}{(1 - p) + \frac{p}{n}}$$

That is Gene Amdahl, 1967, and the consequence is unforgiving.

Put $p = 0.95$ into it — ninety-five percent of your program parallel, which is
very good going. As $n$ grows, the limit is $1/0.05 = 20$. Twenty times faster, and
that is the ceiling however many processors you buy. On this machine's 24 cores you
would see about 13.

Now try $p = 0.5$. The ceiling is 2.

**The sequential part dominates, and it dominates much sooner than intuition
suggests.** Which gives a practical rule worth following: before parallelizing
anything, find out what fraction of the time is actually spent in the part you can
divide.

Chapter 18's insistence on measuring first applies here with unusual force, because
concurrency does not only cost effort. It costs correctness — as the next lesson is
about to demonstrate — and paying in correctness for a 1.3-times speedup is a bad
trade in any currency.

Next: what goes wrong.
