# Important Figures

## Heinz Hopf (1894–1971)
*Algebraic topologist; discovered the Hopf fibration and the Hopf invariant, opening the modern study of homotopy groups.*

Heinz Hopf was born in Breslau (now Wrocław) and spent most of his career at ETH Zürich, where he transformed the mathematics department and trained a generation of algebraic topologists. He received the first Wolf Prize in mathematics in 1979 (posthumously; Hopf died in 1971). His mathematical work spans combinatorial topology, differential geometry, and Lie theory, but his most lasting contributions are to homotopy theory.

In 1931, Hopf published "Über die Abbildungen der dreidimensionalen Sphäre auf die Kugelfläche," introducing what is now called the **Hopf fibration** $\eta : S^3 \to S^2$. The construction: viewing $S^3 \subset \mathbb{C}^2$ as the unit sphere $\{(z_0, z_1) \mid |z_0|^2 + |z_1|^2 = 1\}$ and $S^2 \cong \mathbb{CP}^1$ as the complex projective line, the map sends $(z_0, z_1) \mapsto [z_0 : z_1]$ (the complex ratio). The fibers are circles ($S^1$), and the total space is $S^3$: the Hopf fibration is a principal $U(1)$-bundle $S^1 \hookrightarrow S^3 \to S^2$.

Hopf proved that $\eta$ is not null-homotopic — indeed, he introduced the **Hopf invariant** as a tool to detect this: for a map $f : S^3 \to S^2$, the Hopf invariant $H(f) \in \mathbb{Z}$ measures the linking number of the preimages of two regular values. For $\eta$, $H(\eta) = 1$. This meant, definitively, that $\pi_3(S^2)$ is nontrivial — a result that shocked the mathematical community and launched the study of higher homotopy groups. Hopf's later work on the topology of Lie groups (1941) showed that any compact connected Lie group has the rational cohomology of a product of odd spheres, providing a structural explanation for why sphere-like behavior appears throughout Lie group theory.

---

## Witold Hurewicz (1904–1956)
*Algebraic topologist; defined higher homotopy groups and proved the Hurewicz theorem.*

Witold Hurewicz was born in Łódź, Poland (then part of the Russian Empire), studied in Vienna, and held positions at Amsterdam, MIT, and the University of North Carolina before his tragic death — a fall from a pyramid in Mexico while attending a topology conference. His mathematical output in two decades was extraordinary in both scope and depth.

Hurewicz defined the **higher homotopy groups** $\pi_n(X, x_0)$ in his 1935–36 papers. For $n \geq 2$: $\pi_n(X, x_0) = [(S^n, s_0), (X, x_0)]$ — homotopy classes of basepoint-preserving maps from the $n$-sphere to $X$, with group operation given by pinching the equator and mapping into $X$. He proved that $\pi_n$ is abelian for $n \geq 2$ (unlike $\pi_1$, which can be non-abelian), and that $\pi_n$ is a homotopy-type invariant (unchanged by homotopy equivalences).

His main theorem — now called the **Hurewicz theorem** — states: if $X$ is $(n-1)$-connected (meaning $\pi_k(X) = 0$ for $k < n$), then the **Hurewicz homomorphism** $h : \pi_n(X) \to H_n(X)$ (sending a map $f : S^n \to X$ to the image of the fundamental class $[S^n] \in H_n(S^n) \cong \mathbb{Z}$) is an isomorphism for the lowest nontrivial degree and a surjection one degree higher. This is the fundamental comparison theorem between homotopy and homology, and it is the main tool connecting the harder-to-compute homotopy groups to the more accessible homology groups. In HoTT, the Hurewicz theorem has a direct synthetic proof, and its formulation in terms of the truncated type theory is a nontrivial result of synthetic homotopy theory.

---

## Jean-Pierre Serre (1926–)
*Algebraic geometer and topologist; introduced spectral sequences for fibrations and computed homotopy groups of spheres.*

Jean-Pierre Serre is a professor emeritus at the Collège de France in Paris and one of the most celebrated living mathematicians. He received the Fields Medal in 1954 and the Abel Prize in 2003. His work spans algebraic geometry, algebraic topology, number theory (modular forms, Galois representations, and their connections to arithmetic), and group theory.

For Chapter 14, Serre's 1951 thesis "Homologie singulière des espaces fibrés" is the key contribution. He introduced the **Serre spectral sequence**: given a fibration $F \hookrightarrow E \to B$, there is a spectral sequence $E_2^{p,q} = H_p(B; H_q(F)) \Rightarrow H_{p+q}(E)$ computing the homology of the total space $E$ from the homologies of base and fiber. This is the primary computational tool for homotopy groups: by applying the spectral sequence to the path-loop fibration $\Omega X \hookrightarrow PX \to X$ iteratively, Serre computed $\pi_n(S^k)$ for many values of $n$ and $k$ and proved that: $\pi_n(S^k) = 0$ for $n < k$; $\pi_k(S^k) = \mathbb{Z}$ for all $k$; and $\pi_n(S^k)$ is finite for $n > k$ except when $k$ is even and $n = 2k-1$ (where $\pi_{2k-1}(S^k)$ contains a $\mathbb{Z}$ summand). The techniques of Serre's thesis — mod-$p$ spectral sequences, use of Eilenberg-MacLane spaces, finiteness arguments for homotopy groups — remain central tools in algebraic topology seventy years later.

---

## J.H.C. Whitehead (1904–1960)
*Algebraic topologist; invented CW complexes and proved Whitehead's theorem.*

John Henry Constantine Whitehead was a nephew of the philosopher Alfred North Whitehead, was born in India, educated at Oxford, and became a professor at Oxford after working in Princeton with Oswald Veblen. He was one of the central figures in establishing algebraic topology as a rigorous discipline. He died suddenly of a heart attack at 55.

Whitehead's two 1949 papers "Combinatorial Homotopy I and II" introduced **CW complexes** — spaces built by attaching cells of increasing dimension via attaching maps. The construction: begin with a discrete set $X^0$ (0-skeleton = set of points), attach 1-cells (intervals) to get $X^1$, attach 2-cells (disks) to $X^1$ via maps from the boundary circle, and so on. Every "nice" topological space (manifold, simplicial complex, algebraic variety) is either a CW complex or homotopy equivalent to one.

CW complexes are the right category for homotopy theory because they have good closure properties (homotopy equivalences, products, fibrations) and because **Whitehead's theorem** holds: a map $f : X \to Y$ between CW complexes that induces isomorphisms $f_* : \pi_n(X) \to \pi_n(Y)$ for all $n$ is a homotopy equivalence. This is deeply non-trivial: it says that homotopy groups *completely determine* the homotopy type within the category of CW complexes (though not for general spaces). In HoTT, the analogous statement is that a map inducing equivalences on all homotopy groups and $\pi_0$ is itself an equivalence — this follows from the fact that types in HoTT are ∞-groupoids and equivalences of ∞-groupoids are exactly maps inducing isomorphisms on all homotopy groups.

Whitehead also proved the **Seifert-van Kampen theorem** in a general form suitable for CW complexes, developed the theory of **simple homotopy equivalences** (detecting when a homotopy equivalence can be built from elementary moves), and made major contributions to differentiable manifolds and obstruction theory.

---

## Frank Adams (1930–1989)
*Algebraic topologist; introduced the Adams spectral sequence and resolved the Hopf invariant one problem.*

John Frank Adams was a professor at Cambridge from 1970 until his death in a car accident in 1989. He is credited with establishing **stable homotopy theory** as a systematic discipline.

Adams's most celebrated result is the solution of the **Hopf invariant one problem**: for which $n$ does there exist a map $f : S^{2n-1} \to S^n$ with Hopf invariant 1? Hopf's 1931 construction gave such a map for $n = 2$ ($\eta : S^3 \to S^2$); similar constructions give maps with Hopf invariant 1 for $n = 4$ (quaternionic Hopf fibration $S^7 \to S^4$) and $n = 8$ (octonionic Hopf fibration $S^{15} \to S^8$). Adams proved in 1960 that these are the only cases: Hopf invariant one maps exist only for $n = 1, 2, 4, 8$. This is equivalent to the theorem that the only normed division algebras over $\mathbb{R}$ are $\mathbb{R}$, $\mathbb{C}$, $\mathbb{H}$ (quaternions), and $\mathbb{O}$ (octonions), and that $S^1, S^3, S^7$ are the only parallelizable spheres.

Adams introduced the **Adams spectral sequence** $E_2^{s,t} = \mathrm{Ext}^{s,t}_{\mathcal{A}}(\mathbb{F}_p, \mathbb{F}_p) \Rightarrow \pi_{t-s}^{\text{st}}(S^0)$ (converging to the stable homotopy groups of spheres, computed at prime $p$) as the primary computational tool for stable homotopy theory. This spectral sequence is still in active use. Adams also proved the **$J$-homomorphism theorem** (determining the image of $J : \pi_n(SO) \to \pi_n^{\text{st}}(S^0)$) and developed **Adams operations** in K-theory, used in his proof of the Hopf invariant one theorem.

---

## John Milnor (1931–)
*Algebraic and differential topologist; proved the existence of exotic spheres and developed fundamental tools in homotopy theory.*

John Milnor is a professor at Stony Brook University and one of the giants of 20th-century mathematics. He received the Fields Medal in 1962, the Wolf Prize in 1989, and the Abel Prize in 2011. His contributions span differential topology, algebraic K-theory, dynamical systems, and algebraic topology.

For Chapter 14, Milnor's most relevant contributions are: the discovery of **exotic spheres** (1956) — smooth manifolds homeomorphic but not diffeomorphic to $S^7$, proving that differential topology is distinct from topological topology; the development of **Morse theory** and its application to the Bott periodicity theorem; and the **Milnor fiber** construction in singularity theory.

Milnor's discovery of exotic spheres is paradigmatic for the interaction between homotopy theory and differential topology. He constructed smooth manifolds $\Sigma^7$ that are homeomorphic to $S^7$ (same homotopy type, same topological structure) but not diffeomorphic (different smooth structure). The existence of exotic spheres depends on the failure of the **h-cobordism theorem** in dimension 7 and on the homotopy groups of $SO(7)$. This result, which shocked the mathematical community in 1956, shows that topology (homotopy type) does not determine smooth structure, and led to the development of surgery theory and the systematic study of the relationship between homotopy groups and differential topology. For Chapter 14, the Milnor fiber is an important example: given a polynomial $f : \mathbb{C}^n \to \mathbb{C}$ with an isolated singularity at the origin, the **Milnor fiber** $F = f^{-1}(\delta) \cap B_\varepsilon$ (for small $\delta$, $\varepsilon > 0$) is a smooth manifold whose homotopy type is a wedge of $(n-1)$-spheres. The number of spheres — the **Milnor number** — is an invariant of the singularity, computable from the homotopy groups of the Milnor fiber.
