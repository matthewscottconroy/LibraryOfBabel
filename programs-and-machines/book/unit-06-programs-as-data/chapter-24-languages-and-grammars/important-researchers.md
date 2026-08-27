# Important Researchers

**Noam Chomsky** (born 1928) is a linguist, and the hierarchy in Section 24.1.1 is
from his 1956 paper *Three Models for the Description of Language*. He was arguing
about human language — specifically that a finite-state model could not account
for English — and the mathematical apparatus he built to make the argument turned
out to describe programming languages exactly. It is one of the cleanest cases of
a result outliving its motivation: the linguistics remains contested, and the
hierarchy is in every compiler textbook. Chomsky did not write it for computer
scientists and has shown limited interest in what they did with it.

**John Backus** (1924–2007) needed a way to state ALGOL 58's syntax precisely and
invented the notation to do it. Before that, language definitions were English
prose and no two implementations agreed. Backus also led the team that built the
first FORTRAN compiler in 1957, an achievement whose difficulty is hard to
overstate — nobody knew whether a compiler could produce code competitive with
hand-written assembly, and the project's credibility rested on proving it could.
Later he became a critic of the style of programming FORTRAN had established, and
his 1977 Turing Award lecture, "Can Programming Be Liberated from the von Neumann
Style?", is an argument for functional programming that Chapter 26 will echo.
Turing Award, 1977.

**Peter Naur** (1928–2016) edited the ALGOL 60 report and refined Backus's
notation into the form that carries both their names. The report is a landmark for
a reason beyond notation: it is a language definition precise enough that
implementations could be checked against it, which had not previously been true of
anything. Naur later argued that programming is fundamentally about building a
*theory* of the problem in the programmers' heads, and that documentation and code
are both lossy projections of that theory — an idea that explains, better than
most, why a codebase becomes unmaintainable when the people leave. Turing Award,
2005.

**Donald Knuth** (born 1938) invented LR parsing in 1965, which made it possible to
generate a parser mechanically from a grammar and settled what class of grammars
could be parsed deterministically in one left-to-right pass. Knuth appears
elsewhere in this book for analysis of algorithms and for *The Art of Computer
Programming*, and Chapter 32 quotes his warning about premature optimization. The
parsing result is the one that turned compiler construction from craft into
engineering. Turing Award, 1974.

**Frances Allen** (1932–2020) established the theory of compiler optimization —
control-flow analysis, data-flow analysis, and the framework within which
essentially all modern optimizing compilers work, including the JIT that Chapter
21 measured. Her 1966 paper introduced the control-flow graph. She was the first
woman to receive the Turing Award, in 2006, forty years after that paper.

**Alfred Aho** (born 1941) and **Jeffrey Ullman** (born 1942) wrote the book —
*Compilers: Principles, Techniques, and Tools*, universally the Dragon Book — and
built the tools, `lex` and `yacc`, that made grammar-driven parser generation
routine. A generation of compilers began as a `.y` file. They shared the Turing
Award in 2020.
