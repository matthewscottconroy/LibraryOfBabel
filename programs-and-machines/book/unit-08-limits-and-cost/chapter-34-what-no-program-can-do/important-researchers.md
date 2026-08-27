# Important Researchers

**Alan Turing** (1912–1954) proved this chapter's central result in 1936, at
twenty-three, in a paper whose subject was not computing at all. Hilbert had asked
whether there is an effective procedure deciding the truth of any mathematical
statement — the *Entscheidungsproblem* — and answering it required first making
"effective procedure" precise. Turing's definition was the machine of Chapter 6,
and the halting proof followed, and the answer to Hilbert was no.

The computer is a side effect of that paper. Turing invented an abstract machine
to prove something impossible, and the machine turned out to be buildable.

His later life is well known and worth stating plainly: the codebreaking at
Bletchley Park that shortened the war, the 1950 paper that founded artificial
intelligence as a subject, the prosecution for homosexuality in 1952, the chemical
castration imposed as an alternative to prison, and his death in 1954 at
forty-one. He received a posthumous royal pardon in 2013. The field's highest
honor carries his name.

**Kurt Gödel** (1906–1978) proved the incompleteness theorems in 1931, five years
before Turing, and they are the same argument in a different setting: any
consistent formal system strong enough for arithmetic contains true statements it
cannot prove. Gödel's construction is a statement asserting its own unprovability;
Turing's is a program contradicting its own halting prediction. Both are Cantor's
diagonal. The results ended Hilbert's programme — the hope that mathematics could
be placed on a complete, provably consistent, mechanically checkable foundation.

**Georg Cantor** (1845–1918) invented the technique in 1891, showing that the real
numbers cannot be listed. The argument was received badly enough that it damaged
his career and his health; Kronecker opposed it publicly and at length. It is now
one of the first proofs a mathematics student sees, and it underlies both results
above and the counting arguments of Chapter 33.

**Alonzo Church** (1903–1995) appears for the fourth time. He proved the same
result as Turing, independently and slightly earlier, using the lambda calculus of
Chapters 13 and 26 rather than a machine. That two entirely different formalisms
gave the same answer is the evidence for the Church–Turing thesis, and it is why
Chapter 6 could claim that "computable" means one thing rather than several.
Turing was his doctoral student, which is a remarkable pairing.

**Henry Gordon Rice** (1920–2003) proved in 1951 that the halting problem is not a
special case but the general one: every non-trivial property of a program's
behavior is undecidable. It is the theorem that closes off the search for a clever
exception, and it is why the line between syntax and behavior is where every
analysis tool sits.

**Andrey Kolmogorov** (1903–1987) gave the definition of information in Section
34.2.1, and separately the axioms that put probability theory on a rigorous
foundation, and substantial results in turbulence, topology and dynamical systems.
The complexity measure was arrived at independently by **Ray Solomonoff** (1926–2009)
slightly earlier, working on inductive inference, and by **Gregory Chaitin**
(born 1947) as a teenager — which is why it is sometimes called
Kolmogorov–Chaitin complexity, and why Solomonoff's priority is often overlooked.

**Gregory Chaitin** (born 1947) turned the Berry paradox into the uncomputability
proof of Section 34.2.1 and defined the halting probability $\Omega$ — a specific
real number, definable in a sentence, whose digits are uncomputable. It is a
sharper form of everything in this chapter: a number that exists, is well defined,
and cannot be known.
