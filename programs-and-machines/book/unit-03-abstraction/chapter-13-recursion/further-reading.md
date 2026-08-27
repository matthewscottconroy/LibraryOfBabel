# Further Reading

## The essential one

Abelson, H., & Sussman, G. J. with Sussman, J. (1996). *Structure and
Interpretation of Computer Programs* (2nd ed.). MIT Press. Chapter 1.

Freely available online. Section 1.2, "Procedures and the Processes They
Generate", is the source of the recursive-procedure versus recursive-process
distinction in Section 13.2.2, and treats it far more thoroughly. The book is in
Scheme rather than Java, which is a smaller obstacle than it looks — the first
chapter needs almost no syntax.

If any part of this chapter interested you, read that chapter. It is the closest
thing to a direct ancestor this book has.

## On induction

Velleman, D. J. (2019). *How to Prove It: A Structured Approach* (3rd ed.).
Cambridge University Press. Chapter 6.

Induction taught properly, for people who have not done proofs before. The
connection to recursion is drawn explicitly, and the exercises are good.

Graham, R. L., Knuth, D. E., & Patashnik, O. (1994). *Concrete Mathematics*
(2nd ed.). Addison-Wesley. Chapter 1.

Harder, funnier, and full of recurrences — the mathematical objects that recursive
methods compute. The treatment of the Tower of Hanoi in the first few pages is a
model of how to derive a recursive solution.

## The historical papers

McCarthy, J. (1960). "Recursive Functions of Symbolic Expressions and Their
Computation by Machine, Part I." *Communications of the ACM*, 3(4), 184–195.

The founding paper of Lisp, and the origin of `eval`. Read it after Chapter 25,
when you have built one yourself.

Peano, G. (1889). *Arithmetices principia, nova methodo exposita*.

Of historical interest mainly; the axioms are stated in every logic textbook and
are easier to meet there.

## Algorithms

Cormen, T. H., Leiserson, C. E., Rivest, R. L., & Stein, C. (2009). *Introduction
to Algorithms* (3rd ed.). MIT Press. Chapters 2 and 4.

Chapter 2 covers merge sort — the canonical divide-and-conquer recursion. Chapter
4 covers how to analyze the cost of a recursive algorithm, which is what Section
13.2.1 gestures at and Chapter 32 will need.

## On memoization and dynamic programming

Bellman, R. (1957). *Dynamic Programming*. Princeton University Press.

The origin of the technique that fixes the Fibonacci problem. Bellman's account of
choosing the name — he wanted something a hostile Secretary of Defense could not
object to — is in his autobiography and is worth looking up.

## Practice

Project Euler, and the recursion exercises in any algorithms course.

Recursion is a skill rather than a fact, and the transition from bewildering to
obvious happens after some number of problems solved rather than after any amount
of reading. Twenty is a reasonable estimate.
