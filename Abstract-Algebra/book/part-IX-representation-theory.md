# Part IX — Representation Theory of Finite Groups

**Chapters 42–45**

---

## What This Part Establishes

Part IX develops the representation theory of finite groups: the study of how groups act linearly on vector spaces. If group theory studies symmetry abstractly, representation theory realizes that symmetry concretely, as matrices acting on a vector space. The result is a remarkably complete theory, with a powerful invariant (the character) that classifies representations up to isomorphism and encodes the internal structure of the group.

Chapter 42 sets up the framework: representations as group homomorphisms $G \to GL(V)$, equivalently as$k[G]$-modules, and Schur's lemma as the key structural constraint on maps between irreducible representations. Chapter 43 proves Maschke's theorem (representations in characteristic$\nmid |G|$ decompose into irreducibles) and the Artin–Wedderburn theorem (the group algebra over$\mathbb{C}$ is a product of matrix algebras), deducing the fundamental counting formula$\sum n_i^2 = |G|$. Chapter 44 develops character theory: the trace of the representation, the orthogonality relations, and the character table, culminating in powerful group-theoretic applications including Burnside's theorem. Chapter 45 proves Frobenius reciprocity — the adjunction between induction and restriction — and develops Mackey theory and Frobenius groups.

By the end of Part IX, the reader can compute character tables of finite groups, decompose representations, and see how representation theory connects finite group theory to linear algebra and module theory.

---

## Internal Dependency Map

```
Ch 42 (Representations, Schur's Lemma)
            |
            v
Ch 43 (Maschke's Theorem, Artin-Wedderburn, Group Algebra)
            |
            v
Ch 44 (Characters, Orthogonality, Character Tables)
            |
            v
Ch 45 (Induced Representations, Frobenius Reciprocity)
```

---

## Chapter 42 — Group Representations

**What it establishes:** The two equivalent definitions of a representation (as a homomorphism $G \to GL(V)$ and as a$k[G]$-module); the first examples; and Schur's lemma, the fundamental structural constraint on intertwining maps between irreducible representations.

---

**42.1 Representations**

**42.1.1 Definitions: $\rho: G \to GL(V)$; Matrix Representations**
A *representation* of a group $G$ over a field$k$ is a group homomorphism$\rho: G \to GL(V)$ where$V$ is a$k$-vector space and$GL(V)$ is the group of invertible linear maps$V \to V$. For each$g \in G$,$\rho(g): V \to V$ is a$k$-linear automorphism of$V$. The *dimension* (or *degree*) of the representation is$\dim_k V$. A *matrix representation* is obtained by choosing a basis:$\rho: G \to GL_n(k)$ (invertible$n \times n$ matrices). The two formulations are equivalent once a basis is chosen, but the abstract formulation is better for conceptual arguments.

**42.1.2 Representations as $k[G]$-Modules**
A representation of $G$ on$V$ is equivalent to a$k[G]$-module structure on$V$: extend$\rho$ to the group algebra$k[G] = \bigoplus_{g \in G} k \cdot g$ by$k$-linearity, defining$(\sum a_g g) \cdot v = \sum a_g \rho(g)(v)$. Conversely, any$k[G]$-module$V$ gives a representation via$\rho(g)(v) = g \cdot v$. This equivalence is fundamental: it means that the representation theory of$G$ is the module theory of$k[G]$. All the machinery of module theory (Schur's lemma, Maschke's theorem, decomposition) is module theory of the group algebra.

**42.1.3 Equivalent Representations; $G$-Maps (Intertwiners)**
Two representations $(V, \rho)$ and$(W, \sigma)$ are *equivalent* (or *isomorphic*) if there exists an isomorphism$\phi: V \to W$ of$k$-vector spaces that is *$G$-equivariant*:$\phi \circ \rho(g) = \sigma(g) \circ \phi$ for all$g \in G$. Such a$\phi$ is called a *$G$-map* or *intertwining operator*. In the language of$k[G]$-modules,$G$-maps are exactly$k[G]$-module homomorphisms. Equivalent representations are "the same up to a change of basis in$V$," and the goal of representation theory is to classify representations up to equivalence.

**42.1.4 Subrepresentations; Quotient Representations**
A *subrepresentation* of $(V, \rho)$ is a subspace$W \subseteq V$ that is$G$-stable:$\rho(g)(W) \subseteq W$ for all$g \in G$. The restriction of$\rho$ to$W$ gives a representation$\rho|_W: G \to GL(W)$, and the quotient$V/W$ inherits a representation$G \to GL(V/W)$ by$g \cdot (v + W) = \rho(g)(v) + W$. Subrepresentations are the same as$k[G]$-submodules, and the classification of representations reduces to understanding their subrepresentation structure.

**42.1.5 Direct Sums, Tensor Products, and Dual Representations**
*Direct sum:* $(V \oplus W, \rho \oplus \sigma)$ where$(\rho \oplus \sigma)(g)(v, w) = (\rho(g)(v), \sigma(g)(w))$. *Tensor product:*$(V \otimes W, \rho \otimes \sigma)$ where$(\rho \otimes \sigma)(g)(v \otimes w) = \rho(g)(v) \otimes \sigma(g)(w)$. *Dual representation:*$(V^*, \rho^*)$ where$(\rho^*(g)(f))(v) = f(\rho(g^{-1})(v))$ (the "contragredient"). These operations make the set of representations of$G$ into a ring (the *representation ring*$R(G)$), and the character will turn out to be a ring homomorphism from$R(G)$ to the class functions on$G$.

---

**42.2 Key Examples**

**42.2.1 The Trivial Representation; the Sign Representation of $S_n$**
The *trivial representation*: $V = k$ with$\rho(g) = \mathrm{id}_k$ for all$g$. Every group has a trivial representation. The *sign representation* of$S_n$:$V = k$ with$\rho(\sigma) = \mathrm{sgn}(\sigma) \cdot \mathrm{id}_k$ ($+1$ for even permutations,$-1$ for odd). The sign representation is one-dimensional and nontrivial for$n \geq 2$. Both are irreducible (no proper nonzero subrepresentations, since$V$ is one-dimensional).

**42.2.2 The Permutation Representation; the Standard Representation**
The *permutation representation*: Let $G$ act on a set$X = \{x_1, \ldots, x_n\}$. The *permutation representation* on$V = k^X$ (with basis$\{e_{x_1}, \ldots, e_{x_n}\}$) is$\rho(g)(e_{x_i}) = e_{g \cdot x_i}$. For$G = S_n$ acting on$\{1, \ldots, n\}$, this gives the standard$n$-dimensional permutation representation. The subspace$U = k \cdot (e_{x_1} + \cdots + e_{x_n})$ is invariant (the trivial subrepresentation). The orthogonal complement$W = \ker(\sum_i e_i^*)$ (the "standard representation") is the natural complement and is usually irreducible.

**42.2.3 The Regular Representation $k[G]$**
The *regular representation*: $G$ acts on$V = k[G]$ (with basis$G$) by left multiplication:$\rho(g)(h) = gh$. The regular representation has dimension$|G|$ and contains every irreducible representation of$G$ as a subrepresentation (with multiplicity equal to its dimension — proved by Maschke's theorem and the Artin–Wedderburn theorem). Computing the decomposition of the regular representation essentially solves the classification problem for representations of$G$.

**42.2.4 Representations of Abelian Groups: All Irreducibles Are 1-Dimensional**
If $G$ is a finite abelian group and$k$ is algebraically closed of characteristic$\nmid |G|$, then every irreducible representation of$G$ is 1-dimensional. Proof: since$G$ is abelian, all$\rho(g)$ commute with each other; since$k$ is algebraically closed, each$\rho(g)$ has an eigenvector; a common eigenvector of all the$\rho(g)$ spans a 1-dimensional invariant subspace, which must be all of$V$ for an irreducible. The irreducible representations of an abelian group$G$ are group homomorphisms$G \to k^*$ — the *characters of$G$*, or *Pontryagin dual*.

---

**42.3 Schur's Lemma**

**42.3.1 $G$-Maps between Irreducibles Are 0 or Isomorphisms**
*Schur's Lemma:* Let $V$ and$W$ be irreducible representations of$G$ over any field$k$, and let$\phi: V \to W$ be a$G$-map. Then either$\phi = 0$ or$\phi$ is an isomorphism. Proof:$\ker \phi \subseteq V$ is a$G$-stable subspace; since$V$ is irreducible,$\ker \phi = 0$ or$\ker \phi = V$. Similarly,$\mathrm{im}\, \phi \subseteq W$ is$G$-stable; since$W$ is irreducible,$\mathrm{im}\, \phi = 0$ or$W$. Combined: either$\phi = 0$ or$\phi$ is an isomorphism.

**42.3.2 $\mathrm{Hom}_G(V,V) = k$ for Irreducible$V$ over Algebraically Closed$k$**
If $V$ is an irreducible representation over an algebraically closed field$k$ and$\phi: V \to V$ is a$G$-map, then$\phi = \lambda \cdot \mathrm{id}_V$ for some$\lambda \in k$. Proof: since$k$ is algebraically closed,$\phi$ has an eigenvalue$\lambda \in k$; then$\phi - \lambda \mathrm{id}$ is a$G$-map with a nontrivial kernel; by Schur's lemma,$\phi - \lambda \mathrm{id} = 0$. Therefore$\mathrm{Hom}_G(V, V) \cong k$ (the only$G$-maps from$V$ to itself are scalar multiples of the identity). This is the "strong form" of Schur's lemma and is the foundation for the orthogonality of characters.

**42.3.3 Consequences for Structure of Representations**
From Schur's lemma: if $V \not\cong W$ are non-isomorphic irreducibles, then$\mathrm{Hom}_G(V, W) = 0$. The endomorphism algebra$\mathrm{End}_G(V)$ of an irreducible$V$ over an algebraically closed field is a division algebra (by Schur), and equals$k$ (since$k = \bar k$). These facts immediately imply that any$G$-map between direct sums of irreducibles is block-diagonal (zero between non-isomorphic blocks, scalar matrices within isotypic blocks), which is the key to proving the Artin–Wedderburn decomposition.

---

## Chapter 43 — Complete Reducibility and the Group Algebra

**What it establishes:** Maschke's theorem as the key structural fact about representations in characteristic $\nmid |G|$; the Artin–Wedderburn theorem decomposing the group algebra into matrix algebras; and the fundamental numerical constraints on dimensions and multiplicities of irreducible representations.

---

**43.1 Maschke's Theorem**

**43.1.1 Statement: $\mathrm{char}(k) \nmid |G| \Rightarrow$ Complete Reducibility**
*Maschke's Theorem:* If $G$ is a finite group and$k$ is a field with$\mathrm{char}(k) \nmid |G|$ (i.e., the characteristic of$k$ does not divide the order of$G$), then every representation of$G$ is completely reducible: it decomposes as a direct sum of irreducible subrepresentations. In module language:$k[G]$ is a semisimple ring (every$k[G]$-module is semisimple, i.e., a direct sum of simple modules).

**43.1.2 Proof: Averaging over $G$ to Produce an Equivariant Projection**
The proof produces an equivariant projection onto any subrepresentation $W \subseteq V$: start with any projection$p: V \to W$ (as vector spaces, not necessarily$G$-equivariant); average over$G$:$\bar p = \frac{1}{|G|} \sum_{g \in G} \rho(g) \circ p \circ \rho(g)^{-1}$. The averaged projection$\bar p$ is$G$-equivariant (interchanges with$\rho(g)$), fixes$W$ (since$W$ is$G$-stable and$p$ fixes$W$), and maps$V$ into$W$. Division by$|G|$ is valid because$\mathrm{char}(k) \nmid |G|$. The complement$\ker \bar p$ is a$G$-stable complement to$W$, giving$V = W \oplus \ker \bar p$.

**43.1.3 Why the Theorem Fails in Characteristic $p \mid |G|$**
If $\mathrm{char}(k) = p$ and$p \mid |G|$, the averaging trick fails (division by$|G|$ is division by 0). And indeed, Maschke's theorem fails: the augmentation ideal$I = \ker(\varepsilon: k[G] \to k)$ (where$\varepsilon$ sums coefficients) is a submodule of the regular representation$k[G]$ with no$G$-stable complement. The study of representations in this "modular" case (Chapter 51) is significantly harder and requires new tools (Brauer characters, blocks, defect groups).

**43.1.4 Reformulation: $k[G]$ Is a Semisimple Ring**
Maschke's theorem is equivalent to: $k[G]$ is a semisimple ring (a ring with every module projective, or equivalently: the Jacobson radical is zero). Over an algebraically closed field$k$ of characteristic 0,$k[G]$ is thus a product of matrix algebras — a consequence of the Artin–Wedderburn theorem. The semisimplicity of$k[G]$ is the algebraic form of Maschke's theorem and connects representation theory to ring theory.

---

**43.2 The Artin–Wedderburn Theorem**

**43.2.1 Semisimple Rings: Every Module Is Projective**
A ring $R$ is *semisimple* if every left$R$-module is semisimple (a direct sum of simple modules), equivalently, every module is projective. Semisimple rings are completely determined by their simple modules, and the structure theorem (Artin–Wedderburn) classifies them completely. Every simple ring is a matrix ring over a division algebra (by Wedderburn), and every semisimple ring is a product of simple rings.

**43.2.2 Simple Rings and Matrix Rings over Division Algebras**
A ring $R$ is *simple* if its only two-sided ideals are$0$ and$R$. By *Wedderburn's theorem*: every simple Artinian ring (satisfying the descending chain condition on left ideals) is isomorphic to$M_n(D)$ for some$n$ and some division ring$D$. Over an algebraically closed field$k$, the only finite-dimensional division algebra is$k$ itself (by the Frobenius theorem or by noting$D$ as a$k$-algebra has only$k$ as a subfield). So over an algebraically closed field, simple Artinian rings are matrix algebras$M_n(k)$.

**43.2.3 Artin–Wedderburn: $R \cong \prod M_{n_i}(D_i)$**
*Artin–Wedderburn Theorem:* Every semisimple Artinian ring is isomorphic to a product $R \cong M_{n_1}(D_1) \times M_{n_2}(D_2) \times \cdots \times M_{n_r}(D_r)$ where each$D_i$ is a division ring. The decomposition is unique up to reordering. The simple modules of$R$ are the column spaces of each matrix factor$M_{n_i}(D_i)$, one for each factor.

**43.2.4 Application to $k[G]$ over$\mathbb{C}$:$\mathbb{C}[G] \cong \prod M_{n_i}(\mathbb{C})$**
When $k = \mathbb{C}$ and$G$ is a finite group,$\mathbb{C}[G]$ is semisimple (Maschke) and every finite-dimensional division algebra over$\mathbb{C}$ is$\mathbb{C}$ itself. So$\mathbb{C}[G] \cong \prod_{i=1}^r M_{n_i}(\mathbb{C})$, where$r$ is the number of irreducible representations (up to isomorphism) and$n_i = \dim V_i$ is the dimension of the$i$th irreducible. The simple modules are the matrix algebra column spaces, corresponding to the irreducible representations$V_1, \ldots, V_r$.

---

**43.3 The Decomposition of the Regular Representation**

**43.3.1 Each Irreducible Appears in $k[G]$ with Multiplicity$= \dim V_i$**
In the decomposition of the regular representation $k[G]$ as a left$k[G]$-module, the irreducible representation$V_i$ appears with multiplicity$n_i = \dim V_i$. This follows from the Artin–Wedderburn decomposition:$k[G] \cong \prod M_{n_i}(k)$, and as a left module over itself,$M_n(k) \cong k^n \oplus \cdots \oplus k^n$ ($n$ copies of the irreducible column module). So the regular representation contains each irreducible$n_i$ times.

**43.3.2 The Dimension Formula $\sum n_i^2 = |G|$**
Since $\dim k[G] = |G|$ and the regular representation decomposes with each$V_i$ appearing$n_i$ times, we get$\sum_i n_i \cdot n_i = |G|$, i.e.,$\sum_i n_i^2 = |G|$. This is a fundamental constraint: the sum of squares of the dimensions of the irreducible representations equals the order of the group. For$S_3$:$1^2 + 1^2 + 2^2 = 1 + 1 + 4 = 6 = |S_3|$ — three irreducibles, of dimensions 1, 1, 2.

**43.3.3 Number of Irreducibles = Number of Conjugacy Classes**
The number of irreducible representations of $G$ over$\mathbb{C}$ equals the number of conjugacy classes of$G$. Proof: the center of$\mathbb{C}[G]$ has dimension equal to the number of conjugacy classes (a basis consists of conjugacy class sums); under the Artin–Wedderburn isomorphism, the center maps to$\prod Z(M_{n_i}(\mathbb{C})) \cong \mathbb{C}^r$, which has dimension$r$ (the number of irreducibles). So the dimension of the center =$r$ = number of irreducibles = number of conjugacy classes.

---

## Chapter 44 — Character Theory

**What it establishes:** The character as the complete invariant of a representation; the orthogonality relations as the inner product structure on class functions; and the character table as the encoding of all representation-theoretic information of a group, with applications to group structure.

---

**44.1 Characters**

**44.1.1 The Character $\chi_V(g) = \mathrm{tr}(\rho(g))$**
The *character* of a representation $(V, \rho)$ is the function$\chi_V: G \to k$ defined by$\chi_V(g) = \mathrm{tr}(\rho(g))$ — the trace of the matrix representing$g$. The trace is independent of the choice of basis (conjugate matrices have the same trace), so the character is well-defined for an abstract representation. Characters are the "shadows" of representations: they are easier to compute with than representations themselves, and (over$\mathbb{C}$) they carry all the essential information.

**44.1.2 Characters Are Class Functions; Basic Properties**
A *class function* on $G$ is a function$G \to k$ that is constant on conjugacy classes:$f(hgh^{-1}) = f(g)$ for all$g, h \in G$. Characters are class functions:$\chi_V(hgh^{-1}) = \mathrm{tr}(\rho(h)\rho(g)\rho(h)^{-1}) = \mathrm{tr}(\rho(g)) = \chi_V(g)$ (cyclicity of trace). Additional properties:$\chi_V(e) = \dim V$ (trace of the identity = dimension);$\chi_V(g^{-1}) = \overline{\chi_V(g)}$ (complex conjugate, since$\rho(g)$ has eigenvalues that are roots of unity, hence of modulus 1).

**44.1.3 Characters of Direct Sums, Tensor Products, and Duals**
$\chi_{V \oplus W}(g) = \chi_V(g) + \chi_W(g)$ (trace is additive).$\chi_{V \otimes W}(g) = \chi_V(g) \cdot \chi_W(g)$ (trace of a tensor product is the product of traces).$\chi_{V^*}(g) = \overline{\chi_V(g)} = \chi_V(g^{-1})$ (trace of the dual/contragredient). These formulas make the character map$V \mapsto \chi_V$ a ring homomorphism from the representation ring$R(G)$ to the ring of class functions, where the ring structure on class functions is pointwise addition and multiplication.

---

**44.2 Orthogonality Relations**

**44.2.1 The Inner Product on Class Functions**
The *inner product* of class functions $f, g: G \to \mathbb{C}$ is$\langle f, g \rangle = \frac{1}{|G|} \sum_{h \in G} f(h) \overline{g(h)}$. This makes the space of complex-valued class functions on$G$ into an inner product space. The number of conjugacy classes is the dimension of this space. The inner product is the key structure: it allows one to decompose any class function (in particular, any character) into irreducible pieces.

**44.2.2 First Orthogonality: Irreducible Characters Are Orthonormal**
*First orthogonality relation:* $\langle \chi_i, \chi_j \rangle = \delta_{ij}$: the irreducible characters form an orthonormal set with respect to the inner product. This is proved by computing$\langle \chi_i, \chi_j \rangle$ using the explicit formula for the averaging integral over$G$ and the Artin–Wedderburn decomposition. The orthonormality implies: (a) the irreducible characters are linearly independent; (b) together, they form a basis for the space of class functions (since the number of irreducibles equals the number of conjugacy classes, which is the dimension).

**44.2.3 Second Orthogonality: Column Orthogonality of the Character Table**
*Second orthogonality relation:* $\sum_i \chi_i(g) \overline{\chi_i(h)} = |C_G(g)| \cdot \delta_{[g],[h]}$, where the sum is over all irreducible characters,$[g]$ denotes the conjugacy class of$g$, and$|C_G(g)|$ is the size of the centralizer of$g$. While first orthogonality is "row orthogonality" of the character table, second orthogonality is "column orthogonality." Together, the two orthogonality relations fully determine the character table from either its rows or its columns.

**44.2.4 Decomposing a Representation: $m_i = \langle \chi_V, \chi_{V_i} \rangle$**
For any representation $V = \bigoplus_i m_i V_i$ (Maschke), the multiplicity of the$i$th irreducible is$m_i = \langle \chi_V, \chi_{V_i} \rangle$. This formula is the main computational tool in representation theory: given a representation$V$ (e.g., the permutation representation), compute its character, take inner products with all irreducible characters, and read off the decomposition. The dimension formula gives$\dim V = \sum m_i n_i$.

**44.2.5 Characters Classify Representations: $\chi_V = \chi_W \Leftrightarrow V \cong W$**
Two representations $V$ and$W$ are isomorphic if and only if they have the same character:$\chi_V = \chi_W$. The forward direction is trivial (isomorphic representations have the same trace). The backward direction:$\chi_V = \chi_W$ implies$\langle \chi_V, \chi_{V_i} \rangle = \langle \chi_W, \chi_{V_i} \rangle$ for all$i$, so they have the same multiplicities, so they are isomorphic. This theorem says that the character is a *complete invariant* for representations over$\mathbb{C}$: the character encodes all information about the representation.

---

**44.3 The Character Table**

**44.3.1 Layout: Rows = Irreducibles, Columns = Conjugacy Classes**
The *character table* of $G$ is the$r \times r$ matrix (where$r$ is the number of conjugacy classes = number of irreducibles) with rows indexed by irreducible characters$\chi_1, \ldots, \chi_r$ and columns indexed by conjugacy classes$C_1, \ldots, C_r$ (with representative$g_j$), and entry$\chi_i(g_j)$. The first row is the trivial character (all 1's); the first column is the dimensions$n_i = \chi_i(e)$. The character table is one of the most information-dense objects in finite group theory.

**44.3.2 Constraints: Dimension Formula, Divisibility, Row/Column Norms**
Constraints on the character table: (i) $\sum_i n_i^2 = |G|$ (dimension formula); (ii) each$n_i$ divides$|G|$ (a deep theorem using the ring of algebraic integers); (iii) each$\chi_i$ takes algebraic integer values; (iv)$||\chi_i||^2 = \langle \chi_i, \chi_i \rangle = 1$ (rows are unit vectors); (v) column norms:$\sum_i |\chi_i(g)|^2 = |C_G(g)|$. These constraints often suffice to determine the character table of small groups without computing the representations explicitly.

**44.3.3 Examples: $S_3$,$S_4$,$A_4$,$D_4$,$Q_8$**
*$S_3$:* Three conjugacy classes ($e$, transpositions, 3-cycles); three irreducibles of dimensions 1, 1, 2 (trivial, sign, standard). *$A_4$:* Four conjugacy classes; four irreducibles of dimensions 1, 1, 1, 3 (three 1-dimensional over$\mathbb{Q}(\omega)$, one 3-dimensional). *$D_4$:* Five conjugacy classes; five irreducibles of dimensions 1, 1, 1, 1, 2. *$Q_8$:* Five conjugacy classes; same dimension list as$D_4$ but different characters — demonstrating that the character table distinguishes groups that happen to have the same dimension list.

**44.3.4 How to Compute a Character Table**
Standard methods for constructing character tables: (i) start with known representations (trivial, sign, permutation, tensor products); (ii) use the dimension formula to constrain remaining irreducible dimensions; (iii) apply inner product constraints (rows must be orthonormal); (iv) use the fact that characters take algebraic integer values in the cyclotomic field $\mathbb{Q}(\zeta_{|G|})$; (v) use induced characters (Chapter 45) and tensor products to construct candidates. The computation is algorithmically feasible and computer algebra systems (GAP, Magma) can do it automatically.

---

**44.4 Applications of Characters**

**44.4.1 Burnside's Theorem: Groups of Order $p^a q^b$ Are Solvable**
*Burnside's theorem:* If $|G| = p^a q^b$ for primes$p, q$, then$G$ is solvable. The proof uses character theory: it shows that$G$ has a non-trivial proper normal subgroup by finding a conjugacy class of prime power order and applying the theorem that a character is zero on that class (from the divisibility of character values and the norm-1 condition). Burnside's theorem was proved in 1904 using characters; the first purely group-theoretic proof (Goldschmidt–Matsuyama) came 70 years later, showing that character theory can prove theorems that pure group theory finds very hard.

**44.4.2 The Divisibility of Degrees by $|G|/|Z(G)|$**
The dimension $n_i$ of any irreducible representation divides$|G|/|Z(G)|$ (where$|Z(G)|$ is the center). More precisely,$n_i$ divides$[G : Z(G)]$. This divisibility is a consequence of the fact that$\chi_i(g) / n_i$ is an algebraic integer for any class sum element, combined with the norm-1 condition. The case where an irreducible has dimension equal to$[G:Z(G)]^{1/2}$ leads to the theory of M-groups.

**44.4.3 Detecting Normal Subgroups via Characters**
A subgroup $N \trianglelefteq G$ is normal iff$N = \bigcup_i \ker \chi_i^{(S)}$ for some subset$S$ of irreducible characters. More precisely:$N = G$ iff$\chi_i(g) = n_i$ for all$g$ (only the trivial representation sees$g = e$);$N = \{e\}$ iff... The key result:$N \trianglelefteq G$ iff$N$ is a union of conjugacy classes; the kernel of an irreducible character$\ker \chi_i = \{g : \chi_i(g) = \chi_i(e)\}$ is always a normal subgroup. The intersection of kernels of certain irreducibles gives all normal subgroups.

**44.4.4 The Character Ring (Representation Ring) $R(G)$**
The *representation ring* (or *Burnside ring*) $R(G)$ is the free abelian group generated by isomorphism classes of irreducible representations, with multiplication given by tensor products:$[V] \cdot [W] = [V \otimes W]$. After decomposing$V \otimes W$ into irreducibles (using the inner product with characters), the structure constants$c_{ij}^k$ (with$[V_i] \cdot [V_j] = \sum_k c_{ij}^k [V_k]$) are non-negative integers. The character map$R(G) \to \mathrm{Cl}(G, \mathbb{C})$ (class functions) is an injective ring homomorphism, making the character table the "multiplication table" of$R(G)$ in a precise sense.

---

## Chapter 45 — Induced Representations and Frobenius Theory

**What it establishes:** Induced representations as the mechanism for building representations of $G$ from representations of a subgroup$H$; Frobenius reciprocity as the adjunction between induction and restriction; Mackey theory for analyzing the composition of induction and restriction; and Frobenius groups as a classical structural result.

---

**45.1 Restriction and Induction**

**45.1.1 Restriction $\mathrm{Res}^G_H V$: Viewing a$G$-Representation as an$H$-Representation**
If $V$ is a representation of$G$ and$H \leq G$ is a subgroup, the *restricted representation*$\mathrm{Res}^G_H V$ (or$V|_H$) is$V$ viewed as a representation of$H$ by restriction:$\rho_H(h) = \rho(h)$ for$h \in H$. Restriction does not change$V$ as a vector space but forgets the action of elements of$G$ not in$H$. In the language of modules:$\mathrm{Res}^G_H V$ is the$k[H]$-module obtained from the$k[G]$-module$V$ by restricting along the inclusion$k[H] \hookrightarrow k[G]$.

**45.1.2 Induced Representation $\mathrm{Ind}^G_H W = k[G] \otimes_{k[H]} W$**
If $W$ is a representation of$H \leq G$, the *induced representation*$\mathrm{Ind}^G_H W = k[G] \otimes_{k[H]} W$ is a representation of$G$ built by "extending"$W$ from$H$ to all of$G$. Concretely, choose a set of coset representatives$\{g_1, \ldots, g_m\}$ of$H$ in$G$ ($m = [G:H]$); then$\mathrm{Ind}^G_H W = g_1 \otimes W \oplus g_2 \otimes W \oplus \cdots \oplus g_m \otimes W$ as a vector space, with the$G$-action permuting and transforming the summands in the way cosets are permuted and twisted by the$H$-action.

**45.1.3 Dimension of the Induced Representation: $[G:H] \cdot \dim W$**
$\dim_k (\mathrm{Ind}^G_H W) = [G:H] \cdot \dim_k W$. This follows directly from the coset description:$\mathrm{Ind}^G_H W$ has$[G:H]$ coset summands, each of dimension$\dim W$. Induction increases dimension by the index$[G:H]$, while restriction keeps the dimension the same (but may cause reducibility).

**45.1.4 The Character Formula for Induced Representations**
The character of the induced representation is:
$$\chi_{\mathrm{Ind}^G_H W}(g) = \frac{1}{|H|} \sum_{x \in G, \, x^{-1}gx \in H} \chi_W(x^{-1}gx)$$
This formula sums the character values of $W$ over all conjugates of$g$ that land in$H$, normalized by$|H|$. Equivalently, it can be written as$\chi_{\mathrm{Ind} W}(g) = \sum_{t \in H \backslash G / \langle g \rangle} \chi_W(\ldots)$ in terms of double cosets. The formula shows that the induced character is determined by the original character on$H$ and the group structure of$G$.

---

**45.2 Frobenius Reciprocity**

**45.2.1 Statement: $\langle \mathrm{Ind}^G_H W, V \rangle_G = \langle W, \mathrm{Res}^G_H V \rangle_H$**
*Frobenius Reciprocity:* For any representation $V$ of$G$ and$W$ of$H \leq G$:
$$\langle \chi_{\mathrm{Ind}^G_H W}, \chi_V \rangle_G = \langle \chi_W, \chi_{\mathrm{Res}^G_H V} \rangle_H$$
In words: the multiplicity of an irreducible of $G$ in the induced representation$\mathrm{Ind}^G_H W$ equals the multiplicity of the corresponding restricted representation in$W$. This is the key computational tool: to decompose$\mathrm{Ind}^G_H W$, compute inner products with irreducible characters of$H$.

**45.2.2 Categorical Interpretation: $\mathrm{Ind} \dashv \mathrm{Res}$**
Frobenius reciprocity says precisely that $\mathrm{Ind}^G_H$ is *left adjoint* to$\mathrm{Res}^G_H$:$\mathrm{Hom}_{k[G]}(\mathrm{Ind}^G_H W, V) \cong \mathrm{Hom}_{k[H]}(W, \mathrm{Res}^G_H V)$ naturally. This adjunction is the tensor-hom adjunction from module theory in disguise:$\mathrm{Ind}^G_H = k[G] \otimes_{k[H]} -$ is left adjoint to$\mathrm{Hom}_{k[G]}(k[G], -) = \mathrm{Res}^G_H$ (restriction is taking$k[G]$-module maps from$k[G]$, which is the same as forgetting to an$H$-module). Every adjoint pair in representation theory ultimately comes from a tensor-hom adjunction.

**45.2.3 Applications: Computing Characters of Induced Representations**
Frobenius reciprocity reduces computations: to decompose $\mathrm{Ind}^G_H W = \bigoplus_i m_i V_i$, compute$m_i = \langle \chi_{\mathrm{Ind} W}, \chi_i \rangle_G = \langle \chi_W, \chi_i|_H \rangle_H$. This turns an induction computation into a restriction computation — which is often simpler. Example: to find all representations of$S_n$, start with the trivial representation of$H = S_{n-1}$, induce to$S_n$, decompose by Frobenius, and iterate. This approach (Young tableaux and Schur functors) classifies all irreducible representations of$S_n$.

---

**45.3 Mackey Theory**

**45.3.1 Double Cosets and the Mackey Formula**
For subgroups $H, K \leq G$ and a representation$W$ of$H$, the *Mackey formula* describes the restriction of the induced representation to$K$:
$$\mathrm{Res}^G_K \mathrm{Ind}^G_H W \cong \bigoplus_{HsK \in H \backslash G / K} \mathrm{Ind}^K_{K \cap s^{-1}Hs} (s^*W)$$
where the sum is over the double cosets $HsK$ and$s^*W$ denotes$W$ twisted by conjugation by$s$ (viewed as an$s^{-1}Hs$-module). The double coset decomposition$H \backslash G / K$ organizes the "ways$H$ and$K$ interact in$G$."

**45.3.2 Mackey's Irreducibility Criterion**
By Frobenius reciprocity, $\mathrm{Ind}^G_H W$ is irreducible iff$\langle \mathrm{Res}^G_H \mathrm{Ind}^G_H W, W \rangle_H = 1$ (the induced representation appears once in itself upon restriction-then-induction). The Mackey formula computes this inner product in terms of double cosets, giving *Mackey's irreducibility criterion*:$\mathrm{Ind}^G_H W$ is irreducible iff$W$ is irreducible and for every$s \notin H$,$s^*W|_{H \cap s^{-1}Hs}$ and$W|_{H \cap s^{-1}Hs}$ have no common irreducible components. This criterion is the primary tool for proving that specific induced representations are irreducible.

**45.3.3 Applications to Subgroup Structure**
Mackey theory has applications beyond computing characters: it constrains the subgroup structure of $G$ and is used in the classification of primitive permutation representations (transitive actions with no proper block system). When combined with the Sylow theorems, Mackey theory gives control over the intersection of Sylow subgroups and the structure of their normalizers.

---

**45.4 Frobenius Groups**

**45.4.1 Definition and Structure of Frobenius Groups**
A group $G$ is a *Frobenius group* if it has a Frobenius action: a faithful transitive action on a set$X$ such that every non-identity element of$G$ fixes at most one point of$X$. In terms of the point stabilizer$H = G_x$ (for some$x \in X$):$G$ is a Frobenius group with *Frobenius complement*$H$ if$H \neq 1$,$H \neq G$, and$H \cap gHg^{-1} = \{e\}$ for all$g \notin H$. The condition says the conjugates of$H$ in$G$ intersect trivially (except with themselves).

**45.4.2 The Frobenius Kernel; Frobenius's Theorem on Its Existence**
*Frobenius's theorem:* If $G$ is a Frobenius group with complement$H$, then the *Frobenius kernel*$K = G \setminus \bigcup_{g \in G} (gHg^{-1} \setminus \{e\}) = \{e\} \cup \{g \in G : g \text{ fixes no point of } X \setminus \{x\}\}$ is a normal subgroup of$G$, and$G = KH$ with$K \cap H = \{e\}$ (so$G = K \rtimes H$). The striking fact is that$K$ is a subgroup at all — this is not obvious and was proved by Frobenius using character theory. The proof is one of the most beautiful applications of characters: one shows$K$ is normal by proving a character-theoretic criterion and checking it for all irreducibles of$G$.

**45.4.3 Examples: Dihedral Groups, $AGL(1, p)$**
*Dihedral group $D_n$ ($n$ odd):* The rotation subgroup$C_n$ is a Frobenius kernel and the reflection subgroup$C_2$ is a Frobenius complement. *Affine group$AGL(1, p)$* (for prime$p$): consists of maps$x \mapsto ax + b$ on$\mathbb{F}_p$ with$a \neq 0$. The translations$\{x \mapsto x + b\}$ form the Frobenius kernel (isomorphic to$\mathbb{F}_p$); the stabilizer of$0$ is$\{x \mapsto ax\}$ (isomorphic to$\mathbb{F}_p^*$). Frobenius groups appear naturally as groups acting on affine spaces and are the simplest transitive groups that are "almost regular."

---

*Next: [Part X — Lie Theory](part-X-lie-theory.md)*

*Prerequisites satisfied: Part I (logic, sets), Part II (linear algebra — matrix groups, trace, inner products), Part III (group theory — subgroups, normal subgroups, group actions), Part IV (ring theory — group rings), Part V (modules — $k[G]$-modules), Part VII (category theory — adjoint functors, abelian categories).*
