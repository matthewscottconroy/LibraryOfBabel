# People

## John von Neumann (1903–1957)

Von Neumann appears a fourth time, and this one is about memory.

The EDVAC design of Chapter 6 specified memory as a linearly addressed store —
a sequence of numbered locations, each reachable in the same time as any other.
That property, **random access**, is what makes the address arithmetic of Section
15.1.1 possible.

It was not inevitable. Contemporary machines used delay lines and magnetic drums,
where data circulated and you waited for what you wanted to come round. On such a
machine, reading location 4,700,000 costs far more than location 47, and the
constant-time array does not exist.

So the array's central property is inherited from a hardware decision, and the
programming-model term "random access memory" is a description of that decision.

The uniformity is, incidentally, no longer strictly true — Section 15.2.3's cache
effects are the modern machine leaking through an abstraction that was accurate in
1945 and is now an approximation.

## Grace Hopper (1906–1992)

Hopper appears a third time, for the language that introduced subscripting to a
wide audience.

FORTRAN had arrays first, in 1957, and indexed them from 1. COBOL, which Hopper
shaped through FLOW-MATIC, also indexed from 1. So did ALGOL 60, which allowed
arbitrary bounds — you could declare an array indexed from −5 to 17.

C's decision to index from 0 was not the obvious inheritance; it was a consequence
of C's treatment of an array name as a pointer to its first element, where `a[i]`
is *defined* as `*(a + i)` and 0-based indexing is forced. Java inherited it from
C, along with the syntax.

Worth knowing because it makes 0-based indexing look like what it is: a convention
that won, rather than a mathematical truth. Dijkstra's argument in Chapter 9 is a
defense of the convention after the fact, and a good one, but the convention
arrived first.

## Maurice Wilkes (1913–2010)

Wilkes appears a third time, for the invention that Section 15.2.3 is about.

His 1965 paper "Slave Memories and Dynamic Storage Allocation" proposed a small,
fast memory holding recently used contents of a larger, slower one — what is now
called a **cache**. The word arrived later; Wilkes called it a slave memory.

The idea rests on **locality of reference**: programs do not access memory
randomly, but tend to reuse what they have just used and to use what is nearby.
That is an empirical observation about programs rather than a theorem, and it has
held for sixty years across enormous changes in how programs are written.

The consequence for you is Section 15.2.3's measurement. The abstraction says every
element costs the same; the machine says consecutive access is several times
cheaper; and the gap between those has widened steadily as processors have
outpaced memory.

## Tony Hoare (born 1934)

Hoare appears again — Chapter 9 for the loop invariant, and here for the check
that Section 15.1.3 is about.

His ALGOL W and later work took the position that array bounds should be checked
at run time, and he described the decision by some later languages to omit the
check as one that had cost the industry dearly. In his 1980 Turing Award lecture
he recalled that the ALGOL 60 implementers considered bounds checking essential,
and observed that a language which allowed a program to write outside an array was
one whose programs could not be reasoned about at all.

He also noted, pointedly, that some customers asked for the checks to be disabled
in production for speed — and that this is precisely backwards, since a test run
is where you can afford a failure and a production run is not.

That argument is now widely accepted, and Java's unconditional checking is a
direct descendant of it.
