# Part V — Module Theory

**Chapters 25–28**

---

## What This Part Establishes

Part V introduces modules — the common generalization of vector spaces, abelian groups, and ideals. A module over a ring $R$ is like a vector space, except that the "scalars" form a ring rather than a field: they need not be invertible, and the structure can be considerably more complex. This generalization, while seemingly small, enables a dramatic unification: the theory of$R$-modules simultaneously captures abelian group theory (modules over$\mathbb{Z}$), linear algebra (modules over a field), the Jordan canonical form (modules over$F[x]$), and much of algebraic topology and geometry.

Chapter 25 develops the basic theory: the module axioms, submodules, quotients, homomorphisms, direct sums, and the first class of examples. Chapter 26 establishes the three varieties of "good" modules — free, projective, and injective — which are the building blocks of homological algebra. Chapter 27 proves the structure theorem for finitely generated modules over a PID, the master theorem that simultaneously classifies finitely generated abelian groups and explains the rational and Jordan canonical forms of matrices. Chapter 28 extends the tensor product to the module setting, establishes its right-exactness (and the failure of left-exactness), and derives the Hom–Tensor adjunction that underlies all of homological algebra.

By the end of Part V, the reader has the algebraic infrastructure for homological algebra (Part VIII), understands why canonical forms work, and has the language of modules needed for representation theory (Parts IX, XI) and category theory (Part VII).

---

## Internal Dependency Map

```
Ch 25 (Modules: Axioms, Submodules, Hom, Direct Sums)
                     |
          ___________|____________
          |                      |
          v                      v
    Ch 26                    Ch 27
(Free/Projective/Injective)  (Structure Thm/PIDs)
          |                      
          v                      
    Ch 28
(Tensor Products of Modules)
```

---

## Chapter 25 — Modules over Rings

**What it establishes:** The module axioms and their equivalences; the key examples that demonstrate modules generalize both vector spaces and abelian groups; the basic structure theory (submodules, quotients, homomorphisms, direct sums); and the simplest class of modules (cyclic and simple).

---

**25.1 The Module Axioms**

**25.1.1 Left $R$-Modules; Right Modules; Bimodules**
A *left $R$-module* is an abelian group$(M, +)$ together with a ring action$R \times M \to M$, written$(r, m) \mapsto rm$, satisfying:$r(m + m') = rm + rm'$;$(r + s)m = rm + sm$;$(rs)m = r(sm)$;$1_R \cdot m = m$ for all$r, s \in R$ and$m, m' \in M$. Right$R$-modules have the action on the right:$(r, m) \mapsto mr$ with$(rs)m = s(rm)$ replaced by$m(rs) = (mr)s$. When$R$ is commutative, left and right modules coincide. A *bimodule* carries both a left$R$-action and a right$S$-action that commute:$r(ms) = (rm)s$.

**25.1.2 The Correspondence: $R$-Modules$\leftrightarrow$ Ring Maps$R \to \mathrm{End}(M)$**
A left $R$-module structure on an abelian group$M$ is equivalent to a ring homomorphism$\rho: R \to \mathrm{End}(M)$ (where$\mathrm{End}(M)$ is the endomorphism ring of$(M, +)$): define$\rho(r)(m) = rm$. This correspondence shows that an$R$-module is precisely a representation of$R$ in the endomorphisms of an abelian group — the ring-theoretic analogue of a group representation. When$R = k[G]$, an$R$-module is exactly a representation of$G$ over$k$.

**25.1.3 Key Examples: Abelian Groups ($R = \mathbb{Z}$), Vector Spaces ($R$ a Field),$F[x]$-Modules**
Every abelian group $(M, +)$ is a$\mathbb{Z}$-module via$n \cdot m = m + m + \cdots + m$ ($n$ times, with$(-n) \cdot m = -(n \cdot m)$). Every vector space over a field$F$ is an$F$-module. The most important new example: if$V$ is an$F$-vector space and$T: V \to V$ is a linear operator, then$V$ becomes an$F[x]$-module via$f \cdot v = f(T)v$ (applying the polynomial in$T$). Different operators give different$F[x]$-module structures on$V$. The classification of$F[x]$-modules is equivalent to the classification of linear operators up to similarity — i.e., the theory of canonical forms (Jordan, rational).

---

**25.2 Submodules and Quotients**

**25.2.1 Submodules; the Submodule Test**
A subset $N \subseteq M$ is a *submodule* if it is an abelian subgroup of$(M, +)$ and is closed under the$R$-action:$rn \in N$ for all$r \in R$,$n \in N$. The submodule test:$N \neq \emptyset$; for all$m, n \in N$ and$r \in R$,$m - n \in N$ and$rm \in N$. Submodules generalize subspaces (when$R$ is a field) and normal subgroups (when$R = \mathbb{Z}$, since all subgroups of abelian groups are normal).

**25.2.2 Quotient Modules $M/N$; Well-Definedness**
If $N \leq M$ is a submodule, the *quotient module*$M/N$ consists of cosets$m + N$, with addition$(m + N) + (m' + N) = (m + m') + N$ and$R$-action$r(m + N) = rm + N$. Well-definedness of the$R$-action requires$N$ to be a submodule:$r(m + N) = rm + N$ because$r(m + n) - rm = rn \in N$ for$n \in N$. The quotient module construction is the same construction as for vector spaces; it works because submodules play the role of subspaces.

**25.2.3 The Isomorphism Theorems for Modules**
The three isomorphism theorems hold for modules, with the same statements and analogous proofs as for groups and rings: First: $M/\ker f \cong \mathrm{im}\, f$ for any$R$-module homomorphism$f: M \to N$. Second:$(M + N)/N \cong M/(M \cap N)$ for submodules$M, N$ of a larger module. Third:$(M/N)/(L/N) \cong M/L$ for submodules$N \subseteq L \subseteq M$. The Correspondence Theorem: submodules of$M/N$ correspond to submodules of$M$ containing$N$.

**25.2.4 Correspondence Theorem for Modules**
Submodules of $M/N$ are in bijection with submodules of$M$ containing$N$, under the map$L \leftrightarrow L/N$ (for submodules$L \supseteq N$) and the inverse map$K \leftrightarrow \pi^{-1}(K)$. This is essential for inductive arguments: understanding$M/N$ requires understanding the submodule structure of$M$ above$N$.

---

**25.3 Module Homomorphisms**

**25.3.1 $R$-Linear Maps;$\mathrm{Hom}_R(M,N)$ as an Abelian Group**
An *$R$-module homomorphism* (or *$R$-linear map*)$f: M \to N$ is a group homomorphism satisfying$f(rm) = rf(m)$ for all$r \in R$,$m \in M$. The set$\mathrm{Hom}_R(M, N)$ of all$R$-linear maps from$M$ to$N$ is an abelian group under pointwise addition:$(f + g)(m) = f(m) + g(m)$. When$R$ is commutative,$\mathrm{Hom}_R(M, N)$ is itself an$R$-module. When$R$ is a field,$\mathrm{Hom}_R(M, N) = \mathcal{L}(M, N)$, the space of linear maps.

**25.3.2 Kernel, Image, and the First Isomorphism Theorem**
The *kernel* $\ker f = f^{-1}(0)$ and *image*$\mathrm{im}\, f = f(M)$ are submodules of$M$ and$N$ respectively. The first isomorphism theorem:$M/\ker f \cong \mathrm{im}\, f$. The map$f$ is injective iff$\ker f = 0$, surjective iff$\mathrm{im}\, f = N$, an isomorphism iff bijective.

**25.3.3 Endomorphism Rings $\mathrm{End}_R(M)$**
The set $\mathrm{End}_R(M) = \mathrm{Hom}_R(M, M)$ of$R$-linear self-maps of$M$ is a ring under addition and composition. When$M = R^n$ (free module of rank$n$),$\mathrm{End}_R(R^n) \cong M_n(R)$ (the matrix ring). For a simple module$M$ over a field, Schur's lemma generalizes:$\mathrm{End}_R(M)$ is a division ring (but not necessarily a field if$R$ is not commutative). The endomorphism ring of a module encodes all the algebraic information about maps from the module to itself.

---

**25.4 Direct Sums and Products**

**25.4.1 External Direct Sum $\bigoplus M_i$; Universal Property**
The *external direct sum* $\bigoplus_{i \in I} M_i$ consists of all tuples$(m_i)_{i \in I}$ with$m_i \in M_i$ and$m_i = 0$ for all but finitely many$i$. The$R$-action is componentwise. The direct sum satisfies the *universal property of the coproduct*: for any family of$R$-linear maps$f_i: M_i \to N$, there is a unique map$\bigoplus M_i \to N$ restricting to$f_i$ on each$M_i$.

**25.4.2 Direct Product $\prod M_i$; Difference from Direct Sum**
The *direct product* $\prod_{i \in I} M_i$ consists of all tuples$(m_i)_{i \in I}$ with no finiteness restriction. For finite index sets,$\bigoplus_{i=1}^n M_i = \prod_{i=1}^n M_i$. For infinite index sets, the direct sum is a proper submodule of the direct product. The direct product satisfies the universal property of the product (maps into$\prod M_i$ correspond to tuples of maps). The choice between direct sum and direct product matters for infinite families and is relevant to the behavior of Hom and Tensor.

**25.4.3 Internal Direct Sums; Complementary Submodules**
$M$ is the *internal direct sum* of submodules$\{N_i\}$ if every$m \in M$ has a unique expression$m = \sum n_i$ (finite sum,$n_i \in N_i$). Equivalently:$M = \sum N_i$ (the$N_i$ generate$M$) and$N_i \cap \sum_{j \neq i} N_j = 0$ for all$i$. Internal and external direct sums are isomorphic. Two submodules$N, N'$ with$N \oplus N' = M$ are *complementary*;$N'$ is a *complement* of$N$. Complements do not always exist (unlike for vector spaces), and their existence characterizes projective modules.

---

**25.5 Generating Sets and Cyclic Modules**

**25.5.1 Generating Sets; Finitely Generated Modules**
A subset $S \subseteq M$ *generates*$M$ (as an$R$-module) if every$m \in M$ is a finite$R$-linear combination of elements of$S$:$m = r_1 s_1 + \cdots + r_n s_n$ with$r_i \in R$,$s_i \in S$.$M$ is *finitely generated* if it has a finite generating set. Finitely generated modules are the "manageable" modules; over Noetherian rings, submodules of finitely generated modules are finitely generated. The structure theorem (Chapter 27) classifies finitely generated modules over PIDs completely.

**25.5.2 Cyclic Modules $R/I$; Annihilators**
A module $M$ is *cyclic* if$M = Rm$ for a single element$m$ — the smallest submodule containing$m$ is all of$M$. The *annihilator* of$m$ is$\mathrm{Ann}_R(m) = \{r \in R : rm = 0\}$, a left ideal of$R$. The cyclic module$Rm \cong R/\mathrm{Ann}_R(m)$ via$r \mapsto rm$. Every cyclic$R$-module is isomorphic to a quotient$R/I$ for some left ideal$I$. Cyclic modules are the "atoms" of finitely generated module theory: the structure theorem decomposes every finitely generated module over a PID into a direct sum of cyclic modules.

**25.5.3 Simple Modules (Irreducible Modules)**
A nonzero module $M$ is *simple* (or *irreducible*) if it has no submodules other than$0$ and$M$. Simple modules are cyclic:$M = Rm$ for any nonzero$m$, and$\mathrm{Ann}(m)$ must be a maximal left ideal. *Schur's lemma:* any nonzero homomorphism between simple modules is an isomorphism. Simple modules are the correct notion of "irreducible" in module theory, and the theory of semisimple modules — direct sums of simple modules — is the content of Chapters 42–43.

---

## Chapter 26 — Free, Projective, and Injective Modules

**What it establishes:** The three fundamental classes of modules — free (with a basis), projective (direct summands of free), and injective (duals of projective in a precise sense) — and the fourth class (flat) relevant to tensor products. These classes are the building blocks of homological algebra.

---

**26.1 Free Modules**

**26.1.1 Bases and Free Modules; the Universal Property**
A subset $B \subseteq M$ is a *basis* if every element of$M$ has a unique expression as a finite$R$-linear combination of elements of$B$. A module with a basis is *free*. The universal property: for any$R$-module$N$ and function$f: B \to N$, there exists a unique$R$-linear map$\tilde f: M \to N$ extending$f$. Free modules are the "most general" modules with no relations between their generators; they are the module-theoretic analogue of vector spaces.

**26.1.2 $\mathrm{Hom}_R(R^n, M) \cong M^n$; Free Modules Are the Most General**
For the free module $R^n$ with standard basis$\{e_1, \ldots, e_n\}$, a linear map$f: R^n \to M$ is determined by$f(e_1), \ldots, f(e_n)$: any choice$(m_1, \ldots, m_n) \in M^n$ extends uniquely to an$R$-linear map. Thus$\mathrm{Hom}_R(R^n, M) \cong M^n$ as abelian groups (and as$R$-modules when$R$ is commutative). This shows free modules represent the functor$M \mapsto M^n$, the archetypal representable functor.

**26.1.3 Rank; When Is the Rank Well-Defined?**
The *rank* of a free module is the cardinality of a basis. For free modules over commutative rings (and many noncommutative rings), the rank is well-defined: any two bases have the same cardinality. (This fails for some noncommutative rings: there exist rings with $R \cong R^2$ as modules.) For commutative rings, rank is well-defined because tensoring with the residue field$R/\mathfrak{m}$ converts$R^n \to (R/\mathfrak{m})^n$, a vector space of dimension$n$.

**26.1.4 Every Module Is a Quotient of a Free Module**
For any $R$-module$M$ with generating set$S$ (possibly infinite), the free module$R^{(S)}$ (with basis indexed by$S$) surjects onto$M$ via$e_s \mapsto s$. The kernel of this surjection is the *module of relations* or *first syzygy*. Every module is a quotient of a free module — this is the starting point for free resolutions (Chapter 39) and the definition of projective modules.

---

**26.2 Projective Modules**

**26.2.1 The Lifting Property; Projective Modules as Summands of Free Modules**
An $R$-module$P$ is *projective* if for any surjective$R$-linear map$g: M \to N$ and any$R$-linear map$f: P \to N$, there exists a *lift*$\tilde f: P \to M$ with$g \circ \tilde f = f$. Equivalently,$P$ is projective iff it is a direct summand of a free module:$F = P \oplus Q$ for some free module$F$ and some module$Q$. The lifting property is the key categorical property, appearing in the definition of projective objects in any abelian category.

**26.2.2 Free $\Rightarrow$ Projective; Local Projectivity**
Every free module is projective (lifts can be defined on the basis). Projective modules are precisely those for which every short exact sequence $0 \to M' \to M \to P \to 0$ splits. Over a local ring (a ring with a unique maximal ideal), projective modules are free (Nakayama's lemma). Over a field, all modules are projective (vector spaces are free).

**26.2.3 Projective Modules over PIDs Are Free**
Over a PID, every projective module is free. This is a non-trivial theorem: projective modules can be non-free over general rings (e.g., the ideal $(2, 1 + \sqrt{-5})$ in$\mathbb{Z}[\sqrt{-5}]$ is projective but not free). The fact that projective = free over PIDs is essential for the structure theorem (Chapter 27).

**26.2.4 Projective Modules in Algebraic K-Theory**
The *Grothendieck group* $K_0(R)$ is the abelian group generated by isomorphism classes$[P]$ of finitely generated projective modules, subject to$[P] = [P'] + [P'']$ whenever$P \cong P' \oplus P''$. For$R = F$ a field,$K_0(F) \cong \mathbb{Z}$ (generated by$[F]$). The group$K_0(R)$ measures "how far projective modules are from being free" and is the starting point of algebraic K-theory, a deep connection between ring theory, topology, and number theory.

---

**26.3 Injective Modules**

**26.3.1 The Extension Property; Injective Modules**
An $R$-module$Q$ is *injective* if for any injective$R$-linear map$i: M \hookrightarrow N$ and any$R$-linear map$f: M \to Q$, there exists an *extension*$\tilde f: N \to Q$ with$\tilde f \circ i = f$. Equivalently,$Q$ is injective iff every short exact sequence$0 \to Q \to M \to M'' \to 0$ splits. Injective modules are the "dual" of projective modules: projective modules have the lifting property (for surjections), injective modules have the extension property (for injections).

**26.3.2 Baer's Criterion for Injectivity**
*Baer's criterion:* An $R$-module$Q$ is injective iff for every ideal$I \subseteq R$ and every$R$-linear map$f: I \to Q$, there exists$q \in Q$ with$f(r) = rq$ for all$r \in I$ (i.e.,$f$ extends to all of$R$). Baer's criterion reduces injectivity — a condition about all inclusions$M \hookrightarrow N$ — to a condition about inclusions of ideals. It is the main tool for proving that specific modules are injective.

**26.3.3 Injective Hulls: The Minimal Injective Extension**
Every module $M$ embeds into an injective module (an *injective envelope* or *injective hull*)$E(M)$, the smallest injective module containing$M$ in an essential way (every nonzero submodule of$E(M)$ meets$M$ nontrivially). Injective hulls are unique up to isomorphism. They are the injective analogue of the completion or algebraic closure: the "smallest" injective module that contains$M$, and an indispensable tool in homological algebra.

**26.3.4 Injective Modules over $\mathbb{Z}$: Divisible Abelian Groups**
Over $\mathbb{Z}$, an abelian group$M$ is injective iff it is *divisible*: for every$m \in M$ and$n \in \mathbb{Z} \setminus \{0\}$, there exists$m' \in M$ with$nm' = m$. Examples:$\mathbb{Q}$,$\mathbb{Q}/\mathbb{Z}$, and$\mathbb{Z}(p^\infty) = \mathbb{Z}[1/p]/\mathbb{Z}$ (the Prüfer$p$-group) are all injective$\mathbb{Z}$-modules. The injective hull of$\mathbb{Z}/n\mathbb{Z}$ is$\mathbb{Z}(p^\infty)$ for$n = p^k$ a prime power. Divisibility is the right notion of "injectivity" for abelian groups and motivates the abstract definition.

---

**26.4 Flat Modules**

**26.4.1 Flatness: Exactness of $- \otimes_R M$**
A right $R$-module$M$ is *flat* if the functor$- \otimes_R M$ is exact: it preserves all short exact sequences$0 \to A \to B \to C \to 0$. (Tensor products are always right-exact; the question is whether left-exactness is also preserved.) Flatness is a weaker condition than projectivity. The geometric meaning: flatness of a module (or sheaf) over a ring (or scheme) captures the idea of a "continuously varying" family without "jumping" fibers.

**26.4.2 Free $\Rightarrow$ Projective$\Rightarrow$ Flat; Counterexamples to Reversal**
Every free module is projective; every projective module is flat. Neither implication reverses in general: $\mathbb{Q}$ is a flat$\mathbb{Z}$-module (since$\mathbb{Z}$ is a domain, localization is flat) but not projective (not a summand of a free$\mathbb{Z}$-module). A finitely generated flat module over a Noetherian local ring is free (by Nakayama's lemma), but this fails without the Noetherian hypothesis.

**26.4.3 Flat Modules and Torsion-Free Modules**
Over a PID, flat and torsion-free are equivalent for finitely generated modules (and in fact for all modules: a $\mathbb{Z}$-module is flat iff it is torsion-free). Over a general ring, torsion-free does not imply flat. These equivalences make flatness computable in practice: for modules over$\mathbb{Z}$ or$F[x]$, flatness reduces to the absence of torsion.

**26.4.4 Flat Base Change in Commutative Algebra**
If $R \to S$ is a ring map and$M$ is a flat$R$-module, then for any exact sequence of$R$-modules$0 \to A \to B \to C \to 0$, the sequence$0 \to A \otimes_R M \to B \otimes_R M \to C \otimes_R M \to 0$ is exact. This *flat base change* principle is one of the most used tools in commutative algebra and algebraic geometry: it allows one to extend exact sequences along flat morphisms. Localizations are flat (inverting elements is flat), making the local–global principles of Chapter 24 precise.

---

## Chapter 27 — The Structure Theorem for Modules over PIDs

**What it establishes:** The master classification theorem for finitely generated modules over a PID — a single theorem that simultaneously classifies finitely generated abelian groups (setting $R = \mathbb{Z}$) and explains all canonical forms for linear operators (setting$R = F[x]$).

---

**27.1 Finitely Generated Modules over PIDs**

**27.1.1 The Setting: $R$ a PID,$M$ Finitely Generated**
Let $R$ be a principal ideal domain and$M$ a finitely generated$R$-module. Since every module is a quotient of a free module,$M \cong R^n / N$ for some$n$ and some submodule$N \leq R^n$. By the PID property,$N$ is also free of rank$m \leq n$. The question is: how does$N$ sit inside$R^n$? The answer — the structure theorem — says one can always choose bases for$N$ and$R^n$ such that$N$ is "diagonally embedded."

**27.1.2 Smith Normal Form of a Matrix over a PID**
If $N$ is free of rank$m$ inside$R^n$, choose bases and represent the inclusion$N \hookrightarrow R^n$ by an$n \times m$ matrix$A$ over$R$. By performing row and column operations over$R$ (each operation corresponds to a change of basis in either$R^n$ or$N$), one reduces$A$ to *Smith normal form*: a diagonal matrix$\mathrm{diag}(d_1, d_2, \ldots, d_r, 0, \ldots, 0)$ with$d_i \in R$ nonzero and$d_1 \mid d_2 \mid \cdots \mid d_r$. Row operations correspond to the invertible$n \times n$ matrix group$GL_n(R)$ acting on the left; column operations to$GL_m(R)$ on the right.

**27.1.3 The Structure Theorem: Invariant Factor Form**
*Structure theorem for finitely generated modules over a PID:* Every finitely generated $R$-module$M$ is isomorphic to a direct sum
$$M \cong R^{\oplus r} \oplus R/(d_1) \oplus R/(d_2) \oplus \cdots \oplus R/(d_k)$$
where $r \geq 0$ (the *free rank*),$d_1 \mid d_2 \mid \cdots \mid d_k$ (the *invariant factors*, nonzero non-units). This is the *invariant factor form*. The Smith normal form of the presentation matrix directly gives the invariant factors$d_i$.

**27.1.4 The Structure Theorem: Primary Decomposition Form**
By the Chinese Remainder Theorem, each $R/(d_i)$ decomposes as a product of *primary cyclic modules*$R/(p^a)$ over the prime powers in the factorization of$d_i$. This gives the *primary decomposition form*:$M$ is a direct sum of copies of$R$ and modules of the form$R/(p^a)$ for prime$p$ and$a \geq 1$. The primary decomposition is easier for classification (determining all modules of a given order) while the invariant factor form is more natural for canonical form computations.

**27.1.5 Uniqueness of the Invariant Factors**
The invariant factors $r$ and$d_1, \ldots, d_k$ are uniquely determined by$M$: they are not artifacts of the particular presentation or Smith normal form computation. Uniqueness is proved by showing that$d_1 \cdots d_i$ is the GCD of all$i \times i$ minors of any presentation matrix (these GCDs are independent of the choice of presentation). The uniqueness makes the structure theorem a genuine *classification*: two finitely generated modules over a PID are isomorphic iff they have the same invariant factors.

---

**27.2 Corollary: Finitely Generated Abelian Groups**

**27.2.1 Setting $R = \mathbb{Z}$; Recovering the FTFGAG**
When $R = \mathbb{Z}$, a finitely generated$\mathbb{Z}$-module is a finitely generated abelian group. The structure theorem gives:
$$A \cong \mathbb{Z}^r \oplus \mathbb{Z}/d_1 \oplus \cdots \oplus \mathbb{Z}/d_k$$
with $d_1 \mid \cdots \mid d_k \geq 2$. This is the Fundamental Theorem of Finitely Generated Abelian Groups, whose proof via Smith normal form is now transparent: choose generators for$A$, write the relations as an integer matrix, reduce to Smith normal form, and read off the invariant factors. Chapter 19's presentation of this theorem as a standalone result is now seen as a special case.

**27.2.2 Classification of Finite Abelian Groups of Any Given Order**
The finite abelian groups of order $n$ correspond to the invariant factor sequences$(d_1, \ldots, d_k)$ with$d_1 \mid \cdots \mid d_k$,$d_i \geq 2$, and$d_1 \cdots d_k = n$. Enumerating these is equivalent to choosing a partition of each prime power$p^{v_p(n)}$ into a sequence of non-decreasing powers. For example, abelian groups of order$p^3$:$\mathbb{Z}/p^3$,$\mathbb{Z}/p \oplus \mathbb{Z}/p^2$,$(\mathbb{Z}/p)^3$ — corresponding to the partitions$(3)$,$(1, 2)$,$(1, 1, 1)$ of 3.

---

**27.3 Corollary: Rational Canonical Form**

**27.3.1 The $F[x]$-Module Structure on a Vector Space with Operator**
Let $V$ be a finite-dimensional$F$-vector space and$T: V \to V$ a linear operator. Define an$F[x]$-module structure on$V$ by$f \cdot v = f(T)(v)$. Since$V$ is finitely generated over$F$ (hence over$F[x]$), and$F[x]$ is a PID, the structure theorem applies to$V$ as an$F[x]$-module. The invariant factors are polynomials$f_1 \mid f_2 \mid \cdots \mid f_k$ in$F[x]$, and$V \cong F[x]/(f_1) \oplus \cdots \oplus F[x]/(f_k)$ as$F[x]$-modules.

**27.3.2 Recovering Invariant Factors = Invariant Factors of the Operator**
The invariant factors $f_1, \ldots, f_k$ of the$F[x]$-module$V$ are the *invariant factors of the operator$T$*. In any basis consistent with the direct sum decomposition,$T$ acts on each summand$F[x]/(f_i)$ by the companion matrix of$f_i$. The resulting block-diagonal matrix (with companion matrix blocks) is the *rational canonical form* of$T$. It is defined over any field$F$ (without requiring$F$ to be algebraically closed) and is the canonical representative of the similarity class of$T$.

**27.3.3 Recovering Jordan Form over Algebraically Closed Fields**
When $F$ is algebraically closed, each invariant factor$f_i$ factors completely into linear factors:$f_i = \prod (x - \lambda_j)^{m_j}$. By the primary decomposition form of the structure theorem, each summand$F[x]/(f_i)$ decomposes into summands$F[x]/((x - \lambda)^m)$. The action of$T$ on$F[x]/((x-\lambda)^m)$ is by a *Jordan block* of size$m$ with eigenvalue$\lambda$. Thus the Jordan canonical form is the primary decomposition form of the$F[x]$-module$V$ over an algebraically closed field — a fact that makes the Jordan form a structural theorem, not just a computational trick.

---

## Chapter 28 — Tensor Products of Modules

**What it establishes:** The universal bilinear construction for modules over a ring; the failure of left-exactness (measuring the non-flatness of modules); and the fundamental Hom–Tensor adjunction that underlies all of homological algebra.

---

**28.1 Tensor Products over a Ring**

**28.1.1 Bilinear Maps over $R$; the Universal Property of$M \otimes_R N$**
For a right $R$-module$M$ and a left$R$-module$N$, a map$f: M \times N \to A$ (to an abelian group$A$) is *$R$-balanced* if it is biadditive and satisfies$f(mr, n) = f(m, rn)$ for all$r \in R$. The *tensor product*$M \otimes_R N$ is the abelian group with the universal$R$-balanced map$\otimes: M \times N \to M \otimes_R N$: every$R$-balanced map$f: M \times N \to A$ factors uniquely through$\otimes$. Elements$m \otimes n$ (pure tensors) span$M \otimes_R N$, but not every element is a pure tensor.

**28.1.2 Construction; Balance Conditions**
Concretely, $M \otimes_R N = F(M \times N) / K$, where$F(M \times N)$ is the free abelian group on the set$M \times N$ and$K$ is the subgroup generated by the bilinearity and balance relations:$(m + m', n) - (m, n) - (m', n)$;$(m, n + n') - (m, n) - (m, n')$;$(mr, n) - (m, rn)$. The image of$(m, n)$ in the quotient is$m \otimes n$. The balance condition$mr \otimes n = m \otimes rn$ is the essential new relation beyond the vector space tensor product.

**28.1.3 Properties: Associativity, Commutativity (for Commutative $R$), Units**
For a commutative ring $R$:$(M \otimes_R N) \otimes_R P \cong M \otimes_R (N \otimes_R P)$ (associativity);$M \otimes_R N \cong N \otimes_R M$ (commutativity);$R \otimes_R M \cong M$ (unit). For non-commutative$R$, commutativity fails and extra care with left/right module structure is needed. These isomorphisms make$(\mathrm{Mod}_R, \otimes_R, R)$ a *symmetric monoidal category*, the categorical structure underlying Hopf algebras and quantum groups.

**28.1.4 Base Change: $M \otimes_R S$ for a Ring Map$R \to S$**
If $\phi: R \to S$ is a ring map, then any$R$-module$M$ gives an$S$-module$M \otimes_R S$ (making$S$ an$R$-module via$\phi$). This *base change* or *extension of scalars* is left adjoint to restriction of scalars. Examples:$\mathbb{Z}/n\mathbb{Z} \otimes_{\mathbb{Z}} \mathbb{Q} = 0$ (torsion vanishes over$\mathbb{Q}$);$\mathbb{Z}[i] \otimes_{\mathbb{Z}} \mathbb{R} \cong \mathbb{C}$ (complexification);$M \otimes_R R_P \cong M_P$ (localization). Base change is one of the most important constructions in algebra and geometry.

---

**28.2 Right Exactness of Tensor**

**28.2.1 Tensor Preserves Surjections; Fails to Preserve Injections**
For any module $M$ (over$R$), the functor$- \otimes_R M$ is *right exact*: if$A \to B \to C \to 0$ is exact, then$A \otimes M \to B \otimes M \to C \otimes M \to 0$ is exact. However,$- \otimes_R M$ need not preserve injections: if$0 \to A \to B$ is exact,$0 \to A \otimes M \to B \otimes M$ may fail to be exact. The failure is measured by the derived functor$\mathrm{Tor}$ (Chapter 40).

**28.2.2 Examples of Non-Flatness: $\mathbb{Z}/n \otimes_{\mathbb{Z}} \mathbb{Z}/m$**
Consider the injection $0 \to \mathbb{Z} \xrightarrow{\times n} \mathbb{Z}$. Tensoring with$\mathbb{Z}/m$ gives$\mathbb{Z}/m \xrightarrow{\times n} \mathbb{Z}/m$, which is the map$x \mapsto nx$ on$\mathbb{Z}/m$. This map has kernel$\{x : nx \equiv 0 \pmod m\} = \gcd(n,m)\mathbb{Z}/m\mathbb{Z}$, which is nonzero when$\gcd(n,m) > 1$. So tensoring with$\mathbb{Z}/m$ is not left-exact when$m \nmid n$. The formula$\mathbb{Z}/n \otimes_{\mathbb{Z}} \mathbb{Z}/m \cong \mathbb{Z}/\gcd(n,m)$ captures both the image and the kernel failure.

**28.2.3 The Exact Triangle: Right Exactness and the Cokernel**
Given a short exact sequence $0 \to A \xrightarrow{f} B \xrightarrow{g} C \to 0$, tensoring with$M$ gives an exact sequence$A \otimes M \to B \otimes M \to C \otimes M \to 0$. The left end is not$0$: the map$f \otimes 1: A \otimes M \to B \otimes M$ may have a nontrivial kernel. The *Tor* groups measure this failure: there is a long exact sequence$\cdots \to \mathrm{Tor}_1(B,M) \to \mathrm{Tor}_1(C,M) \to A \otimes M \to B \otimes M \to C \otimes M \to 0$.

---

**28.3 The Hom–Tensor Adjunction**

**28.3.1 $\mathrm{Hom}_R(M \otimes N, P) \cong \mathrm{Hom}_R(M, \mathrm{Hom}_R(N,P))$**
For $R$-modules$M$,$N$,$P$ (with appropriate left/right structure), there is a natural isomorphism$\mathrm{Hom}_R(M \otimes_R N, P) \cong \mathrm{Hom}_R(M, \mathrm{Hom}_R(N, P))$. This is the *tensor-hom adjunction*: a map out of$M \otimes N$ into$P$ is the same as an$R$-linear map from$M$ into the hom-module$\mathrm{Hom}_R(N, P)$. "Currying" in the module-theoretic sense.

**28.3.2 This Is an Adjunction: $- \otimes N \dashv \mathrm{Hom}(N, -)$**
The tensor-hom adjunction says that $- \otimes_R N$ is *left adjoint* to$\mathrm{Hom}_R(N, -)$. This adjunction is one of the most important in mathematics: it encodes the duality between tensor (universal bilinear maps) and hom (linear maps), and it underlies the definitions of projective and injective modules (via their behavior in Hom), the definitions of flat modules (via$\mathrm{Tor}$), and the construction of derived functors.

**28.3.3 Consequences: Left Exactness of Hom, Right Exactness of Tensor**
From the adjunction: since right adjoints preserve limits, $\mathrm{Hom}_R(N, -)$ is left exact (it preserves kernels). Since left adjoints preserve colimits,$- \otimes_R N$ is right exact (it preserves cokernels). These are not proved by direct verification but fall out of the categorical structure: the adjunction implies the exactness properties. The failure of tensor to be left-exact and the failure of hom to be right-exact are the fundamental measurements of homological algebra, leading to Ext and Tor in Chapter 40.

---

*Next: [Part VI — Field Theory and Galois Theory](part-VI-field-galois.md)*

*Prerequisites satisfied: Part I (logic, sets, Zorn's Lemma), Part II (linear algebra, especially the structure of vector spaces and linear maps), Part III (group theory, especially cyclic groups and group actions), Part IV (ring theory, especially PIDs, polynomial rings, and quotient rings).*
