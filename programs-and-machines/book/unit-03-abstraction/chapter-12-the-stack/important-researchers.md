# People

## David Wheeler (1927–2004)

Wheeler appeared in Chapter 11 as an inventor of the subroutine. He belongs here
too, because the hard part of the subroutine was not calling it but *returning*.

Jumping to a stored sequence of instructions is easy. Getting back is not, because
the destination differs for every call — the same square-root routine must return
to whichever program called it. Wheeler's solution on EDSAC, the **Wheeler jump**,
had the caller place the return address into the subroutine before transferring
control, so the routine could jump back to a location it had been told.

It was not a stack. EDSAC's arrangement gave each subroutine one place to store a
return address, which means a subroutine could not safely call itself or be called
from within itself — no recursion, and careful discipline about nesting.

The stack is what removes that restriction, and seeing what it fixes is the best
way to appreciate what it is for.

## Friedrich L. Bauer (1924–2015) and Klaus Samelson (1918–1980)

German mathematicians at the Technical University of Munich who, in the late
1950s, introduced the stack as a mechanism for handling nested structure — first
for evaluating arithmetic expressions with parentheses, then for procedure calls.

Bauer and Samelson filed a patent on the principle in 1957 and described it in a
1960 paper, "Sequential Formula Translation". Their term was *Keller*, German for
cellar — things are stored downwards and retrieved in reverse order.

The insight is the one in Section 12.1.2: nesting and last-in-first-out are the
same structure. Once seen for parenthesized expressions, it applies unchanged to
procedure calls, and after that the design of ALGOL 60 — the first language with
proper recursion — became possible.

Bauer is also credited with coining the term "software engineering", as the
deliberately provocative title of a 1968 NATO conference he helped organize.

## Edsger W. Dijkstra (1930–2002)

Dijkstra appeared in Chapters 7 and 9. He appears a third time because he wrote
the compiler that made recursion work.

His ALGOL 60 implementation, with Jaap Zonneveld in 1960, was among the first to
support recursive procedures, and required working out how to lay out activation
records so that each invocation gets its own storage. His 1960 note "Recursive
Programming" describes the scheme.

There is a well-known story that recursion entered the ALGOL 60 report almost by
accident — a sentence saying that a procedure could call itself, inserted without
the committee fully appreciating the implementation consequences. Whether or not
the anecdote is exact, the implementers had to solve the problem afterwards, and
the stack is what they solved it with.

## James Gosling (born 1955)

Gosling appeared in Chapter 5 as Java's designer. He is here for two decisions
this chapter rests on.

**No pointer arithmetic.** Java has references, which can be followed, copied,
compared, and nulled, and nothing else. This removes the buffer-overflow attacks
of Chapter 6 and the dangling-pointer bugs that come from holding an address into
a discarded frame, at the cost of the low-level control that systems languages
need.

**Uniform pass-by-value.** Java has no `&` parameter, no `ref` or `out`, no way to
give a method access to the caller's variable. Every argument is copied. That
uniformity is why one rule explains all the cases in Section 12.2.2 — and,
ironically, why the topic confuses so many people, since the *uniform* rule
produces two very different-looking behaviors depending on what was in the box.
