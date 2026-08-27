# Many Things at Once, Really

Chapter 30's event loop did one thing at a time and gave the appearance of doing
several. This chapter is about actually doing several, on several processors,
simultaneously — and about why that is the hardest material in the book.

It is hard for a specific reason, and naming it early helps. Everything you have
learned about reading a program assumes that statements happen in order and that a
value you read is the value that was there. With more than one thread, neither
holds. Two threads interleave at a granularity you did not choose, the compiler
and the processor reorder your instructions, and a variable can change between
your reading it and your using it.

The result is a class of bug that does not reproduce, does not appear under a
debugger, passes every test, and fails in production once a week.

Chapter 26 already showed one. A lambda incrementing a counter over a million
elements in parallel produced 97282, then 78637, then 906250. This chapter
explains that, and Section 31.1.2 reproduces it in a smaller and even more
dramatic form.

Section 31.1 is the core. Why concurrency at all — the answer has changed since
2005 and the reason is a fact about physics. Then shared state and races, with
measurements. Then locks: what they buy, what they cost, and the two ways they go
wrong.

Section 31.2 goes outward to other machines. Sockets, protocols, and the fact that
a network is a place where nothing is shared and everything is slow — which is a
harder problem and, in one specific way, an easier one.

The chapter's position, stated up front:

**Do not share mutable state between threads.** Every technique in Section 31.1
exists to make sharing safe, and every one has a cost and a failure mode. The
techniques that work best are the ones that avoid the problem: immutable data,
confinement to one thread, and message passing.

Chapter 20 argued for immutability on grounds of clarity. This is where it stops
being a preference.
