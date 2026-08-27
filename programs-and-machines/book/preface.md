# Preface {.unnumbered}

## Who this is for {.unnumbered}

You do not need to have written a program before. If you have, some of the early
chapters will move quickly, and you should let them — but I would ask you not to
skip them, because they are not a review. They are an attempt to put a floor
underneath things you were probably taught as rules.

Here is what I mean. If you have programmed before, you have likely been told
that an `int` in Java holds whole numbers from about negative two billion to
about positive two billion. That is true. But it is the kind of true that is
useless when something goes wrong, because it does not tell you *why* the range
is that, why it is not symmetric, or what happens at the edge. You were handed a
fact where you needed a reason. This book is mostly in the business of supplying
reasons.

You do need some comfort with high-school algebra — you should not flinch at
seeing `f(n)`, or at the idea that a letter can stand for a number you have not
chosen yet. That is genuinely all the mathematics assumed. Where the book needs
more, it builds it.

## What the book is about {.unnumbered}

There is a single claim underneath all eight units, and it is worth stating
plainly at the start so you can watch it recur:

> A computer holds patterns and changes them. Everything else — numbers, text,
> pictures, sound, objects, programs, meaning — is an agreement we have layered
> on top of those patterns.

That sounds like philosophy. It is not; it is the most practical thing in the
book. Almost every bug that will genuinely confuse you in your first years of
programming comes from a moment where you forgot which agreement you were
operating under. You thought you had a number and you had a pattern. You thought
you had a copy and you had a second name for one thing. You thought you had text
and you had bytes that someone else had agreed to read differently.

So we start at the bottom, with voltage and agreement, and we climb.

## Why Java {.unnumbered}

Java is the instrument in this book, not the subject.

I want to be honest about that, because it changes how you should read. This is
not a book that will march through the Java language feature by feature and
declare victory when the features run out. It is a book about how computation
works, which uses Java as the place where those ideas become concrete enough to
run.

Java is a good instrument for this. It is explicit where beginners need
explicitness: you must say what type a thing is, you must say who can see it,
you must say what a method promises to return. Languages that let you skip those
declarations are pleasant to write and terrible to learn from, because they hide
exactly the decisions you are trying to learn to make. Java also runs on a
machine that was designed and documented rather than one that accumulated, which
means when we ask "what actually happens here" there is an answer we can look up.

Java is also, by the standards of this book, quite verbose. You will type
`public static void main(String[] args)` many times before you fully understand
every word in it. I will not pretend that is elegant. What I will do is come
back and explain each word at the point where you have the machinery to
understand it, rather than asking you to accept all of it on day one.

## How the book is arranged {.unnumbered}

Eight units, each one making the central claim in a different register.

**Unit I — Representation** asks what a machine can hold. Bits, numbers, the
arithmetic of a finite box, text, and the general problem of encoding. Java does
not appear until the fifth chapter, and when it does it arrives as a tool for
looking at bits.

**Unit II — Computation** asks what it means to take a step. State, transition,
choice, repetition, and the loop invariant — which is the first place in the
book where you will prove something rather than test it.

**Unit III — Abstraction by Procedure** is about naming a process. Methods, the
call stack, parameter passing, and recursion.

**Unit IV — Compound Data** is about holding many things at once, and about the
invariant that turns a heap of values into a structure. Arrays, collections,
and text processing.

**Unit V — Objects, State, and Identity** treats classes as a design decision
rather than a syntax. This is where identity and equality get separated, which
is one of the two or three genuinely hard ideas in the book.

**Unit VI — Programs as Data** is the center. We write a small interpreter in
Java, for a small language of our own, and the line between a program and its
meaning stops being a metaphor.

**Unit VII — The World Outside the Program** is what happens when a program
stops being a pure calculation and meets a world that does not cooperate:
failure, files, events, and concurrency.

**Unit VIII — Limits and Cost** closes the argument. What programs cost, how
much information a message carries, and what no program can do at all.

## How to read it {.unnumbered}

Slowly, with a machine in front of you.

Every program printed in this book compiles and runs as printed. You should run
them. More than that: you should break them. When a chapter shows you a program
that works, change a number and predict what will happen before you press enter.
The prediction is the learning; the running is just the grading.

The exercises at the end of each chapter are not optional in the way exercises
usually are. A few in every set introduce an idea that a later chapter goes on to
assume, and those are marked **[carries forward]**. Do at least those.

## A word about being confused {.unnumbered}

You are going to be confused, repeatedly, and I want to reframe that before it
happens.

Confusion is not a signal that you lack aptitude. It is a signal that you are
holding two ideas that do not yet fit together, which is precisely the state
that precedes understanding something. The students I have watched struggle
most are not the ones who get confused; they are the ones who treat confusion as
evidence they should stop.

When you are stuck, the useful question is almost never "am I smart enough for
this". It is "what exactly did I expect, and what exactly happened instead". That
question has an answer you can go and find. Most of this book is training in
asking it.

Let us start with voltage.
