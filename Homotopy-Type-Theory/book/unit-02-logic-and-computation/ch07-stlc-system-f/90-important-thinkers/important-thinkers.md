# Important Thinkers: STLC and System F

## Alonzo Church (1903–1995)

Church invented the lambda calculus in the 1930s as a proposed foundation for mathematics, and simultaneously proved that the Entscheidungsproblem (Hilbert's decision problem: is there an algorithm for deciding provability in first-order logic?) has a negative answer — by showing that the halting problem for lambda terms is undecidable. The lambda calculus and Turing machines, proved equivalent by Church and Turing in 1936, define the class of computable functions (the Church-Turing thesis). Church also invented the type theory (Church's simple theory of types, 1940) that laid the groundwork for STLC, and introduced the Church numerals, booleans, and the encoding of data types as functions.

## Haskell Brooks Curry (1900–1982)

Curry developed combinatory logic — an alternative to Church's lambda calculus that avoids variable binding, using fixed combinators $B$, $C$, $K$, $W$, $S$, $I$. Curry's 1934 paper on types for combinators first noted the correspondence between combinator types and logical axioms, the seed of the Curry-Howard correspondence. His 1958 textbook (with Feys) *Combinatory Logic* is the systematic treatment. Curry was also an early contributor to metamathematics and intuitionistic logic.

## William Alvin Howard (born 1926)

Howard's 1969 manuscript "The Formulae-as-Types Notion of Construction" (circulated as a preprint, published 1980) established the Curry-Howard correspondence as a theorem: the natural deduction rules for intuitionistic propositional logic are isomorphic to the typing rules of STLC, and proof normalization corresponds to beta reduction. Howard also observed that predicate logic corresponds to dependent types, previewing Martin-Löf's type theory.

## Robin Milner (1934–2010)

Milner developed *ML* (Meta Language, 1973), the first practical programming language with a Hindley-Milner type system and automatic type inference. ML demonstrated that polymorphic type checking could be efficient and usable in practice, making the theoretical insights of System F accessible to programmers. Milner also invented *CCS* (Calculus of Communicating Systems), a process algebra for concurrent computation; the *pi-calculus*, a model for mobile concurrent processes; and *bigraphs*, a unifying framework. His work bridges theory and practice in programming languages. He received the Turing Award in 1991.

## Jean-Yves Girard (born 1947)

Girard invented System F in his 1971 doctoral thesis as a type-theoretic representation of second-order propositional logic, proved its strong normalization using candidates of reducibility, and derived from this the consistency of second-order arithmetic. He later invented linear logic (1987) and the Geometry of Interaction (1989). Girard's contributions span proof theory, type theory, and mathematical logic. His 1989 book *Proofs and Types* (with Lafont and Taylor) is an accessible introduction to the Curry-Howard correspondence and typed lambda calculi. He has also written provocatively (and sometimes cryptically) on the foundations of logic and the limits of mathematical reason.

## John C. Reynolds (1935–2013)

Reynolds independently discovered System F in 1974 (slightly after Girard, but independently) in his work on *Forsythe*, a programming language with parametric polymorphism. Reynolds introduced *parametricity* — the semantic framework for free theorems — in his 1983 paper "Types, Abstraction, and Parametric Polymorphism." He also invented *defunctionalization* (converting higher-order programs to first-order ones), *CPS transformation*, and *separation logic* (with Peter O'Hearn), a logic for reasoning about programs with pointers and mutable state. Reynolds was also known for his careful philosophical and conceptual writing about programming languages and their semantics.

## Roger Hindley (born 1938)

Hindley proved (1969) that every typable term in STLC with Curry-style typing has a *principal type* — a most general type from which all other types of the term can be derived by substitution. He proved this using a syntactic unification algorithm. This result is the theoretical foundation of ML-style type inference.

## Robin Milner (type inference, again) and Damas

Luis Damas and Robin Milner (1982) proved the completeness of the *Hindley-Milner type inference algorithm* (also known as Algorithm W). They proved that for every typeable term in the rank-1 polymorphic fragment of System F, Algorithm W produces the principal type in polynomial time. The algorithm is the basis of type inference in ML, Haskell, OCaml, and many other languages. Damas' doctoral thesis contained the formal proof.

## Henk Barendregt (born 1947)

Barendregt is the foremost authority on the lambda calculus, known for his comprehensive 1984 monograph *The Lambda Calculus: Its Syntax and Semantics*, which remains the standard reference. In 1991, he organized the *lambda cube*, classifying eight type systems by their allowed dependencies between terms and types. The lambda cube provides a unified framework for understanding STLC, System F, System F$\omega$, LF (Logical Framework), and the Calculus of Constructions. Barendregt's work has also contributed to church-style type theory, intersection types, and the theory of pure type systems.

## Thierry Coquand (born 1961)

Coquand introduced the Calculus of Constructions (CoC, with Gérard Huet) and is one of the main developers of the type theories underlying Coq and Agda. He also developed *cubical type theory*, which provides a computational interpretation of the univalence axiom for HoTT. His contributions to type theory span pure theory and practical implementation.
