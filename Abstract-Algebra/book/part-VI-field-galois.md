# Part VI — Field Theory and Galois Theory

**Chapters 29–32**

---

## What This Part Establishes

Part VI develops the algebra of field extensions and their symmetries. A field extension $E/F$ is a pair of fields with$F \subseteq E$; the central question of field theory is: what is the structure of$E$ over$F$, and what are the "symmetries" of$E$ that fix$F$? Galois theory answers this question in full by establishing a precise dictionary between the structure of a Galois extension and the structure of its automorphism group.

The part begins (Chapter 29) with the basic vocabulary of field extensions: algebraic vs. transcendental elements, the degree $[E:F]$, simple extensions built by adjoining a root of an irreducible polynomial, and the existence and uniqueness of algebraic closures. Chapter 30 introduces the two key properties a Galois extension must have — normality (the extension is the splitting field of some polynomial) and separability (all minimal polynomials are squarefree) — and proves the primitive element theorem, which shows every finite separable extension is simple.

Chapter 31 is the heart of the part: the fundamental theorem of Galois theory, which establishes a bijection between intermediate fields $K$ (with$F \subseteq K \subseteq E$) and subgroups$H \leq \mathrm{Gal}(E/F)$, reversing inclusions and sending normal subgroups to Galois extensions. Chapter 32 gives the major applications: the theorem that a polynomial is solvable by radicals iff its Galois group is solvable (explaining why the quintic cannot be solved), the classification of ruler-and-compass constructible numbers, and the complete structure theory of finite fields.

By the end of Part VI, the reader can compute Galois groups, apply the Galois correspondence, understand the insolvability of the quintic, and see the seeds of the Langlands program in the arithmetic of Galois representations.

---

## Internal Dependency Map

```
Ch 29 (Field Extensions: degree, algebraic, algebraic closure)
                     |
                     v
Ch 30 (Normal, Separable, Primitive Element Theorem)
                     |
                     v
Ch 31 (Galois Theory: FTGT)
                     |
              _______|_______
              |             |
              v             v
         Ch 32a          Ch 32b/c
   (Solvability by     (Constructions,
     Radicals)         Finite Fields)
```

---

## Chapter 29 — Field Extensions

**What it establishes:** The foundational vocabulary of field extensions — degrees, algebraic and transcendental elements, simple extensions, and algebraic closures — which provides the stage on which Galois theory performs.

---

**29.1 Basic Definitions**

**29.1.1 Field Extensions $E/F$; Subfields and Extension Fields**
A *field extension* $E/F$ is a pair where$F$ is a subfield of$E$: both are fields and$F \subseteq E$ with the same addition and multiplication. We write$E/F$ (read "$E$ over$F$") and call$E$ the *extension field* and$F$ the *base field*. Every field extension makes$E$ a vector space over$F$ (the$F$-vector space structure on$E$ using field multiplication as scalar multiplication). This vector space structure is the foundation of the theory.

**29.1.2 The Degree $[E:F]$; Finite and Infinite Extensions**
The *degree* (or *index*) of the extension $E/F$ is$[E:F] = \dim_F E$, the dimension of$E$ as an$F$-vector space. If$[E:F] < \infty$, the extension is *finite*; otherwise *infinite*. Examples:$[\mathbb{C}:\mathbb{R}] = 2$ (basis$\{1, i\}$);$[\mathbb{R}:\mathbb{Q}] = \infty$ (uncountably many real numbers are linearly independent over$\mathbb{Q}$);$[\mathbb{Q}(\sqrt{2}):\mathbb{Q}] = 2$ (basis$\{1, \sqrt{2}\}$). The degree is the fundamental quantitative invariant of a field extension.

**29.1.3 The Tower Law: $[E:F] = [E:K][K:F]$**
If $F \subseteq K \subseteq E$ is a *tower* of field extensions, then$[E:F] = [E:K] \cdot [K:F]$. Proof: if$\{v_i\}$ is a basis of$E$ over$K$ and$\{w_j\}$ is a basis of$K$ over$F$, then$\{v_i w_j\}$ is a basis of$E$ over$F$. The Tower Law is one of the most useful computational tools in field theory: knowing any two of the three degrees determines the third. It implies that$[K:F]$ divides$[E:F]$ for any intermediate field$K$.

---

**29.2 Algebraic Extensions**

**29.2.1 Algebraic Elements; the Minimal Polynomial $\mathrm{min}_{F,\alpha}$**
An element $\alpha \in E$ is *algebraic over$F$* if it satisfies a nonzero polynomial with coefficients in$F$:$f(\alpha) = 0$ for some$f \in F[x]$,$f \neq 0$. The *minimal polynomial*$\mathrm{min}_{F,\alpha}$ of$\alpha$ over$F$ is the monic polynomial of least degree satisfied by$\alpha$; it is the generator of the kernel of the evaluation map$F[x] \to E$,$f \mapsto f(\alpha)$. The minimal polynomial is irreducible over$F$ (if$f = gh$ with$g(\alpha) = 0$ and$g$ of smaller degree, that contradicts minimality). Its degree equals$[F(\alpha):F]$.

**29.2.2 Transcendental Elements; Transcendental Extensions**
An element $\alpha \in E$ is *transcendental over$F$* if no nonzero polynomial over$F$ has$\alpha$ as a root — equivalently, the evaluation map$F[x] \to E$ is injective and$F(\alpha) \cong F(x)$ (the rational function field). Transcendental extensions are "bigger" than algebraic ones: the degree$[F(\alpha):F]$ is infinite for transcendental$\alpha$. The real numbers$\pi$ and$e$ are transcendental over$\mathbb{Q}$ (though this is non-trivial to prove). Transcendental extensions are the setting of the theory of function fields and Galois theory over curves.

**29.2.3 Simple Extensions: $F(\alpha) \cong F[x]/(\mathrm{min}_{F,\alpha})$**
The *simple extension* $F(\alpha)$ is the smallest subfield of$E$ containing$F$ and$\alpha$. If$\alpha$ is algebraic with minimal polynomial$f$ of degree$n$, then$F(\alpha) \cong F[x]/(f)$ via$\bar x \mapsto \alpha$, and$F(\alpha)$ has$F$-basis$\{1, \alpha, \ldots, \alpha^{n-1}\}$, so$[F(\alpha):F] = n$. The isomorphism$F(\alpha) \cong F[x]/(f)$ shows that working with simple algebraic extensions is the same as working in quotients of polynomial rings — making Chapter 23 directly applicable.

**29.2.4 Algebraic Extensions; Algebraic Closure under Composition**
An extension $E/F$ is *algebraic* if every element of$E$ is algebraic over$F$. Key facts: every finite extension is algebraic (bounded degree forces polynomial relations); the composition of algebraic extensions is algebraic (if$\alpha$ is algebraic over$F(\beta)$ and$\beta$ is algebraic over$F$, then$\alpha$ is algebraic over$F$). Algebraic extensions are the domain of Galois theory; transcendental extensions lie outside.

**29.2.5 Every Finite Extension Is Algebraic**
If $[E:F] = n < \infty$, then for any$\alpha \in E$, the$n+1$ elements$1, \alpha, \alpha^2, \ldots, \alpha^n$ are linearly dependent over$F$ (since$E$ is$n$-dimensional), so there exist$a_i \in F$ with$\sum a_i \alpha^i = 0$ — a polynomial relation for$\alpha$. Thus$\alpha$ is algebraic over$F$, and$E/F$ is algebraic. The converse fails: there exist algebraic extensions of infinite degree (for example, the algebraic closure$\bar{\mathbb{Q}}/\mathbb{Q}$).

---

**29.3 Algebraic Closures**

**29.3.1 Algebraically Closed Fields; the Fundamental Theorem of Algebra**
A field $k$ is *algebraically closed* if every nonconstant polynomial over$k$ has a root in$k$ — equivalently, every irreducible polynomial over$k$ is linear. The *Fundamental Theorem of Algebra* (proved in complex analysis, or via Galois theory) states that$\mathbb{C}$ is algebraically closed: every complex polynomial factors completely into linear factors. Algebraically closed fields are the "perfect" setting for polynomial algebra, where no roots are "missing."

**29.3.2 Existence of Algebraic Closures (via Zorn's Lemma)**
Every field $F$ has an *algebraic closure*$\bar F$: an algebraically closed field that is algebraic over$F$. The proof uses Zorn's Lemma: consider all algebraic extensions of$F$ (within a fixed large ambient field); the union of a chain of algebraic extensions is algebraic, so there is a maximal algebraic extension, which must be algebraically closed. The same idea can be made precise without a fixed ambient field by Artin's construction (taking a quotient of$F[x_f : f \text{ irreducible}]$ by a maximal ideal).

**29.3.3 Uniqueness up to Isomorphism**
Any two algebraic closures of $F$ are isomorphic (as$F$-algebras). The proof uses a Zorn's Lemma argument to extend an isomorphism step by step. Uniqueness-up-to-isomorphism means we can speak of "the" algebraic closure$\bar F$ unambiguously. However, the isomorphism is not canonical (there are many choices), and the automorphism group$\mathrm{Aut}(\bar F / F)$ — the absolute Galois group — is a deep and central object of number theory.

**29.3.4 $\bar{\mathbb{Q}}$: The Field of Algebraic Numbers**
The algebraic closure of $\mathbb{Q}$ is the field$\bar{\mathbb{Q}}$ of all algebraic numbers — complex numbers satisfying a polynomial equation with rational coefficients. It is countable (countably many polynomials, each with finitely many roots) and is contained in$\mathbb{C}$. The absolute Galois group$G_{\mathbb{Q}} = \mathrm{Gal}(\bar{\mathbb{Q}}/\mathbb{Q})$ is one of the central objects of the Langlands program: understanding its representations is essentially the same as understanding all Galois representations over$\mathbb{Q}$.

---

## Chapter 30 — Normal and Separable Extensions

**What it establishes:** The two properties defining a Galois extension — normality and separability — and the role of the characteristic of the base field in controlling separability; the Primitive Element Theorem showing all finite separable extensions are simple.

---

**30.1 Normal Extensions**

**30.1.1 Normal Extensions: Every Irreducible with a Root in $E$ Splits in$E$**
An algebraic extension $E/F$ is *normal* if for every irreducible polynomial$f \in F[x]$ that has at least one root in$E$, all roots of$f$ are in$E$ (i.e.,$f$ splits completely in$E[x]$). Normality is the condition that$E$ is "closed" under the action of all$F$-automorphisms of$\bar F$: if$\alpha \in E$ and$\sigma$ is an$F$-automorphism of$\bar F$, then$\sigma(\alpha) \in E$. Normal extensions are the ones whose automorphism groups are "as large as possible" and are the correct domain for the Galois correspondence.

**30.1.2 Normal $\Leftrightarrow$ Splitting Field of Some Polynomial**
A finite extension $E/F$ is normal if and only if$E$ is the *splitting field* of some polynomial$f \in F[x]$ — the smallest extension of$F$ over which$f$ factors completely into linear factors. The splitting field of$f$ always exists (adjoin roots one by one) and is unique up to$F$-isomorphism. This equivalence gives a concrete way to construct and recognize normal extensions: they are exactly the splitting fields.

**30.1.3 The Normal Closure of an Extension**
For any finite extension $E/F$, the *normal closure*$\tilde E$ is the smallest normal extension of$F$ containing$E$. Concretely,$\tilde E$ is the splitting field over$F$ of the product of all minimal polynomials$\mathrm{min}_{F,\alpha}$ for$\alpha \in E$. The normal closure is finite over$F$ (since$E/F$ is finite). Every finite extension embeds in its normal closure, and the normal closure is the "smallest normal extension above$E$."

---

**30.2 Separability**

**30.2.1 Separable Polynomials: No Repeated Roots in $\bar{F}$**
A polynomial $f \in F[x]$ is *separable* if it has no repeated roots in$\bar F$ — equivalently,$f$ and$f'$ (the formal derivative) are coprime in$F[x]$. A non-separable polynomial is called *inseparable*. Repeated roots are the algebraic obstruction to the Galois correspondence working correctly: if a minimal polynomial has repeated roots, then some automorphisms "collapse" to the same permutation of roots, reducing the size of the Galois group below the degree.

**30.2.2 Separability via the Formal Derivative: $\gcd(f, f') = 1$**
The formal derivative $f' = \sum n a_n x^{n-1}$ is defined algebraically (no limits required) and satisfies the usual differentiation rules. A polynomial$f$ is separable iff$\gcd(f, f') = 1$ in$F[x]$. The polynomial$f$ is inseparable iff$f$ and$f'$ share a common factor, i.e., iff$f'$ has a root in common with$f$ (over$\bar F$). This criterion is computable and is the primary tool for checking separability.

**30.2.3 Separable Extensions; All Elements Have Separable Minimal Polynomials**
An algebraic extension $E/F$ is *separable* if every element$\alpha \in E$ has a separable minimal polynomial over$F$. Separability is a condition on the whole extension, not just on generators. The composition of separable extensions is separable (a standard inductive argument). Every algebraic extension in characteristic 0 is separable.

**30.2.4 Separability in Characteristic 0 vs. Characteristic $p$**
In characteristic 0, every irreducible polynomial is separable: if $f$ is irreducible and$f' \neq 0$, then$\deg f' < \deg f$ forces$\gcd(f, f') = 1$ (since$f$ is irreducible). Thus in characteristic 0, every algebraic extension is separable. In characteristic$p > 0$, the derivative of a polynomial$f(x^p)$ is zero, so inseparable polynomials can exist:$x^p - a$ for$a \notin F^p$ is inseparable.

**30.2.5 The Frobenius Endomorphism in Characteristic $p$**
In characteristic $p$, the *Frobenius map*$\phi: a \mapsto a^p$ is a ring endomorphism of any commutative ring of characteristic$p$:$\phi(a + b) = (a+b)^p = a^p + b^p$ (by the binomial theorem and$\binom{p}{k} \equiv 0 \pmod p$ for$0 < k < p$). For a field$F$ of characteristic$p$, the Frobenius is injective (hence an automorphism of$F$ over its prime subfield$\mathbb{F}_p$). Over finite fields$\mathbb{F}_{p^n}$, the Frobenius is the canonical generator of the Galois group.

**30.2.6 Inseparable Extensions; Perfect Fields**
A field $F$ is *perfect* if every algebraic extension is separable, equivalently:$F$ has characteristic 0, or$F$ has characteristic$p$ and$F = F^p$ (every element is a$p$th power). All finite fields are perfect. The algebraic closure of any field is perfect. A field that is not perfect has purely inseparable extensions, and the theory of inseparable extensions requires additional tools (Hasse–Schmidt derivations,$p$-bases).

---

**30.3 The Primitive Element Theorem**

**30.3.1 Statement: Finite Separable Extensions Are Simple**
*Primitive Element Theorem:* If $E/F$ is a finite separable extension, then$E = F(\theta)$ for some single element$\theta \in E$, called a *primitive element*. In other words, every finite separable extension is simple — generated by a single element. This is a non-trivial theorem: while every finite extension is generated by finitely many elements (one per basis vector), the theorem guarantees that a single element suffices when separability holds.

**30.3.2 Proof and the Existence of a Primitive Element**
For infinite $F$: consider$E = F(\alpha, \beta)$. One shows that for all but finitely many$c \in F$, the element$\theta = \alpha + c\beta$ is a primitive element for$E/F$ (i.e.,$F(\theta) = F(\alpha, \beta)$). The key step uses the fact that the minimal polynomials of$\alpha$ and$\beta$ are separable, so they have finitely many roots, allowing the argument about "all but finitely many$c$" to go through. For finite$F$, the multiplicative group$F^*$ is cyclic, and a generator provides a primitive element directly.

**30.3.3 When the Theorem Fails: Inseparable Extensions**
The Primitive Element Theorem can fail for inseparable extensions. Over a field $F$ of characteristic$p$, the extension$F(x^{1/p}, y^{1/p})/F(x, y)$ (where$x, y$ are transcendental) is a degree-$p^2$ extension that is not simple: no single element$\theta$ generates it over$F(x,y)$. The failure is precisely due to inseparability. Understanding when and how the theorem fails motivates the study of$p$-bases and the structure theory of inseparable extensions.

---

## Chapter 31 — Galois Theory

**What it establishes:** The fundamental theorem of Galois theory — an exact, inclusion-reversing bijection between subgroups of the Galois group and intermediate fields — and the computational tools for identifying Galois groups as permutation groups.

---

**31.1 Galois Extensions and Galois Groups**

**31.1.1 Galois Extensions: Normal and Separable**
An algebraic extension $E/F$ is *Galois* if it is both normal and separable. For finite extensions, this is equivalent to:$E$ is the splitting field of a separable polynomial over$F$. The Galois condition is the "right" condition for the fundamental theorem to hold: it ensures that the Galois group is large enough ($|\mathrm{Gal}(E/F)| = [E:F]$) and that the correspondence is an exact bijection (no information is lost).

**31.1.2 The Galois Group $\mathrm{Gal}(E/F)$: Automorphisms Fixing$F$**
The *Galois group* $\mathrm{Gal}(E/F)$ is the group of all field automorphisms$\sigma: E \to E$ that fix$F$ pointwise:$\sigma(a) = a$ for all$a \in F$. The group operation is composition. When$E/F$ is the splitting field of$f \in F[x]$, every$\sigma \in \mathrm{Gal}(E/F)$ permutes the roots of$f$ (since$\sigma$ fixes$F$ and therefore respects polynomial equations over$F$), giving an embedding$\mathrm{Gal}(E/F) \hookrightarrow S_n$ where$n = \deg f$.

**31.1.3 The Order Theorem: $|\mathrm{Gal}(E/F)| = [E:F]$**
For a Galois extension $E/F$,$|\mathrm{Gal}(E/F)| = [E:F]$. This is the key quantitative result: the number of symmetries of$E$ over$F$ equals the dimension of$E$ over$F$. The proof (via Artin's theorem) goes: if$H = \mathrm{Gal}(E/F)$, then$[E:E^H] = |H|$ (Artin) and$E^H = F$ (by the Galois condition). When$E/F$ is not Galois,$|\mathrm{Aut}(E/F)| < [E:F]$.

**31.1.4 Fixed Fields: $E^H$ for$H \leq \mathrm{Gal}(E/F)$**
For a subgroup $H \leq \mathrm{Gal}(E/F)$, the *fixed field* is$E^H = \{e \in E : \sigma(e) = e \text{ for all } \sigma \in H\}$. It is a subfield of$E$ containing$F$. *Artin's theorem*: if$H$ is a finite group of automorphisms of$E$, then$[E:E^H] = |H|$ and$\mathrm{Gal}(E/E^H) = H$. Artin's theorem is the fundamental technical result underlying the whole Galois correspondence.

---

**31.2 The Fundamental Theorem of Galois Theory**

**31.2.1 Statement: The Galois Correspondence**
*Fundamental Theorem of Galois Theory:* Let $E/F$ be a finite Galois extension with Galois group$G = \mathrm{Gal}(E/F)$. There is a bijection
$$\{\text{intermediate fields } F \subseteq K \subseteq E\} \longleftrightarrow \{\text{subgroups } H \leq G\}$$
given by $K \mapsto \mathrm{Gal}(E/K)$ (the subgroup fixing$K$) and$H \mapsto E^H$ (the fixed field of$H$). This bijection is the *Galois correspondence*.

**31.2.2 The Bijection: Subgroups $\leftrightarrow$ Intermediate Fields (Inclusion-Reversing)**
The Galois correspondence is *inclusion-reversing*: if $K_1 \subseteq K_2$, then$\mathrm{Gal}(E/K_2) \leq \mathrm{Gal}(E/K_1)$ (fixing more means fewer symmetries). And if$H_1 \leq H_2$, then$E^{H_2} \subseteq E^{H_1}$. The degrees satisfy$[E:K] = |\mathrm{Gal}(E/K)|$ and$[K:F] = [G:\mathrm{Gal}(E/K)]$. The Galois correspondence is an isomorphism of posets between the lattice of intermediate fields (ordered by inclusion) and the lattice of subgroups (ordered by inclusion), with the orders reversed.

**31.2.3 Normality Correspondence: $H \trianglelefteq G \Leftrightarrow E^H/F$ Is Galois**
The correspondence respects normality: a subgroup $H$ is normal in$G$ if and only if the fixed field$E^H$ is a Galois extension of$F$ (not just of$E$). This is because normality of$H$ in$G$ means$G$ acts on$E^H$ (conjugation by$G$ sends$H$-fixed points to$H$-fixed points iff$H$ is normal), and the Galois condition for$E^H/F$ requires a full set of automorphisms.

**31.2.4 Quotient Groups as Galois Groups: $\mathrm{Gal}(E^H/F) \cong G/H$**
When $H \trianglelefteq G$, the Galois group of$E^H/F$ is$\mathrm{Gal}(E^H/F) \cong G/H$ — the quotient of$G$ by the normal subgroup$H$. This is the restriction map: every$\sigma \in G$ restricts to an automorphism of$E^H$ fixing$F$, and this restriction is surjective with kernel$H$. The first isomorphism theorem then gives$G/H \cong \mathrm{Gal}(E^H/F)$. This is the deepest structural consequence of the Galois correspondence: the structure of the quotient group is visible in the Galois theory of the intermediate extension.

**31.2.5 Proof Sketch: Artin's Theorem and the Main Argument**
The proof of the FTGT proceeds as follows. Given a subgroup $H \leq G$ with fixed field$K = E^H$: Artin's theorem gives$[E:K] = |H|$, so$[K:F] = |G|/|H| = [G:H]$. One checks that$E/K$ is Galois with group$H$ (normality uses that$E/K$ is the splitting field of any$H$-invariant polynomial), and that$K = F$ when$H = G$ (which follows from$E$ being Galois over$F$). The maps$K \mapsto \mathrm{Gal}(E/K)$ and$H \mapsto E^H$ are mutually inverse bijections by these degree and containment counts.

---

**31.3 Computing Galois Groups**

**31.3.1 Galois Groups as Permutation Groups on Roots**
If $E$ is the splitting field of$f \in F[x]$ with roots$\alpha_1, \ldots, \alpha_n$, every$\sigma \in \mathrm{Gal}(E/F)$ permutes the roots. This gives an injective homomorphism$\mathrm{Gal}(E/F) \hookrightarrow S_n$. The image is determined by: (i) which permutations are compatible with the algebraic relations among the roots; (ii) whether$f$ is irreducible (the Galois group acts transitively on the roots iff$f$ is irreducible). Computing the Galois group of$f$ means identifying its image in$S_n$.

**31.3.2 Cyclotomic Fields: $\mathrm{Gal}(\mathbb{Q}(\zeta_n)/\mathbb{Q}) \cong (\mathbb{Z}/n\mathbb{Z})^*$**
Let $\zeta_n = e^{2\pi i/n}$ be a primitive$n$th root of unity. The *cyclotomic field*$\mathbb{Q}(\zeta_n)$ is the splitting field of$x^n - 1$ (or of the$n$th cyclotomic polynomial$\Phi_n$). Every automorphism of$\mathbb{Q}(\zeta_n)$ fixing$\mathbb{Q}$ is determined by$\zeta_n \mapsto \zeta_n^k$ for some$k$ coprime to$n$. This gives$\mathrm{Gal}(\mathbb{Q}(\zeta_n)/\mathbb{Q}) \cong (\mathbb{Z}/n\mathbb{Z})^*$, an abelian group. Cyclotomic fields are the prototypical examples of abelian extensions, central to the Kronecker–Weber theorem.

**31.3.3 The Discriminant and the Sign of the Galois Group**
The *discriminant* $\Delta(f) = \prod_{i < j} (\alpha_i - \alpha_j)^2$ of a polynomial$f$ is an element of$F$. Every$\sigma \in \mathrm{Gal}(f) \leq S_n$ acts on$\delta = \prod_{i < j}(\alpha_i - \alpha_j)$ by the sign of the permutation:$\sigma(\delta) = \mathrm{sgn}(\sigma) \cdot \delta$. The Galois group$\mathrm{Gal}(f) \subseteq A_n$ iff$\sigma(\delta) = \delta$ for all$\sigma$ iff$\delta \in F$ iff$\Delta(f)$ is a perfect square in$F$. The discriminant thus determines whether the Galois group is contained in the alternating group — a parity condition with immediate consequences for cubic and quartic Galois groups.

**31.3.4 Examples: Degree 2, 3, 4 Polynomials; Galois Group Computation**
*Degree 2:* $\mathrm{Gal}(f) = \mathbb{Z}/2\mathbb{Z}$ (the two roots are swapped). *Degree 3:*$\mathrm{Gal}(f)$ is$A_3 \cong \mathbb{Z}/3\mathbb{Z}$ if$\Delta(f)$ is a square in$F$, and$S_3$ otherwise. *Degree 4:* there are five conjugacy classes of transitive subgroups of$S_4$:$S_4$,$A_4$,$D_4$,$V_4 = \mathbb{Z}/2 \times \mathbb{Z}/2$, and$\mathbb{Z}/4\mathbb{Z}$. The resolvent cubic determines which case applies. These explicit computations make the abstract Galois correspondence concrete and computable.

**31.3.5 Composite and Tower Extensions**
For a tower $F \subseteq K \subseteq E$ with$E/F$ Galois and Galois group$G$: the Galois correspondence gives$H = \mathrm{Gal}(E/K) \leq G$ and$[K:F] = [G:H]$. For the compositum$E_1 E_2$ of two Galois extensions of$F$ (inside a common algebraic closure),$\mathrm{Gal}(E_1 E_2/F)$ embeds diagonally into$\mathrm{Gal}(E_1/F) \times \mathrm{Gal}(E_2/F)$, and is the full product when$E_1 \cap E_2 = F$. These rules allow computation of Galois groups of composite extensions from those of their components.

---

## Chapter 32 — Applications of Galois Theory

**What it establishes:** The three great classical applications of Galois theory — the solvability of polynomials by radicals, the impossibility of classical geometric constructions, and the complete structure theory of finite fields — demonstrating the power of the Galois correspondence in resolving problems that had been open for centuries.

---

**32.1 Solvability by Radicals**

**32.1.1 Radical Extensions; Solvability by Radicals**
A *radical extension* of $F$ is an extension obtained by successively adjoining$n$th roots:$F = F_0 \subseteq F_1 \subseteq \cdots \subseteq F_k$ where each$F_{i+1} = F_i(\alpha_i)$ with$\alpha_i^{n_i} \in F_i$. A polynomial$f \in F[x]$ is *solvable by radicals* over$F$ if its roots lie in some radical extension of$F$ — they can be expressed using the four arithmetic operations and$n$th root extractions. The quadratic, cubic, and quartic formulas demonstrate solvability by radicals for degree$\leq 4$.

**32.1.2 Solvable Groups; the Derived Series**
The Galois group of a polynomial solvable by radicals must be *solvable*: its derived series $G \supset G' \supset G'' \supset \cdots$ must reach$\{e\}$ in finitely many steps. The key lemma: if$F$ contains the appropriate roots of unity, then a radical extension of$F$ has abelian Galois group; composites of such extensions have solvable Galois groups. Conversely, solvable Galois groups correspond to radical towers (the *Galois resolvent*).

**32.1.3 The Theorem: $f$ Solvable by Radicals$\Leftrightarrow$ $\mathrm{Gal}(f/F)$ Is Solvable**
*Galois's theorem:* A separable polynomial $f \in F[x]$ (char$F = 0$) is solvable by radicals iff its Galois group$\mathrm{Gal}(f/F)$ is a solvable group. The forward implication follows by analyzing the Galois groups of radical extensions (they are abelian, via Kummer theory). The backward implication is more subtle and uses the abelianness of each step of the derived series to construct the corresponding radical tower. This theorem is Galois's central achievement and the reason Galois theory was invented.

**32.1.4 The General Quintic Is Not Solvable: $\mathrm{Gal} = S_5$**
For the "general quintic" (the polynomial $x^5 + a_4 x^4 + \cdots + a_0$ over$\mathbb{Q}(a_0, \ldots, a_4)$), the Galois group is$S_5$. Since$S_5$ is not solvable (it contains$A_5$, which is simple and nonabelian), the general quintic is not solvable by radicals. This settles the problem that had challenged mathematicians since the quadratic formula: there is no analogue of the quadratic/cubic/quartic formulas for degree 5, not just one that hasn't been found, but one that provably cannot exist.

**32.1.5 Abel–Ruffini: A Historical Perspective**
Abel (1824) proved that the general quintic is not solvable by radicals, using an argument similar to but predating Galois theory. Ruffini had earlier attempted incomplete proofs. Galois (1832) provided the complete theory, framing the question in terms of groups and establishing the general criterion. The Abel–Ruffini theorem is historically the first major result proved by what would become abstract algebra, and the story of its discovery — including Galois's death in a duel at 20 — is one of mathematics' most dramatic.

---

**32.2 Classical Geometric Constructions**

**32.2.1 Ruler-and-Compass Constructibility: Degree-$2^k$ Extensions**
A real number $\alpha$ is *constructible by ruler and compass* (from$0$ and$1$) iff$\alpha$ can be obtained from the rationals by a sequence of field operations and square root extractions — iff$\alpha$ is contained in a field$F$ with$[\mathbb{Q}(\alpha):\mathbb{Q}]$ a power of 2. More precisely:$\alpha$ is constructible iff there is a tower$\mathbb{Q} = F_0 \subseteq F_1 \subseteq \cdots \subseteq F_k$ with each$[F_{i+1}:F_i] = 2$ and$\alpha \in F_k$. This characterization makes ruler-and-compass constructibility a precise algebraic condition, amenable to Galois-theoretic analysis.

**32.2.2 Doubling the Cube: Degree-3 Extension, Not Constructible**
*Doubling the cube:* given a cube of volume 1, construct a cube of volume 2. This requires constructing $\sqrt[3]{2}$. But$[\mathbb{Q}(\sqrt[3]{2}):\mathbb{Q}] = 3$, which is not a power of 2. Therefore$\sqrt[3]{2}$ is not constructible, and the cube cannot be doubled by ruler and compass. The proof is immediate once the algebraic characterization of constructibility is established: a 3-dimensional field extension cannot arise in a tower of quadratic extensions.

**32.2.3 Trisecting the Angle: Similar Obstruction**
*Trisecting the angle:* given a $60°$ angle, construct a$20°$ angle. This requires constructing$\cos(20°)$. The minimal polynomial of$\cos(20°)$ over$\mathbb{Q}$ is$8x^3 - 6x - 1$, which has degree 3 and is irreducible over$\mathbb{Q}$ (Eisenstein after substitution). Since$[\mathbb{Q}(\cos 20°):\mathbb{Q}] = 3$ is not a power of 2, the trisection is impossible by ruler and compass. Similar arguments handle most angles: only those for which$\cos(\theta/3)$ has degree a power of 2 over$\mathbb{Q}$ are trisectable.

**32.2.4 Squaring the Circle: Transcendence of $\pi$**
*Squaring the circle:* construct a square with the same area as a given circle. This requires constructing $\sqrt{\pi}$. Since$\pi$ is transcendental over$\mathbb{Q}$ (Lindemann, 1882),$\sqrt{\pi}$ is also transcendental, hence not algebraic over$\mathbb{Q}$, and certainly not in a finite tower of quadratic extensions. The squaring of the circle is thus impossible not merely because of a degree obstruction, but because$\pi$ itself is transcendental — a strictly stronger statement.

**32.2.5 Constructible Regular Polygons: Gauss and the 17-gon**
Gauss proved (1796) that the regular 17-gon is constructible, and Gauss–Wantzel (1837) completely determined which regular $n$-gons are constructible: the regular$n$-gon is constructible by ruler and compass iff$n = 2^k p_1 \cdots p_r$ where$p_1, \ldots, p_r$ are distinct Fermat primes ($p = 2^{2^m} + 1$ for some$m \geq 0$). The known Fermat primes are 3, 5, 17, 257, 65537. The constructibility of the 17-gon was the result that convinced Gauss to pursue mathematics rather than philology.

---

**32.3 Finite Fields**

**32.3.1 Existence and Uniqueness of $\mathbb{F}_{p^n}$**
For every prime $p$ and every$n \geq 1$, there exists a field with exactly$p^n$ elements, and this field is unique up to isomorphism. Existence: the splitting field of$x^{p^n} - x$ over$\mathbb{F}_p$ has exactly$p^n$ elements (the roots of$x^{p^n} - x$ form a field, since the Frobenius is a field homomorphism). Uniqueness: any field of characteristic$p$ and order$p^n$ must contain the roots of$x^{p^n} - x$ (since$\alpha^{p^n} = \alpha$ for all$\alpha$ in a field of order$p^n$), and these roots form the unique subfield.

**32.3.2 $\mathbb{F}_{p^n}$ as the Splitting Field of$x^{p^n} - x$**
The field $\mathbb{F}_{p^n}$ is the splitting field of$x^{p^n} - x$ over$\mathbb{F}_p$, and this polynomial is separable (since its derivative is$-1 \neq 0$, so$\gcd(x^{p^n} - x, -1) = 1$). Therefore$\mathbb{F}_{p^n}/\mathbb{F}_p$ is a Galois extension of degree$n$. The$p^n$ elements of$\mathbb{F}_{p^n}$ are precisely the roots of$x^{p^n} - x$, and these form a field because the set of roots is closed under the field operations (a consequence of the Frobenius being a field automorphism).

**32.3.3 $(\mathbb{F}_{p^n}^*, \cdot)$ Is Cyclic**
The multiplicative group $\mathbb{F}_{p^n}^* = \mathbb{F}_{p^n} \setminus \{0\}$ is cyclic of order$p^n - 1$. The proof:$\mathbb{F}_{p^n}^*$ is a finite abelian group; by the structure theorem, it is a direct product of cyclic groups of prime power order; a polynomial of degree$d$ has at most$d$ roots in any field; using this to count elements of each order shows the group must be cyclic. A generator of$\mathbb{F}_{p^n}^*$ is called a *primitive root* modulo$p^n$.

**32.3.4 $\mathrm{Gal}(\mathbb{F}_{p^n}/\mathbb{F}_p) \cong \mathbb{Z}/n\mathbb{Z}$, Generated by Frobenius**
The Galois group $\mathrm{Gal}(\mathbb{F}_{p^n}/\mathbb{F}_p)$ is cyclic of order$n$, generated by the *Frobenius automorphism*$\phi: x \mapsto x^p$. That$\phi$ fixes$\mathbb{F}_p$ is clear (Fermat's little theorem). That$\phi$ has order$n$ follows from:$\phi^k = \mathrm{id}$ iff every element satisfies$x^{p^k} = x$ iff$p^k \equiv 0 \pmod{p^n - 1}$... more precisely, the smallest$k$ with$\phi^k = \mathrm{id}$ is$n$. The Galois group being cyclic means the Galois correspondence for$\mathbb{F}_{p^n}/\mathbb{F}_p$ is the same as the lattice of divisors of$n$.

**32.3.5 Subfields of $\mathbb{F}_{p^n}$:$\mathbb{F}_{p^d}$ for$d \mid n$**
The Galois correspondence for the cyclic Galois group $\mathbb{Z}/n\mathbb{Z}$ says: subgroups of$\mathbb{Z}/n\mathbb{Z}$ correspond to intermediate fields. Subgroups of$\mathbb{Z}/n\mathbb{Z}$ are$\mathbb{Z}/d\mathbb{Z}$ for$d \mid n$, corresponding to the unique subfield$\mathbb{F}_{p^d} \subseteq \mathbb{F}_{p^n}$ for each$d \mid n$. This gives a complete picture:$\mathbb{F}_{p^n}$ has exactly one subfield of each order$p^d$ for$d \mid n$, and no others. The lattice of subfields is isomorphic to the lattice of divisors of$n$ — a complete and beautiful classification.

---

*Next: [Part VII — Category Theory](part-VII-category-theory.md)*

*Prerequisites satisfied: Part I (logic, sets, functions, Zorn's Lemma), Part II (linear algebra, vector spaces), Part III (groups, including cyclic groups, quotient groups, solvable groups), Part IV (ring theory, polynomial rings, quotient rings, field definition), Part V (modules, briefly).*
