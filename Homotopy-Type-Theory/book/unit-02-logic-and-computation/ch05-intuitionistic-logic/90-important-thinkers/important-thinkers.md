# Important Thinkers in Intuitionistic Logic

## L.E.J. Brouwer (1881–1966)

Luitzen Egbertus Jan Brouwer is the founder of intuitionism and the first mathematician to systematically reject the Law of Excluded Middle in mathematics. His philosophical position — that mathematics is a mental activity in which mathematical objects are constructed rather than discovered — is called *mathematical intuitionism*. His 1907 doctoral dissertation first articulated the intuitionistic critique of classical logic. His 1920s work on the "creating subject" and *bar induction* developed a uniquely Brouwerian constructive analysis. Brouwer was also a major figure in topology: his fixed-point theorem, invariance of domain, and the concept of dimension are classical results. His philosophical intransigence and the "mathematische Grundlagenstreit" (foundations dispute) with Hilbert is one of the most dramatic intellectual conflicts in modern mathematics.

## Arend Heyting (1898–1980)

Heyting formalized Brouwer's intuitionistic logic into an explicit axiom system — the Heyting calculus, which is now the standard formalization of IPC. His 1930 paper gave the first formal axiomatization of intuitionistic propositional logic, and his 1956 monograph *Intuitionism: An Introduction* made intuitionistic mathematics accessible to a wider audience. Heyting also articulated what is now called the BHK interpretation — the clauses specifying what a constructive proof of each connective consists of. He gave the interpretation of intuitionistic implication as a "function" explicitly and precisely, anticipating the Curry-Howard correspondence.

## Andrei Nikolaevich Kolmogorov (1906–1987)

Kolmogorov contributed to the BHK interpretation independently of Heyting, in a 1932 paper interpreting intuitionistic logic as a "calculus of problems" — a proposition is interpreted as a problem, and a proof is a solution. This is remarkably close to the Curry-Howard correspondence. Kolmogorov is also the founder of modern probability theory and a pioneer in information theory and algorithmic complexity. His interpretation of intuitionistic logic connects to his later work on algorithmic randomness: the "Kolmogorov complexity" of a string is related to the computational content of a constructive proof of its existence.

## Andrei Andreevich Markov (Jr.) (1903–1979)

Markov (the son, not the probabilist father) developed the Russian school of constructive mathematics, based on recursive functions and explicit algorithms. Markov's principle — that for decidable predicates, double negation elimination holds over existential statements — is named for him. The Russian constructivist school differs from Bishop's school in accepting Markov's principle and in identifying "construction" with recursive computation. Markov developed the theory of *normal algorithms* (Markov algorithms, a variant of string rewriting systems) as an alternative foundation for computability.

## Errett Bishop (1928–1983)

Bishop's 1967 book *Foundations of Constructive Analysis* is the most significant single work in 20th-century constructive mathematics. He demonstrated that virtually all of classical analysis — real analysis, complex analysis, measure theory, functional analysis — can be developed constructively, without LEM or non-effective existence arguments. His constructivism is intentionally neutral on foundational questions: he does not argue for any particular philosophical position, but simply demands that proofs carry computational content. Bishop's work inspired the *Formal Topology* program in Sweden (Sambin, Coquand) and the development of constructive algebra and analysis in proof assistants.

## Saul Kripke (born 1940)

Kripke developed the possible-worlds semantics for modal logic in 1959 (at age 18!) and extended it to intuitionistic logic in 1965. Kripke semantics for IPC — frames with partial orders, forcing relations, and monotonicity conditions — gave intuitionistic logic its first fully satisfactory semantics, making it possible to prove completeness and to construct explicit countermodels to classical principles. Kripke is also known for his Naming and Necessity lectures (1970) on modal metaphysics and the rigid designation of proper names, and for his influential (and controversial) 1982 book on Wittgenstein on rules and private language.

## William A. Howard (born 1926)

Howard's 1969 manuscript "The Formulae-as-Types Notion of Construction" — unpublished until 1980 — made the connection between natural deduction proofs and typed lambda terms precise. His observation that the simply typed lambda calculus is isomorphic to intuitionistic natural deduction, and that proof normalization corresponds to beta reduction, is one of the great insights in foundations. Howard was responding to Curry's earlier observations and developing them into a complete correspondence. The Curry-Howard correspondence is named partly for Howard's contributions.

## Per Martin-Löf (born 1942)

Martin-Löf synthesized intuitionistic logic, proof theory, and computation into Martin-Löf Type Theory (MLTT), first presented in 1973. MLTT takes the BHK interpretation as its foundation: propositions are types, proofs are terms, and the four fundamental judgments of the theory (type formation, term formation, type equality, term equality) formalize the constructive meaning of the logical connectives. Martin-Löf's philosophical writings on the "meaning explanations" of type theory — grounding the theory in pre-formal intuitions about mathematical objects — have been influential in both logic and philosophy of mathematics. His 1984 Padova lectures (*Intuitionistic Type Theory*) are essential reading.

## Thierry Coquand (born 1961)

Coquand, working with Gérard Huet, developed the Calculus of Constructions (CoC) in the late 1980s, which became the foundation of the Coq proof assistant. He has since been a central figure in constructive foundations, contributing to the development of the Calculus of Inductive Constructions, type theory for homotopy theory (cubical type theory), and formalization of constructive mathematics in Agda and Coq. His work on *constructive set theory* and *formal topology* extends Bishop's program into type-theoretic foundations. The *cubical type theory* he developed with colleagues provides a computational interpretation of the univalence axiom, solving a major open problem in HoTT.
