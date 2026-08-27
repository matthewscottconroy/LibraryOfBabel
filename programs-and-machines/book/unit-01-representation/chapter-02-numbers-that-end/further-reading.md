# Further Reading

## On representation and arithmetic

Knuth, D. E. *The Art of Computer Programming, Volume 2: Seminumerical
Algorithms* (3rd ed., 1997). Addison-Wesley. Section 4.1, "Positional Number
Systems".

Knuth's survey of positional systems is far wider than this chapter — it includes
negative bases, irrational bases, and balanced ternary — and it is a good place to
see that the choices we treat as settled were choices. Dense, and worth the
effort in small doses.

Warren, H. S. (2012). *Hacker's Delight* (2nd ed.). Addison-Wesley.

A whole book about what can be done with fixed-width integers and bit
manipulation. Not a beginner's book, but if two's complement has caught your
interest this is where that interest leads. The chapters on overflow detection
are directly relevant to Section 2.

Patterson, D. A., & Hennessy, J. L. *Computer Organization and Design*. Morgan
Kaufmann. Chapter 3, "Arithmetic for Computers".

Builds the adder circuits this chapter describes in words, and goes on to
multiplication and division hardware.

## The bugs

Bloch, J. (2006). "Extra, Extra — Read All About It: Nearly All Binary Searches
and Mergesorts Are Broken." Google Research Blog, 2 June 2006.

Two pages. Read it.

Lions, J.-L., et al. (1996). *ARIANE 5: Flight 501 Failure — Report by the
Inquiry Board*. European Space Agency, Paris, 19 July 1996.

Also short, and unusually clear. Section 2 gives the technical chain of events.

## Historical

Brahmagupta (628). *Brāhmasphuṭasiddhānta*. Discussed in Plofker, K. (2009),
*Mathematics in India*, Princeton University Press — a careful modern account and
a better entry point than the primary text for most readers.

von Neumann, J. (1945). *First Draft of a Report on the EDVAC*. Moore School of
Electrical Engineering, University of Pennsylvania.

Widely available. Sections 5 and 6 contain the argument for binary. Note the
dating and authorship controversy referenced in the profile; Haigh, Priestley and
Rope's *ENIAC in Action* (2016) treats it carefully.

## Reference

*The Java Language Specification*, Java SE 17 edition. Oracle. Chapter 4, "Types,
Values, and Variables", section 4.2.

The authoritative statement that Java's integral types are two's complement, with
the exact ranges. Specifications are not pleasant reading, but knowing that this
one exists — and that questions like "what does Java guarantee about overflow"
have a definite documented answer — is part of learning to work without guessing.
