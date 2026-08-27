# Information

Chapter 1 asked what a bit is and answered: a choice between two possibilities.
Everything since has taken that as a starting point.

This chapter asks the harder version. **How much information is in a message?**
Not how many bytes it occupies — that is a fact about the encoding — but how much
it actually tells you.

The answer exists, it is exact, and it was given by one person in one paper in
1948. Claude Shannon's *A Mathematical Theory of Communication* defined
information as **surprise**, measured it in bits, and in doing so created a field.
It is one of the few papers in this book's bibliography that has no real
antecedents; before it, the question was not thought to have a numerical answer.

Section 33.1 develops the measure. Why a certain message carries no information.
Why an unlikely one carries more than a likely one. And **entropy**, which is the
average surprise of a source and turns out to be exactly the number of bits per
symbol you need.

Section 33.2 does something with it. Variable-length codes, where frequent symbols
get short codes — the idea Morse had in 1838 and Huffman made optimal in 1952.
Then the proof that **no compressor can shrink every input**, which is a counting
argument three lines long and the first outright impossibility result in this book.

That last item is the reason this chapter comes before Chapter 34. The halting
problem's argument is harder, but it has the same shape: count the things that
exist, count the things a program can distinguish, and observe that the second
number is smaller.

Two threads from earlier chapters close here.

Chapter 1 said an encoding is an agreement and that fixed-width encodings waste
space when symbols are unequally likely. This chapter says exactly how much, and
gives the code that stops wasting it.

Chapter 4 said UTF-8 gives common characters short encodings and rare ones long
ones. That is Section 33.2.1's principle, and it is why UTF-8 won.
