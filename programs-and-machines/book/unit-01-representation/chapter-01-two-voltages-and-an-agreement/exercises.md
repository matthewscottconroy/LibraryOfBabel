# Exercises

Exercises marked **[carries forward]** introduce something the next chapter
assumes. Do those ones even if you skip the rest.

## Warm-up

**1.1.** A wire in a circuit reads 1.7 volts. The convention in force says
"below 1.0 V is a 0, above 2.0 V is a 1". What value is the wire holding?

**1.2.** With 5 bits, how many distinct patterns are there? If one of them is
reserved for zero and the rest represent consecutive positive integers starting
at 1, what is the largest value?

**1.3.** How many bits do you need to give every one of the 50 US states its own
distinct pattern? How many patterns are left over?

**1.4.** A colleague says "a byte can hold up to 256". Correct them, and explain
the off-by-one in a single sentence.

## Working through

**1.5. [carries forward]** Write out all eight patterns of 3 bits, in the order
you would get by counting: `000`, `001`, and so on. Next to each, write the
number of 1s it contains. Now count how many patterns have exactly two 1s. Do
the same for 4 bits and patterns with exactly two 1s. Can you say what is
happening without doing the 5-bit case?

**1.6.** The section on noise argued that a decimal circuit running on 5 volts
gets about 0.25 V of noise margin. Redo that estimate for a ternary circuit —
three levels on the same 5 volts. How does its margin compare to binary's?

**1.7. [carries forward]** The pattern `11010110` is read two ways: as a single
unsigned number, and as two 4-bit values side by side. Give both readings, and
say which of the six readings in Section 1.2.3 each one resembles.

(You may wonder about reading it as a character. Byte values above 127 are
exactly where the character-encoding agreements stop agreeing with one another —
which is Chapter 4's subject, and why that reading is not asked for here.)

**1.8.** Take the 32-bit pattern from the last section and change only the very
last bit from 0 to 1. Which of the six readings change, and by how much? Which
change the most, in proportion to their original value? What does that tell you
about where a single-bit error hurts most?

**1.9.** A file written on a big-endian machine contains a 32-bit integer with
the value 1. It is read on a little-endian machine that does not correct for
byte order. What value does the little-endian machine report? Show the bytes.

## Going further

**1.10.** Section 1.1 claimed that digital circuits "restore" a signal, so noise
does not accumulate. Construct an argument for why an analogue circuit cannot do
the same thing. What would it need to know that it does not have?

**1.11.** Suppose someone proposes a computer using five voltage levels, on the
grounds that five is a nice compromise. Write the strongest case you can for
this design, then the strongest case against. Which consideration from this
chapter is decisive, and why?

**1.12.** The chapter claimed no pattern can announce its own encoding, because
the announcement would itself need one. Yet real file formats *do* announce their
encoding — a PNG file begins with a distinctive byte sequence, and an XML
document may declare its character set. Resolve the apparent contradiction. What
has actually been agreed in advance in those cases?

**1.13.** Shannon's definition makes a bit the information gained from a question
whose two answers were equally likely. Under that definition, how much
information do you gain from being told the outcome of a coin flip using a coin
you know has heads on both sides? Does this match the everyday sense of the word
"information"?

**1.14. [carries forward]** Invent an encoding. Choose 4 bits and assign a
meaning to all sixteen patterns — anything you like, as long as the assignment is
complete and unambiguous. Then write down, in one sentence each: what operation
would count as "adding one" under your encoding, and what should happen when you
add one to the last pattern. You have now made the same three decisions the
designers of `int` had to make.
