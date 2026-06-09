# Part IV — Ring Theory

**Chapters 20–24**

---

## What This Part Establishes

Part IV introduces rings: algebraic structures with two operations (addition and multiplication) linked by distributivity. Rings are the algebraic home of arithmetic — the integers, polynomial rings, and matrix rings are all rings — and the theory developed here is the algebraic foundation for number theory, algebraic geometry, and every subsequent part of this book.

The part begins (Chapter 20) with the basic definitions: the ring axioms, first examples, and the structure-preserving maps (ring homomorphisms). Chapter 21 introduces ideals — the ring-theoretic analogue of normal subgroups — and the quotient ring construction, which produces new rings by "modding out" an ideal. The key structural feature here is the classification of ideals as prime or maximal, whose corresponding quotient rings are integral domains or fields respectively.

Chapter 22 develops the hierarchy of integral domains: from general domains through Euclidean domains, principal ideal domains, and unique factorization domains. This hierarchy organizes the arithmetic of divisibility and generalizes the unique factorization of the integers to a broad class of rings. Chapter 23 specializes to polynomial rings, which are simultaneously the most important family of examples and the universal tool for constructing field extensions and new rings by adjoining elements. The part concludes (Chapter 24) with commutative algebra: Noetherian rings, localization, integral extensions, and the Krull dimension — the algebraic machinery underlying algebraic geometry.

By the end of Part IV, the reader is fluent in the language of rings and ideals, understands the domain hierarchy, and has the tools to construct field extensions (Chapter 23) and understand the algebraic geometry lurking in the spectrum of a commutative ring (Chapter 24).

---

## Internal Dependency Map

```
Ch 20 (Rings, Homomorphisms)
       |
       v
Ch 21 (Ideals, Quotients, Prime/Maximal Ideals)
       |
  _____|___________
  |               |
  v               v
Ch 22           Ch 23
(Domain      (Polynomial
 Hierarchy)    Rings)
  |               |
  |_______________| 
         |
         v
      Ch 24
  (Commutative Algebra)
```

---

## Chapter 20 — Rings and Ring Homomorphisms

**What it establishes:** The two-operation structure of a ring and the diverse family of examples that rings encompass; the basic maps between rings and their kernel–image theory.

---

**20.1 The Ring Axioms**

**20.1.1 Additive Group, Multiplicative Monoid, and Distributivity**
A *ring* $(R, +, \cdot)$ is a set$R$ with two binary operations satisfying:$(R, +)$ is an abelian group;$(R, \cdot)$ is associative (a monoid if we require an identity); and the two operations are linked by the distributive laws$a(b + c) = ab + ac$ and$(a + b)c = ac + bc$. The additive group gives the "vector space" structure; the multiplicative monoid gives the "multiplication"; distributivity intertwines them. Rings generalize both the integers and the real numbers to a common algebraic framework.

**20.1.2 Conventions: Unital Rings; Commutativity**
We follow the convention that rings have a multiplicative identity $1$ (called a *unital* ring). Some authors do not require this. A ring is *commutative* if$ab = ba$ for all$a, b$. The commutativity of multiplication is a strong additional condition; most rings in this book are commutative (except matrix rings and group rings), but noncommutative rings are essential in representation theory and noncommutative algebra.

**20.1.3 Zero Divisors; Nilpotent Elements; Units**
An element $a \neq 0$ is a *zero divisor* if there exists$b \neq 0$ with$ab = 0$ or$ba = 0$. An element$a$ is *nilpotent* if$a^n = 0$ for some$n \geq 1$. A *unit* is an element with a two-sided multiplicative inverse. These three conditions partition the nonzero elements of a ring and determine much of its arithmetic character: fields have no zero divisors and every nonzero element is a unit; integral domains have no zero divisors but not all nonzero elements need be units; general rings can have all three types.

**20.1.4 The Characteristic of a Ring**
The *characteristic* of a ring $R$ is the smallest positive integer$n$ such that$n \cdot 1 = 0$ (where$n \cdot 1 = 1 + 1 + \cdots + 1$,$n$ times), or 0 if no such$n$ exists. The characteristic of$\mathbb{Z}$ is 0; of$\mathbb{Z}/n\mathbb{Z}$ is$n$; of any field is either 0 or a prime$p$. The characteristic is a fundamental invariant of a ring and determines its prime subring. In characteristic$p$, the Frobenius map$a \mapsto a^p$ is a ring endomorphism — a fact central to the arithmetic of finite fields.

---

**20.2 First Examples**

**20.2.1 $\mathbb{Z}$,$\mathbb{Q}$,$\mathbb{R}$,$\mathbb{C}$; the Integers Modulo$n$**
The integers $\mathbb{Z}$ are the prototypical ring: commutative, unital, an integral domain, but not a field (most elements have no multiplicative inverse). The rationals$\mathbb{Q}$, reals$\mathbb{R}$, and complex numbers$\mathbb{C}$ are all fields — rings where every nonzero element is a unit. The integers modulo$n$, written$\mathbb{Z}/n\mathbb{Z}$, form a commutative ring that is a field if and only if$n$ is prime and an integral domain if and only if$n$ is prime.

**20.2.2 Matrix Rings $M_n(R)$; Non-Commutativity**
The ring $M_n(R)$ of$n \times n$ matrices with entries in a ring$R$, under matrix addition and multiplication, is the canonical example of a noncommutative ring for$n \geq 2$. Even when$R = \mathbb{R}$,$M_n(\mathbb{R})$ has zero divisors (singular matrices) and non-units (non-invertible matrices). Matrix rings appear in representation theory as the "building blocks" of semisimple algebras via the Artin–Wedderburn theorem.

**20.2.3 Polynomial Rings $R[x]$ and$R[x_1,\ldots,x_n]$**
The polynomial ring $R[x]$ consists of all formal sums$a_0 + a_1 x + \cdots + a_n x^n$ with coefficients in$R$, with the usual polynomial addition and multiplication. When$R$ is an integral domain, so is$R[x]$ (the leading coefficient of a product is the product of the leading coefficients, so nonzero polynomials have nonzero product). Polynomial rings are the universal tool for constructing new rings and for encoding algebraic information about elements satisfying polynomial equations.

**20.2.4 Power Series Rings; Function Rings**
The formal power series ring $R[[x]]$ consists of all formal sums$\sum_{n=0}^\infty a_n x^n$ (without convergence requirements), with multiplication by convolution. Power series rings are local rings with a unique maximal ideal$(x)$ and are essential in algebraic geometry (completions of local rings) and number theory (the$p$-adic integers$\mathbb{Z}_p$ are$\varprojlim \mathbb{Z}/p^n\mathbb{Z}$). The ring of continuous functions$C(X, \mathbb{R})$ for a topological space$X$ gives a rich family of commutative rings connecting algebra to topology.

**20.2.5 Group Rings $R[G]$; Quaternions$\mathbb{H}$**
The *group ring* $R[G]$ is the free$R$-module on the set$G$, with multiplication extending the group operation by$R$-linearity:$(\sum a_g g)(\sum b_h h) = \sum_{g,h} a_g b_h (gh)$. Group rings are the algebraic encoding of representation theory: a representation of$G$ over$R$ is exactly an$R[G]$-module. The *quaternion algebra*$\mathbb{H} = \{a + bi + cj + dk : a,b,c,d \in \mathbb{R}\}$ is a noncommutative division ring (every nonzero element has an inverse) but not a field. It is the unique real division algebra that is not$\mathbb{R}$ or$\mathbb{C}$.

---

**20.3 Ring Homomorphisms**

**20.3.1 Definition; Preservation of All Ring Structure**
A *ring homomorphism* $\phi: R \to S$ is a function satisfying$\phi(a + b) = \phi(a) + \phi(b)$ and$\phi(ab) = \phi(a)\phi(b)$ for all$a, b \in R$, and$\phi(1_R) = 1_S$ (in the unital case). A homomorphism must preserve both operations and the identity. This is a stronger condition than being a group homomorphism for the additive group alone.

**20.3.2 Kernel and Image; Isomorphisms**
The *kernel* $\ker \phi = \{r \in R : \phi(r) = 0\}$ is an ideal of$R$ (not just a subring). The *image*$\mathrm{im}\, \phi = \{\phi(r) : r \in R\}$ is a subring of$S$. A ring homomorphism is injective if and only if$\ker \phi = \{0\}$; bijective homomorphisms are *ring isomorphisms*. The structure of quotient rings (Chapter 21) is exactly characterized by ring homomorphisms via the first isomorphism theorem.

**20.3.3 The Characteristic as a Ring Map from $\mathbb{Z}$**
For any unital ring $R$, there is a unique ring homomorphism$\phi: \mathbb{Z} \to R$ defined by$\phi(n) = n \cdot 1_R$. The image of$\phi$ is the *prime subring* of$R$; the kernel of$\phi$ is$n\mathbb{Z}$ for some$n \geq 0$. This$n$ is the characteristic of$R$. So the characteristic is not just an integer attached to a ring — it is the kernel of the canonical map from$\mathbb{Z}$.

---

**20.4 Subrings**

**20.4.1 The Subring Test**
A subset $S \subseteq R$ is a *subring* if it is closed under addition, multiplication, and additive inverses, and contains$1_R$ (in the unital case). The two-step test:$1_R \in S$; for all$a, b \in S$,$a - b \in S$ and$ab \in S$. A subring is a ring in its own right with the inherited operations. Note that a subring must contain$1_R$, not just any identity for itself — this is a notational convention that avoids pathologies.

**20.4.2 The Prime Subring; Subrings Generated by Elements**
The *prime subring* of $R$ is the image of the canonical map$\mathbb{Z} \to R$; it is the smallest subring of$R$. The subring generated by a subset$S \subseteq R$ is the smallest subring containing$S$, namely the intersection of all subrings containing$S$. When$S = \{a\}$ for a single element$a$, the generated subring is$\mathbb{Z}[a]$ (if$R$ contains$\mathbb{Z}$) or$\mathbb{F}_p[a]$ (if char$R = p$).

---

## Chapter 21 — Ideals and Quotient Rings

**What it establishes:** The ideals of a ring as the two-sided kernels of ring homomorphisms; the quotient ring construction; and the crucial distinction between prime and maximal ideals, whose quotient rings are integral domains and fields respectively.

---

**21.1 Ideals**

**21.1.1 Left, Right, and Two-Sided Ideals; the Absorption Property**
A subset $I \subseteq R$ is a *left ideal* if$(I, +)$ is a subgroup of$(R, +)$ and$RI \subseteq I$ (i.e.,$ra \in I$ for all$r \in R$,$a \in I$). Right ideals satisfy$IR \subseteq I$. A *two-sided ideal* (usually just "ideal") satisfies both. The absorption property$RI \subseteq I$ distinguishes ideals from subrings: an ideal absorbs multiplication from the ambient ring, while a subring is closed only under its own multiplication. Ideals are to rings as normal subgroups are to groups: the kernels of homomorphisms and the correct domains of quotient constructions.

**21.1.2 The Kernel of a Ring Homomorphism is an Ideal**
If $\phi: R \to S$ is a ring homomorphism, then$\ker \phi$ is a two-sided ideal of$R$. Conversely, every ideal arises as the kernel of a ring homomorphism (namely, the projection$R \to R/I$). This perfect correspondence between ideals and kernels is the ring-theoretic analogue of the correspondence between normal subgroups and group homomorphism kernels.

**21.1.3 Principal Ideals $(a) = aR$; Generated Ideals**
For $a \in R$, the *principal ideal* generated by$a$ is$(a) = RaR = \{r_1 a s_1 + \cdots + r_n a s_n\}$ (in a noncommutative ring); in a commutative ring,$(a) = aR = \{ra : r \in R\}$. More generally, the ideal generated by a subset$S$ is the smallest ideal containing$S$. In a commutative ring, the ideal generated by$a_1, \ldots, a_n$ is$(a_1, \ldots, a_n) = \{r_1 a_1 + \cdots + r_n a_n : r_i \in R\}$.

**21.1.4 Operations on Ideals: Sum, Product, Intersection**
Ideals can be combined: the *sum* $I + J = \{a + b : a \in I, b \in J\}$ is an ideal; the *intersection*$I \cap J$ is an ideal; the *product*$IJ = \{\sum a_i b_i : a_i \in I, b_i \in J\}$ is an ideal (contained in$I \cap J$). The quotient$I : J = \{r \in R : rJ \subseteq I\}$ is also an ideal. These operations give the set of ideals a rich lattice structure, analogous to the lattice of subgroups but with additional multiplicative structure.

---

**21.2 Quotient Rings**

**21.2.1 Constructing $R/I$; Well-Definedness Requires the Ideal Property**
Given an ideal $I \trianglelefteq R$, the *quotient ring*$R/I$ has underlying set the set of cosets$\{a + I : a \in R\}$ of the additive subgroup$I \leq (R, +)$. Addition is$(a + I) + (b + I) = (a + b) + I$; multiplication is$(a + I)(b + I) = ab + I$. The well-definedness of multiplication — the key step — uses the two-sided absorption property of$I$: if$a + I = a' + I$, then$ab + I = a'b + I$ because$(a' - a) \in I$ implies$(a' - a)b \in IR \subseteq I$. Without the ideal property, multiplication on cosets is not well-defined.

**21.2.2 The Universal Property of Quotient Rings**
If $I \trianglelefteq R$ and$\phi: R \to S$ is a ring homomorphism with$I \subseteq \ker \phi$, then$\phi$ factors uniquely through$R/I$: there exists a unique ring homomorphism$\bar\phi: R/I \to S$ with$\phi = \bar\phi \circ \pi$, where$\pi: R \to R/I$ is the projection. This universal property characterizes$R/I$ up to isomorphism and shows that ring quotients are the categorical coequalizers in the category of rings.

**21.2.3 The Isomorphism Theorems for Rings**
The ring isomorphism theorems parallel the group versions exactly: First: $R/\ker\phi \cong \mathrm{im}\,\phi$ for any ring homomorphism$\phi: R \to S$. Second: if$I \trianglelefteq R$ and$S \leq R$ is a subring, then$(S + I)/I \cong S/(S \cap I)$. Third: if$I \subseteq J$ are both ideals of$R$, then$(R/I)/(J/I) \cong R/J$. Fourth (Correspondence): ideals of$R/I$ correspond bijectively to ideals of$R$ containing$I$. The proofs are essentially identical to the group theory versions, reflecting the categorical nature of these theorems.

**21.2.4 Correspondence Theorem: Ideals of $R/I$ Correspond to Ideals of$R$ Containing$I$**
By the Correspondence Theorem, the ideal lattice of $R/I$ is isomorphic to the interval$[I, R]$ in the ideal lattice of$R$. This means understanding the quotient ring$R/I$ is equivalent to understanding the portion of$R$'s ideal structure above$I$. When$I$ is maximal (no ideal strictly between$I$ and$R$), the quotient$R/I$ has no nontrivial proper ideals — exactly the condition for being a field.

---

**21.3 Prime and Maximal Ideals**

**21.3.1 Prime Ideals: $ab \in P \Rightarrow a \in P$ or$b \in P$**
An ideal $P \subsetneq R$ is *prime* if whenever$ab \in P$ then$a \in P$ or$b \in P$. This is the ideal-theoretic formulation of the primeness condition from arithmetic:$p \mid ab \Rightarrow p \mid a$ or$p \mid b$. Equivalently, the complement$R \setminus P$ is closed under multiplication (a multiplicative set). Prime ideals are the central objects of commutative algebra and algebraic geometry: the prime spectrum$\mathrm{Spec}(R)$ of a ring is the set of all prime ideals, with the Zariski topology.

**21.3.2 Maximal Ideals: No Ideal Lies Strictly Between $M$ and$R$**
An ideal $M \subsetneq R$ is *maximal* if there is no ideal$I$ with$M \subsetneq I \subsetneq R$. Maximal ideals exist in every nonzero ring (by Zorn's lemma). Every maximal ideal is prime (since fields are integral domains). Not every prime ideal is maximal: in$\mathbb{Z}$,$(0)$ is prime but not maximal (it is properly contained in$(p)$ for any prime$p$).

**21.3.3 $P$ Prime$\Leftrightarrow$ $R/P$ Is a Domain**
An ideal $P$ is prime if and only if the quotient ring$R/P$ is an integral domain. Proof:$ab \in P \Leftrightarrow (a + P)(b + P) = 0 + P$ in$R/P$, i.e., the quotient has no zero divisors iff$P$ is prime. This characterization makes prime ideals the algebraic shadows of integral domains, and the irreducible varieties in algebraic geometry correspond precisely to prime ideals.

**21.3.4 $M$ Maximal$\Leftrightarrow$ $R/M$ Is a Field**
An ideal $M$ is maximal if and only if$R/M$ is a field. Proof:$R/M$ has no nontrivial proper ideals (by the correspondence theorem) iff every nonzero element of$R/M$ is a unit — the definition of a field. This elegant equivalence connects the algebraic structure of ideals to the arithmetic of fields: to adjoin a root of an irreducible polynomial$f$ to a field$F$, one forms$F[x]/(f)$, which is a field precisely because$(f)$ is maximal in$F[x]$.

**21.3.5 Existence of Maximal Ideals via Zorn's Lemma**
In any nonzero ring $R$, every proper ideal is contained in a maximal ideal. Proof via Zorn's Lemma: the set of proper ideals of$R$ containing a given proper ideal$I$ is partially ordered by inclusion; every chain has an upper bound (the union); hence a maximal element exists. This is the most common application of Zorn's Lemma in ring theory. As a consequence, every nonzero ring has at least one maximal ideal, and the quotient by any maximal ideal is a field.

**21.3.6 The Spectrum $\mathrm{Spec}(R)$: A Preview of Algebraic Geometry**
The *prime spectrum* $\mathrm{Spec}(R)$ of a commutative ring$R$ is the set of all prime ideals, topologized by the *Zariski topology*: the closed sets are$V(I) = \{P \in \mathrm{Spec}(R) : I \subseteq P\}$ for ideals$I$. For$R = k[x_1, \ldots, x_n]$ ($k$ algebraically closed),$\mathrm{Spec}(R)$ recovers the affine$n$-space over$k$, with points corresponding to maximal ideals and irreducible subvarieties to prime ideals. This is the starting point of modern algebraic geometry: geometry is encoded in the commutative algebra of coordinate rings.

---

## Chapter 22 — Divisibility and the Domain Hierarchy

**What it establishes:** The hierarchy of integral domains ordered by the strength of their divisibility theory — from general domains through UFDs, PIDs, and Euclidean domains — ending with the failure of unique factorization and the introduction of ideals as a remedy.

---

**22.1 Divisibility in Integral Domains**

**22.1.1 Divisibility; Associates; Units**
In an integral domain $R$, we say$a$ *divides*$b$ (written$a \mid b$) if$b = ac$ for some$c \in R$. Elements$a$ and$b$ are *associates* if$a = ub$ for some unit$u$; associates generate the same principal ideal,$(a) = (b)$. The divisibility relation modulo units is the correct notion of "divisibility up to insignificant differences" in a ring. For$\mathbb{Z}$,$a$ and$-a$ are associates; for$F[x]$,$f$ and$cf$ (constant multiple) are associates.

**22.1.2 Irreducible Elements and Prime Elements**
An element $p \in R$ (nonzero, non-unit) is *irreducible* if$p = ab$ implies$a$ or$b$ is a unit. It is *prime* if$p \mid ab$ implies$p \mid a$ or$p \mid b$ — equivalently, if$(p)$ is a prime ideal. In$\mathbb{Z}$, prime numbers are both irreducible and prime. In general rings, these notions diverge: the distinction between prime and irreducible is a central theme of the domain hierarchy.

**22.1.3 Prime $\Rightarrow$ Irreducible; Failure of the Converse**
Every prime element is irreducible (direct verification). The converse fails in general domains: in $\mathbb{Z}[\sqrt{-5}]$, the element$3$ is irreducible (cannot be factored further in that ring) but not prime ($3 \mid (1 + \sqrt{-5})(1 - \sqrt{-5}) = 6$ but$3 \nmid (1 \pm \sqrt{-5})$). The failure of the converse is precisely what breaks unique factorization.

**22.1.4 GCDs and LCMs; When They Exist**
A *greatest common divisor* of $a$ and$b$ is an element$d$ with$d \mid a$,$d \mid b$, and any$d'$ dividing both$a$ and$b$ must divide$d$. GCDs are unique up to associates when they exist, but may not exist in a general domain. In a PID, GCDs always exist and equal the generator of the ideal$(a) + (b) = (d)$. GCDs are part of the divisibility theory that distinguishes the levels of the domain hierarchy.

---

**22.2 Unique Factorization Domains**

**22.2.1 UFDs: Unique Factorization into Irreducibles (up to Associates and Order)**
An integral domain $R$ is a *unique factorization domain (UFD)* if every nonzero non-unit element can be written as a finite product of irreducibles, and this factorization is unique up to order and associates. The integers$\mathbb{Z}$ are the prototypical UFD; the fundamental theorem of arithmetic is the statement that$\mathbb{Z}$ is a UFD. The UFD condition is the correct generalization of unique prime factorization.

**22.2.2 In a UFD, Irreducible $\Leftrightarrow$ Prime**
In a UFD, an element is irreducible if and only if it is prime. This equivalence (whose failure in general domains is the signature of non-unique factorization) follows because: if $p$ is irreducible and$p \mid ab$, write$ab = pc$, factor both sides into irreducibles, and use uniqueness of factorization to conclude$p$ appears in the factorization of$a$ or$b$.

**22.2.3 GCDs Exist in UFDs**
In a UFD, the GCD of any two elements exists (take the product of all irreducibles appearing in both factorizations, with the minimum exponent). This explains why unique factorization is so useful for number theory: it makes GCDs, LCMs, and divisibility fully explicit.

**22.2.4 Gauss's Theorem: $R$ UFD$\Rightarrow$ $R[x]$ UFD**
*Gauss's lemma* and the theorem that $R[x]$ is a UFD when$R$ is a UFD are among the most important structural results in ring theory. Gauss's lemma states that the product of primitive polynomials (polynomials whose coefficients have GCD 1) is primitive. From this, one deduces that irreducibles in$R[x]$ are either irreducibles of$R$ or primitive polynomials irreducible over the fraction field$F = \mathrm{Frac}(R)$. As a consequence,$\mathbb{Z}[x]$ and$\mathbb{Z}[x_1, \ldots, x_n]$ are UFDs.

---

**22.3 Principal Ideal Domains**

**22.3.1 PIDs: Every Ideal Is Principal**
An integral domain $R$ is a *principal ideal domain (PID)* if every ideal of$R$ is principal — generated by a single element. The integers$\mathbb{Z}$ and polynomial rings$F[x]$ over a field are PIDs. PIDs have an extremely clean ideal theory: ideals are in bijection with associate classes of elements, and the divisibility theory is completely explicit.

**22.3.2 PIDs Are UFDs (via the Ascending Chain Condition)**
Every PID is a UFD. The key lemma is the *ascending chain condition (ACC)*: in a PID, every ascending chain of ideals $I_1 \subseteq I_2 \subseteq \cdots$ eventually stabilizes. (Proof: their union$\bigcup I_n$ is an ideal, so$\bigcup I_n = (a)$ for some$a$, and$a \in I_N$ for some$N$, giving stabilization.) From ACC, one proves every element factors into irreducibles (existence), and primeness of irreducibles in a PID gives uniqueness.

**22.3.3 Ideals in a PID: $(a) + (b) = (\gcd(a,b))$**
In a PID, the sum of two principal ideals is principal: $(a) + (b) = (\gcd(a,b))$. This immediately gives Bézout's identity:$\gcd(a,b) = ua + vb$ for some$u, v \in R$, generalizing the familiar identity for integers. Similarly, the intersection of principal ideals in a PID is$(a) \cap (b) = (\mathrm{lcm}(a,b))$.

**22.3.4 Examples: $\mathbb{Z}$,$F[x]$,$\mathbb{Z}[i]$ (Gaussian Integers)**
The integers $\mathbb{Z}$ are a PID (every ideal is$n\mathbb{Z}$). The polynomial ring$F[x]$ over a field is a PID (division algorithm shows every ideal is generated by the monic GCD of its elements). The Gaussian integers$\mathbb{Z}[i] = \{a + bi : a, b \in \mathbb{Z}\}$ are a PID (the norm$N(a + bi) = a^2 + b^2$ serves as a Euclidean norm). Gaussian integers are used to prove which primes are sums of two squares.

---

**22.4 Euclidean Domains**

**22.4.1 Definition: Euclidean Norms and the Division Algorithm**
An integral domain $R$ is a *Euclidean domain* if there exists a function$N: R \setminus \{0\} \to \mathbb{Z}_{\geq 0}$ (the *Euclidean norm*) such that for any$a \in R$ and$b \in R \setminus \{0\}$, there exist$q, r \in R$ with$a = bq + r$ and either$r = 0$ or$N(r) < N(b)$. The division algorithm is the defining feature: it allows one to divide with remainder, and the remainder decreases with respect to the norm.

**22.4.2 Euclidean Domains Are PIDs**
Every Euclidean domain is a PID. The proof uses the division algorithm to show that any ideal $I$ is generated by an element of minimum norm: if$a \in I$ has minimum norm and$b \in I$ is arbitrary, then$b = qa + r$ with$N(r) < N(a)$; since$r = b - qa \in I$, minimality forces$r = 0$, so$a \mid b$ and$I = (a)$.

**22.4.3 The Euclidean Algorithm; GCDs by Algorithm**
The *Euclidean algorithm* in a Euclidean domain: to compute $\gcd(a, b)$, repeatedly apply the division algorithm$a = bq_1 + r_1$,$b = r_1 q_2 + r_2$, \ldots until the remainder is 0; the last nonzero remainder is a GCD. The algorithm terminates because the norms strictly decrease. This gives an effective computation of GCDs and, by back-substitution, Bézout coefficients — making Euclidean domains the most computationally accessible level of the domain hierarchy.

**22.4.4 The Hierarchy: $\text{Fields} \subset \text{ED} \subset \text{PID} \subset \text{UFD} \subset \text{Domain}$**
The domain hierarchy, from strongest to weakest arithmetic: fields (every nonzero element is a unit) $\subset$ Euclidean domains (division algorithm)$\subset$ principal ideal domains (every ideal is principal)$\subset$ unique factorization domains (unique factorization into irreducibles)$\subset$ integral domains (no zero divisors). Each inclusion is strict, witnessed by specific examples:$\mathbb{Z}[x]$ is a UFD but not a PID; an example of a PID that is not Euclidean requires more work but exists.

---

**22.5 Failure of Unique Factorization**

**22.5.1 $\mathbb{Z}[\sqrt{-5}]$: A Domain That Is Not a UFD**
In $\mathbb{Z}[\sqrt{-5}]$, we have$6 = 2 \cdot 3 = (1 + \sqrt{-5})(1 - \sqrt{-5})$: two essentially different factorizations of 6 into irreducibles. (One checks that$2, 3, 1 \pm \sqrt{-5}$ are all irreducible using the norm$N(a + b\sqrt{-5}) = a^2 + 5b^2$.) This failure of unique factorization is not an isolated pathology but the rule in "most" rings of integers of number fields. The failure motivated Kummer and Dedekind to develop the theory of ideals.

**22.5.2 Ideals as a Repair: Dedekind Domains**
In a *Dedekind domain* — an integrally closed Noetherian domain of Krull dimension 1, a class that includes all rings of integers of number fields — unique factorization of elements may fail, but unique factorization of *ideals* into prime ideals always holds. The failure of unique factorization in $\mathbb{Z}[\sqrt{-5}]$ is "repaired" by working with ideals: the principal ideal$(6)$ factors uniquely as a product of prime ideals even though 6 itself does not factor uniquely into irreducibles.

**22.5.3 Ideal Class Groups and Class Numbers**
The *ideal class group* of a Dedekind domain $R$ is the group of fractional ideals modulo principal fractional ideals. It measures the failure of unique factorization in$R$: the class group is trivial if and only if$R$ is a PID. For rings of integers$\mathcal{O}_K$ of number fields$K$, the ideal class group is finite, with order the *class number*$h_K$. Class numbers are central objects of algebraic number theory, connected to the Langlands program and the distribution of primes.

---

## Chapter 23 — Polynomial Rings

**What it establishes:** The universal tool for constructing field extensions and new algebraic structures; the irreducibility theory of polynomials; the quotient field extension $F[x]/(f)$ for irreducible$f$; and Gröbner bases as the computational engine of polynomial arithmetic in several variables.

---

**23.1 The Ring $R[x]$ and the Division Algorithm**

**23.1.1 Polynomials over a Ring; Degree; Leading Coefficient**
A polynomial over a ring $R$ is a formal expression$f = a_0 + a_1 x + \cdots + a_n x^n$ where the$a_i \in R$ and$x$ is a formal symbol (an *indeterminate*). The *degree*$\deg f = n$ is the largest index with$a_n \neq 0$; the *leading coefficient* is$a_n$. Polynomials are added and multiplied in the usual way, treating$x$ as a formal symbol with no imposed relation. The ring$R[x]$ is a free object in the category of$R$-algebras: specifying a map$R[x] \to S$ of$R$-algebras is equivalent to choosing where to send$x$.

**23.1.2 Division with Remainder When the Leading Coefficient Is a Unit**
If $f, g \in R[x]$ with the leading coefficient of$g$ a unit in$R$, then there exist unique$q, r \in R[x]$ with$f = qg + r$ and$\deg r < \deg g$ (or$r = 0$). The leading coefficient of$g$ must be a unit to allow the long division algorithm to proceed. This division algorithm is the key property of$F[x]$ for a field$F$, making$F[x]$ a Euclidean domain and hence a PID.

**23.1.3 $F[x]$ Is a Euclidean Domain for Any Field$F$**
For a field $F$, the polynomial ring$F[x]$ is a Euclidean domain with Euclidean norm$N(f) = \deg f$. The division algorithm holds because every nonzero leading coefficient is a unit (fields have all nonzero elements invertible). It follows that$F[x]$ is a PID, a UFD, and that its ideals are all of the form$(f)$ for some polynomial$f$. The monic GCD of polynomials in$F[x]$ is well-defined and computable by the Euclidean algorithm.

**23.1.4 Roots and Factors: $(x-a) \mid f \Leftrightarrow f(a) = 0$**
An element $a \in R$ is a *root* of$f \in R[x]$ if$f(a) = 0$. The *factor theorem*:$(x - a) \mid f$ if and only if$f(a) = 0$. Proof: divide$f$ by$(x - a)$ to get$f = (x - a)q + r$ where$r$ is a constant (degree 0); evaluating at$a$ gives$f(a) = r$. A polynomial of degree$n$ over an integral domain has at most$n$ roots. This bound fails over non-domains (e.g.,$x^2 - 1$ has four roots in$\mathbb{Z}/8\mathbb{Z}$).

---

**23.2 Irreducibility**

**23.2.1 Irreducibility in $F[x]$; Degree Bounds**
A polynomial $f \in F[x]$ (nonzero, non-unit, i.e., degree$\geq 1$) is *irreducible* if$f = gh$ implies$g$ or$h$ is a unit (i.e., a nonzero constant). Irreducible polynomials are the primes of$F[x]$; every polynomial factors uniquely into irreducibles (by the UFD property of$F[x]$). Over a field,$f$ is irreducible iff it cannot be factored into polynomials of strictly smaller degree. Polynomials of degree 1 are always irreducible; degree 2 or 3 polynomials are irreducible iff they have no roots.

**23.2.2 Rational Root Test; Irreducibility over $\mathbb{Q}$**
If $f = a_n x^n + \cdots + a_0 \in \mathbb{Z}[x]$ and$p/q$ (in lowest terms) is a rational root of$f$, then$p \mid a_0$ and$q \mid a_n$. This *rational root test* reduces checking for rational roots to a finite computation. Combined with Gauss's lemma (which relates irreducibility over$\mathbb{Q}$ to irreducibility over$\mathbb{Z}$), it becomes an effective tool for proving irreducibility over$\mathbb{Q}$.

**23.2.3 Eisenstein's Criterion**
*Eisenstein's criterion:* If $f = a_n x^n + \cdots + a_0 \in \mathbb{Z}[x]$ and there exists a prime$p$ with$p \nmid a_n$,$p \mid a_i$ for$0 \leq i \leq n-1$, and$p^2 \nmid a_0$, then$f$ is irreducible over$\mathbb{Q}$. Eisenstein's criterion proves, for instance, that$x^{p-1} + x^{p-2} + \cdots + 1$ (the$p$th cyclotomic polynomial) is irreducible over$\mathbb{Q}$ for any prime$p$, and that$x^n - p$ is irreducible for any prime$p$. It is the most widely used irreducibility criterion in practice.

**23.2.4 Reduction Modulo $p$: Irreducibility Tests**
*Reduction modulo $p$:* if$f \in \mathbb{Z}[x]$ has the same degree as its reduction$\bar f \in \mathbb{F}_p[x]$ and$\bar f$ is irreducible over$\mathbb{F}_p$, then$f$ is irreducible over$\mathbb{Q}$. This is often practical because irreducibility over the finite field$\mathbb{F}_p$ can be checked by testing all elements. It fails if$\bar f$ factors (irreducibility over$\mathbb{Q}$ does not follow from reducibility over$\mathbb{F}_p$) or if reduction changes the degree.

**23.2.5 Irreducibility over $\mathbb{R}$ and$\mathbb{C}$; the Fundamental Theorem of Algebra**
Over $\mathbb{C}$, the *fundamental theorem of algebra* (proved in complex analysis) states that every nonconstant polynomial has a root. Therefore, the only irreducible polynomials over$\mathbb{C}$ are linear factors$x - a$. Over$\mathbb{R}$, complex roots come in conjugate pairs, so the irreducibles are linear factors and irreducible quadratics$x^2 + bx + c$ with$b^2 - 4c < 0$. Every real polynomial factors uniquely into products of these.

---

**23.3 Quotients of Polynomial Rings**

**23.3.1 $F[x]/(f)$ Is a Field When$f$ Is Irreducible**
If $f \in F[x]$ is irreducible, then$(f)$ is a maximal ideal (since$F[x]$ is a PID and prime ideals in a PID are maximal), and$F[x]/(f)$ is a field. The elements of$F[x]/(f)$ are cosets$g + (f)$, which can be represented by polynomials of degree less than$\deg f$. The image$\alpha = x + (f)$ satisfies$f(\alpha) = 0$ in$F[x]/(f)$: one has "adjoined a root of$f$" to$F$, constructing a field extension where$f$ has a root.

**23.3.2 Adjoining a Root: $F(\alpha) \cong F[x]/(\mathrm{min}_{F,\alpha})$**
If $\alpha$ is a root of an irreducible polynomial$f \in F[x]$, then the smallest field containing$F$ and$\alpha$ is$F(\alpha) \cong F[x]/(f)$. This is the *simple extension* of$F$ by$\alpha$. Elements of$F(\alpha)$ are uniquely representable as$F$-linear combinations of$1, \alpha, \ldots, \alpha^{n-1}$ where$n = \deg f$, making$[F(\alpha) : F] = n$. The construction works for any irreducible$f$: to study roots of$f$, one works in$F[x]/(f)$ rather than in some unspecified algebraic extension.

**23.3.3 The Chinese Remainder Theorem for $F[x]$**
If $f_1, \ldots, f_k \in F[x]$ are pairwise coprime, then$F[x]/(f_1 \cdots f_k) \cong F[x]/(f_1) \times \cdots \times F[x]/(f_k)$. This is the polynomial ring version of the classical Chinese Remainder Theorem (for$\mathbb{Z}$). When each$f_i$ is irreducible, each factor is a field; the theorem decomposes a quotient of$F[x]$ by a squarefree polynomial into a product of field extensions. This decomposition is fundamental to the structure theory of semisimple algebras and to splitting field constructions.

---

**23.4 Polynomial Rings in Several Variables**

**23.4.1 $R[x_1,\ldots,x_n]$ as Iterated Polynomial Rings**
The polynomial ring in several variables is defined inductively: $R[x_1, \ldots, x_n] = R[x_1, \ldots, x_{n-1}][x_n]$. A *monomial* is a product$x_1^{a_1} \cdots x_n^{a_n}$; a *polynomial* is a finite$R$-linear combination of monomials. Unlike$R[x]$, the ring$R[x_1, \ldots, x_n]$ for$n \geq 2$ is not a PID even when$R$ is a field — not every ideal is principal. The study of ideals in multivariable polynomial rings is the content of algebraic geometry and computational algebra.

**23.4.2 Monomial Orders; Division Algorithm in Several Variables**
A *monomial order* is a total well-order on monomials compatible with multiplication. The most common examples are *lexicographic order* and *graded reverse lexicographic order*. Given a monomial order, one can define the *leading monomial* of a polynomial and attempt a division algorithm, but division by a set of polynomials in several variables is not unique in general (the remainder depends on the order of division). This ambiguity motivates the theory of Gröbner bases.

**23.4.3 Gröbner Bases and Buchberger's Algorithm (Introduction)**
A *Gröbner basis* for an ideal $I \subseteq F[x_1, \ldots, x_n]$ (with respect to a fixed monomial order) is a generating set$\{g_1, \ldots, g_m\}$ for$I$ such that the leading monomials of the$g_i$ generate the ideal of all leading monomials of elements of$I$. Key property: the remainder of any polynomial upon division by a Gröbner basis is unique (independent of division order). *Buchberger's algorithm* computes a Gröbner basis from any generating set by systematically computing and reducing *S-polynomials*. Gröbner bases make ideal membership decidable and provide algorithmic access to the geometry of algebraic varieties.

**23.4.4 Hilbert's Basis Theorem: $R$ Noetherian$\Rightarrow$ $R[x]$ Noetherian**
*Hilbert's Basis Theorem:* If $R$ is a Noetherian ring (every ideal is finitely generated), then$R[x]$ is also Noetherian. Since fields are Noetherian,$F[x_1, \ldots, x_n]$ is Noetherian for any field$F$: every ideal has a finite generating set (and hence a finite Gröbner basis). Hilbert's Basis Theorem guarantees that the computations of algebraic geometry are always finite, and it is the foundational result for Noetherian ring theory.

---

## Chapter 24 — Commutative Algebra

**What it establishes:** The algebraic machinery underlying algebraic geometry and algebraic number theory: the Noetherian condition that controls chain conditions on ideals; localization that focuses on a single prime at a time; integral extensions that generalize algebraic extensions of fields; and Krull dimension that measures the geometric dimension of a ring.

---

**24.1 Noetherian Rings and Modules**

**24.1.1 The Ascending Chain Condition; Noetherian Rings**
A ring $R$ satisfies the *ascending chain condition (ACC)* on ideals if every ascending chain$I_1 \subseteq I_2 \subseteq I_3 \subseteq \cdots$ eventually stabilizes:$I_N = I_{N+1} = \cdots$ for some$N$. A *Noetherian ring* is a ring satisfying ACC. Equivalent conditions: every ideal is finitely generated; every nonempty collection of ideals has a maximal element. Noetherian rings are the correct generality for most theorems of commutative algebra: they are general enough to include fields,$\mathbb{Z}$, and polynomial rings, but controlled enough to avoid pathological infinite chains.

**24.1.2 Hilbert's Basis Theorem**
Hilbert's Basis Theorem ($R$ Noetherian$\Rightarrow$ $R[x]$ Noetherian) is restated and proved here in full: if$I \subseteq R[x]$ is an ideal, let$J_n$ be the ideal of leading coefficients of degree-$n$ elements of$I$; the chains$J_0 \subseteq J_1 \subseteq \cdots$ and$J_n \subseteq J_{n+1}$ both stabilize by Noetherianness of$R$; finiteness of these stabilizations gives a finite generating set for$I$. The theorem implies all finitely generated$k$-algebras are Noetherian.

**24.1.3 Noetherian Modules; Submodules of Finitely Generated Modules**
A module $M$ over a ring$R$ is *Noetherian* if every ascending chain of submodules stabilizes. Key fact: if$R$ is Noetherian, then every finitely generated$R$-module is Noetherian. In particular, every submodule of a finitely generated module over a Noetherian ring is finitely generated — a fact used constantly in algebraic geometry (coherent sheaves) and algebraic number theory (ideals in rings of integers are finitely generated).

---

**24.2 Localization**

**24.2.1 The Fraction Field of a Domain**
For an integral domain $R$, the *fraction field*$\mathrm{Frac}(R)$ is constructed as equivalence classes$a/b$ with$a, b \in R$,$b \neq 0$, under the relation$a/b \sim c/d \Leftrightarrow ad = bc$. Addition and multiplication are defined as for fractions. The fraction field is the smallest field containing$R$; the natural map$R \to \mathrm{Frac}(R)$ is an injective ring homomorphism.

**24.2.2 Localization at a Multiplicative Set $S^{-1}R$**
A *multiplicative set* is a subset $S \subseteq R$ closed under multiplication with$1 \in S$ and$0 \notin S$. The *localization*$S^{-1}R$ consists of fractions$a/s$ with$a \in R$,$s \in S$, modulo$a/s \sim b/t \Leftrightarrow u(at - bs) = 0$ for some$u \in S$. The localization inverts exactly the elements of$S$; there is a ring homomorphism$R \to S^{-1}R$ sending$a \mapsto a/1$. Important cases:$S = R \setminus \{0\}$ gives the fraction field;$S = \{1, f, f^2, \ldots\}$ gives the *localization at$f$*,$R_f = R[1/f]$.

**24.2.3 Localizing at a Prime: The Local Ring $R_P$**
For a prime ideal $P \subseteq R$, the complement$S = R \setminus P$ is multiplicative (since$P$ is prime:$ab \notin P$ implies$a \notin P$ and$b \notin P$). The localization$R_P = (R \setminus P)^{-1}R$ is called the *local ring of$R$ at$P$*; it has a unique maximal ideal$PR_P = \{a/s : a \in P, s \notin P\}$. Geometrically,$R_P$ is "the ring of functions defined near the point$P$." Localization at a prime isolates the arithmetic at a single prime and is the algebraic model for the notion of studying a geometric object "near a point."

**24.2.4 Properties of Localization; Local–Global Principles**
Localization is *exact*: if $0 \to M' \to M \to M'' \to 0$ is an exact sequence of$R$-modules, then$0 \to S^{-1}M' \to S^{-1}M \to S^{-1}M'' \to 0$ is also exact. The *local–global principle*: a module$M$ is zero iff$M_P = 0$ for all prime ideals$P$; an$R$-module map$f: M \to N$ is injective (resp. surjective) iff$f_P: M_P \to N_P$ is injective (resp. surjective) for all primes$P$. These principles reduce many global questions to local ones, where the ring$R_P$ is simpler (it is local).

---

**24.3 Integral Extensions**

**24.3.1 Integral Elements; Integral Closure**
An element $\alpha$ in a ring extension$S \supseteq R$ is *integral over$R$* if$\alpha$ satisfies a monic polynomial with coefficients in$R$:$\alpha^n + a_{n-1}\alpha^{n-1} + \cdots + a_0 = 0$,$a_i \in R$. The set of elements of$S$ integral over$R$ is the *integral closure* of$R$ in$S$; it is itself a ring. If every element of$S$ is integral over$R$, then$S$ is an *integral extension* of$R$. Integrality generalizes algebraicity from fields to rings and plays the same role for rings that algebraic extensions play for fields.

**24.3.2 The Going-Up and Going-Down Theorems**
*Going-Up Theorem:* If $S$ is integral over$R$ and$P \subseteq P'$ are primes in$R$ with$Q$ a prime of$S$ lying over$P$, then there exists a prime$Q' \supseteq Q$ in$S$ lying over$P'$. *Going-Down Theorem* (requires$R$ integrally closed,$S$ domain): the analogous statement for descending chains of primes. These theorems control how primes behave under integral extensions and are the key tools for the dimension theory of integral extensions — proving, for instance, that$\mathrm{Spec}(S)$ maps surjectively onto$\mathrm{Spec}(R)$ when$S$ is integral over$R$.

**24.3.3 Integrally Closed Domains; Dedekind Domains**
An integral domain $R$ is *integrally closed* (or *normal*) if every element of$\mathrm{Frac}(R)$ integral over$R$ is already in$R$. UFDs are integrally closed. A *Dedekind domain* is a Noetherian integrally closed domain of Krull dimension 1 (every nonzero prime ideal is maximal). Rings of integers$\mathcal{O}_K$ of number fields are Dedekind domains, as are the coordinate rings of smooth affine curves over a field. In a Dedekind domain, every nonzero ideal factors uniquely into prime ideals.

---

**24.4 Dimension Theory**

**24.4.1 The Krull Dimension of a Ring**
The *Krull dimension* $\dim R$ of a commutative ring$R$ is the supremum of lengths$n$ of chains$P_0 \subsetneq P_1 \subsetneq \cdots \subsetneq P_n$ of prime ideals in$R$. For a field,$\dim = 0$. For$\mathbb{Z}$,$\dim = 1$ (chains$(0) \subsetneq (p)$). The Krull dimension measures the "geometric dimension" of the scheme$\mathrm{Spec}(R)$: it is the algebraic analogue of the topological dimension of an algebraic variety.

**24.4.2 Dimension of $F[x_1,\ldots,x_n]$ Is$n$**
The Krull dimension of $F[x_1, \ldots, x_n]$ (for a field$F$) is$n$. The chain$(0) \subsetneq (x_1) \subsetneq (x_1, x_2) \subsetneq \cdots \subsetneq (x_1, \ldots, x_n)$ shows$\dim \geq n$; the other inequality uses more careful analysis. This matches geometric intuition: the affine$n$-space$\mathbb{A}^n_F$ has dimension$n$, and its coordinate ring is$F[x_1, \ldots, x_n]$.

**24.4.3 Hilbert's Nullstellensatz (Statement)**
*Nullstellensatz (Hilbert):* Let $k$ be an algebraically closed field and$I \subseteq k[x_1, \ldots, x_n]$ an ideal. Then the ideal of polynomials vanishing on the zero set$V(I) \subseteq \mathbb{A}^n_k$ is the radical$\sqrt{I} = \{f : f^m \in I \text{ for some } m\}$. As a consequence, there is a bijection between radical ideals of$k[x_1, \ldots, x_n]$ and algebraic subvarieties of$\mathbb{A}^n_k$. The Nullstellensatz is the fundamental theorem connecting algebraic geometry and commutative algebra: it shows that the geometry of polynomial zero sets is completely captured by the algebra of polynomial ideals.

**24.4.4 Connection to Algebraic Geometry: $\mathrm{Spec}(R)$**
The *spectrum* $\mathrm{Spec}(R)$ of a commutative ring$R$ — the set of prime ideals, topologized by the Zariski topology — is the fundamental object of algebraic geometry. Maximal ideals correspond to "points," prime ideals to "irreducible subvarieties," and the nilradical (the intersection of all prime ideals) to the "nilpotent functions." A ring map$R \to S$ induces a continuous map$\mathrm{Spec}(S) \to \mathrm{Spec}(R)$ by pullback of prime ideals. This functoriality makes$\mathrm{Spec}$ a bridge between commutative algebra and the geometry of schemes — the foundational objects of modern algebraic geometry (further developed in Appendix C).

---

*Next: [Part V — Module Theory](part-V-module-theory.md)*

*Prerequisites satisfied: Part I (logic, sets, functions), Part II (linear algebra, particularly the notions of quotient space and linear map), Part III (groups, homomorphisms, normal subgroups).*
