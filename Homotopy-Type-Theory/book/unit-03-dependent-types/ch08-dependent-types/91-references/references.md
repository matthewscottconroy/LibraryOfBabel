# References: Chapter 8

## Primary Sources

**Martin-Löf, Per.** "An Intuitionistic Theory of Types: Predicative Part." In *Logic Colloquium '73*, edited by H. E. Rose and J. C. Shepherdson, 73–118. North-Holland, 1975.
— The foundational paper. Introduces dependent products, dependent sums, ℕ, and the universe hierarchy. Dense but precise; every rule is given explicitly.

**Martin-Löf, Per.** *Intuitionistic Type Theory* (Bibliopolis Notes). Bibliopolis, 1984.
— The expanded lecture notes from Padova 1980. More accessible than the 1975 paper; covers the four judgments, the meaning explanations, and identity types. Available free online. Strongly recommended as a primary source.

**Howard, William A.** "The Formulae-as-Types Notion of Construction." In *To H. B. Curry: Essays on Combinatory Logic, Lambda Calculus, and Formalism*, edited by J. P. Seldin and J. R. Hindley, 479–490. Academic Press, 1980.
— Howard's original 1969 note, finally published. The foundational document of the propositions-as-types idea.

**Coquand, Thierry, and Gérard Huet.** "The Calculus of Constructions." *Information and Computation* 76, no. 2–3 (1988): 95–120.
— Introduces the Calculus of Constructions. Foundational for Coq/Rocq.

## Standard Textbooks

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics*. Institute for Advanced Study, 2013.
— Chapter 1 covers all the material of Chapter 8 and Chapter 9 in a unified treatment. Available free at homotopytypetheory.org. The standard reference for the field.

**Nordström, Bengt, Kent Petersson, and Jan M. Smith.** *Programming in Martin-Löf's Type Theory*. Oxford University Press, 1990.
— The most comprehensive account of MLTT from the 1980s. Covers the formal rules in full detail. Available free from the authors' websites.

**Thompson, Simon.** *Type Theory and Functional Programming*. Addison-Wesley, 1991.
— More accessible introduction to MLTT from a programming language perspective. Good for intuition before formalism.

**Sorensen, Morten Heine, and Pawel Urzyczyn.** *Lectures on the Curry-Howard Isomorphism*. Elsevier, 2006.
— Thorough treatment of the propositions-as-types correspondence, including the dependent case.

## On Inductive Types

**Dybjer, Peter.** "Inductive Sets and Families in Martin-Löf's Type Theory and Their Set-Theoretic Semantics." In *Logical Frameworks*, edited by G. Huet and G. Plotkin, 280–306. Cambridge University Press, 1991.
— The standard account of inductive types in MLTT. Introduces the pattern of specifying inductive types by their recursors.

**Paulin-Mohring, Christine.** "Inductive Definitions in the System Coq." In *TLCA 1993*, Lecture Notes in Computer Science, 328–345. Springer, 1993.
— How inductive types were added to the Calculus of Constructions. Foundational for Coq.

**Abbott, Michael, Thorsten Altenkirch, and Neil Ghani.** "Containers: Constructing Strictly Positive Types." *Theoretical Computer Science* 342, no. 1 (2005): 3–27.
— A uniform treatment of strictly positive inductive types via containers. More advanced but clarifies the theory behind the positivity restriction.

## On Universes and Girard's Paradox

**Hurkens, Antonius J. C.** "A Simplification of Girard's Paradox." In *TLCA 1995*, Lecture Notes in Computer Science, 266–278. Springer, 1995.
— The simplified proof that Type : Type is inconsistent. More accessible than Girard's original.

**Luo, Zhaohui.** *Computation and Reasoning: A Type Theory for Computer Science*. Oxford University Press, 1994.
— Covers ECC (Extended Calculus of Constructions) with a universe hierarchy. Good treatment of universe polymorphism.

## Proof Assistants

**The Agda Documentation.** Available at agda.readthedocs.io.
— The official documentation for Agda, including the standard library. The HoTT development in Agda uses `--without-K` or `--cubical`.

**Lean 4 Documentation.** Available at leanprover.github.io.
— Lean 4's dependent type theory. Lean uses Russell-style universes with built-in universe polymorphism.

**The Coq Reference Manual.** Available at coq.inria.fr.
— The official reference for Coq (now Rocq). The most detailed account of the Calculus of Inductive Constructions.

**Brady, Edwin.** *Type-Driven Development with Idris*. Manning, 2017.
— A practical introduction to dependent types in Idris, the dependently typed programming language. More accessible than formal type theory texts; focuses on programming applications.
