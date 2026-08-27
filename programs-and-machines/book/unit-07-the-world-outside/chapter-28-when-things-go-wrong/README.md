# When Things Go Wrong

A method has a job. Sometimes it cannot do it.

The file is not there. The number does not parse. The balance is too low. The
network timed out. None of these are bugs — they are conditions the world
produced, and the method has to say something about them.

This chapter is about what it says.

The mechanism is **exceptions**, and Java's version has more to it than the syntax
suggests. Section 28.1 works up to it: why the obvious alternatives — returning
`-1`, returning `null`, setting a flag — fail, and then throwing, catching, and
Java's genuinely contested distinction between checked and unchecked exceptions.

Section 28.2 is the design half, and it is the more useful one. **Where** should a
failure be handled, which is nearly always further up than people put it. **How**
to release a resource when something has gone wrong, which is what
try-with-resources is for. And **failing loudly** — the argument that a program
which continues after something impossible has happened is worse than one that
stops.

One framing to carry through. An exception is not an error message; it is a
**value that travels**. It is constructed at the point of failure, carries
information about it, and moves up the call stack until something is prepared to
deal with it. That is a control-flow mechanism, and Chapter 12's stack is what it
travels along — a thrown exception unwinds frames, which is why the stack trace
tells you the whole story of how you got there.

The chapter's opinions, stated at the top so you can argue with them:

Exceptions are for conditions the caller cannot reasonably prevent by checking
first. Catching an exception and doing nothing is nearly always wrong. Checked
exceptions were a good idea whose costs turned out to exceed their benefits, and
you should understand why rather than repeat either slogan about them. And a
program that fails immediately and clearly is more valuable than one that limps.
