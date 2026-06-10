# Important Figures

## Cyril Cohen (present)
*Co-author of the CCHM paper; algebraist and type theorist; specialist in formalization of algebra and effective mathematics.*

Cyril Cohen is a researcher at Inria Sophia-Antipolis (France), working at the intersection of type theory, formal mathematics, and algebra. His mathematical background spans algebraic geometry, commutative algebra, and effective algorithms — fields where the gap between informal mathematics and rigorous computation is especially prominent. Cohen has been a contributor to the Mathematical Components library (Coq/Ssreflect) as well as to the cubical type theory project, bringing a mathematical practitioner's perspective to the theoretical development.

In the CCHM paper, Cohen's role included the treatment of composition for the Sigma type and Pi type — the most technically intricate parts of the paper, because composition for a Sigma type must simultaneously fill the base and the fiber in a coherent way. His contribution also shaped the face formula system: the choice to use a De Morgan algebra (with complement `~` as a primitive operation) rather than just a distributive lattice was partly motivated by the algebraic structure needed for coherent composition in dependent types. The `~` operation makes path reversal definitional — `sym (sym p) ≡ p` holds by reduction, not by a propositional proof — and this definitional symmetry propagates through the composition rules for all type formers.

Cohen's broader research context is relevant to understanding the chapter: his work on effective mathematics (algorithms in algebraic geometry that can be extracted as programs) meant that CCHM was developed with an eye not just toward logical consistency but toward computational efficiency. The canonicity result — that every closed term of type `ℕ` reduces to a numeral — is not just a theoretical desideratum but a prerequisite for using the type theory as a programming language in which verified programs can actually run.

---

## Thierry Coquand (1961–present)
*Co-creator of CoC and CCHM; the central theorist behind the cubical sets approach; the figure most responsible for the computational interpretation of univalence.*

Thierry Coquand is a professor at Chalmers University of Technology with research spanning constructive algebra, type theory, and the foundations of mathematics. His career arc — from the Calculus of Constructions in the 1980s to the cubical sets model in the 2010s — represents one of the most sustained contributions to the foundations of computer science of the past half-century.

For this chapter specifically, Coquand's crucial contribution is the identification of the *composition operation* as the key to making univalence computable. The earlier Bezem-Coquand-Huber 2013 model had the right idea (interval, cubical sets) but lacked the full Kan filling property. Coquand recognized that the De Morgan structure on the interval, combined with a composition operation that takes *open boxes* (cubes with specified faces) and fills them to *full cubes*, gives a type theory in which every computation terminates and every term of type `ℕ` normalizes to a numeral. This is a non-trivial insight: the Kan condition (every open box can be filled) is well-known in topology as the characterization of fibrant objects in the Quillen model structure, but translating it into a definitional computation rule required significant theoretical work.

Coquand has also contributed to the broader theoretical ecosystem around cubical type theory: the cofibrant replacement construction (how to handle HITs), the connection to Bishop's constructive mathematics (constructive Kan sets as a setting for computable analysis), and the development of XTT (a cubical type theory for extensional type theory with observable equality). His influence on the chapter is total: the interval, the face formulas, the composition and transport operations, the Glue type, and the canonicity theorem all trace directly to his theoretical work.

---

## Simon Huber (present)
*PhD student under Coquand; implementer of cubicaltt; co-author of CCHM; provider of the definitive formal treatment of the type theory.*

Simon Huber received his PhD from Chalmers University of Technology in 2016 under Coquand's supervision, with a thesis titled "Cubical Interpretations of Type Theory" that remains the most complete formal treatment of CCHM. Huber was responsible for the implementation of `cubicaltt` — the original stand-alone cubical type checker — and for the formalization of the full CCHM theory with all typing rules made explicit.

The technical contribution of Huber's thesis to this chapter is substantial. Section 2 of this chapter (composition and transport) draws directly on Huber's formal specification of `hcomp`. The typing rule for `hcomp`:

$$\frac{\Gamma \vdash A : \mathcal{U} \quad \Gamma, i : \mathbb{I} \vdash u : A[\phi] \quad \Gamma \vdash u_0 : A[i \mapsto 0][\phi \mapsto u]}{\Gamma \vdash \mathsf{hcomp}^A[\phi \mapsto u]\, u_0 : A}$$

with its boundary condition $\mathsf{hcomp}^A[\phi \mapsto u]\, u_0 = u_0$ when $\phi = 1$, is formalized in Huber's thesis with the full attendant side conditions on face formulas and consistency of the system $u$. Section 3 (the Glue type and univalence) similarly draws on Huber's formal treatment of the Glue formation rule and its reduction behavior.

Huber also contributed to the implementation of cubical primitives in Agda (with Mörtberg and Vezzosi), bringing the theory from the `cubicaltt` prototype to a production proof assistant. His practical implementation experience gives him a unique perspective on which aspects of the CCHM theory are straightforward to implement and which create subtle difficulties — experience that shaped the final form of the Cubical Agda primitives.

---

## Anders Mörtberg (present)
*Co-author of CCHM; principal Cubical Agda library maintainer; author of key pedagogical materials; the figure who most connects the theory to working formalization.*

(See also the ch22 important-figures entry; Mörtberg's contributions to both the theory and the implementation are central to both chapters.)

Mörtberg's specific contribution to the topics of *this* chapter (as distinct from Chapter 22's implementation focus) is his role in the Glue type and the proof of univalence within CCHM. The Glue type is the type constructor that enables univalence:

$$\mathsf{Glue}[\phi \mapsto (T, e)] A$$

is a type that "glues" a partial type $T$ (defined over the face $\phi$) to the ambient type $A$ via an equivalence $e : T \simeq A$. The proof that this gives univalence — that every equivalence $f : A \simeq B$ yields a path $\mathsf{ua}(f) : A = B$ — follows from the Glue formation rule and its computation rules. Mörtberg's contribution to this argument in the CCHM paper includes the key step that the composition rule for Glue types is consistent and computes correctly.

In the broader context of Section 4 (variations), Mörtberg has also contributed to the comparison between CCHM and Cartesian cubical type theory: his lectures and papers have clarified which features of CCHM depend on the De Morgan structure (definitional `sym`) and which can be replicated in the Cartesian setting (composition via the "box" operation). His pedagogical materials — lecture notes, the `cubicaltt` examples, and the cubical library — are the primary means by which researchers learn to work with the theory.

---

## Carlo Angiuli (present)
*Co-designer of Cartesian cubical type theory; contributor to computational type theory at CMU; implementer of cooltt.*

Carlo Angiuli received his PhD from Carnegie Mellon University under Robert Harper's supervision, with his thesis work focusing on the computational interpretation of higher-dimensional type theory. He is a co-author of the Angiuli-Favonia-Harper POPL 2017 paper on Computational Higher-Dimensional Type Theory (CHTT) — the Cartesian cubical approach to the same problems CCHM solves — and of subsequent papers refining and extending this approach.

The Cartesian cubical type theory that Angiuli co-developed takes a different route to computing with paths than CCHM. Rather than the De Morgan algebra (with `~`, `∧`, `∨` on the interval), Cartesian cubical uses only the two endpoints and a *box* operation — a composition in a specific direction that doesn't require the full De Morgan structure. The resulting type theory is arguably simpler to understand: there are fewer primitives, and the computational semantics is more directly motivated by the operational semantics of programs. The tradeoff is that path reversal (`sym`) is not definitional: `sym (sym p)` is only *propositionally* equal to `p`, not definitionally.

For Section 4 of this chapter (variations and comparisons), Angiuli's work is the primary reference. The CHTT approach has its own implementation (cooltt, which Angiuli helped develop) and its own growing community of users and researchers. Understanding the comparison between CCHM and CHTT — which design choices are forced by the goal of computable univalence, and which are contingent — is essential for a sophisticated understanding of the field.

---

## Favonia (Kuen-Bang Hou) (present)
*Co-designer of Cartesian cubical type theory; formalization researcher; contributor to computational HoTT and RedTT/RedPRL.*

Favonia (who publishes under the name Kuen-Bang Hou in some papers and Favonia in others) is an associate professor at the University of Minnesota, having done their PhD at CMU and postdoctoral work at IAS and CMU. Their work spans the computational interpretation of HoTT, the implementation of computational type theory (in the RedPRL, redtt, and cooltt systems), and the formalization of homotopy-theoretic results.

Favonia's specific contribution to this chapter's topics is the computational semantics of CHTT: their POPL 2017 paper (with Angiuli and Harper) gave the first type-theoretic presentation of a cubical theory with *definitional* computation rules for all type formers — including the fundamental result that `transport (ua e) a` definitionally reduces to `e a`. This computation rule, whether in the CCHM or CHTT version, is the payoff of the entire cubical enterprise: it makes univalence *computational* in the precise sense that matters.

Favonia has also contributed to the formalization of synthetic homotopy theory — the calculation of homotopy groups of spheres — in a computationally meaningful type theory. Their formalization of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ (following Brunerie's synthetic proof) using a computational type theory represents one of the most impressive results in the intersection of HoTT mathematics and proof assistant implementation. This work demonstrates that the canonicity result — every `ℕ` term reduces to a numeral — is not just a theoretical guarantee but an actual computational capability: the Brunerie number (the generator of $\pi_4(S^3)$) can, in principle, be computed.

---

## Lars Birkedal (1969–present)
*Semanticist; developer of guarded cubical type theory; theorist of step-indexed models and presheaf semantics.*

Lars Birkedal is a professor at Aarhus University (Denmark), leading the Logic and Semantics group. His research program spans denotational semantics, step-indexed models, separation logic, and type theory. Most relevant to this chapter is his development (with Rasmus Ejlers Møgelberg, Bas Spitters, Andrea Vezzosi, and others) of *guarded cubical type theory* — a combination of the cubical interval structure with guarded recursion.

Guarded type theory, developed earlier by Nakano and extended by Birkedal's group, provides a type-theoretic account of coinductive types (streams, infinite trees, potentially-diverging computations) by using a "later" modality `▷A` that marks computations that will be available one step in the future. The combination with cubical type theory allows guarded recursion and cubical paths to coexist: you can write functions that are both coinductively productive and path-respecting, opening the door to verified reactive programming in a HoTT setting.

For Section 4 (variations), Birkedal's work represents one direction in which cubical type theory is being extended: rather than restricting the interval (as Cartesian cubical does) or extending the algebraic structure (as some later variants do), guarded cubical type theory extends the *logic* that sits alongside the type theory. The resulting system can formalize results about programming language semantics — operational equivalence, contextual equivalence, step-indexed logical relations — that ordinary cubical type theory cannot express directly. This makes it relevant for anyone interested in using HoTT-based type theory for programming language semantics and verified compilation.
