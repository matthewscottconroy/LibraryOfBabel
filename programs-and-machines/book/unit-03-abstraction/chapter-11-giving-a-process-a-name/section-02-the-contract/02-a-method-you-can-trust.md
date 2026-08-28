# A Method You Can Trust

Your precondition says `values` must be non-empty.

Somebody passes an empty array.

They were not being careless, particularly. They had an array, it happened to have
nothing in it that day, and they called your method. Now what should happen?

Java will let you choose any of four answers, and choosing is a real design
decision rather than a formality.

## The four answers

**1. Let it fall over.** Do nothing; whatever happens, happens. Here `values[0]`
throws `ArrayIndexOutOfBoundsException` from somewhere in your method's body.

It costs nothing to implement and the failure is baffling. Your caller asked for a
maximum and received a complaint about array indices from inside code they have
never read. The message describes the *symptom*, at the place it surfaced, rather
than the *mistake*, at the place it was made.

**2. Check, and say so.** Catch the violation and name it:

```java
static int largest(int[] values) {
    if (values == null)       throw new IllegalArgumentException("values must not be null");
    if (values.length == 0)   throw new IllegalArgumentException("values must not be empty");
    ...
}
```

Now the caller gets a sentence about the thing they actually did. Chapter 28 gives
exceptions the treatment they deserve; for now, `throw` stops the method and
reports.

**3. Hand back a sentinel.** Return something meaning "no answer" — `-1`, or
`null`, or `Integer.MIN_VALUE`.

Convenient, and quietly dangerous, because nothing forces the caller to look. A
sentinel that slips through and gets used as a real answer produces a wrong result
with no error attached — the worst outcome on this list. And note that `-1` is a
perfectly plausible maximum.

**4. Change the deal.** Decide what the empty case *should* return, and say so in
the contract. For a maximum there is no defensible answer, which is exactly why
the precondition exists. For a sum, zero is not a fudge — it is right, and the
precondition was never needed.

## Choosing between them

The guidance that survives contact with real code:

**Public methods, called by people you have never met: check and throw.** You
cannot make strangers read your documentation, and a clear exception at the point
of the mistake is worth what it costs.

**Private helpers inside one class: trust the precondition.** Every caller is in
the same file and you can see all of them. Checking here is noise, and noise makes
the real checks harder to notice.

**Never return a sentinel that could pass for a real answer.** If you must signal
absence, use something that cannot be mistaken for presence — `Optional`, or an
exception. `-1` is fine for an index, because there is no such thing as element
minus one. It is not fine for a temperature.

**Fail as early as you possibly can.** This one deserves more than a bullet.

## Why early matters so much

Picture a method that accepts a negative age and stores it without comment.

Nothing fails. The program carries on. Sometime later a report divides by it, or
sorts by it, or renders it, and a number comes out looking wrong three subsystems
away from the place the bad value walked in.

Now you are debugging. And you are not looking for a mistake — you are working
backwards through everything that value touched, over a search space vastly larger
than it needed to be. The single piece of information that would have ended this in
ten seconds, namely *who passed a negative age*, evaporated hours ago.

An exception thrown at the front door would have carried the entire diagnosis in
its stack trace, for free.

So: **the distance between a mistake and its symptom is the cost of the bug.**
Anything that shortens that distance is worth doing, and checking a precondition is
the cheapest way to shorten it that anyone has found.

## A construct that is not quite what it looks like

Java has something for stating things you believe:

```java
assert values.length > 0 : "largest requires a non-empty array";
```

If the condition is false you get an `AssertionError` carrying that message.

And here is the catch that catches everyone once: **assertions are switched off by
default.** Unless somebody runs the program with `-ea`, that line does nothing at
all. Which makes it entirely unsuitable for validating input from outside — a
check that does not run in production is not a check, it is a comment with
punctuation.

What they are genuinely good for is your *internal* beliefs. An invariant you
expect to hold. A branch you are confident is unreachable. During development they
catch a broken assumption the moment it breaks; in production they cost nothing
because they are not there.

| what you are checking | what to use |
|---|---|
| input from outside your control | `if` and `throw` |
| a belief about your own code | `assert` |

## What all of this was for

Here is the chapter, in one sentence.

A method is worth having when you can call it **without reading it**.

That takes three things, and you now have all three. The name has to say what it
does. The signature has to say what goes in and what comes out. The contract has to
say what is required and what is guaranteed. When all three hold, the method is a
real unit of thought, and your attention is genuinely free for something else.

When any one of them fails — a name that misleads, a precondition nobody wrote
down, a method that quietly does one extra thing — the abstraction leaks. And now
you have to remember the method's peculiarities on top of everything else, which
is *more* to carry than the four lines it replaced.

Trustworthiness is not a nicety here. It is the property that makes the whole
device work.

Next, the machinery underneath: what actually happens when a method is called, and
why Java's parameter passing has a surprise in it.
