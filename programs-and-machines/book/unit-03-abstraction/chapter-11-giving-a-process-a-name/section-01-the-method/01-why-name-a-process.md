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

The machine does identical work. The same values move through the same registers
in the same order; if you traced both, the traces would match. Nothing has become
faster and nothing has become possible that was impossible before.

So what did we buy?

## Four things, and reuse is the least of them

Ask a room of programmers and almost everyone says **reuse** first. And it is
true: if you need the largest element in three places, you now write one loop
instead of three. But if that were the whole story, methods would be a
convenience, and they are not a convenience. They are the thing the rest of this
book is built on.

Here is what else happened.

**You gave a reader a unit of thought.** Look again at the four-line version and
notice what it demands of you. To find out what it accomplishes, you have to *run
it in your head* — take the first element, walk the rest, keep the bigger one.
Ten seconds, maybe. Now look at the named version. You read a word.

Those ten seconds do not sound like much until you remember that they are spent
by every person who ever reads this code, including you next March, and that they
are spent on something you already solved.

**You made a place to be correct once.** That loop has an off-by-one lurking in it
— start `i` at 0 instead of 1 and it still works, start `best` at 0 instead of
`scores[0]` and it quietly returns 0 for an array of negatives. Write it three
times and you have three chances to make that mistake and three places to fix it
when you find it. Write it once and it is one place, and once it is right it
stays right.

**You drew a wall.** Inside `largest`, there is a variable called `i` and a
variable called `best`. Outside, there are not. Not "outside, you shouldn't use
them" — outside, they do not exist, and no amount of carelessness can reach them.
Narrow scope was advice a few chapters ago. A method makes it a fact.

## The name is the abstraction

Now change one thing. Call the method `process`.

```java
int best = process(scores);
```

Every benefit above just evaporated except reuse. You cannot read that line; you
have to go and look. The wall is still there, the correctness is still in one
place, and the unit of thought has become a unit of homework.

Which is the thing to take from this lesson, and it is stronger than it sounds:

> **Naming is not labeling something you already built. It is the point of
> building it.**

A method whose purpose you cannot say in a few words is usually a method that has
no single purpose — and that is a fact about the method, not about your
vocabulary.

So here is a test you can run on anything you write, and it takes four seconds:

> Can I say what this does in one sentence, without using the word "and"?

If the sentence needs an "and", the method is probably doing two things.
Sometimes that is fine and honest — `readAndValidate` may genuinely be one
operation with a compound name. But usually the "and" is a seam, and it is much
cheaper to notice it now than to find it in eight months when you need one half of
the method without the other.

## Watch what becomes possible

Once `largest` exists and you trust it:

```java
int best = largest(scores);
int worst = smallest(scores);
double spread = best - worst;
```

Three lines. You understood all of it on the first pass without slowing down.
Written out in full it would be a dozen lines of nearly identical loops, and the
*purpose* — we are measuring how spread out these are — would be somewhere in the
middle of them, waiting to be reconstructed.

But there is something better going on here than brevity. You just thought about
spread **without thinking about loops at all.** The loop is a solved problem,
sealed behind a name, and your attention — which is the scarce thing, always — was
free for the question you actually cared about.

That is what abstraction is for. The limit on what you can program has never been
what the machine can compute; it is what a person can hold in mind at once. A
method is how you take something out of your head and put it somewhere you can
rely on without holding it.

## What it costs

None of this is free, and pretending otherwise is how you end up with a program
made of two hundred tiny methods that nobody can read.

**Every name is a lookup.** To find out what `largest` really does, you have to go
somewhere else. If the name is good you never need to — but if it is not, you have
added a journey to every attempt at understanding.

**A wrong abstraction is worse than none.** Suppose `largest` returns the largest
value *except* on an empty array, where it returns 0. Now the name is a lie. The
reader has to know the name *and* the exception, which is more to carry than they
had before you helped.

**Layers compound.** A call seven deep, each level doing one small thing, can be
harder to follow than the flat version, because understanding any of it means
holding the whole chain at once.

The judgment you are looking for is a level where each name matches something a
reader would already think of as a single operation. There is no formula for it.
There are heuristics coming in Chapter 14, and the honest answer is that you learn
it by writing a lot of code and noticing, months later, which of your decompositions
you were grateful for.

Next: how information gets in.
