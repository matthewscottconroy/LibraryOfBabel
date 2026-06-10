# Important Figures

## Samuel Eilenberg (1913–1998)
*Mathematician; co-inventor of category theory; pioneer of algebraic topology*

Samuel Eilenberg was born in Warsaw, Poland, and studied mathematics there before emigrating to the United States in 1939. He eventually settled at Columbia University, where he spent most of his career. Before meeting Mac Lane, Eilenberg had already made fundamental contributions to algebraic topology: his axioms for homology theory (the Eilenberg-Steenrod axioms, formulated with Norman Steenrod) gave the first rigorous characterization of homology, and his work on singular homology established the chain complex as the central computational tool of algebraic topology.

The encounter with Mac Lane at the University of Michigan in the early 1940s was one of the most productive collaborations in twentieth-century mathematics. Together they produced the 1945 paper "General Theory of Natural Equivalences," which introduced categories, functors, and natural transformations. Eilenberg's topological intuition drove the examples: the natural transformation they were trying to formalize was the map from a vector space to its double-dual, which is natural (it commutes with all linear maps) in a way that the single-dual embedding is not. The paper was initially dismissed by some as "general abstract nonsense" — but within a decade it had transformed algebra, topology, and eventually logic.

Eilenberg also made essential contributions to homological algebra (Cartan-Eilenberg's *Homological Algebra*, 1956, developed much of the modern machinery) and to automata theory (Eilenberg's two-volume *Automata, Languages, and Machines* used category theory to organize the theory of finite automata). In later life, Eilenberg became a serious collector of South and Southeast Asian art; his collection is now at the Metropolitan Museum of Art.

---

## Saunders Mac Lane (1909–2005)
*Mathematician; co-inventor of category theory; author of the canonical textbook*

Saunders Mac Lane was born in Connecticut and studied at Yale, Chicago, and Göttingen (where he learned from Hermann Weyl and Paul Bernays). He became one of the leading algebraists of his generation, contributing to group theory (the Mac Lane-Whitehead theorem on crossed modules), algebraic topology, and eventually category theory. He spent most of his career at the University of Chicago.

Mac Lane's contribution to the 1945 paper was the algebraic and foundational precision. Where Eilenberg brought topological intuition, Mac Lane brought the rigor of group theory and the awareness that the new concepts needed a careful axiomatic foundation. He gave the modern definition of a category (objects and morphisms with composition satisfying identity and associativity) and recognized that the natural transformation concept was the key new idea, with categories and functors introduced mainly to make natural transformations well-defined.

Mac Lane's *Categories for the Working Mathematician* (1971, 2nd ed. 1998) is one of the most influential mathematics textbooks of the twentieth century. It remains the standard graduate reference for category theory: rigorous, comprehensive, and written from the perspective of someone who built the theory. Mac Lane also wrote extensively on the philosophy of mathematics and was a defender of the structuralist view that mathematics is the study of structures, not of particular objects — a view that category theory makes precise. His later work on coherence theorems (the coherence theorem for monoidal categories says that all diagrams built from the structural isomorphisms commute) was essential for the modern development of monoidal category theory.

---

## Alexander Grothendieck (1928–2014)
*Mathematician; created modern algebraic geometry; introduced toposes and sites*

Alexander Grothendieck is widely regarded as the most visionary mathematician of the twentieth century. Born in Berlin to anarchist parents, he spent part of his childhood in a French internment camp before becoming a mathematician of extraordinary power. After early work in functional analysis (his thesis on nuclear spaces is still a major reference), he turned to algebraic geometry and, over the course of the 1950s and 1960s, rebuilt the entire field from the ground up.

Grothendieck's categorical contributions include: the introduction of abelian categories (generalizing both module categories and sheaf categories) in the Tohoku paper (1957); the development of sheaves, sites, and Grothendieck topologies (a generalization of topological spaces in which "open sets" are replaced by "covering families" of morphisms) in the *SGA* seminar notes; the definition of schemes as representable functors on the category of commutative rings (making the Yoneda lemma a workhorse of algebraic geometry); and the concept of a topos as a generalized space whose points can have nontrivial symmetry.

The Grothendieck construction — associating to a functor $F : \mathcal{C} \to \mathbf{Cat}$ a category $\int F$ whose objects are pairs $(c, x)$ with $x \in F(c)$ — is essential to the theory of fibered categories and is the categorical precursor of the type-theoretic notion of a type family. Grothendieck's work on homotopy types (unpublished "Pursuing Stacks" manuscript, 1983) anticipated the homotopy hypothesis and the connection between $\infty$-groupoids and homotopy types that is central to HoTT. He withdrew from mathematics in 1991 under mysterious circumstances and spent his last years in rural France.

---

## F. William Lawvere (1937–present)
*Mathematician and philosopher; creator of categorical logic and topos theory*

William Lawvere is a professor at SUNY Buffalo and the founder of categorical logic — the study of the relationship between category theory and formal logic. His 1963 Columbia thesis on "Functorial Semantics of Algebraic Theories" introduced Lawvere theories (essentially, categories with finite products used to axiomatize algebraic structures), giving a purely categorical alternative to the universal-algebraic notion of a variety. This was the beginning of the program to replace set-theoretic foundations with categorical ones.

Working with Myles Tierney in 1969–1970, Lawvere co-invented the elementary topos: a category satisfying a small number of axioms (existence of finite limits, a subobject classifier, exponentials) that together ensure the category has an internal intuitionistic higher-order logic. The Lawvere-Tierney topology (a generalization of the subobject classifier to model modalities) showed that sheaves, forcing, and modal logic are all aspects of a single categorical phenomenon.

Lawvere's 1969 paper "Adjointness in Foundations" is a philosophical manifesto arguing that adjoint functors are the fundamental connective tissue of all mathematics: the logical quantifiers ∀ and ∃ are adjoints to substitution; the relationship between a formal theory and its models is an adjunction; the free-forgetful adjunction for any algebraic structure is the categorical content of the notion of "generator." This paper is one of the most intellectually stimulating in all of mathematics, and its perspective directly motivates the categorical semantics of MLTT that appears in Chapter 11.

---

## Daniel Quillen (1940–2011)
*Mathematician; creator of model category theory and algebraic K-theory*

Daniel Quillen was a professor at MIT and later at Oxford, and he won the Fields Medal in 1978. His most relevant contribution for this curriculum is the introduction of *model categories* in his 1967 monograph *Homotopical Algebra*. A model category is a category equipped with three distinguished classes of morphisms — weak equivalences, fibrations, and cofibrations — satisfying axioms that allow the construction of a homotopy theory "inside" the category.

Model categories are the technical framework in which Voevodsky's proof of Univalence lives: the category of Kan simplicial sets is a model category, and Voevodsky showed it models all of MLTT + Univalence. More generally, every $\infty$-topos can be presented by a model category, and the correspondence between $\infty$-toposes and HoTT runs through Quillen's framework. Quillen's work thus provides the technical foundation for understanding why HoTT is consistent.

Quillen also introduced the higher algebraic K-theory groups $K_n(R)$ for a ring $R$ using a categorical construction (the Q-construction), demonstrating that category theory was not just a language for existing mathematics but a source of genuinely new mathematical content.

---

## Charles Ehresmann (1905–1979)
*Mathematician; defined categories, groupoids, and structured categories*

Charles Ehresmann was a French mathematician who worked independently from Eilenberg and Mac Lane on the foundations of differential geometry and topology. He defined categories (calling them "catégories," a term he coined independently around the same time as Eilenberg-Mac Lane) and in particular groupoids — categories in which every morphism is an isomorphism — as a tool for studying fiber bundles and connections in differential geometry.

Ehresmann's groupoids are directly relevant to HoTT: the fundamental groupoid $\Pi_1(X)$ of a topological space $X$ (whose objects are points, morphisms are homotopy classes of paths, and every morphism is invertible) is Ehresmann's groupoid. In MLTT, every type $A$ has a groupoid structure: objects are elements $a : A$, morphisms from $a$ to $b$ are elements of $a =_A b$, and every identity proof is invertible (symmetry). This is no coincidence — the connection between MLTT's identity types and Ehresmann's groupoids is one of the key insights of HoTT.

Ehresmann also defined fibered categories (categories fibered over a base category, with the notion of Cartesian morphism) in 1959–1963, creating the framework that Grothendieck would later systematize and that now underlies the semantics of dependent type theory: a type family $B : A \to \mathsf{Type}$ is modeled by a category fibered over the category of contexts.

---

## Nobuo Yoneda (1930–1996)
*Mathematician; discoverer of the Yoneda Lemma*

Nobuo Yoneda was a Japanese mathematician who worked at Tokyo's Gakushuin University. He is known almost exclusively for a single result — the Yoneda Lemma — which he communicated to Saunders Mac Lane informally at the Gare du Nord train station in Paris in 1954, and which Mac Lane recorded in *Categories for the Working Mathematician*. The result states: for any locally small category $\mathcal{C}$, functor $F : \mathcal{C}^{op} \to \mathbf{Set}$, and object $A$, natural transformations from the representable functor $\mathsf{Hom}(-, A)$ to $F$ are in bijection with elements of $F(A)$.

The Yoneda Lemma is considered one of the deepest elementary theorems in mathematics, encoding the principle that an object is determined entirely by its external relationships. It implies that the Yoneda embedding $\mathcal{C} \hookrightarrow [\mathcal{C}^{op}, \mathbf{Set}]$ is fully faithful (morphisms in $\mathcal{C}$ correspond bijectively to natural transformations between representable functors), which in turn justifies the practice of working with objects purely through their universal properties.

In type theory, the Yoneda Lemma has a direct translation: for a type $A$ and a family $B : A \to \mathsf{Type}$, the type $\prod_{x:A} (x =_A a) \to B(x)$ is equivalent to $B(a)$. This is provable by path induction (the J rule) without any axioms. That the Yoneda Lemma is provable in pure MLTT via the J rule illustrates how deep the connection between homotopy theory and type theory runs.

Yoneda's other mathematical work included contributions to homological algebra, but none achieved the influence of the lemma that bears his name. He died in 1996.
