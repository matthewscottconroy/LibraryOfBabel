# People

## Grace Hopper (1906–1992)

Hopper appeared in Chapter 5 for compilers. She appears here for a moth.

In September 1947, working on the Harvard Mark II, her team traced a fault to an
actual moth caught in a relay. They removed it, taped it into the logbook, and
wrote "First actual case of bug being found." The page survives in the
Smithsonian.

The story is usually told as the origin of the word "bug", which it is not — the
term was already in engineering use, and Edison used it in 1878. The logbook joke
depends on the word being familiar: *first actual case* is funny only if the
usual case is metaphorical.

What the anecdote does record is the practice of *logging* a fault: writing down
what was observed, what was found, and what was done. That habit is the ancestor
of this chapter's method, and it predates any of the theory.

## Maurice Wilkes (1913–2010)

The builder of EDSAC at Cambridge, one of the first stored-program computers, and
a Turing Award winner in 1967.

He is here for one sentence, from his memoirs, describing a moment in 1949:

> As soon as we started programming, we found to our surprise that it wasn't as
> easy to get programs right as we had thought. Debugging had to be discovered. I
> can remember the exact instant when I realised that a large part of my life from
> then on was going to be spent in finding mistakes in my own programs.

That is the earliest clear statement of the fact this chapter is built around,
and it was a surprise to the people who invented the machines. The expectation had
been that programming would be transcription. It turned out to be a discipline
whose main activity is discovering that you were wrong.

Wilkes also introduced the subroutine library — Chapter 11's subject — and the
concept of microprogramming.

## David Agans

Not an academic; a working engineer who wrote *Debugging: The Nine Indispensable
Rules for Finding Even the Most Elusive Software and Hardware Problems* (2002).

The book's value is that it treats debugging as a teachable method rather than a
talent, which almost nothing else does. Its rules — understand the system, make it
fail, quit thinking and look, divide and conquer, change one thing at a time, keep
an audit trail, check the plug, get a fresh view, and if you didn't fix it it
ain't fixed — are this chapter's content stated as slogans that stick.

"Quit thinking and look" is the one most worth internalizing. The instinct when a
program misbehaves is to reason about what might be wrong. Frequently the faster
route is to observe what *is* wrong, which requires no cleverness and no
hypothesis.

## Brian Kernighan (born 1942)

A Bell Labs researcher, co-author with Dennis Ritchie of *The C Programming
Language* and with P. J. Plauger of *The Elements of Programming Style*, and now
at Princeton.

He is quoted here for an observation that has aged well:

> Everyone knows that debugging is twice as hard as writing a program in the
> first place. So if you're as clever as you can be when you write it, how will
> you ever debug it?

That is an argument for writing plainly, and it is the reason this book keeps
preferring the readable form over the compact one — the guard clause over the
nested conditional, the named boolean over the clever expression, `while (true)`
over `for (;;)`.

Kernighan is also responsible, with Rob Pike, for *The Practice of Programming*,
whose chapter on debugging is the best short treatment in print.
