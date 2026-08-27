# Further Reading

## On immutability

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Items 17 and 63.

Item 17, "Minimize mutability", is the general argument behind `String`'s design,
with the five rules for writing an immutable class. Item 63, "Beware the
performance of string concatenation", is Section 18.1.2 in one page.

## On regular expressions

Friedl, J. E. F. (2006). *Mastering Regular Expressions* (3rd ed.). O'Reilly.

The definitive book, and much more than most people need. Chapters 1 and 2 are a
good tutorial; Chapter 6 on efficiency explains the backtracking behavior behind
ReDoS.

Cox, R. (2007). "Regular Expression Matching Can Be Simple And Fast."

A clear explanation of why Thompson's construction is linear and why most modern
engines are not, with graphs showing an exponential blowup on a pattern that looks
harmless. Free online, and it will change how you write patterns.

Kleene, S. C. (1951). "Representation of Events in Nerve Nets and Finite Automata."
RAND Research Memorandum RM-704.

Where regular expressions were introduced, in a paper about neural nets.

Thompson, K. (1968). "Regular Expression Search Algorithm." *Communications of the
ACM*, 11(6), 419–422.

Two pages, and the beginning of practical text searching.

## On Unicode and comparison

*The Unicode Standard*, current version. Chapter 3 on conformance and Annex #15 on
normalization forms.

Unicode Technical Standard #10, "Unicode Collation Algorithm".

Heavy going, and the place where the answers actually are. Worth knowing they
exist so that when a sorting bug turns out to be cultural you know it is a solved
problem rather than your mistake.

## Java specifics

The `java.lang.String`, `java.lang.StringBuilder`, `java.util.regex.Pattern`, and
`java.text.Collator` documentation.

The `Pattern` class documentation is unusually good and doubles as a regular
expression reference for Java's dialect. Read it once rather than searching for
syntax each time.

The `java.time` package documentation, and the JSR 310 design rationale.

For why the old `Date` and `Calendar` classes were replaced, which is a case study
in an API that got almost every decision wrong and had to be abandoned rather than
fixed.

## On text processing generally

Kernighan, B. W., & Pike, R. (1984). *The UNIX Programming Environment*.
Prentice Hall.

The tradition that produced `grep`, `sed`, and `awk`, and the philosophy that text
is the universal interface. Dated in its specifics and sound in its argument.
