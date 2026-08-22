# Applied Exercises

The abstract machinery of simplicial type theory — Segal types, Rezk types, the synthetic Yoneda lemma — acquires its power when applied to concrete structures. The exercises in this section show that the theory of Chapter 24 is not only beautiful mathematics but a practical conceptual toolkit: the Segal condition is really a statement about compositionality, the Rezk condition is a statement about when two implementations should be considered the same, and the Yoneda lemma is a statement about universal representations. Each exercise below takes a concrete domain and works through how the STT machinery applies. Working these exercises carefully will strengthen your understanding of why the definitions are the way they are.

---

## Exercise A.1: Software Packages as a Segal Type
*Domain: Software Engineering / Dependency Management*

**Setup:** A package manager (such as Cargo, npm, or pip) maintains a dependency graph: packages are nodes, and an edge from package $A$ to package $B$ means $A$ depends on $B$. If $A$ depends on $B$ and $B$ depends on $C$, then $A$ transitively depends on $C$. This is composition. If $A$ depends on $A$ (trivially), that's the identity. The system of packages and their dependency relations forms a directed structure — not a groupoid, because dependency is not symmetric.

**Questions:**

1. Spell out the Segal type structure on the system of packages: what are the objects, what is the hom type $\mathsf{hom}(A, B)$, and what is the identity morphism $\mathsf{id}_A$? Is the hom type a proposition (at most one dependency relation between two packages), or can there be multiple "kinds" of dependency?

2. State the Segal condition for this system. What does "inner horn filling" mean concretely? A composable pair is a chain $A \to B \to C$ (A depends on B, B depends on C). The unique filler is the composite dependency $A \to C$. Is this Segal condition automatically satisfied, or could it fail? Give a concrete scenario where the Segal condition would fail if we try to extend the structure.

3. Now suppose we want the Rezk condition: two packages are "equal" iff they are isomorphic in the dependency system. An isomorphism between packages $A$ and $B$ means $A$ depends on $B$ and $B$ depends on $A$ (mutual dependency). Is the dependency graph Rezk? Should it be? What does Rezk completion mean concretely for this system — and why might a package manager want to enforce it?

4. *Extension:* A "version upgrade" replaces one package with a new version. Model this as a natural transformation between dependency functors. The Yoneda lemma then says: a natural transformation from the representable functor $\mathsf{hom}(\text{packageA}, -)$ to a functor $F$ (e.g., "computable on all dependencies of packageA") is completely determined by an element of $F(\text{packageA})$. What does this say concretely about how package upgrade behavior is determined?

*Abstract concept illustrated: The Segal condition as a precise statement about compositionality; the Rezk condition as equivalence-invariance for implementations.*

---

## Exercise A.2: The Rezk Completion as Strictification
*Domain: Distributed Systems / Coherence*

**Setup:** In a distributed system, different nodes may maintain slightly different views of a shared resource (for example, different replicas of a database). These views are related by "synchronization morphisms" — processes that bring one view up to date with another. Suppose two views $V_1$ and $V_2$ are "weakly equivalent" if there exist synchronization morphisms in both directions (mutual synchronizability). The system has a Segal structure but may fail the Rezk condition: two views that are mutually synchronizable may not be literally equal.

**Questions:**

1. Explain why the system of database views with synchronization morphisms is a Segal type. What is the hom type? What is the composition of synchronization morphisms?

2. Explain why this Segal type fails the Rezk condition in general. Give a concrete example: two distinct views $V_1$ and $V_2$ such that there are synchronization morphisms $V_1 \to V_2$ and $V_2 \to V_1$ satisfying the isomorphism condition, but $V_1$ and $V_2$ are not identical as data structures.

3. Describe the Rezk completion of this system. In the Rezk completion, $V_1$ and $V_2$ from question 2 become equal. What is the universal property of the Rezk completion? Why is it the "right" thing to do when you want to work "up to synchronization equivalence"?

4. Identify a real-world situation where enforcing the Rezk condition prematurely (identifying all mutually synchronizable views) would cause problems, and one where failing to enforce it causes problems. What does this say about when the Rezk condition is the "right" design choice for a system?

5. *Extension:* The CAP theorem (Consistency, Availability, Partition tolerance) says distributed systems can guarantee at most two of these three properties. Map this onto the categorical structure: consistency corresponds to the Segal condition (composition is defined), and availability/partition tolerance correspond to different aspects of the Rezk condition. Is this analogy exact? Where does it break down?

*Abstract concept illustrated: The Rezk completion as a formal procedure for "working up to equivalence"; directed univalence as the correct notion of identity for ∞-categories.*

---

## Exercise A.3: The Synthetic Yoneda Lemma for Groups
*Domain: Abstract Algebra / Representation Theory*

**Setup:** The category $\mathbf{Grp}$ of groups (with group homomorphisms as morphisms) forms a Segal type in STT. The representable functor $\mathsf{hom}_{\mathbf{Grp}}(G, -)$ sends each group $H$ to the set of homomorphisms from $G$ to $H$. The Yoneda lemma says natural transformations from this representable functor to any functor $F : \mathbf{Grp} \to \mathsf{Type}$ correspond to elements of $F(G)$.

**Questions:**

1. Apply the synthetic Yoneda lemma to the case $G = \mathbb{Z}$ (the integers, as a group under addition) and $F = \mathsf{hom}_{\mathbf{Grp}}(-, H)$ for a group $H$. The Yoneda lemma says:
$$\mathsf{Nat}(\mathsf{hom}(\mathbb{Z}, -), \mathsf{hom}(-, H)) \simeq \mathsf{hom}(\mathbb{Z}, H)$$
What does this say concretely? A natural transformation between these functors is determined by where the identity homomorphism $\mathsf{id}_{\mathbb{Z}}$ goes. Express the bijection explicitly: what element of $\mathsf{hom}(\mathbb{Z}, H)$ corresponds to a given natural transformation?

2. Now apply the Yoneda lemma to show that $\mathbb{Z}$ represents the "underlying-set" forgetful functor $U : \mathbf{Grp} \to \mathsf{Set}$. Specifically, use the Yoneda lemma to prove that group homomorphisms from $\mathbb{Z}$ to $H$ correspond to elements of $H$. Write down the correspondence explicitly: given $h \in H$, construct the corresponding homomorphism $\phi_h : \mathbb{Z} \to H$, and verify it is indeed a homomorphism.

3. In STT, the fact that "naturality is automatic" means the natural transformation constructed in question 2 satisfies naturality without you having to check it. Identify the naturality square that you would have to verify in the classical (non-synthetic) proof, and explain why it is automatic in STT.

4. *Extension:* The group $\mathbb{Z}/n\mathbb{Z}$ represents $n$-torsion in groups: homomorphisms $\mathbb{Z}/n\mathbb{Z} \to H$ correspond to $n$-torsion elements of $H$ (elements $h$ with $nh = 0$). State this as a Yoneda lemma instance and prove it. How does this generalize to other cyclic groups?

*Abstract concept illustrated: The synthetic Yoneda lemma as a general representability theorem; the advantage of automatic naturality over classical proofs.*

---

## Exercise A.4: Functors as Typed Programs with Coherent Structure
*Domain: Programming Language Semantics / Type Theory*

**Setup:** In the semantics of typed effectful programs, a computation that reads from a store or writes to a log can be modeled as a morphism in a Kleisli category. The Kleisli category $\mathsf{Kl}(T)$ for a monad $T$ on types has types as objects and programs-with-effects as morphisms: a morphism from $A$ to $B$ in $\mathsf{Kl}(T)$ is a program of type $A \to T(B)$. Composition is monadic bind. In STT, $\mathsf{Kl}(T)$ is a Segal type.

**Questions:**

1. Verify that $\mathsf{Kl}(T)$ is Segal. What is the composition operation? The Segal condition says composition is unique — in the Kleisli category, this means monadic bind is uniquely determined by the monad laws. What are the monad laws, and how do they ensure the Segal condition?

2. A functor $F : \mathsf{Kl}(T_1) \to \mathsf{Kl}(T_2)$ between Kleisli categories is a "translation" between effect systems. In STT, this is just a function. What does preservation of composition (automatic in STT) mean concretely? It means the translation interacts well with program composition — translating a composed program is the same as composing the translated programs. Write out this condition in terms of the effect operations.

3. A natural transformation $\alpha : F \Rightarrow G$ between functors $\mathsf{Kl}(T_1) \to \mathsf{Kl}(T_2)$ is a "program transformation" that maps $T_1$-programs to $T_2$-programs in a coherent way. The naturality condition (automatic in STT) says the transformation commutes with program sequencing. Write out the naturality square for this situation and explain what "commuting with program sequencing" means for the concrete effect operations.

4. *Extension:* The Yoneda lemma for $\mathsf{Kl}(T)$: natural transformations from the representable functor $\mathsf{hom}_{\mathsf{Kl}(T)}(A, -)$ to a functor $F$ correspond to elements of $F(A)$. What does this say about programs? It says: any coherent way of transforming programs-from-$A$ is determined by a single element of $F(A)$. Interpret this in the case where $F$ is the "observable outputs" functor.

*Abstract concept illustrated: Functions between Segal types as automatically functorial morphisms; the Kleisli category as a Segal type modeling effectful computation.*

---

## Exercise A.5: The Segal Condition as Compositionality
*Domain: Module Systems / Software Architecture*

**Setup:** Consider a system of software modules with well-typed interfaces. A module morphism $f : M \to N$ is a "refinement" — a way of implementing module $N$ using module $M$ (i.e., $M$ provides all the functionality required by $N$). This is a directed notion: $M$ refining $N$ does not mean $N$ refines $M$. The identity morphism is "every module refines itself." Composition: if $M$ refines $N$ and $N$ refines $P$, then $M$ refines $P$ (transitivity of interface compatibility).

**Questions:**

1. Formulate the module refinement system as a Segal type. What is the hom type $\mathsf{hom}(M, N)$? Is it a proposition (module $M$ either refines $N$ or it doesn't), or can there be multiple refinement morphisms? How does your answer affect whether the Segal type is "thin" (like a poset) or "fat" (like a genuine ∞-category)?

2. State the Segal condition for module refinement. In the "thin" case (hom types are propositions), the Segal condition is automatic from transitivity. But in a richer model where a refinement is a *translation map* (a function from $N$'s interface to $M$'s implementation), the Segal condition says: for any two composable translations, there is a unique composite translation. Why is this composition unique (up to homotopy)? What goes wrong if it is not?

3. The Rezk condition for the module system: two modules are "equal" iff they mutually refine each other — each implements the other's interface. In software engineering, this is called "Liskov substitution equivalence." Is the Rezk condition the right notion for module equality? What would break if you identified all Liskov-equivalent modules?

4. A functor $F : \mathsf{Modules} \to \mathsf{Modules}$ is a module transformer: it maps modules to modules, preserving interface compatibility. In STT, every function between Segal types is automatically a functor. What does this mean for module transformers? Give an example: the "adding logging" transformer that enriches every module with a logging facility should be functorial. Verify that it is.

5. *Extension:* The module system of a large application can be abstracted as a Segal type where "composable pairs" of morphisms correspond to interface compatibility chains. The Segal condition is then the assertion that "the composition of compatible interfaces is compatible." Formalize this using the inner horn filling language: what is the composable pair, what is the inner horn, and what is the unique filler?

*Abstract concept illustrated: The Segal condition as abstract compositionality; the difference between "thin" Segal types (poset-like) and "rich" Segal types (genuine ∞-categories).*
