# Important Thinkers in Cubical Type Theory

## Thierry Coquand (University of Gothenburg)

Coquand is the central figure of the entire program. His career spans Coq (which he co-designed with Huet), type theory, proof assistants, and constructive mathematics. The CCHM paper bears his name prominently — and the "C" in CCHM is, in fact, his initial.

Coquand's philosophical commitment is to *constructivity*: mathematics should be computable, proofs should be programs, and the foundations should not postulate things that cannot be witnessed. The canonicity problem in Book HoTT was, for him, not a technical nuisance but a genuine foundational failure. The cubical solution — making univalence computational — is the expression of this commitment at the level of the foundational theory.

Beyond CCHM, Coquand has worked on the *setoid model* of type theory, constructive cubical sets, and most recently on synthetic algebraic geometry in cubical type theory. His ongoing program connects cubical foundations to algebraic geometry, attempting to prove results about schemes and algebraic varieties in a computationally complete foundation.

## Cyril Cohen (INRIA Sophia-Antipolis)

Cohen's background is in mathematical components (MathComp), the Coq library for computational algebraic geometry and number theory. He brought to CCHM the perspective of a working formalization mathematician: what does a type theory need to actually support proofs of hard mathematical results?

Cohen's work on the De Morgan algebra structure of the interval — the specific choice to include complement — was motivated by computational considerations. With complement, path reversal is definitional. Without it, many constructions that should be "obvious" become bureaucratic. Cohen's insistence on the De Morgan structure gave cubical type theory the operational smoothness that makes working with paths in Cubical Agda practical.

## Simon Huber (Chalmers University)

Huber's role was foundational in the technical sense: he proved the *metatheorems* for CCHM. His PhD thesis establishes canonicity, consistency, and the model-theoretic properties of the system. The proof of canonicity for CCHM — that every closed term of type $\mathbb{N}$ normalizes to a numeral — is Huber's.

The canonicity proof is non-trivial. It requires constructing a *realizability model* for cubical type theory in which the interval is interpreted by the actual unit interval $[0,1]$, and showing that the interpretation of `hcomp` and `transp` gives correct values. The Glue type requires careful treatment in this model.

Huber is also responsible for the implementation of Cubical Agda, working with the Agda development team to integrate the cubical primitives into the existing Agda architecture.

## Anders Mörtberg (Stockholm University)

Mörtberg is the primary developer and maintainer of Cubical Agda and the cubical library. He has done more than anyone to make the CCHM theory practically usable: designing the API for cubical operations, building the library, writing documentation, fixing bugs, and guiding users.

Mörtberg has also worked on the *Brunerie number* computation — one of the most striking demonstrations of Cubical Agda's power. The Brunerie number is a term of type $\mathbb{Z}$ defined by Brunerie using the homotopy group $\pi_4(S^3)$. In Book HoTT, it is stuck. In Cubical Agda, it evaluates to $-2$.

More recently, Mörtberg has worked on *synthetic algebraic geometry* in Cubical Agda: formalizing results from algebraic geometry (Zariski topology, étale maps) using cubical foundations.

## Carlo Angiuli (Carnegie Mellon University)

Angiuli developed *Cartesian cubical type theory* (CCTT) and the `cooltt` implementation, in collaboration with Jon Sterling and Daniel Gratzer. CCTT strips the De Morgan complement from the interval, giving a simpler theory whose metatheory is more tractable.

Angiuli's contributions are on the theoretical side: NbE for cubical type theory, rigorous canonicity proofs for CCTT, and connections to *computational type theory* (CTT) — the tradition going back to Per Martin-Löf and Nuprl. His perspective is that cubical type theory should be grounded in a clear operational semantics, not just a model-theoretic one.

## Jon Sterling (Cambridge / Carnegie Mellon)

Sterling is the co-designer of XTT and the `cooltt` implementation. His work focuses on the *synthetic* aspects of type theory: treating type theory itself as an object of mathematical study, using the tools of category theory and sheaf theory.

Sterling's XTT paper introduced boundary separation — the principle that terms agreeing on all faces are definitionally equal — and developed its consequences for the path groupoid laws. He has also worked on *synthetic Tait computability*, a general framework for constructing NbE models of type theories, which is the theoretical foundation for the `cooltt` implementation.

Sterling's broader program, influenced by Longley and MacIntyre's realizability theory, aims to give type theory a rigorous operational semantics grounded in computability theory, connecting the proof-theoretic and program-theoretic perspectives.

## Guillaume Brunerie (Stockholm University)

Brunerie's contribution is more focused but singularly striking: the computation of $\pi_4(S^3)$ in HoTT, with Cubical Agda as the computational witness.

In his PhD thesis, Brunerie defined a term $\mathsf{Brunerie} : \mathbb{Z}$ whose value, if the fundamental theorem of algebraic topology is to be believed, must be $\pm 2$. The term is defined via a chain of synthetic homotopy constructions: the Hopf fibration, the Freudenthal suspension theorem, and the James splitting. In Book HoTT, evaluating this term requires computation that cannot proceed due to the non-computational `ua`.

In Cubical Agda, it runs. Brunerie and Mörtberg verified that the evaluation terminates and returns $-2$. This is the most concrete demonstration that cubical type theory has achieved its goal: not just formal computation, but actual, running computation of non-trivial mathematical results.
