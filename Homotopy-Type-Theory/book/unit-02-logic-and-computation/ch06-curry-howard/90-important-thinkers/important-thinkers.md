# Important Thinkers: The Curry-Howard Correspondence

## Haskell B. Curry (1900–1982)

Curry was an American logician who spent his career developing combinatory logic — a reformulation of the lambda calculus using fixed combinators rather than variable binding. In the 1930s, he noticed that the types of the basic combinators (B, C, K, W) correspond to axioms of implicational logic. He also noticed (in his 1934 paper) that type systems for lambda calculus could be organized to mirror logical systems. These observations were made casually, not developed into a full correspondence, and Curry himself did not pursue their deepest implications. The Curry-Howard correspondence bears his name because he first noticed the structural similarity, even if he did not make it precise.

## William A. Howard (born 1926)

Howard's 1969 manuscript (unpublished until 1980) "The Formulae-as-Types Notion of Construction" is the foundational document of the Curry-Howard correspondence as a precise theorem. Howard showed that the natural deduction rules for intuitionistic propositional logic are isomorphic to the typing rules of the simply typed lambda calculus, and that proof normalization corresponds to beta reduction. He also observed that predicate logic corresponds to dependent types, anticipating Martin-Löf's type theory. Howard's manuscript was widely circulated before publication and was the direct inspiration for much of the subsequent work in type theory and proof assistants.

## Per Martin-Löf (born 1942)

Martin-Löf extended the Curry-Howard correspondence from propositional to predicate logic by introducing dependent types — types that depend on values. His 1973 (first version) and 1975 (published version) Martin-Löf Type Theory introduced the $\Pi$ and $\Sigma$ types corresponding to universal and existential quantification, the identity type, and a universe hierarchy. Martin-Löf's philosophical contribution is equally important: his "meaning explanations" for type theory ground the formal rules in pre-formal intuitions about mathematical objects, making MLTT a foundational system rather than just a formal calculus. MLTT is the direct precursor of Coq, Agda, Lean, and the type theory underlying HoTT.

## Nicolaas Govert de Bruijn (1918–2012)

De Bruijn invented Automath, the first proof assistant, and independently discovered many of the ideas underlying dependent type theory. He introduced the concept of *de Bruijn indices* (representing bound variables by the number of binders between the variable and its declaration, avoiding variable capture), which is used in essentially all modern proof assistant implementations. His work on Automath demonstrated that large portions of classical mathematics could be formalized mechanically, at a time when this seemed impractical. De Bruijn was also a mathematician of considerable breadth, with major results in combinatorics (the BEST theorem on Eulerian circuits) and complex analysis.

## Jean-Yves Girard (born 1947)

Girard extended the Curry-Howard correspondence to second-order logic via System F (developed in his 1971 doctoral thesis), proving that universal quantification over types corresponds to polymorphic functions and that the resulting system is strongly normalizing. He invented linear logic in 1987, extending the correspondence to resource-sensitive logic. His *Geometry of Interaction* program interprets proof normalization as the composition of strategies in a dynamical system (modeled by operator algebras or traced monoidal categories), giving a more computational view of cut elimination. Girard's work connects logic to computer science, algebra, and mathematical physics.

## Thierry Coquand (born 1961)

Coquand developed the Calculus of Constructions (CoC) with Gérard Huet, which combines System F with dependent types to give a single framework for both programs and proofs. CoC is the theoretical foundation of Coq. Coquand has also contributed to the development of *cubical type theory*, which provides a computational (rather than axiomatic) interpretation of the univalence axiom in HoTT, solving a major open problem. His work on the *setoid model* of type theory and on *constructive set theory* connects type theory to other foundational frameworks.

## Christine Paulin-Mohring

Paulin-Mohring developed the *Calculus of Inductive Constructions* (CIC) by adding inductive types to CoC, making it possible to define natural numbers, lists, trees, and other recursive data structures within the type theory. Her 1993 doctoral thesis established the theoretical foundations of CIC, which is the type theory underlying modern Coq. The addition of inductive types to CoC is the step that made proof assistants practically useful for mathematics, since most mathematical structures are defined inductively.

## Leonardo de Moura (born 1972)

De Moura is the primary developer of Lean 4, the most powerful current proof assistant for mainstream mathematics. His earlier work on the Z3 SMT solver (an industrial satisfiability modulo theories solver) brought formal methods to a wide engineering audience. Lean 4 combines a dependent type theory with classical axioms and a powerful tactic system, making it accessible to mathematicians without deep type theory background. The Lean community's Mathlib project has formalized more mathematics than any previous effort, demonstrating that the Curry-Howard correspondence can scale to research-level mathematics.

## Vladimir Voevodsky (1966–2017)

Voevodsky was a Fields Medal-winning algebraic geometer who turned, in the last years of his career, to the foundations of mathematics — partly out of concern that some of his own complex proofs might contain undetected errors. He developed the *Univalent Foundations* program, built on HoTT, as a new foundation for mathematics in which the concept of equality is enriched by homotopy theory. His discovery of the *univalence axiom* — that equivalent types are equal — is the central new idea in HoTT. He formalized significant parts of algebraic K-theory in Coq and advocated for formalization as a standard of mathematical practice. His early death in 2017 was a profound loss to mathematics.
