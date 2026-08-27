# Bisecting a Bug

Sometimes there is no error message. The program runs, produces an answer, and
the answer is wrong. Now what?

## The unproductive method

Look at the code, form a suspicion, change something, rerun. If the symptom
persists, change something else.

This has a name in other fields — it is a random search — and its problems are
worth naming because the method is so tempting.

It has **no memory**: nothing accumulates, and after an hour you know little more
than at the start. It **cannot terminate**: there is no point at which you have
excluded anything. And it **frequently masks rather than fixes**, because a change
that makes the symptom disappear is not thereby a change that removes the cause.

## Bisection

The productive method borrows from binary search.

The program starts in a state you believe correct — the inputs are what you think
they are. It ends in a state you know is wrong. Somewhere in between is the first
moment the state stopped matching your expectations.

**Look in the middle. Ask whether the state is still correct there.** If it is,
the divergence is in the second half. If not, it is in the first. Either way you
have halved the search.

Repeat. Each observation halves what is left, so a program of a thousand steps is
narrowed in about ten questions, and a million steps in about twenty. That is
Chapter 32's logarithm, arriving in a practical setting.

## Doing it

```java
int[] data = readData();          // is data what I expect here?
int[] cleaned = clean(data);      // is cleaned right?
int[] sorted = sort(cleaned);     // is sorted right?
double result = analyze(sorted);  // wrong answer comes out
```

Do not start at the top. Start in the middle: print `cleaned`, or check it in a
debugger.

If `cleaned` is right, the problem is in `sort` or `analyze` and you have
eliminated half the program without reading it. If `cleaned` is wrong, the
problem is in `readData` or `clean`.

Then bisect the half that is wrong, and keep going until you are inside a single
method, then a single loop, then a single line.

## What makes it work

**You must have an expectation at each point.** Bisection is not "look at values
and see if they seem odd". It is "I expect 500 elements sorted ascending; are
there 500 and are they sorted?" Without a prediction, an observation tells you
nothing.

This is why the technique fails for people who do not understand what their own
program is supposed to do at intermediate stages. If you cannot say what
`cleaned` should contain, you cannot check it, and the only remaining strategy is
guessing.

**Check what you are certain of.** The most common outcome of a good bisection is
discovering the input was not what you assumed — the file had a header row, the
list was already sorted, the array had one element. Beliefs you have not examined
are where bugs live, precisely because you have not examined them.

## Bisecting through time

The same technique applies to history rather than execution.

If the program worked last week and does not now, some change broke it. Version
control lets you check out an old state and test it, and bisecting over commits
finds the culprit in logarithmic time. A hundred commits, seven checkouts.

Git has this built in as `git bisect`, which will drive the process: you tell it
a good revision and a bad one, and it walks you through the halvings. Appendix D
covers version control; the point here is that the *method* is the same one, and
recognizing that is what lets you apply it in a new setting.

## Minimizing

A relative of bisection, and often the fastest route.

Rather than narrowing where in the program, narrow **what input** provokes the
failure. Start from the failing case and remove things: fewer records, shorter
strings, fewer options. After each removal, check the bug still occurs. If it
stops, put the last thing back.

You end with the smallest input that still fails, and it is usually far more
informative than the original. A bug that needs ten thousand records is
mysterious. The same bug reduced to two records, where one has an empty name
field, is nearly self-explaining.

This is also what turns a bug into something you can report to someone else, and
the discipline of producing a minimal reproducing case is respected precisely
because it does most of the diagnostic work.

## The general shape

Bisection, minimization, and desk checking are the same move.

You have a space of possibilities — locations in the program, elements of the
input, revisions in history — and you make observations that **eliminate a
fraction of it** rather than testing one candidate at a time.

Testing candidates one by one is linear. Halving is logarithmic. The difference is
between an afternoon and five minutes, and it is available in almost every
debugging situation, which is why it is worth making explicit rather than
discovering by accident.

Next: the tool that makes the observations cheap.
