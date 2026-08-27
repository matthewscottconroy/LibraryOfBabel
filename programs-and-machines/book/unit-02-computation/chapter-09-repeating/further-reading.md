# Further Reading

## On invariants

Dijkstra, E. W. (1982). "Why Numbering Should Start at Zero." EWD831.

One handwritten page, freely available in the EWD archive. The argument for
half-open ranges, with the reasoning laid out properly. Read it; it takes four
minutes and you will index arrays differently afterwards.

Hoare, C. A. R. (1969). "An Axiomatic Basis for Computer Programming."
*Communications of the ACM*, 12(10), 576–580.

The founding paper of program verification, and short. The `while` rule on the
fourth page is the loop invariant in its formal dress; you can now read it.

Floyd, R. W. (1967). "Assigning Meanings to Programs." *Proceedings of Symposia in
Applied Mathematics*, 19, 19–32.

The flowchart-annotation version, and the origin of the idea.

Gries, D. (1981). *The Science of Programming*. Springer.

The best book-length treatment of deriving programs from specifications, and much
more approachable than Dijkstra's own. If Section 9.2.1's third use of invariants
— writing the loop from the invariant — appealed to you, this is where to go.

## More approachable

Cormen, T. H., Leiserson, C. E., Rivest, R. L., & Stein, C. (2009). *Introduction
to Algorithms* (3rd ed.). MIT Press. Section 2.1.

Introduces loop invariants using insertion sort, with the three obligations named
exactly as in this chapter. Two pages, and a good second exposure.

Dijkstra, E. W. *The EWD Archive*, University of Texas at Austin.

Over a thousand notes, scanned and transcribed. EWD1036 ("On the cruelty of really
teaching computing science") is the most provocative; EWD498 is the source of "the
question of whether machines can think is about as relevant as the question of
whether submarines can swim".

## Termination and Collatz

Lagarias, J. C. (2010). *The Ultimate Challenge: The 3x+1 Problem*. American
Mathematical Society.

Everything known about the Collatz conjecture, collected. The introductory survey
is readable without specialist background and conveys how much effort has gone
into a problem statable in one sentence.

## Java specifics

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Item 58, "Prefer
for-each loops to traditional for loops".

The argument of Section 9.2.3's first habit, with the failure modes enumerated.

*The Java Language Specification*, Java SE 17 edition. Oracle. Section 14.14.

The precise semantics of the three `for` forms, including the scope rule for a
counter declared in the header.
