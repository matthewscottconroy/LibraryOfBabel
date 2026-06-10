# Important Figures

## Ulf Norell (1979–present)
*Designer and principal implementer of Agda 2; architect of the interactive proof development model that defines the Agda experience.*

Ulf Norell completed his PhD at Chalmers University of Technology in 2007 under the supervision of Catarina Coquand and Patrik Jansson. His thesis, "Towards a Practical Programming Language Based on Dependent Type Theory," was not merely an academic exercise: it was an implementation. The Agda that Norell built in his thesis is the Agda that researchers use today — the elaboration algorithm, the implicit argument solving, the universe polymorphism, the `with`-clauses for deep pattern matching, and the interactive hole-filling development model are all Norell's design.

The practical innovations in Norell's Agda deserve emphasis because they are easy to take for granted. Before Norell's work, dependent type theory proof assistants (Coq, NuPRL, Lego) required users to supply fully explicit proof terms or to navigate a tactic language. Norell's insight was that the elaboration algorithm — the system that fills in implicit arguments, infers universe levels, and resolves type classes — should be powerful enough to allow programming in a style indistinguishable from ordinary functional programming in Haskell, while still checking against dependent types. The result is that Agda code looks like Haskell code and feels like programming, yet every definition is a formally verified mathematical object. This "practical" design philosophy explains why Agda became the preferred tool of the type theory research community.

For this chapter, Norell's contribution is the foundation: without his Agda, there is no Cubical Agda. The `--cubical` pragma is implemented within the Agda elaboration system that Norell designed. Every use of pattern matching, every `with`-clause, every implicit argument in the cubical library runs through machinery that is directly descended from Norell's 2007 thesis.

---

## Nils Anders Danielsson (present)
*Principal author of the Agda standard library; researcher in guarded type theory, partiality, and coinduction.*

Nils Anders Danielsson is a researcher at Chalmers University of Technology, working in the intersection of type theory, program verification, and semantics. His most visible contribution to the Agda community is as the primary architect and long-term maintainer of the Agda standard library (`agda-stdlib`), which provides the mathematical infrastructure — natural numbers, lists, vectors, relations, finite sets, functions — that most Agda developments depend on. Building a standard library for a dependently typed language is substantially harder than for a simply-typed language: every decision about how to represent equality, how to structure the algebraic hierarchy, and how to handle universe polymorphism has downstream consequences for every user.

Danielsson's research work is equally important for understanding the chapter's content. His work on guarded type theory and the modelling of coinductive types in type theory — types whose elements can be infinitely deep, like streams or possibly-infinite trees — developed the theoretical basis for productive coinduction in Agda. The `Size` annotation system in Agda (which allows writing coinductive definitions that the termination checker accepts) bears the influence of his work. For the cubical library specifically, Danielsson's influence is felt in the standard library foundations: the `--without-K` design of `agda-stdlib` (avoiding the K axiom, thereby being compatible with HoTT) reflects the community's recognition, which Danielsson helped articulate, that the K axiom is a non-trivial strength assumption.

His work on partial functions and the partiality monad — representing computations that may not terminate as types in a constructive type theory — is also directly relevant to the semantics of `hcomp`: the composition operation in cubical type theory is, in essence, a way of "filling in" partial cubes to total ones, and the formal treatment of partiality shares structural features with the face-formula system.

---

## Thierry Coquand (1961–present)
*Co-creator of the Calculus of Constructions; originator of the cubical sets model; one of the founding figures of the entire field.*

Thierry Coquand is a professor at Chalmers University of Technology and one of the most influential figures in type theory and constructive mathematics of the past forty years. His collaboration with Gérard Huet in the 1980s produced the Calculus of Constructions (CoC), which is the type theory underlying Coq, Lean, and (in extended form) Agda. The CoC paper established the basic framework of a dependent type theory with an impredicative universe of propositions — the architecture that all modern proof assistants inherit.

In the context of this chapter, Coquand's most important contribution is the cubical sets model of type theory, developed with Marc Bezem and Simon Huber beginning around 2013. The insight was that homotopy type theory — specifically the univalence axiom and higher inductive types — could be given *computational* content by interpreting types not as sets (the standard Tarski-style semantics) but as *cubical sets*: presheaves over a category of cubes. In this model, a path is literally a function from the interval $[0,1]$ to a type, and transport and composition have canonical definitions. This gave the first constructive model of univalence — establishing that HoTT is not just consistent but actually computable.

Coquand was also the lead author on the CCHM paper (with Cohen, Huber, and Mörtberg), which refined the Bezem-Coquand-Huber model into a full type theory with all the features needed for a working proof assistant. For the topics in this chapter — the interval, face formulas, `hcomp`, `transp`, the Glue type, univalence as a theorem, HITs with path constructors — Coquand's theoretical work is the direct source.

---

## Simon Huber (present)
*Implementer of the first cubical type checker; co-author of CCHM; contributor to the Agda cubical implementation.*

Simon Huber completed his PhD at Chalmers University of Technology under Coquand's supervision, with his 2016 thesis "Cubical Interpretations of Type Theory" providing the detailed type-theoretic treatment of the CCHM theory. The thesis contains the formal specification of all the cubical rules: the typing of dimension variables, the formation rules for path types, the `hcomp` and `transp` primitives with their computation rules, the Glue type and the proof of univalence, and the canonicity theorem for natural numbers. It is the most complete single reference for understanding the theory at the level of typing rules, and is essential reading alongside the CCHM paper for anyone working through the formal details.

Huber implemented the first stand-alone cubical type checker (`cubicaltt`) — a small, clean implementation of the CCHM theory separate from Agda, available at [github.com/mortberg/cubicaltt](https://github.com/mortberg/cubicaltt). This checker is valuable pedagogically: it is small enough to read completely (a few thousand lines of Haskell), and running it makes the reduction behavior of `transp` and `hcomp` visible. The `cubicaltt` language was the testing ground for the theory before it was implemented in Agda.

After the `cubicaltt` implementation, Huber (together with Vezzosi and Mörtberg) ported the cubical theory into Agda proper. This implementation work — adapting the cubical judgments to fit Agda's existing elaboration infrastructure, handling the interaction of cubical features with universe polymorphism and `with`-clauses, making the primitive operations typecheck in a system not originally designed for them — is the technical basis for the `--cubical` pragma that every line of code in this chapter depends on.

---

## Anders Mörtberg (present)
*Lead maintainer of the Cubical Agda library; key contributor to CCHM theory and its formalization; developer of the encode-decode proof of π₁(S¹) = ℤ.*

Anders Mörtberg is an associate professor at Stockholm University, having previously worked at Carnegie Mellon University and Chalmers. He is the primary architect and maintainer of the Cubical Agda library (`github.com/agda/cubical`), and one of the four co-authors of the CCHM paper. His research spans the theoretical development of cubical type theory, its implementation in Agda, and the mathematical content formalized in the cubical library.

Mörtberg's specific contributions to the topics in this chapter are pervasive. The library structure — `Cubical.Core`, `Cubical.Foundations`, `Cubical.HITs`, `Cubical.Homotopy` — is largely his design. The formalization of path algebra (symmetry, concatenation, the `_∙_` operation, filling arguments), equivalences (`isEquiv`, `Equiv`, the `_≃_` notation), univalence (`ua`, `uaβ`, `pathToEquiv`), and the foundations of h-level theory in `Cubical.Foundations` are primarily Mörtberg's work. The `Cubical.HITs.S1` module — defining the circle with `base : S¹` and `loop : base ≡ base`, and proving $\pi_1(S^1) \cong \mathbb{Z}$ via the encode-decode method — is one of the most-cited pieces of cubical Agda, and Mörtberg is among its principal authors.

The encode-decode proof in `Cubical.HITs.S1.Base` deserves special mention: it proves that the fundamental group of the circle is $\mathbb{Z}$ by defining a covering space (the universal cover $\widetilde{S^1} \simeq \mathbb{R}$, represented as $\mathbb{Z}$ in the discrete approximation), encoding paths as integers (winding number), and decoding integers as paths (iterated concatenation of `loop`). In Cubical Agda this is fully constructive — there are no axioms, and the computation `transport (loop-path n)` definitionally reduces. Mörtberg's formalization makes this reduction visible in the Agda interactive mode.

---

## Andrea Vezzosi (present)
*Agda core developer; implementer of cubical features in Agda; researcher in guarded cubical type theory.*

Andrea Vezzosi is a researcher at KTH Royal Institute of Technology, having completed a PhD at Chalmers. He is one of the three implementors who brought cubical type theory into Agda proper (with Mörtberg and Huber), and is a co-author of the Cubical Agda ICFP 2019 paper. Within the Agda repository, Vezzosi is responsible for significant portions of the cubical implementation — the reduction rules for `transp` applied to each type former (path type, Sigma type, record type, data type), the interaction of cubical primitives with Agda's existing pattern matching compilation, and the `hcomp` kernel.

Vezzosi's research has also pushed in the direction of *guarded cubical type theory* — a combination of cubical type theory with Nakano-style guarded recursion, which allows coinductive types (streams, infinite trees) and productive recursion to coexist with the cubical path structure. This is relevant to the chapter because it demonstrates that cubical type theory is not a narrow specialized tool but an architecture that can be extended in multiple directions. The `--guarded` pragma in Agda (still experimental) is Vezzosi's work.

For practical Cubical Agda users, Vezzosi's contribution is most felt in the reliability of the `transp` reduction rules. The rule that `transp (λ i → A) i0 a = a` (transport at a constant type is the identity) and the more complex reduction rules for Sigma types and path types are Vezzosi's implementation; when these reduce correctly in the interactive mode, it is his work that makes that possible.
