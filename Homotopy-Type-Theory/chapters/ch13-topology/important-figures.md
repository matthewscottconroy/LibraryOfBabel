# Important Figures

## Henri Poincaré (1854–1912)
*Mathematician and physicist; founder of algebraic topology and of the study of the fundamental group.*

Henri Poincaré was perhaps the last mathematician to work across all areas of pure and applied mathematics simultaneously: he made major contributions to differential equations (Poincaré recurrence theorem, qualitative theory of ODEs), celestial mechanics, complex analysis (uniformization theorem), non-Euclidean geometry, special relativity, and — most relevant here — topology. He was born in Nancy, France, trained at the École Polytechnique and the École des Mines, and became a professor at the Sorbonne where he remained until his early death at 58.

For Chapter 13, Poincaré's foundational paper "Analysis Situs" (1895) is the starting point for the entire subject. In it, he defined what is now called the **fundamental group** $\pi_1(X, x_0)$: the group of homotopy classes of loops based at a point $x_0$, with loop concatenation as the group operation. He recognized that this group is a topological invariant — if two spaces are homeomorphic, their fundamental groups are isomorphic. He also introduced **homology groups** (initially as Betti numbers, later systematized by Noether into groups) and **Poincaré duality** (for closed orientable manifolds, $H_k(M) \cong H_{n-k}(M)$). His "complements" to Analysis Situs introduced torsion and developed the computational tools.

Poincaré's lasting influence on Chapter 13 is total: the central objects (fundamental group, homology, homotopy invariants) and the central questions (how to distinguish spaces; what are the algebraic invariants of a space?) are his. He also famously stated, in 1904, what is now called the **Poincaré conjecture**: every simply connected closed 3-manifold is homeomorphic to the 3-sphere. This conjecture drove much of 20th-century topology and was only proved in 2003 by Grigori Perelman.

---

## Felix Hausdorff (1868–1942)
*Mathematician; founder of point-set topology and of the modern abstract theory of topological spaces.*

Felix Hausdorff was born in Breslau (now Wrocław, Poland) and was a professor at Bonn and Greifswald. His mathematical interests were wide — he also wrote under the pseudonym Paul Mongré, publishing philosophical essays and a literary work — but his lasting contribution is the rigorous foundation of set-theoretic topology. Tragically, Hausdorff and his wife committed suicide in January 1942 to escape deportation to a Nazi concentration camp.

Hausdorff's 1914 book *Grundzüge der Mengenlehre* established modern point-set topology. Before Hausdorff, "topological space" had no standard definition; the field worked with metric spaces and vague notions of "nearness." Hausdorff gave the first systematic axiomatization: a topological space is a set with a collection of "neighborhoods" satisfying four axioms (now called the Hausdorff axioms, slightly stronger than the modern open-set axioms). The **Hausdorff separation axiom** (T₂) — that distinct points have disjoint neighborhoods — is named for him and is the condition that makes many theorems of point-set topology work cleanly.

Hausdorff proved foundational results on **compactness** (characterizing compact metric spaces as totally bounded and complete), **completeness** (the Baire category theorem for complete metric spaces), and the structure of **totally disconnected** and **zero-dimensional** spaces. He proved what is now called the **Hausdorff maximality principle** (every partially ordered set has a maximal totally ordered subset), an equivalent of the axiom of choice and a tool used repeatedly in topology. The concept of a **Hausdorff dimension** for fractal sets is also due to him (1919). Every chapter in Chapter 13 — spaces, separation, compactness — is shaped by Hausdorff's framework.

---

## L.E.J. Brouwer (1881–1966)
*Mathematician; proved the fixed-point theorem and degree theory; founder of the intuitionistic critique of classical logic.*

Luitzen Egbertus Jan Brouwer was a Dutch mathematician with a double legacy: in pure topology, where he proved some of the deepest theorems of the early 20th century, and in the foundations of mathematics, where he developed intuitionism — the view that mathematical objects exist only insofar as they can be mentally constructed.

For Chapter 13, Brouwer's topological contributions are central. In 1910–1912 he proved the **Brouwer fixed-point theorem**: every continuous map from the closed $n$-disk $D^n$ to itself has a fixed point. This is one of the most celebrated theorems in mathematics, with applications in economics (Nash equilibria), differential equations (existence theorems), and game theory. The proof rests on the fact that there is no continuous retraction of $D^n$ onto its boundary $S^{n-1}$ — a fact proved using degree theory.

Brouwer also proved the **invariance of domain theorem** (1912): if $U \subseteq \mathbb{R}^n$ is an open set and $f : U \to \mathbb{R}^n$ is a continuous injective map, then $f(U)$ is open. This established the topological invariance of dimension — a seemingly obvious fact that requires genuine work to prove. And he introduced the concept of **simplicial approximation** (any continuous map between polyhedra can be approximated by a simplicial map), a tool used throughout combinatorial and algebraic topology.

Brouwer's intuitionism has a surprising connection to HoTT: the Curry-Howard correspondence between proofs and programs and the constructive character of HoTT both resonate with Brouwer's insistence that mathematical objects be constructed. In Chapter 13's context, the fixed-point theorem is non-constructive (it proves existence without providing the fixed point), and its failure in the constructive setting is a key example of the distinction between classical and constructive topology.

---

## Emmy Noether (1882–1935)
*Algebraist; transformed algebraic topology by recasting homological invariants as abstract groups.*

Emmy Noether is one of the greatest algebraists in history. Born in Erlangen, Germany, she was denied a professorship at Göttingen for years due to her sex (Hilbert famously protested: "I do not see that the sex of the candidate is an argument against her admission as a Privatdozent"). She eventually taught there informally, was later given a position, and was expelled in 1933 by the Nazis, dying two years later in the United States.

Her contribution to topology was not a proof but a *conceptual reorganization* that transformed the field. Before Noether, the topological invariants of a space were **Betti numbers** — integers counting the ranks of homological data — and **torsion coefficients** — additional integers capturing torsion in the homology. Around 1925, in informal discussions and lectures at Göttingen, Noether insisted that these invariants should be presented as a single algebraic object: an **abelian group** (the homology group). The Betti number is the rank of this group; the torsion is its torsion subgroup.

This shift from numbers to groups may seem minor, but it was transformative. It allowed algebraic tools — exact sequences, group homomorphisms, long exact sequences — to be applied directly to topological situations. Noether's **abstract algebra** (her book *Moderne Algebra*, written with van der Waerden) provided the algebraic language that Eilenberg and Mac Lane would use to define category theory (motivated in part by algebraic topology), and that Cartan, Serre, and Grothendieck would use to build homological algebra and sheaf cohomology. The entire algebraic apparatus of Chapter 14 (homotopy groups, long exact sequences) descends from Noether's insight that algebraic topology should be genuinely algebraic.

---

## Paul Alexandrov (1896–1982)
*Topologist; developed combinatorial topology, the theory of compact spaces, and inverse limits.*

Pavel Sergeyevich Alexandrov (also transliterated Alexandroff) was a Soviet mathematician at Moscow State University, known for his long collaboration with Pavel Urysohn (who died by drowning in 1924) and his subsequent work on the foundations of point-set and algebraic topology.

Alexandrov and Urysohn's joint work on compact spaces, in their 1929 paper "Mémoire sur les espaces topologiques compacts," established the theory of compactness in full generality: they characterized compact spaces via convergence of nets, proved the Tychonoff theorem for finite products, and gave the Urysohn lemma and Urysohn metrization theorem. Alexandrov then developed the theory of **inverse limits** of compact spaces (the "solenoids" and $p$-adic integers arise this way) and proved the **Alexandrov duality** theorem relating the cohomology of a compact space to the homology of its complement.

In combinatorial topology, Alexandrov proved fundamental results about simplicial complexes and the nerve of an open cover (the **nerve theorem**: the nerve of a good open cover is homotopy equivalent to the space), a result now central to both algebraic topology and topological data analysis. His **Alexandrov topology** on a preorder gives a functor from posets to topological spaces that is used in domain theory (Scott topology) and, through geometric realization, connects to the theory of ∞-categories. Alexandrov's textbook (with Hopf) *Topologie* (1935) was a standard reference for several decades.

---

## James Munkres (1930–)
*Mathematician and educator; author of the standard topology textbook.*

James R. Munkres is an emeritus professor of mathematics at MIT. His research contributions are in combinatorial topology and algebraic topology, including an early proof that $\mathbb{R}^n$ has no exotic differentiable structures (by an independent route from Milnor's exotic spheres), work on the Hauptvermutung for manifolds, and contributions to simplicial homology theory. He has also been a highly influential teacher and expositor.

For Chapter 13, Munkres's primary significance is his textbook *Topology* (1975; 2nd edition 2000), which has been the standard first course in topology for American mathematicians for fifty years. The book is notable for its clarity, its careful balance between point-set and algebraic topology, its extensive exercise sets, and its efficient progression from the basic axioms through the Tychonoff theorem, Urysohn metrization theorem, and classification of compact surfaces to the fundamental group and covering spaces.

Munkres's research paper "Elementary Differential Topology" (1963, Annals of Mathematics Studies) and its companion book established the differential topology foundations needed to make Morse theory and transversality precise, filling a gap in the literature between smooth manifold theory and the algebraic topology of Chapter 14. His textbook *Analysis on Manifolds* is a rigorous undergraduate treatment of multivariable calculus and differential forms. Among pedagogues, Munkres stands with Rudin and Lang as writers who established the standard vocabulary and proof style of their subjects.
