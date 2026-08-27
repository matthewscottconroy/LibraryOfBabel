# Why Name a Process

Here is a fragment that computes something:

```java
int best = scores[0];
for (int i = 1; i < scores.length; i++) {
    if (scores[i] > best) best = scores[i];
}
```

And here is the same computation, named:

```java
int best = largest(scores);
```

The machine does identical work. Chapter 6 would say the sequence of states is
essentially unchanged. So what did we gain?

## Four things, and only one of them is reuse

**Reuse** is the answer everyone gives, and it is the least important. Yes, if you
need the largest element in three places you now write one loop instead of three.
Real, and if it were the only benefit, methods would matter much less than they
do.

**A unit of thought.** This is the big one. Before, a reader encountering those
four lines had to *execute them mentally* to find out what they accomplish. After,
they read a word. The reader's attention is freed for the code around it, which is
presumably what they came for.

**A place to be correct once.** The four-line loop has an off-by-one waiting in it
— Chapter 9's `int best = scores[0]` versus starting at 0. Written three times,
that is three chances to get it wrong and three places to fix it. Written once, it
is one place, and once it is right it stays right.

**A boundary for reasoning.** Inside `largest`, the variables `i` and `best` exist.
Outside, they do not. Chapter 7 argued that narrow scope limits what a reader must
track; a method is the strongest form of that, because it makes the internals
*unreachable* rather than merely inadvisable to touch.

## The name is the abstraction

A point that sounds like pedantry and is not.

The value of `largest(scores)` comes from the *name*. If the method were called
`process()` or `doIt()`, the reader would still have to go and read it, and every
benefit above evaporates except reuse.

So: **naming is not labeling a thing you already built. It is the point of
building it.** A method whose purpose you cannot name in a few words is usually a
method that does not have a single purpose — which means it should be two
methods, or a different one.

This gives you a genuinely useful test, and it is worth applying every time:

> Can I describe what this method does in one sentence, without using the word
> "and"?

If the sentence needs "and", the method probably does two things. Sometimes that
is fine — `readAndValidate` might be a coherent operation — but usually it is a
seam where the method should be split, and noticing it early is much cheaper than
noticing it later.

## What abstraction actually buys

Look at what you can do once `largest` exists and is trusted:

```java
int best = largest(scores);
int worst = smallest(scores);
double spread = best - worst;
```

Three lines, and a reader understands the whole thing at a glance. Written out,
it would be a dozen lines of loops among which the *purpose* would be difficult
to see.

More importantly: you can now think about the spread calculation **without
thinking about loops at all**. The loop is a solved problem, sealed behind a
name, and your attention is available for the thing you are actually doing.

That is the entire game. Chapter 6 said the constraint on programs is not what
the machine can compute but what a person can hold in mind. A method is a device
for taking something out of your head and putting it somewhere you can rely on it
without holding it.

## The cost

Abstraction is not free, and pretending otherwise produces the two-hundred-tiny-
methods program.

**Indirection.** To find out what `largest` does, you must go somewhere else. If
the name is good you never need to; if it is not, you have added a step to every
attempt at understanding.

**A wrong abstraction is worse than none.** If `largest` returns the largest
*except* when the array is empty, when it does something surprising, then the name
is a lie and the reader must know both the name and the exception. Now there is
more to track than before, not less.

**Too many layers.** Each layer costs a lookup. A call chain seven deep, where
each method does one small thing, can be harder to follow than the flat version —
because understanding any of it requires holding the whole chain.

The judgment is in finding the level where each name corresponds to something a
reader would naturally think of as one operation. There is no formula. Chapter 14
offers heuristics, and the honest summary is that it is learned by writing a lot
of code and noticing which decompositions you were grateful for six months later.

Next: how information gets in.
