# People

## John Backus (1924–2007)

Backus appeared in Chapter 5 for FORTRAN. He appears here for a smaller and more
consequential decision: FORTRAN used `=` for assignment, and essentially every
mainstream language since has inherited it.

The choice was reasonable in context. FORTRAN's statements looked like formulas,
and `X = Y + 1` read naturally as a definition. The consequence — that actual
equality needed a different symbol, and that generations of programmers would
type `=` where they meant `==` — was not foreseeable in 1957.

It is a useful case of a decision that is locally sensible and globally
expensive, and of how hard such decisions are to reverse once a language has
users.

## Niklaus Wirth (1934–2024)

A Swiss computer scientist who designed Pascal, Modula-2, and Oberon, and won the
Turing Award in 1984.

Wirth used `:=` for assignment and `=` for equality, following Algol, and was
explicit that the FORTRAN convention was a mistake worth not repeating. Pascal
was designed for teaching, and a great many people learned to program in it
precisely because its notation was chosen to be unambiguous.

His larger conviction was that languages should be small — small enough that a
compiler could be written by one person and the whole language understood by any
user. He held this while the industry moved in the opposite direction, and
Oberon, his last language, is smaller than Pascal rather than larger. Whether he
was right is still argued about; that the argument is worth having is not.

His book *Algorithms + Data Structures = Programs* (1976) is one of the titles
that shaped how the subject is taught, and its thesis is close to this book's.

## Barbara Liskov (born 1939)

An MIT computer scientist, Turing Award winner in 2008, who did more than anyone
to establish that *limiting* what code can see is a technique rather than an
inconvenience.

Her work on the CLU language in the 1970s introduced abstract data types as a
language feature — the idea that a program should be built from units whose
internals are unreachable from outside. Scope, as this chapter describes it, is
the smallest version of that idea; Unit IV's abstract data types and Unit V's
encapsulation are the full one.

She appears in this chapter because the argument for narrow scope — that
restricting visibility restricts what you must reason about — is essentially
hers, generalized. We will meet her again in Chapter 21, where the substitution
principle that governs inheritance carries her name.

## Edsger W. Dijkstra (1930–2002)

A Dutch computer scientist who argued, more forcefully than anyone, that
programming should be a mathematical discipline in which programs are proved
correct rather than tested until they seem to work.

He is here for a specific reason. Dijkstra was among the first to state clearly
that assignment — mutation — is what makes programs hard to reason about, because
it means a variable's meaning depends on time. His preferred style minimized it,
and his methods for reasoning about programs that do mutate are the direct
ancestor of the loop invariants in Chapter 9, where we will meet him properly.

He also wrote the 1968 letter published as "Go To Statement Considered Harmful",
which is the origin of a phrase that has been reused past exhaustion. The
argument in it is narrower and better than its reputation: unrestricted jumps
make it impossible to describe where a program has got to in terms of the source
text, which is exactly a claim about state.
