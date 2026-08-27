# People

## Grace Hopper (1906–1992)

A mathematician and US Navy officer who worked on the Harvard Mark I and later
led the team that built the A-0 system in 1952 — one of the first programs that
translated symbolic notation into machine code.

The idea met resistance that is difficult to imagine now. The prevailing view was
that computers did arithmetic, and that expecting one to translate notation was a
category error. Hopper later said that she was told, repeatedly, that a computer
could not write its own programs. She built it anyway.

Her larger conviction was that programming should be possible in something closer
to English, which led to FLOW-MATIC and through it to COBOL. That conviction is
the reason you can write `System.out.println("Hello")` instead of a sequence of
opcodes, and it is worth remembering that it was once a controversial opinion
rather than an obvious one.

## John Backus (1924–2007)

Led the IBM team that produced FORTRAN, delivered in 1957 — the first
widely-used high-level language, and the first compiler that produced code good
enough to displace hand-written assembly.

That last point was the whole battle. Programmers of the period could write
better machine code than any compiler, and knew it. Backus understood that the
project would fail unless the generated code was competitive, so a large part of
the FORTRAN effort went into optimization rather than translation. It worked, and
the argument was over.

Backus also gave us the notation for describing grammars now called Backus–Naur
Form, which we will use directly in Chapter 24 when we specify a language of our
own. He received the Turing Award in 1977, and used the lecture to argue that the
whole style of programming he had helped create was a dead end — an unusual thing
to do with a prize.

## James Gosling (born 1955)

The principal designer of Java, begun at Sun Microsystems in 1991 under the name
Oak and released in 1995.

The original target was not the internet. It was consumer devices — set-top
boxes, appliances — where processors varied wildly and code had to run on
hardware chosen after the software was written. That constraint produced the
bytecode-and-virtual-machine design, which then turned out to be exactly what the
web needed when the web arrived. It is a good example of a design succeeding for
a reason its designers did not anticipate.

Several of Java's more conservative decisions trace to the same source: no
pointer arithmetic, mandatory bounds checking, automatic memory management. These
cost performance and were chosen deliberately, on the grounds that a program
running on someone else's device must not be able to corrupt it.

## Guy L. Steele Jr. (born 1954)

Co-author of *The Java Language Specification*, and before that co-creator of
Scheme with Gerald Sussman — the language in which *Structure and Interpretation
of Computer Programs* is written, and a direct ancestor of this book's approach.

Steele's specification work is worth a word. A language specification is a
document that must be precise enough to settle any argument about what a program
means, which is a genuinely hard writing problem. The Java specification is
unusually good at it, and it is the document that answers questions like "is
integer overflow defined behavior?" — yes, it wraps, and it says so.

He also gave a well-known talk, *Growing a Language* (1998), constructed so that
every word of one syllable is defined before it is used, and longer words are
defined in terms of those. The talk is about language design; the technique is a
demonstration of its thesis. It is worth an hour.
