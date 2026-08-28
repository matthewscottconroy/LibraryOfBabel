# Encapsulation

Something in this section is going to feel like paperwork, and it is worth saying
in advance what it is for.

Chapter 16 argued that a data structure is its invariant — the promise about how
the fields relate — and that the promise is only worth anything if the code able to
break it is small enough to check. This section supplies the mechanism that makes
"small enough" true, and then argues about where to draw the line.

Three lessons.

`private` first — what it does, and the argument that it is not a security
feature but a reasoning one. Then the public surface: what to expose, and why the
answer is usually less than you think. Then `static`, which shares the syntax of
everything else in the chapter and means something quite different, and which is
where the Chapter 5 debts are finally paid.
