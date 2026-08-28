# A Last Word

The temptation at the end of a book is to congratulate the reader. This is more
useful than that.

What follows is what you know, what you do not, the one habit worth keeping, and an
admission that some of the opinions in these thirty-five chapters are wrong.

An honest account of where you are, which is more useful than congratulation.

## What you know

You can read a program and say what it does. You can write one of a few hundred
lines that works. You know why an `int` overflows, why a `double` cannot hold one
tenth, why `==` on two `Integer`s can surprise you, and why a file read in the
wrong charset produces `GÃ¶del`.

You have built a parser and an interpreter, which most people who write software
for a living have not.

You know that a race condition is three operations where you saw one, that a
lambda is an interface implementation, that generics disappear at run time, and
that no program decides halting. You know what to measure and what to derive, and
that a claim about performance without a number is an opinion.

That is a real foundation and it is not small.

## What you do not know

More than the above, and this is the part worth being clear about.

**You have not maintained anything.** Every program in this book was written once
and read once. The hardest problems in software are not in writing code; they are
in changing code you did not write, six months after everyone who understood it
left. Nothing in a first course prepares you for it, and Chapter 23's design
principles are advice you have not yet had the experience to properly evaluate.

**You have not worked with other people.** Reviewing, disagreeing about design,
inheriting decisions you would not have made, explaining your reasoning to
somebody who is not convinced. This is most of the job.

**You have not built anything large.** The techniques that matter at ten thousand
lines are different from the ones that matter at three hundred, and there is no
way to learn them at three hundred.

**Your Java is narrow.** You have not used the module system, build tools beyond
the basics, most of the standard library, or any framework. That is deliberate —
those are learnable in days when you need them, and the ideas in this book are
not — but do not mistake having finished this book for knowing Java.

**You have not been on call.** Nothing you wrote here had to keep running. Chapter
28 and Chapter 31 described a world you have read about.

## The thing that will actually matter

Of everything in this book, the habit most worth keeping is smaller than any
chapter.

**When something surprises you, find out why.**

Not "it works now", not "I changed something and the error went away". The actual
mechanism. It costs twenty minutes and it is the entire difference between people
who get steadily better at this and people who plateau early with a large
collection of things that seem to work.

Every measurement in this book exists because of that habit. The `LinkedList`
result was surprising, so Chapter 15's cache material got written. The parallel
counter gave three different wrong answers, so Chapter 31 exists. Chapter 32's
bubble-sort row disagreed with the prediction and was reported rather than
removed, because the alternative was teaching something false.

That is what the discipline looks like from the inside: you were surprised, you
looked, and what you found went in.

## On being confused, again

The preface asked you to reframe confusion before it happened. Now that it has,
the reframing can be sharper.

You were confused, repeatedly, and you are still reading. That is the only
evidence about aptitude that means anything, and it is better evidence than
finding it easy would have been — the people who find the early material easy
frequently stall later, because they never developed a method for being stuck.

You have one now. *What exactly did I expect, and what exactly happened instead?*
It has an answer, the answer is findable, and finding it is the job.

## A last honesty

This book has opinions and some of them are wrong.

The position on checked exceptions is the consensus and there are thoughtful
people who disagree. The preference for composition over inheritance is a heuristic
that occasionally misleads. The insistence on measuring can become its own
avoidance, and some designs are clearly better before any measurement. The claim
that writing an interpreter changes how you read your own language is my
experience and not a study.

Take the reasoning and not the conclusions. Where a chapter argued for a position,
the argument is the thing on offer, and you should be able to reconstruct it well
enough to find the cases where it does not hold. Where a chapter measured
something, the number is from one machine on one day and your machine will
disagree.

The habit of asking for the reason is what the book was for. Applied to the book
itself, it is working exactly as intended.

## The end

You started with two voltages and an agreement.

You now have thirty-five chapters of agreements built on that one, up through
numbers, text, methods, stacks, recursion, structures, objects, interfaces,
grammars, evaluators, functions, reflection, failure, files, events, threads,
costs, information, and the boundary past which no program reaches.

None of it was in the voltages. All of it is agreement, and every layer was
somebody's decision, and the decisions could have gone another way — several of
them did, in other languages, for reasons those chapters gave.

That is the last claim, and it is the one worth carrying out of here: **the systems
you use were designed by people, under constraints, with trade-offs they could
name.** They are not natural facts and they are not beyond you. You can read the
specification. You can find out why. You can, eventually, be one of the people
making those decisions.

Go and build something.
