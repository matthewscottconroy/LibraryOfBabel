# Information

A message arrives. It reads: *the sun rose this morning.*

Another one arrives. It reads: *the sun did not rise this morning.*

Same language, same length, same count of bytes on the wire. And one of them has
told you nothing whatsoever, while the other is the most important sentence you
will ever be handed.

So whatever it is that the second message has more of, your byte count cannot see
it. Something is being measured here that we have not yet learned how to measure.

We have been counting bits since the first chapter — a bit being a choice between
two possibilities. This chapter asks the far harder question hiding underneath
that one. **How much information is in a message?** Not how much room it takes up.
How much it actually tells you.

There is an answer. It is exact, it is numerical, and one person produced the
whole of it in a single paper in 1948. Claude Shannon's *A Mathematical Theory of
Communication* defined information as **surprise**, gave it a unit, and brought a
field into existence in about eighty pages. Almost nothing in this book's
bibliography arrived so completely without ancestors. Before Shannon, nobody
thought the question had a number for an answer.

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
