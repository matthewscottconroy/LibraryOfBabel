# Important Thinkers in Proof Theory

## Gerhard Gentzen (1909–1945)

Gentzen is the father of proof theory as a discipline. In his 1935 paper *Untersuchungen über das logische Schließen* (Investigations into Logical Deduction), written at age 25, he introduced both natural deduction and sequent calculus, proved the normalization theorem for natural deduction, and proved cut elimination for sequent calculus. He then used sequent calculus to prove the consistency of Peano Arithmetic using transfinite induction up to $\varepsilon_0$ — the cleanest possible consistency proof, using precisely the minimal principle that Peano Arithmetic cannot itself prove. Gentzen died in 1945 at age 35 in a Czech internment camp after World War II. His work, published during the war, was not widely appreciated until the 1950s and 1960s, when it became the foundation of structural proof theory.

## Dag Prawitz (born 1936)

Prawitz is the logician who proved the normalization theorem for natural deduction in its modern, precise form. His 1965 monograph *Natural Deduction: A Proof-Theoretical Study* proved weak and strong normalization for intuitionistic natural deduction and established the subformula property as a consequence. He introduced the terminology of "redex," "normal form," and the explicit reduction rules for each connective. Prawitz also developed the theory of *general elimination rules* — elimination rules stated in a uniform format that makes the introduction-elimination correspondence completely explicit. His work made proof theory a mature mathematical subject.

## William Tait (born 1929)

Tait introduced the *logical relations* method (also called the *reducibility candidates* method) for proving strong normalization. His 1967 paper "Intensional Interpretations of Functionals of Finite Type I" defined the reducibility predicate for the simply typed lambda calculus and proved that all well-typed terms are reducible, hence strongly normalizing. This method was later extended by Girard to System F and by Martin-Löf to dependent type theory. The logical relations method remains the standard tool for normalization proofs and is central to research in type theory and programming language semantics.

## Jean-Yves Girard (born 1947)

Girard extended the proof-theoretic tradition in several directions. His 1971 doctoral thesis proved strong normalization for System F (second-order polymorphic lambda calculus) using an elaborated version of Tait's method involving *candidates of reducibility*. He invented *linear logic* (1987), a resource-sensitive logic that splits the classical structural rules into multiplicative and additive versions, giving a fine-grained account of proof structure. Linear logic has applications in concurrency, type theory, and the study of proof nets — a two-dimensional notation for proofs that makes cut elimination into a geometric operation. Girard also invented the *geometry of interaction*, a dynamic interpretation of proof normalization using operator algebras.

## Per Martin-Löf (born 1942)

Martin-Löf synthesized the insights of proof theory, intuitionistic logic, and programming language theory into *Martin-Löf Type Theory* (MLTT), first published in 1973. MLTT internalizes the Curry-Howard correspondence: propositions are types, proofs are terms, and the four fundamental judgments (type formation, term introduction, type equality, term equality) are the judgment forms of the theory. Martin-Löf proved strong normalization for MLTT and established it as a foundation for constructive mathematics. His philosophical writings on the meaning of the logical constants — grounding them in the introduction rules — have been influential in both logic and philosophy of mathematics. MLTT is the direct precursor of HoTT.

## Kurt Gödel (1906–1978)

Although not a proof theorist in the technical sense, Gödel's contributions are foundational. His Incompleteness Theorems (1931) established that any consistent formal system strong enough to express arithmetic cannot prove its own consistency — the context in which Gentzen's consistency proof must be understood. His completeness theorem (1930) established soundness and completeness of first-order classical logic, giving the syntactic and semantic notions their canonical relationship. His *Dialectica interpretation* (1958) gave a computational interpretation of classical arithmetic via functionals of finite type — a precursor to the Curry-Howard correspondence for classical logic.

## Stephen Cole Kleene (1909–1994)

Kleene developed *realizability*, the first precise mathematical interpretation of the BHK conditions. His 1945 paper defined a formula as realizable if there is a recursive function that computes a witness for it, giving a rigorous account of "constructive proof." Kleene also proved fundamental results about the meta-theory of intuitionistic logic — the disjunction property and existence property for formal intuitionistic arithmetic — and developed the theory of recursive functions that underlies computability theory. His connection between realizability and proof theory anticipated the Curry-Howard correspondence.

## Helmut Schwichtenberg (born 1940)

Schwichtenberg has contributed systematically to proof theory and its computational applications, particularly in the analysis of *proof terms* and the *extraction of programs from proofs*. His work on *program extraction* — transforming constructive proofs into verified programs — is a direct application of the Curry-Howard correspondence in proof-theoretic terms. He is also known for work on *term rewriting systems* and the proof theory of *arithmetic* and *analysis*. His textbook with Anne Schiemann Troelstra is a standard reference in proof theory.

## Samson Abramsky (born 1953)

Abramsky bridged proof theory and computer science through *game semantics* and the *Geometry of Interaction*. He showed that Girard's Geometry of Interaction could be understood as a model of computation via games, where proofs are strategies and cut elimination is the composition of strategies. This gave a fully abstract denotational semantics for PCF (a model programming language) and connected proof theory to the semantics of programming languages. Abramsky also introduced *interaction categories* and *traced monoidal categories*, giving categorical foundations for feedback and recursion in computation.
