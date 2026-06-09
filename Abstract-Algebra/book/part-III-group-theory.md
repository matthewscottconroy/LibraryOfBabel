# Part III — Group Theory

**Chapters 13–19**

---

## What This Part Establishes

Part III introduces the most fundamental algebraic structure: the group. A group is a set equipped with a single binary operation satisfying four axioms — closure, associativity, identity, and inverses. The deceptive simplicity of these axioms conceals extraordinary depth: groups encode the algebra of symmetry in its purest form, and the theory built from these four axioms spans from elementary consequences to one of the great achievements of twentieth-century mathematics, the classification of finite simple groups.

The part moves through several layers of structure. The first layer (Chapters 13–14) establishes what groups are and how a subgroup partitions its parent group into cosets. The second layer (Chapter 15) establishes the maps between groups and the isomorphism theorems — the tools that let us recognize when two groups are "the same." The third layer (Chapter 16) introduces group actions: groups acting on sets, which unifies abstract group theory with combinatorics and provides the machinery for counting and structural analysis. The fourth layer (Chapters 17–18) goes deeper into structure: the Sylow theorems give control over prime-power subgroups of finite groups, and the study of simple, solvable, and nilpotent groups reveals how groups are assembled from simple pieces. The part concludes (Chapter 19) with the complete classification of finitely generated abelian groups — a theorem clean enough to state in one line and deep enough to require the full machinery of Smith normal form.

By the end of Part III, the reader will be fluent in the language of group theory and ready to meet groups again in every subsequent part: as coefficient objects in homological algebra, as Galois groups in field theory, as Lie groups in Part X, and as the organizing structure of representation theory in Parts IX and XI.

---

## Internal Dependency Map

```
Ch 13 (Groups, Subgroups)
       |
       v
Ch 14 (Cosets, Normal Subgroups, Quotients)
       |
       v
Ch 15 (Homomorphisms, Isomorphism Theorems)
       |
  _____|_____
  |         |
  v         v
Ch 16     Ch 19
(Actions) (Fin. Gen. Abelian)
  |
  v
Ch 17 (Sylow Theorems)
  |
  v
Ch 18 (Structure: Simple, Solvable, Nilpotent, Free)
```

---

## Chapter 13 — Groups and Subgroups

**What it establishes:** The four axioms of a group; the first examples spanning number theory, combinatorics, geometry, and linear algebra; and the internal substructure of a group through subgroups and cyclic subgroups.

---

**13.1 The Group Axioms**

**13.1.1 The Four Axioms: Closure, Associativity, Identity, Inverses**
A group $(G, \cdot)$ is a set$G$ with a binary operation$\cdot: G \times G \to G$ satisfying: (i) closure ($a \cdot b \in G$ for all$a, b \in G$); (ii) associativity ($(a \cdot b) \cdot c = a \cdot (b \cdot c)$); (iii) existence of an identity element$e$ with$e \cdot a = a \cdot e = a$; (iv) existence of inverses: for every$a \in G$, there exists$a^{-1}$ with$a \cdot a^{-1} = a^{-1} \cdot a = e$. These four axioms are the entire definition. Everything else in group theory is a consequence.

**13.1.2 Elementary Consequences Provable from the Axioms**
Before introducing examples, we prove what the axioms alone guarantee: the identity element is unique; inverses are unique; $(a^{-1})^{-1} = a$;$(ab)^{-1} = b^{-1}a^{-1}$; the left and right cancellation laws hold. These deductions establish that the axioms are non-redundant and that the initial consequences of any group are determined entirely by the four conditions — there are no hidden assumptions.

**13.1.3 Abelian Groups; Additive vs. Multiplicative Notation**
A group is *abelian* (or commutative) if $ab = ba$ for all elements. For abelian groups, we typically write the operation as$+$, the identity as$0$, and inverses as$-a$. For nonabelian groups, we write the operation multiplicatively. Both notations appear throughout algebra, and fluency with both is essential. This section establishes the convention and distinguishes when commutativity can be assumed.

**13.1.4 The Order of a Group; Finite and Infinite Groups**
The *order* $|G|$ of a group is the cardinality of its underlying set. A group is *finite* if$|G| < \infty$ and *infinite* otherwise. The order of an element$g \in G$ is the smallest positive integer$n$ such that$g^n = e$, or$\infty$ if no such$n$ exists. These two notions of order — group order and element order — are related by Lagrange's theorem (Chapter 14) in the finite case and will reappear throughout the theory.

---

**13.2 First Examples**

**13.2.1 Additive and Multiplicative Groups of Numbers**
The integers $(\mathbb{Z}, +)$, rationals$(\mathbb{Q}, +)$, reals$(\mathbb{R}, +)$, and complex numbers$(\mathbb{C}, +)$ are all abelian groups under addition. Removing zero,$(\mathbb{Q}^*, \cdot)$,$(\mathbb{R}^*, \cdot)$, and$(\mathbb{C}^*, \cdot)$ are abelian groups under multiplication. The integers modulo$n$, written$\mathbb{Z}/n\mathbb{Z}$, form a finite abelian group under addition. These numerical examples ground the abstract definition in familiar arithmetic.

**13.2.2 The Symmetric Group $S_n$; Cycle Notation**
The symmetric group $S_n$ is the group of all bijections (permutations) of the set$\{1, 2, \ldots, n\}$, with composition as the operation. It has order$n!$. We introduce *cycle notation*: a$k$-cycle$(a_1 \, a_2 \, \cdots \, a_k)$ sends$a_1 \mapsto a_2 \mapsto \cdots \mapsto a_k \mapsto a_1$. Every permutation decomposes into a product of disjoint cycles, and this decomposition is unique up to the order of cycles. The symmetric groups are the most important finite groups, appearing as automorphism groups, Galois groups, and the targets of Cayley's theorem.

**13.2.3 The Alternating Group $A_n$; Even and Odd Permutations**
Every permutation can be written as a product of transpositions (2-cycles), and the *parity* of such a decomposition — even or odd — is well-defined (independent of the choice of transposition decomposition). The *alternating group* $A_n$ is the subgroup of even permutations; it has order$n!/2$ for$n \geq 2$. The fact that$A_n$ is a subgroup requires proof: the product of two even permutations is even. That$A_5$ is simple (has no normal subgroups other than$\{e\}$ and itself) will be proved in Chapter 18 and is the key to the unsolvability of the quintic.

**13.2.4 Dihedral Groups $D_n$; Symmetries of the Regular$n$-gon**
The dihedral group $D_n$ is the group of symmetries of a regular$n$-gon, consisting of$n$ rotations and$n$ reflections, for a total order of$2n$. It is generated by a rotation$r$ of order$n$ and a reflection$s$ of order 2, subject to the relation$srs^{-1} = r^{-1}$. The dihedral groups are the first examples of nonabelian groups (for$n \geq 3$) and illustrate how a group can be defined by generators and relations — a theme developed fully in Chapter 18.

**13.2.5 Matrix Groups: $GL_n$,$SL_n$,$O(n)$,$U(n)$**
The *general linear group* $GL_n(F)$ is the group of invertible$n \times n$ matrices over a field$F$, with matrix multiplication as the operation. The *special linear group*$SL_n(F)$ is the subgroup of matrices with determinant 1. The *orthogonal group*$O(n)$ is the group of real matrices with$A^T A = I$; the *unitary group*$U(n)$ is its complex analogue. These matrix groups are the entry point into Lie theory (Part X) and representation theory (Part IX), and connecting the abstract group axioms to linear algebra is a recurring theme throughout the book.

**13.2.6 The Quaternion Group $Q_8$**
The quaternion group $Q_8 = \{\pm 1, \pm i, \pm j, \pm k\}$ has order 8 and satisfies$i^2 = j^2 = k^2 = ijk = -1$. It is nonabelian and provides a critical counterexample:$Q_8$ and the dihedral group$D_4$ are both nonabelian groups of order 8, yet they are not isomorphic. Their different structures illustrate that group order does not determine group structure and that nonabelian groups require more subtle invariants for classification.

---

**13.3 Subgroups**

**13.3.1 The Subgroup Test**
A nonempty subset $H \subseteq G$ is a *subgroup* if it is closed under the group operation and under inverses. Equivalently (the *two-step subgroup test*):$H \neq \emptyset$, and for all$a, b \in H$ we have$ab^{-1} \in H$. For finite groups, closure alone suffices. The subgroup test is the primary tool for verifying that specific subsets form groups without re-checking all four axioms.

**13.3.2 The Center $Z(G)$; Centralizers$C_G(a)$; Normalizers**
The *center* $Z(G) = \{g \in G : gx = xg \text{ for all } x \in G\}$ is the set of elements that commute with everything; it is always a subgroup, and always abelian. The *centralizer*$C_G(a) = \{g \in G : ga = ag\}$ is the set of elements commuting with a specific$a$; also a subgroup. The *normalizer*$N_G(H) = \{g \in G : gHg^{-1} = H\}$ is the largest subgroup in which$H$ is normal. These three constructions encode the internal symmetry structure of a group and will be essential for the class equation and Sylow theory.

**13.3.3 The Subgroup Generated by a Set; Cyclic Subgroups**
Given $S \subseteq G$, the subgroup generated by$S$, written$\langle S \rangle$, is the smallest subgroup containing$S$ — equivalently, all finite products of elements of$S$ and their inverses. When$S = \{g\}$, this gives the *cyclic subgroup*$\langle g \rangle = \{g^n : n \in \mathbb{Z}\}$ generated by$g$. The order of$g$ is$|\langle g \rangle|$. Cyclic subgroups are the atoms from which all groups are built.

**13.3.4 The Order of an Element; Finite vs. Infinite Order**
The order of $g \in G$ is$\mathrm{ord}(g) = |\langle g \rangle|$. In a finite group, every element has finite order dividing$|G|$ (a consequence of Lagrange's theorem). The order of a product$ab$ is not determined by$\mathrm{ord}(a)$ and$\mathrm{ord}(b)$ alone when the group is nonabelian — this subtlety distinguishes group theory from arithmetic.

---

**13.4 Cyclic Groups**

**13.4.1 Cyclic Groups: $G = \langle g \rangle$**
A group $G$ is *cyclic* if$G = \langle g \rangle$ for some element$g$, called a *generator*. Cyclic groups are the simplest possible groups — every element is a power of a single element. Every cyclic group is abelian (since$g^m g^n = g^{m+n} = g^n g^m$). Cyclic groups arise everywhere: as factor groups, as Galois groups of finite fields, and as the basic building blocks in the classification of finite abelian groups.

**13.4.2 Classification: $\mathbb{Z}$ and$\mathbb{Z}/n\mathbb{Z}$**
Every infinite cyclic group is isomorphic to $(\mathbb{Z}, +)$, and every finite cyclic group of order$n$ is isomorphic to$\mathbb{Z}/n\mathbb{Z}$. This classification is proved by defining a surjective homomorphism$\mathbb{Z} \to G$ via$k \mapsto g^k$ and identifying its kernel. The kernel is either$\{0\}$ (giving$G \cong \mathbb{Z}$) or$n\mathbb{Z}$ for some$n > 0$ (giving$G \cong \mathbb{Z}/n\mathbb{Z}$).

**13.4.3 Generators of $\mathbb{Z}/n\mathbb{Z}$ and Euler's$\phi$-function**
The generators of $\mathbb{Z}/n\mathbb{Z}$ are the elements$\bar{k}$ with$\gcd(k, n) = 1$. The number of generators is therefore$\phi(n)$, Euler's totient function. This connects group theory to elementary number theory and illustrates that even the simplest groups have nontrivial arithmetic content.

**13.4.4 Subgroups of Cyclic Groups; the Subgroup Lattice of $\mathbb{Z}/n\mathbb{Z}$**
Every subgroup of a cyclic group is cyclic. In $\mathbb{Z}/n\mathbb{Z}$, the subgroups are exactly$\langle \bar{d} \rangle$ for each divisor$d$ of$n$, and there is exactly one subgroup of each order dividing$n$. The subgroup lattice of$\mathbb{Z}/n\mathbb{Z}$ is isomorphic (as a partially ordered set) to the divisor lattice of$n$, ordered by divisibility. This beautiful structural result will later generalize to the Galois correspondence.

---

## Chapter 14 — Cosets, Normal Subgroups, and Quotient Groups

**What it establishes:** The coset partition of a group by a subgroup (and Lagrange's theorem as its first consequence); the condition under which a subgroup's cosets themselves form a group (normality); and the construction of the quotient group, the group-theoretic analogue of modular arithmetic.

---

**14.1 Cosets**

**14.1.1 Left and Right Cosets; the Coset Partition**
For a subgroup $H \leq G$ and an element$g \in G$, the *left coset* of$H$ containing$g$ is$gH = \{gh : h \in H\}$, and the *right coset* is$Hg = \{hg : h \in H\}$. The left cosets of$H$ in$G$ partition$G$ into disjoint subsets, each of the same size$|H|$. This coset partition is the foundational structural fact from which Lagrange's theorem follows immediately.

**14.1.2 The Index $[G:H]$; Coset Representatives**
The number of distinct left cosets of $H$ in$G$ is called the *index* of$H$ in$G$, written$[G:H]$. A *set of coset representatives* is a choice of one element from each coset. When$G$ is finite,$|G| = [G:H] \cdot |H|$ — a statement with no reference to normality, valid for every subgroup.

**14.1.3 Lagrange's Theorem and Its Proof**
*Lagrange's theorem:* If $G$ is a finite group and$H \leq G$, then$|H|$ divides$|G|$. The proof is the counting argument:$G$ partitions into$[G:H]$ cosets, each of size$|H|$, so$|G| = [G:H] \cdot |H|$. Lagrange's theorem is one of the most important and frequently applied theorems in group theory: it immediately restricts what subgroups and what element orders can exist in a finite group.

**14.1.4 Consequences: Element Orders, Prime-Order Groups, Fermat–Euler**
Since $\langle g \rangle$ is a subgroup of$G$,$\mathrm{ord}(g)$ divides$|G|$ for any$g$ in a finite group. In particular,$g^{|G|} = e$ for all$g$. If$|G| = p$ is prime, then the only subgroups are$\{e\}$ and$G$, so$G$ must be cyclic and$G \cong \mathbb{Z}/p\mathbb{Z}$. Setting$G = (\mathbb{Z}/n\mathbb{Z})^*$ recovers Euler's theorem$a^{\phi(n)} \equiv 1 \pmod{n}$; setting$n = p$ recovers Fermat's little theorem.

---

**14.2 Normal Subgroups**

**14.2.1 The Normality Condition: $gNg^{-1} = N$**
A subgroup $N \leq G$ is *normal* (written$N \trianglelefteq G$) if$gNg^{-1} = N$ for all$g \in G$ — equivalently, conjugation by any group element sends$N$ to itself. Normality is the condition that left and right cosets coincide:$gN = Ng$ for all$g$. Without normality, there is no natural way to multiply cosets, and the quotient construction fails.

**14.2.2 Equivalent Characterizations of Normality**
Normal subgroups can be characterized in five equivalent ways: (i) $gNg^{-1} \subseteq N$ for all$g$; (ii)$gNg^{-1} = N$ for all$g$; (iii)$gN = Ng$ for all$g$; (iv)$N$ is a union of conjugacy classes; (v)$N$ is the kernel of some group homomorphism. Each characterization is useful in different contexts, and fluency with all of them is essential.

**14.2.3 Examples: $A_n$,$Z(G)$,$SL_n$, Kernels of Homomorphisms**
The alternating group $A_n$ is normal in$S_n$ (it is the kernel of the sign homomorphism). The center$Z(G)$ is normal in$G$ (conjugation fixes every central element). The special linear group$SL_n$ is normal in$GL_n$ (it is the kernel of the determinant). Every kernel of a group homomorphism is normal — a fact that gives an inexhaustible supply of examples and that will be used in proving the isomorphism theorems.

**14.2.4 The Correspondence Between Normal Subgroups and Kernels**
Every normal subgroup is the kernel of a homomorphism (namely, the projection to the quotient group). Conversely, every kernel is normal. This bijection — normal subgroups correspond exactly to kernels — is the group-theoretic analogue of the correspondence between ideals and kernels in ring theory, and it determines the structure of the isomorphism theorems.

---

**14.3 Quotient Groups**

**14.3.1 Making the Coset Space Into a Group**
Given $N \trianglelefteq G$, define a multiplication on cosets by$(aN)(bN) = (ab)N$. We verify this is well-defined: if$aN = a'N$ and$bN = b'N$, then$(ab)N = (a'b')N$. (This verification is where normality is used.) The resulting operation satisfies the four group axioms, making the set of cosets$G/N$ into a group.

**14.3.2 The Quotient Group $G/N$ and Its Order**
The quotient group $G/N$ (read "$G$ mod$N$") has order$[G:N] = |G|/|N|$ when$G$ is finite. Its elements are the cosets of$N$, the identity is$N = eN$, and inverses satisfy$(aN)^{-1} = a^{-1}N$. Quotient groups allow us to "collapse" a normal subgroup to a single point and study the remaining structure, just as modular arithmetic collapses multiples of$n$ to zero.

**14.3.3 The Canonical Projection $\pi: G \to G/N$**
The map $\pi: G \to G/N$ defined by$\pi(g) = gN$ is a surjective group homomorphism with kernel$N$. This canonical projection is the universal object associated to$N$ in$G$: any homomorphism$G \to H$ with$N$ in its kernel factors uniquely through$\pi$. This universal property characterizes quotient groups and will be systematized in the isomorphism theorems.

**14.3.4 Examples: $\mathbb{Z}/n\mathbb{Z}$,$S_3/A_3$,$GL_n/SL_n$**
The quotient $\mathbb{Z}/n\mathbb{Z}$ (integers mod$n$) is recovered as$\mathbb{Z}/n\mathbb{Z}$ in additive notation. The quotient$S_3/A_3$ has order 2 and is isomorphic to$\mathbb{Z}/2\mathbb{Z}$ (the sign of a permutation). The quotient$GL_n(F)/SL_n(F)$ is isomorphic to$(F^*, \cdot)$ via the determinant map. These examples illustrate the power of the quotient construction: it extracts meaningful invariants from complex groups.

---

## Chapter 15 — Homomorphisms and the Isomorphism Theorems

**What it establishes:** Group homomorphisms as the structure-preserving maps of group theory; the isomorphism theorems as the fundamental bridge between quotient groups and subgroups; and direct and semidirect products as tools for building new groups from old ones.

---

**15.1 Group Homomorphisms**

**15.1.1 Definition; Immediate Consequences**
A *group homomorphism* $\phi: G \to H$ is a function satisfying$\phi(ab) = \phi(a)\phi(b)$ for all$a, b \in G$. Immediate consequences:$\phi(e_G) = e_H$;$\phi(g^{-1}) = \phi(g)^{-1}$;$\phi(g^n) = \phi(g)^n$; the image of a subgroup is a subgroup; the preimage of a subgroup is a subgroup. The single condition$\phi(ab) = \phi(a)\phi(b)$ implies all of these structural properties.

**15.1.2 Isomorphisms, Endomorphisms, Automorphisms**
A bijective homomorphism is an *isomorphism*; an isomorphism from a group to itself is an *automorphism*. An *endomorphism* is a homomorphism from a group to itself. Two groups are *isomorphic* (written $G \cong H$) if an isomorphism exists between them. Isomorphism is the correct notion of "the same group": isomorphic groups have identical algebraic structure and differ only in the names of elements.

**15.1.3 Kernel and Image as Subgroups**
The *kernel* $\ker \phi = \{g \in G : \phi(g) = e_H\}$ is a normal subgroup of$G$. The *image*$\mathrm{im}\, \phi = \{\phi(g) : g \in G\}$ is a subgroup of$H$. The homomorphism$\phi$ is injective if and only if$\ker \phi = \{e\}$, and surjective if and only if$\mathrm{im}\, \phi = H$. These subgroups measure the non-injectivity and non-surjectivity of$\phi$ respectively.

**15.1.4 Inner Automorphisms; the Automorphism Group $\mathrm{Aut}(G)$**
For each $g \in G$, the map$\phi_g: G \to G$ defined by$\phi_g(x) = gxg^{-1}$ is an automorphism, called an *inner automorphism* (or conjugation by$g$). The set of all automorphisms of$G$ forms a group$\mathrm{Aut}(G)$ under composition. The inner automorphisms form a normal subgroup$\mathrm{Inn}(G) \trianglelefteq \mathrm{Aut}(G)$; the quotient$\mathrm{Out}(G) = \mathrm{Aut}(G)/\mathrm{Inn}(G)$ is the *outer automorphism group*, an important invariant in the classification of groups.

---

**15.2 The Isomorphism Theorems**

**15.2.1 First Isomorphism Theorem: $G/\ker\phi \cong \mathrm{im}\,\phi$**
If $\phi: G \to H$ is a group homomorphism, then$G/\ker\phi \cong \mathrm{im}\,\phi$. The isomorphism sends$g\ker\phi \mapsto \phi(g)$. This is the first and most important isomorphism theorem: it says that every homomorphic image of$G$ is realized as a quotient of$G$. Every subsequent algebraic structure (rings, modules, Lie algebras) has an analogue of this theorem, and the pattern is always the same.

**15.2.2 Second Isomorphism Theorem: $H/(H \cap N) \cong HN/N$**
If $H \leq G$ and$N \trianglelefteq G$, then$HN$ is a subgroup of$G$,$H \cap N$ is normal in$H$, and$H/(H \cap N) \cong HN/N$. This theorem is sometimes called the "diamond theorem" because of the shape of the corresponding subgroup lattice. It describes how a subgroup and a normal subgroup interact and is essential in the proofs of the Sylow theorems and Jordan–Hölder theorem.

**15.2.3 Third Isomorphism Theorem: $(G/N)/(M/N) \cong G/M$**
If $N \trianglelefteq M \trianglelefteq G$ (both normal,$N \subseteq M$), then$M/N$ is normal in$G/N$ and$(G/N)/(M/N) \cong G/M$. This says that "collapsing in stages" gives the same result as "collapsing all at once." It is the group-theoretic analogue of the fraction law$(a/b)/(c/b) = a/c$.

**15.2.4 Correspondence (Fourth) Theorem: Subgroups of $G/N$**
If $N \trianglelefteq G$, then there is a bijection between subgroups of$G/N$ and subgroups of$G$ containing$N$, given by$H \mapsto H/N$ and its inverse$K \mapsto \pi^{-1}(K)$. This bijection preserves inclusion, normality, and index. It is the Correspondence Theorem: passing to a quotient does not destroy the subgroup structure above the kernel, it simply quotients it out.

---

**15.3 Direct and Semidirect Products**

**15.3.1 External and Internal Direct Products**
The *external direct product* $G \times H$ of two groups is the Cartesian product with componentwise operation. Its order is$|G| \cdot |H|$. Conversely, a group$G$ is the *internal direct product* of subgroups$H$ and$K$ if$H, K \trianglelefteq G$,$H \cap K = \{e\}$, and$HK = G$. Internal and external direct products are isomorphic when the conditions hold:$G \cong H \times K$.

**15.3.2 Recognizing Internal Direct Products: Three Conditions**
To recognize a group as an internal direct product $H \times K$: (i) both$H$ and$K$ are normal in$G$; (ii)$H \cap K = \{e\}$; (iii)$HK = G$. All three conditions are necessary. This recognition theorem is used extensively in classifying groups of small order and in the proof of the structure theorem for finite abelian groups.

**15.3.3 Semidirect Products: $N \rtimes_\phi H$**
When $N \trianglelefteq G$ and$H \leq G$ with$N \cap H = \{e\}$ and$NH = G$, but$H$ is not required to be normal,$G$ is a *semidirect product*$N \rtimes_\phi H$. The action$\phi: H \to \mathrm{Aut}(N)$ encodes how$H$ acts on$N$ by conjugation:$\phi(h)(n) = hnh^{-1}$. The group operation on$N \times H$ is$(n_1, h_1)(n_2, h_2) = (n_1 \phi(h_1)(n_2), h_1 h_2)$. When$\phi$ is trivial, this reduces to the direct product.

**15.3.4 Constructing Groups via Semidirect Products**
The semidirect product construction is the primary tool for building nonabelian groups from abelian pieces. Given $N$,$H$, and a homomorphism$\phi: H \to \mathrm{Aut}(N)$, one constructs a new group$N \rtimes_\phi H$. Different choices of$\phi$ can yield non-isomorphic groups. For example, the dihedral group$D_n \cong \mathbb{Z}/n\mathbb{Z} \rtimes \mathbb{Z}/2\mathbb{Z}$, where the generator of$\mathbb{Z}/2\mathbb{Z}$ acts by inversion. This construction will reappear in Sylow analysis and the Levi decomposition of Lie algebras.

---

## Chapter 16 — Group Actions

**What it establishes:** The definition and first theory of groups acting on sets; the orbit-stabilizer theorem as the key computational and structural tool; and the major applications — Cayley's theorem, Burnside's lemma, and the class equation — that demonstrate the unity of group theory with combinatorics.

---

**16.1 Group Actions**

**16.1.1 Definition: A Homomorphism $G \to \mathrm{Sym}(X)$**
A *group action* of $G$ on a set$X$ is a homomorphism$\rho: G \to \mathrm{Sym}(X)$, where$\mathrm{Sym}(X)$ is the group of all bijections$X \to X$. Equivalently, it is a function$G \times X \to X$, written$(g, x) \mapsto g \cdot x$, satisfying:$e \cdot x = x$ and$(gh) \cdot x = g \cdot (h \cdot x)$ for all$g, h \in G$ and$x \in X$. The action axioms encode the idea that "$G$ acts consistently on$X$."

**16.1.2 Left vs. Right Actions; Faithful, Free, and Transitive Actions**
A *left* action satisfies $(gh) \cdot x = g \cdot (h \cdot x)$; a *right* action satisfies$x \cdot (gh) = (x \cdot g) \cdot h$. An action is *faithful* if the only$g$ fixing all$x$ is the identity (the homomorphism$\rho$ is injective); *free* if$g \cdot x = x \Rightarrow g = e$ for all$x$; *transitive* if for every$x, y \in X$ there exists$g$ with$g \cdot x = y$ (there is only one orbit). These properties classify the "quality" of an action and appear constantly in applications.

**16.1.3 Examples: Left Multiplication, Conjugation, Action on Cosets**
*Left multiplication:* $G$ acts on itself by$g \cdot x = gx$ (faithful, transitive, and free). *Conjugation:*$G$ acts on itself by$g \cdot x = gxg^{-1}$ (faithful iff$Z(G) = \{e\}$, not generally transitive). *Action on cosets:*$G$ acts on the set of left cosets$G/H$ by$g \cdot (aH) = (ga)H$. These three canonical actions underlie Cayley's theorem, the class equation, and the embedding of$G$ in$S_{[G:H]}$ respectively.

---

**16.2 Orbits and Stabilizers**

**16.2.1 Orbits; the Orbit Partition of $X$**
The *orbit* of $x \in X$ under$G$ is$G \cdot x = \{g \cdot x : g \in G\}$. Orbits partition$X$: two orbits are either equal or disjoint. The partition of$X$ into orbits is the first structural information an action provides. The action is transitive if and only if there is exactly one orbit.

**16.2.2 Stabilizers as Subgroups**
The *stabilizer* (or *isotropy group*) of $x \in X$ is$G_x = \{g \in G : g \cdot x = x\}$. One checks that$G_x$ is a subgroup of$G$. The stabilizer measures how much of$G$ "fixes" the point$x$. Different points in the same orbit have conjugate stabilizers:$G_{g \cdot x} = g G_x g^{-1}$.

**16.2.3 The Orbit-Stabilizer Theorem: $|G| = |G \cdot x| \cdot |G_x|$**
For a finite group $G$ acting on$X$, the *orbit-stabilizer theorem* states$|G| = |G \cdot x| \cdot |G_x|$ for any$x \in X$. The proof constructs a bijection between the orbit$G \cdot x$ and the coset space$G/G_x$: the map$gG_x \mapsto g \cdot x$ is well-defined and bijective. This theorem is one of the most useful formulas in combinatorial group theory, relating the size of an orbit to the size of a stabilizer.

**16.2.4 The Class Equation of a Finite Group**
When $G$ acts on itself by conjugation, the orbits are the *conjugacy classes*. The class equation partitions$|G|$ as:
$$|G| = |Z(G)| + \sum_i [G : C_G(x_i)]$$
where the sum is over one representative $x_i$ from each conjugacy class of size$> 1$. Since each term$[G : C_G(x_i)]$ divides$|G|$, the class equation provides strong divisibility constraints on the structure of$G$ — constraints that drive the proof that$p$-groups have nontrivial centers.

---

**16.3 Applications of Group Actions**

**16.3.1 $p$-Groups Have Non-Trivial Centers (via Class Equation)**
A *$p$-group* is a finite group of order$p^n$ for some prime$p$ and$n \geq 1$. By the class equation,$p$ divides$|G|$ and each$[G : C_G(x_i)]$, so$p$ divides$|Z(G)|$. Therefore$Z(G) \neq \{e\}$ for any nontrivial$p$-group. This apparently simple observation has enormous consequences: it underlies the classification of groups of order$p^2$ as abelian, and it is the base case for inductive arguments throughout finite group theory.

**16.3.2 Cayley's Theorem: Every Group Embeds in a Symmetric Group**
*Cayley's theorem:* Every group $G$ is isomorphic to a subgroup of$\mathrm{Sym}(G)$. The proof uses the left multiplication action: the map$\rho: G \to \mathrm{Sym}(G)$ sending$g$ to the permutation$x \mapsto gx$ is an injective homomorphism. For finite groups,$G$ embeds in$S_{|G|}$. Cayley's theorem shows that abstract group theory and the theory of permutation groups are equivalent, providing a concrete model for every group.

**16.3.3 Burnside's Lemma: Counting Orbits**
*Burnside's lemma:* The number of orbits of a $G$-action on a finite set$X$ is the average number of fixed points:
$$|\text{orbits}| = \frac{1}{|G|} \sum_{g \in G} |X^g|$$
where $X^g = \{x \in X : g \cdot x = x\}$. This formula reduces orbit-counting problems — such as counting distinct colorings of symmetric objects — to fixed-point counting, which is often tractable. It is the standard tool for combinatorial enumeration problems with symmetry.

**16.3.4 The Action of $G$ on Left Cosets of$H$; Kernel of This Action**
$G$ acts on the set of left cosets$G/H$ by left multiplication. The kernel of the resulting homomorphism$G \to S_{[G:H]}$ is the largest normal subgroup of$G$ contained in$H$ — called the *core* of$H$ in$G$, denoted$\mathrm{Core}_G(H) = \bigcap_{g \in G} gHg^{-1}$. This action provides an embedding of$G/\mathrm{Core}_G(H)$ into$S_{[G:H]}$, giving an upper bound on the index of a non-normal subgroup and a tool for proving simplicity.

---

## Chapter 17 — The Sylow Theorems

**What it establishes:** The existence, conjugacy, and count of prime-power subgroups in any finite group — the three Sylow theorems, which together provide the sharpest general structural information about finite groups and drive most classification arguments.

---

**17.1 $p$-Groups**

**17.1.1 Definition and Basic Properties**
A *$p$-group* is a group in which every element has order a power of the prime$p$. For finite groups, this is equivalent to$|G| = p^n$ for some$n \geq 0$.$p$-Groups are the building blocks of finite group theory: every finite group has$p$-group subgroups (Sylow subgroups), and the structure of a finite group is largely determined by how its Sylow subgroups interact.

**17.1.2 $p$-Groups Have Non-Trivial Centers**
Every nontrivial $p$-group has nontrivial center (proved in Chapter 16 via the class equation). This is the key structural fact about$p$-groups: they are never simple (except the trivial group and$\mathbb{Z}/p\mathbb{Z}$). The center provides an inductive foothold:$G/Z(G)$ is a smaller$p$-group, allowing structural arguments to proceed by induction on$|G|$.

**17.1.3 Groups of Order $p^2$ Are Abelian**
If $|G| = p^2$, then$G$ is abelian. The proof:$Z(G)$ is nontrivial; if$|Z(G)| = p^2$ we are done; if$|Z(G)| = p$, then$G/Z(G)$ has order$p$ and is cyclic, but a group whose quotient by its center is cyclic must be abelian — contradiction. Therefore$G \cong \mathbb{Z}/p^2\mathbb{Z}$ or$G \cong \mathbb{Z}/p\mathbb{Z} \times \mathbb{Z}/p\mathbb{Z}$.

---

**17.2 Sylow Subgroups**

**17.2.1 Definition: A Subgroup of Order $p^a$ When$|G| = p^a m$**
Let $G$ be a finite group with$|G| = p^a m$ where$p \nmid m$. A *Sylow$p$-subgroup* of$G$ is a subgroup of order$p^a$ — the largest possible power of$p$. The Sylow theorems assert that such subgroups exist, are conjugate to each other, and that their number is constrained. The definition identifies the "maximal$p$-power piece" of a finite group.

**17.2.2 Sylow I: Existence of Sylow $p$-Subgroups**
*Sylow's first theorem:* Every finite group $G$ has at least one Sylow$p$-subgroup for every prime$p$ dividing$|G|$. The standard proof uses the action of$G$ on$p$-element subsets of$G$ by left multiplication, applying the orbit-stabilizer theorem and careful divisibility arguments. Existence is the most important of the three Sylow theorems: it guarantees the presence of a large$p$-subgroup in every finite group, regardless of the group's other structure.

**17.2.3 Sylow II: All Sylow $p$-Subgroups Are Conjugate**
*Sylow's second theorem:* Any two Sylow $p$-subgroups of$G$ are conjugate in$G$: if$P$ and$Q$ are both Sylow$p$-subgroups, then$Q = gPg^{-1}$ for some$g \in G$. As a consequence, the Sylow$p$-subgroups form a single conjugacy class in$G$. This conjugacy means that any two Sylow$p$-subgroups are isomorphic and play structurally equivalent roles in$G$.

**17.2.4 Sylow III: The Number $n_p$ Satisfies$n_p \equiv 1 \pmod{p}$ and$n_p \mid m$**
*Sylow's third theorem:* Let $n_p$ denote the number of Sylow$p$-subgroups of$G$. Then$n_p \equiv 1 \pmod{p}$ and$n_p$ divides$m = |G|/p^a$. Since$n_p = [G : N_G(P)]$ (the index of the normalizer of a Sylow subgroup), the constraint$n_p \mid m$ is Lagrange's theorem in disguise. The constraint$n_p \equiv 1 \pmod{p}$ comes from a$p$-group action argument. Together, the two constraints on$n_p$ are often strong enough to force$n_p = 1$, implying the Sylow subgroup is normal.

---

**17.3 Applications of the Sylow Theorems**

**17.3.1 Groups of Small Order: Classification by Sylow Analysis**
The Sylow theorems reduce the classification of finite groups of small order to a systematic case analysis. For each prime power dividing $|G|$, one computes the possible values of$n_p$ using the constraints from Sylow III. When$n_p = 1$, the unique Sylow$p$-subgroup is normal; when multiple Sylow subgroups exist, one studies their intersections and the resulting structure. This analysis classifies groups of orders$1$ through$15$, for instance, without case-by-case ad hoc arguments.

**17.3.2 Sufficient Conditions for a Group to Be Non-Simple**
A group $G$ is *simple* if its only normal subgroups are$\{e\}$ and$G$. The Sylow theorems give sufficient conditions for a group to be non-simple: if$|G| = p^a m$ with$m < p$, then$n_p = 1$ and the Sylow$p$-subgroup is normal. More generally, if the Sylow analysis forces$n_p = 1$ for any prime$p$, then$G$ is not simple. Proving a group is non-simple by Sylow arguments is the standard first step in any group classification.

**17.3.3 Groups of Specific Orders (pq, $p^2 q$, etc.)**
For groups of order $pq$ ($p < q$ primes): if$p \nmid q - 1$, then$G \cong \mathbb{Z}/pq\mathbb{Z}$; otherwise$G \cong \mathbb{Z}/p\mathbb{Z} \rtimes \mathbb{Z}/q\mathbb{Z}$ (the unique nonabelian group of order$pq$). For groups of order$p^2 q$: Sylow analysis forces one of the Sylow subgroups to be normal, allowing$G$ to be expressed as a semidirect product. These case studies are the standard training ground for Sylow analysis.

**17.3.4 Embedding $G$ in$S_{n_p}$ via the Conjugation Action**
$G$ acts on its Sylow$p$-subgroups by conjugation, giving a homomorphism$G \to S_{n_p}$. The kernel is the core of a Sylow$p$-subgroup, which is a normal subgroup of$G$. If$G$ is simple, the kernel must be trivial, giving an embedding$G \hookrightarrow S_{n_p}$. This bounds$|G|$ in terms of$n_p$:$|G| \leq (n_p)!$. This embedding is the key step in proving$A_5$ is the unique simple group of order 60 and in bounding simple groups of other orders.

---

## Chapter 18 — Structure of Groups

**What it establishes:** The deeper architecture of how groups are assembled from simpler pieces — simple groups as the atomic building blocks, composition series and the Jordan–Hölder theorem as the theory of factorization, solvable and nilpotent groups as controlled hierarchies, and free groups and presentations as the language for defining groups by generators and relations.

---

**18.1 Simple Groups and Composition Series**

**18.1.1 Simple Groups: Definition and Small Examples**
A group $G$ is *simple* if$|G| > 1$ and$G$ has no proper nontrivial normal subgroups. Simple groups are the atoms of finite group theory: every finite group is built from simple groups in a precise sense. The simple groups of prime order are$\mathbb{Z}/p\mathbb{Z}$. The first nonabelian simple group is$A_5$, of order 60. The classification of all finite simple groups — completed in 2004 — is the greatest achievement in finite group theory.

**18.1.2 Composition Series; Composition Factors**
A *composition series* of $G$ is a chain$G = G_0 \supset G_1 \supset G_2 \supset \cdots \supset G_k = \{e\}$ where each$G_{i+1}$ is normal in$G_i$ and each quotient$G_i/G_{i+1}$ is simple. The quotients$G_i/G_{i+1}$ are the *composition factors*. Every finite group has a composition series (by induction on$|G|$: take$G_1$ to be a minimal normal subgroup). The composition factors are the "prime factorization" of a finite group.

**18.1.3 The Jordan–Hölder Theorem: Uniqueness of Composition Factors**
*Jordan–Hölder theorem:* Any two composition series of a finite group $G$ have the same length and the same composition factors (up to permutation and isomorphism). This is the group-theoretic analogue of the fundamental theorem of arithmetic: just as every integer has a unique factorization into primes, every finite group has a unique set of simple "factors" (in a composition series sense). The classification of finite groups thus reduces to: (1) classify all finite simple groups; (2) understand all the ways simple groups can be assembled — the *extension problem*.

---

**18.2 Solvable Groups**

**18.2.1 Derived Subgroups: $G' = [G,G]$; the Derived Series**
The *commutator* of $a, b \in G$ is$[a, b] = aba^{-1}b^{-1}$. The *derived subgroup* (or *commutator subgroup*)$G' = [G, G]$ is the subgroup generated by all commutators; it is normal in$G$ and$G/G'$ is the largest abelian quotient of$G$. The *derived series* is$G = G^{(0)} \supset G^{(1)} = G' \supset G^{(2)} = (G')' \supset \cdots$. Each term is characteristic (preserved by all automorphisms) and normal in all previous terms.

**18.2.2 Solvable Groups: Definition and Equivalent Conditions**
A group $G$ is *solvable* if its derived series reaches$\{e\}$ in finitely many steps:$G^{(n)} = \{e\}$ for some$n$. Equivalent conditions:$G$ has a composition series with all factors abelian (equivalently, cyclic of prime order). Every abelian group is solvable (derived series terminates at step 1). Solvable groups are precisely the groups that can be "built up from abelian pieces."

**18.2.3 Subgroups, Quotients, and Extensions of Solvable Groups**
Subgroups of solvable groups are solvable (the derived series of a subgroup is contained in the derived series of the ambient group). Quotients of solvable groups are solvable ($\phi(G^{(n)}) = \phi(G)^{(n)}$). If$N \trianglelefteq G$ and both$N$ and$G/N$ are solvable, then$G$ is solvable. This extension property allows solvability to be proved by induction.

**18.2.4 $S_n$ Is Not Solvable for$n \geq 5$;$A_5$ Is Simple**
We prove $A_5$ is simple (no proper nontrivial normal subgroups) by checking that its conjugacy classes cannot form a normal subgroup. Since$A_5$ is simple and nonabelian,$A_5^{(1)} = A_5$, and the derived series never terminates — so$A_5$ is not solvable. Since$A_n$ is the derived subgroup of$S_n$ for$n \geq 5$, and$A_n$ is not solvable for$n \geq 5$, neither is$S_n$. This is the group-theoretic obstruction to solving the general quintic by radicals, explained in Chapter 32.

---

**18.3 Nilpotent Groups**

**18.3.1 The Lower and Upper Central Series**
The *lower central series* is $G = \gamma_1(G) \supset \gamma_2(G) = [G, G] \supset \gamma_3(G) = [G, \gamma_2(G)] \supset \cdots$, where$\gamma_{i+1}(G) = [G, \gamma_i(G)]$. The *upper central series* is$\{e\} = Z_0(G) \subset Z_1(G) = Z(G) \subset Z_2(G) \subset \cdots$, where$Z_{i+1}(G)/Z_i(G) = Z(G/Z_i(G))$. Both series are intrinsic to the group and measure how far$G$ is from being abelian in a different sense from the derived series.

**18.3.2 Nilpotent Groups and Their Characterizations**
A group $G$ is *nilpotent* if its lower central series reaches$\{e\}$ (equivalently, its upper central series reaches$G$). Nilpotent groups are "more abelian" than merely solvable: every nilpotent group is solvable, but not conversely. Key characterizations:$G$ is nilpotent iff every subgroup is subnormal; iff$G$ is a direct product of its Sylow subgroups; iff every maximal subgroup is normal. These characterizations make nilpotent groups much easier to work with than arbitrary solvable groups.

**18.3.3 Finite Nilpotent Groups Are Products of Their Sylow Subgroups**
A finite group is nilpotent if and only if it is the direct product of its Sylow subgroups. This is one of the most elegant characterizations in group theory: it reduces the structure of finite nilpotent groups completely to that of $p$-groups. In particular, every finite abelian group is nilpotent (and the classification of finite abelian groups in Chapter 19 is a special case of this structure theory).

**18.3.4 Nilpotent $\Rightarrow$ Solvable; the Hierarchy**
The hierarchy of group classes is: abelian $\subset$ nilpotent$\subset$ solvable$\subset$ finite groups. Each containment is strict. The classification of finite simple groups shows that the "non-solvable" simple groups — the sporadic groups and the groups of Lie type — have a rich additional structure. Solvability and nilpotency appear again in Galois theory (Chapter 32), where solvability of the Galois group characterizes solvability of a polynomial by radicals.

---

**18.4 Free Groups and Presentations**

**18.4.1 Free Groups: Universal Property and Construction**
The *free group* $F_S$ on a set$S$ (the *generators*) is the unique group (up to isomorphism) with a function$\iota: S \to F_S$ such that for any group$G$ and function$f: S \to G$, there exists a unique group homomorphism$\phi: F_S \to G$ with$\phi \circ \iota = f$. Concretely,$F_S$ consists of all reduced words in$S \cup S^{-1}$, with concatenation (followed by reduction) as the group operation. The free group on 2 generators is infinite and contains a copy of every finitely generated group as a quotient.

**18.4.2 Group Presentations: Generators and Relations**
A *group presentation* $\langle S \mid R \rangle$ specifies a group as$F_S / N(R)$, where$N(R)$ is the normal closure of a set of relations$R \subseteq F_S$. Every group has a presentation (using all elements as generators and all equations as relations), but nontrivial groups are specified by far fewer generators and relations. Presentations make the structure of a group explicit and computable, though proofs in presented groups can be nontrivial.

**18.4.3 Examples: Dihedral, Quaternion, Triangle Groups**
The dihedral group: $\langle r, s \mid r^n = s^2 = e, \, srs^{-1} = r^{-1} \rangle$. The quaternion group:$\langle i, j \mid i^4 = e, \, i^2 = j^2, \, ij = ji^{-1} \rangle$. Triangle groups:$\langle a, b, c \mid a^p = b^q = c^r = abc = e \rangle$. These presentations make the structure of well-known groups transparent and demonstrate how relations constrain the group's structure. Presentations are the language in which geometric groups, fundamental groups, and Coxeter groups are naturally expressed.

**18.4.4 The Word Problem and Its Undecidability**
The *word problem* for a finitely presented group $\langle S \mid R \rangle$ asks: given a word$w$ in$S \cup S^{-1}$, does$w = e$ in the group? For some groups, the word problem is solvable (there is an algorithm). For others, it is not — Novikov and Boone (1950s) constructed finitely presented groups with undecidable word problems. This is one of the first and most striking connections between algebra and computability theory, and it shows that abstract algebra reaches the limits of algorithmic decidability.

---

## Chapter 19 — Finitely Generated Abelian Groups

**What it establishes:** The complete, clean classification of all finitely generated abelian groups — a theorem that resolves every question about the structure of such groups and whose proof method (Smith normal form) generalizes to classify modules over PIDs and produce canonical forms for matrices.

---

**19.1 Torsion and Rank**

**19.1.1 Torsion Elements; the Torsion Subgroup**
An element $a$ of an abelian group$A$ is a *torsion element* if$na = 0$ for some positive integer$n$ (using additive notation). The set of all torsion elements forms a subgroup$T(A) \leq A$, called the *torsion subgroup*. The quotient$A/T(A)$ is torsion-free. This decomposition — torsion part and torsion-free part — is the first step in classifying$A$.

**19.1.2 Free Abelian Groups; the Rank**
A *free abelian group* of rank $r$ is a group isomorphic to$\mathbb{Z}^r = \mathbb{Z} \oplus \mathbb{Z} \oplus \cdots \oplus \mathbb{Z}$ ($r$ copies). The rank is well-defined (any two bases have the same cardinality). Free abelian groups are the torsion-free finitely generated abelian groups; they play the role of "vector spaces over$\mathbb{Z}$." Every finitely generated torsion-free abelian group is free.

**19.1.3 Finitely Generated Groups: Generators and Relations**
A finitely generated abelian group $A$ is a quotient of$\mathbb{Z}^n$ for some$n$ (choose$n$ generators and map$\mathbb{Z}^n$ surjectively onto$A$). The kernel of this surjection is the "relation subgroup." Since$\mathbb{Z}$ is a PID, every subgroup of$\mathbb{Z}^n$ is itself free abelian of rank$\leq n$. This reduces the classification to a linear algebra problem over$\mathbb{Z}$.

---

**19.2 The Structure Theorem**

**19.2.1 Statement: $\mathbb{Z}^r \oplus \mathbb{Z}/d_1 \oplus \cdots \oplus \mathbb{Z}/d_k$**
*The fundamental theorem of finitely generated abelian groups:* Every finitely generated abelian group is isomorphic to a direct sum
$$\mathbb{Z}^r \oplus \mathbb{Z}/d_1\mathbb{Z} \oplus \mathbb{Z}/d_2\mathbb{Z} \oplus \cdots \oplus \mathbb{Z}/d_k\mathbb{Z}$$
where $r \geq 0$ and$d_1 \mid d_2 \mid \cdots \mid d_k \geq 2$. The rank$r$ is the free part; the$d_i$ are the *invariant factors*, encoding the torsion. This is the complete classification.

**19.2.2 Invariant Factor Form vs. Primary Decomposition Form**
The *invariant factor form* expresses the torsion part as $\mathbb{Z}/d_1 \oplus \cdots \oplus \mathbb{Z}/d_k$ with$d_1 \mid d_2 \mid \cdots \mid d_k$. The *primary decomposition form* (or *elementary divisor form*) decomposes each$\mathbb{Z}/d_i$ further using the Chinese Remainder Theorem into$p$-primary components$\mathbb{Z}/p^{a_j}\mathbb{Z}$. Both forms are useful: invariant factors are convenient for theoretical arguments; elementary divisors for classification by prime power components.

**19.2.3 Uniqueness of the Invariant Factors**
The invariant factors are uniquely determined by $A$: they are not an artifact of a particular decomposition but genuine invariants. The proof of uniqueness uses the fact that the number of summands of each type is determined by the$p$-ranks$\dim_{\mathbb{F}_p}(A[p]/pA[p])$, which are intrinsic to$A$. This uniqueness is what makes the theorem a *classification* rather than a mere *decomposition*.

**19.2.4 Proof Strategy via Smith Normal Form**
The proof proceeds by choosing a surjection $\phi: \mathbb{Z}^n \to A$ and representing the kernel as the image of an$n \times m$ integer matrix$M$. By performing row and column operations over$\mathbb{Z}$ (each corresponding to a change of basis in$\mathbb{Z}^n$ or the kernel), one can reduce$M$ to *Smith normal form* — a diagonal matrix$\mathrm{diag}(d_1, \ldots, d_k, 0, \ldots, 0)$ with$d_1 \mid d_2 \mid \cdots \mid d_k$. The structure theorem falls out immediately. Smith normal form is an algorithm, making the classification effective.

---

**19.3 Applications**

**19.3.1 Classifying Abelian Groups of a Given Order**
Using the structure theorem, the number of abelian groups of order $n$ (up to isomorphism) is$\prod_{p \mid n} p(v_p(n))$, where$p(k)$ is the number of partitions of$k$ and$v_p(n)$ is the$p$-adic valuation of$n$. For example, the abelian groups of order 12 are$\mathbb{Z}/12$,$\mathbb{Z}/2 \oplus \mathbb{Z}/6$, and$\mathbb{Z}/2 \oplus \mathbb{Z}/2 \oplus \mathbb{Z}/3$. The structure theorem makes this enumeration systematic and complete.

**19.3.2 Computing the Group of Units $(\mathbb{Z}/n\mathbb{Z})^*$**
By the Chinese Remainder Theorem, $(\mathbb{Z}/n\mathbb{Z})^* \cong \prod_{p^a \| n} (\mathbb{Z}/p^a\mathbb{Z})^*$. Each factor$(\mathbb{Z}/p^a\mathbb{Z})^*$ is cyclic of order$\phi(p^a) = p^{a-1}(p-1)$ for odd$p$, while$(\mathbb{Z}/2^a\mathbb{Z})^*$ is$\mathbb{Z}/2 \times \mathbb{Z}/2^{a-2}$ for$a \geq 3$. The structure theorem classifies the resulting direct sum, determining which$(\mathbb{Z}/n\mathbb{Z})^*$ are cyclic (equivalently, which$n$ have a primitive root).

**19.3.3 Connection to Module Theory over $\mathbb{Z}$**
Finitely generated abelian groups are exactly finitely generated $\mathbb{Z}$-modules (since a$\mathbb{Z}$-module is just an abelian group). The structure theorem is the first case of the general structure theorem for finitely generated modules over a PID (Chapter 27), which in turn unifies the classification of abelian groups with the theory of rational and Jordan canonical forms. The Smith normal form algorithm generalizes to any PID.

---

*Next: [Part IV — Ring Theory](part-IV-ring-theory.md)*

*Prerequisites satisfied: Part I (logic, sets, functions), Part II (linear algebra, in particular the notion of structure-preserving map and quotient space).*
