# Important Figures

## David Hilbert (1862–1943)
*Architect of the Formalist Program; posed the foundational problems that defined 20th-century proof theory.*

Hilbert was born in Königsberg (now Kaliningrad) and spent most of his career at the University of Göttingen, which he built into the world's leading center of mathematics. His influence was pervasive: he made foundational contributions to invariant theory, algebraic number fields, the foundations of geometry, functional analysis, mathematical physics, and logic. His 1900 address to the International Congress of Mathematicians, presenting 23 open problems, shaped the research agenda of an entire century. His personality was famous for its directness and optimism — the phrase "Wir müssen wissen, wir werden wissen" (We must know, we shall know) was his credo.

Hilbert's *Formalist Program* — the attempt to reduce all of mathematics to a complete and consistent formal system whose consistency could be proved by finitary means — is the central motivation for proof theory. His 1899 *Grundlagen der Geometrie* had already shown how to give a purely formal axiomatic treatment of Euclidean geometry, with consistency relative to arithmetic. His 1900 second problem asked for a proof of the consistency of arithmetic itself. His 1926 lecture "Über das Unendliche" is the most complete statement of the program: mathematics is a formal game with symbols, its objects are strings of marks on paper, and the "meaning" of those marks is irrelevant to the question of consistency. Cut elimination and normalization are the proof-theoretic tools Hilbert had in mind: if every proof can be normalized to a canonical form, and the canonical forms are inspectable, then consistency might be verifiable combinatorially.

Gödel's incompleteness theorems (1931) showed that Hilbert's program, as stated, was impossible: no consistent formal system can prove its own consistency. Hilbert, then nearly 70 and in failing health, never fully accepted this conclusion. Nevertheless, his program left lasting mathematics: axiomatic method, formal proof systems, and the questions about provability and decidability that Gödel, Gentzen, and Turing resolved.

---

## Kurt Gödel (1906–1978)
*Proved the incompleteness theorems; established fundamental limits on formal provability.*

Gödel was born in Brünn (now Brno) in the Austro-Hungarian Empire and studied at Vienna, where he became associated with the Vienna Circle. He completed his doctoral thesis in 1929 (the completeness theorem for first-order logic: every valid sentence is provable) and published the incompleteness theorems the following year, at age 25. After emigrating to the United States in 1940 to escape the Nazi regime, he joined the Institute for Advanced Study in Princeton, where he remained for the rest of his life. He suffered from severe hypochondria and paranoia in later years and died of self-starvation after his wife's hospitalization, believing his food was being poisoned.

Gödel's incompleteness theorems are the most celebrated results in mathematical logic. The *First Incompleteness Theorem* shows that in any consistent, recursively axiomatizable system $T$ extending Robinson Arithmetic $\mathsf{Q}$, there exists a sentence $G_T$ such that neither $G_T$ nor $\neg G_T$ is provable in $T$. The construction uses *Gödel numbering*: a systematic way of encoding formulas and proofs as natural numbers, allowing the system to speak about its own provability. The sentence $G_T$ encodes the statement "I am not provable in $T$." The *Second Incompleteness Theorem* shows that $\mathrm{Con}(T)$ (the formalized assertion that $T$ is consistent) is itself not provable in $T$, assuming $T$ is consistent. These results are directly relevant to Section 3 of this chapter on normalization: Gödel's diagonal lemma is a construction of a fixed point for a predicate, structurally similar to the fixed-point combinators in the $\lambda$-calculus.

Gödel also proved the completeness of first-order logic (his 1929 dissertation), the relative consistency of the Axiom of Choice and the Continuum Hypothesis with ZF (1938–1940), and foundational results in set theory. In later life he developed a philosophical defense of mathematical Platonism and investigated the modal logic of provability.

---

## Gerhard Gentzen (1909–1945)
*Invented natural deduction and sequent calculus; proved cut elimination; established proof theory as a discipline.*

Gentzen was born in Greifswald and studied at Göttingen under Hilbert's influence. He completed his doctorate in 1933 and published his landmark paper *Untersuchungen über das logische Schließen* (Investigations into Logical Deduction) in 1935, at age 25. His short career was interrupted by the Second World War; he was arrested by Soviet forces when Prague fell in 1945, imprisoned in a camp, and died there of starvation at age 35. The mathematical community lost one of its most brilliant logicians.

Gentzen's 1935 paper is the founding document of structural proof theory. In it, he introduces two proof systems: *natural deduction* (in which proofs are trees built by introduction and elimination rules corresponding to logical connectives, mirroring mathematical practice) and *sequent calculus* (a more symmetric system in which proofs are trees of sequents $\Gamma \Rightarrow \Delta$). The *Hauptsatz* — the cut elimination theorem for sequent calculus — is Gentzen's central result: every sequent provable with the cut rule has a cut-free proof. Cut elimination implies consistency (no cut-free proof of $\bot$ exists), the subformula property (cut-free proofs use only subformulas of the endsequent), and the separation of classical and intuitionistic logic. Both natural deduction and sequent calculus are developed fully in Sections 2 and 4 of this chapter.

Gentzen's 1936 proof of the consistency of Peano Arithmetic using transfinite induction up to $\varepsilon_0$ is the foundational achievement of ordinal proof theory. The proof-theoretic ordinal $|T|$ of a formal system $T$ measures exactly how much transfinite induction is needed to prove $T$'s consistency; for Peano Arithmetic, $|PA| = \varepsilon_0$. This gives a precise calibration of formal systems' strength that Gödel's incompleteness theorems had shown was necessary.

---

## Alonzo Church (1903–1995)
*Invented the $\lambda$-calculus; proved the undecidability of the halting problem for $\lambda$-definable functions; formulated Church's thesis.*

Church was born in Washington, D.C. and spent his career at Princeton, where he taught for decades and supervised an extraordinary number of students including Alan Turing, Stephen Kleene, Dana Scott, and Michael Rabin. He was known as an exceptionally rigorous and careful mathematician, and his lectures were said to be models of precision.

Church introduced the $\lambda$-calculus in the early 1930s as a formal system for defining functions, with the intention of using it as a foundation for mathematics. When Kleene showed (1935) that the $\lambda$-definable functions are exactly the recursive functions, Church proposed *Church's thesis*: that the computable functions are precisely the $\lambda$-definable (equivalently, recursive) functions. His 1936 paper proved that the problem of deciding whether two $\lambda$-terms are $\beta$-equivalent (i.e., whether a function application reduces to a given value) is undecidable, answering the Entscheidungsproblem negatively. The $\lambda$-calculus is the formal system underlying functional programming languages, and the simply-typed $\lambda$-calculus (which Church introduced in 1940) is the type system corresponding, via Curry-Howard, to intuitionistic propositional logic — the central connection explored in the final section of this chapter.

---

## Alan Turing (1912–1954)
*Invented the Turing machine; proved the undecidability of the halting problem; connected computation to proof theory.*

Turing was born in London, educated at King's College Cambridge, and completed his doctoral thesis at Princeton under Church. His wartime work breaking Enigma at Bletchley Park had enormous historical impact. After the war he worked on early computer design at the National Physical Laboratory and the University of Manchester. He died at age 41 from cyanide poisoning; the circumstances remain officially uncertain, though a coroner's inquest recorded a verdict of suicide.

Turing's 1936 paper "On Computable Numbers, with an Application to the Entscheidungsproblem" introduced the Turing machine — an abstract model of a computing device with a finite set of states, an infinite tape, and a simple transition function — and proved that no Turing machine can solve the *halting problem*: given a description of a Turing machine $M$ and an input $w$, determine whether $M$ eventually halts on $w$. The proof uses diagonalization: a hypothetical halting oracle leads to a contradiction by considering a machine that halts if and only if it would not halt. This is the prototype for all undecidability proofs and connects directly to Gödel's incompleteness argument. For this chapter, Turing's result is the computability-theoretic analogue of Gödel's: just as there are true but unprovable sentences, there are problems that are correct but uncomputable — and the two phenomena are related by the Curry-Howard correspondence (proofs correspond to programs; unprovability corresponds to non-termination).

Turing also made fundamental contributions to mathematical biology, mathematical logic (Ordinal Logics, 1939), and, through his 1950 paper "Computing Machinery and Intelligence," to the philosophy of mind.

---

## Dag Prawitz (born 1936)
*Proved the normalization theorem in full generality; formulated proof-theoretic semantics.*

Prawitz was born in Stockholm and has spent his career at Stockholm University. He is one of the leading figures in philosophical logic and proof theory, and is the principal architect of *proof-theoretic semantics* — the view that the meaning of a logical connective is determined by its proof rules, not by truth-conditions.

His 1965 monograph *Natural Deduction: A Proof-Theoretical Study* is the definitive treatment of Gentzen's natural deduction, proving the normalization theorem in complete generality for intuitionistic and classical propositional and predicate logic. Prawitz introduces the key notion of *proof reduction* (the rules for eliminating detours: an introduction of a connective immediately followed by its elimination reduces to a simpler proof) and proves that every proof reduces to a normal form with no detours — the *normalization theorem*. Strong normalization (the stronger result that every reduction sequence terminates) was proved by Tait (1967) and by Girard's *candidats de réductibilité* method (1971). This normalization theory, presented in Section 3 of this chapter, is the proof-theoretic counterpart of strong normalization in the $\lambda$-calculus.

Prawitz also developed the theory of *proof-theoretic harmony* and *general elimination rules*, investigating what it means for introduction and elimination rules to be "in balance." This research program connects to categorical logic and provides foundations for Martin-Löf Type Theory.

---

## Per Martin-Löf (born 1942)
*Developed Intuitionistic Type Theory (MLTT); connected proof theory to constructive mathematics and type theory.*

Per Martin-Löf is a Swedish logician who studied under Anders Kanger in Stockholm and has been professor there for most of his career. He initially worked in probability theory (the Kolmogorov complexity of sequences) before turning to the foundations of constructive mathematics. His development of Intuitionistic Type Theory is the direct intellectual predecessor of HoTT; every major system of dependent type theory (Coq, Agda, Lean) descends from his work.

For this chapter, Martin-Löf's most relevant contributions are his proof-theoretic foundations. His 1971 paper "A Theory of Types" and its successors developed a type theory in which the Curry-Howard correspondence is not merely a correspondence but a foundational fact: types are propositions, terms are proofs, definitional equality is computational reduction, and the logic of the system is the constructive logic of intuitionistic predicate logic. The *judgment* $a : A$ ("$a$ is a proof of proposition $A$") is the fundamental notion; Section 1 of this chapter traces how judgments function in proof systems. Martin-Löf's 1984 Bibliopolis monograph *Intuitionistic Type Theory* (based on his Padova lectures) is the definitive reference.

Martin-Löf also contributed foundational work to proof-theoretic semantics, arguing (in a series of philosophical papers from the 1980s–1990s) that the meaning of a proposition is given by the rules that specify what counts as a canonical proof of it — a view that makes proof theory, not model theory, the primary semantic framework.

---

## Haskell B. Curry (1900–1982)
*Developed combinatory logic; observed the earliest form of the Curry-Howard correspondence.*

Curry was an American mathematician who spent his career at Penn State and later the University of Amsterdam. He developed combinatory logic in the 1920s–1930s as an alternative to Church's $\lambda$-calculus: a variable-free calculus of higher-order functions built from a small set of combinators ($S$, $K$, $I$). Combinatory logic and the $\lambda$-calculus are equivalent in expressive power.

Curry observed in 1934 that the types of the combinators $K$ and $S$ correspond precisely to the axioms $A \to (B \to A)$ and $(A \to (B \to C)) \to ((A \to B) \to (A \to C))$ of propositional logic — the axioms of the Hilbert-style proof system. This was the first observation of what would become the Curry-Howard correspondence. Howard's 1969 letter extended this to natural deduction and the full simply-typed $\lambda$-calculus. The correspondence is named for both men. The *currying* operation familiar to functional programmers — converting a function of two arguments to a function returning a function — is also named for Curry; it corresponds to the logical rule that $A \wedge B \to C$ is equivalent to $A \to (B \to C)$.
