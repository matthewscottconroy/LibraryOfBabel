# People

## George Boole (1815–1864)

An English mathematician, largely self-taught, who became professor of
mathematics at Queen's College, Cork, without ever having held a university
degree.

His *An Investigation of the Laws of Thought* (1854) set out to express logical
reasoning in algebraic form — to make deduction a matter of calculation rather
than of judgment. The system used two values, and its operations are the AND, OR,
and NOT of this chapter.

Boole's motivation was philosophical and, in part, theological: he believed the
laws of thought were discoverable and that expressing them mathematically would
illuminate the mind. He had no notion of machines. The algebra sat as a curiosity
of mathematical logic for eighty years before an engineer noticed what it was
for.

That gap is worth registering. The mathematics that makes computers designable
was completed in 1854, and nobody knew what it was good for until 1937.

## Augustus De Morgan (1806–1871)

A contemporary and correspondent of Boole's, professor at University College
London, and the first president of the London Mathematical Society.

The laws named after him — that negating a conjunction gives a disjunction of
negations, and conversely — were known in some form to medieval logicians, but De
Morgan stated them in a form general enough to be used as algebra. They are the
most practically useful identities in this chapter.

He was also an early and clear writer on mathematical induction, and he gave it
that name. It is the reasoning principle behind both the loop invariants of
Chapter 9 and the recursion of Chapter 13.

## Claude Shannon (1916–2001)

Shannon appeared in Chapter 1 for information theory. He appears here for the
other of his two foundational results, and the earlier one.

His 1937 MIT master's thesis, *A Symbolic Analysis of Relay and Switching
Circuits*, made the connection this chapter is built on: the algebra Boole
developed for logic describes the behavior of networks of relays. A relay is open
or closed; a proposition is false or true; series wiring is AND; parallel wiring
is OR.

The consequence was immediate and enormous. Circuit design stopped being an
empirical craft and became a calculation. You could take a specification, write it
as a boolean expression, simplify the expression algebraically, and build the
simplified circuit — with a guarantee that it behaved identically to the original.

It has been called the most important master's thesis of the twentieth century,
and the claim is hard to argue with. He was 21.

## Maurice Karnaugh (1924–2022)

A Bell Labs engineer who in 1953 published the diagram method now called the
Karnaugh map, a way of minimizing boolean expressions by eye.

The technique arranges a truth table in a grid ordered so that adjacent cells
differ in exactly one variable, which makes the terms that can be combined
visually obvious. For expressions of up to four or five variables it is faster
than algebraic manipulation and much less error-prone.

He is included because minimization mattered enormously when every gate was a
physical component with a cost, and because the method is a good example of a
representation chosen to make a structure visible — Unit I's argument, applied to
the display of a function rather than the storage of a value.
