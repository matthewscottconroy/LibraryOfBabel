# Functors and Natural Transformations

If categories capture mathematical structures, functors are the structure-preserving maps *between* structures of structures — and natural transformations compare functors. Historically the order of invention ran backwards: Eilenberg and Mac Lane wanted to make the informal word "natural" precise, and had to invent functors, and then categories, to say what a natural transformation is.

## Functors

**Definition (Functor).** A (covariant) **functor** $F : \mathcal{C} \to \mathcal{D}$ assigns to each object $A$ of $\mathcal{C}$ an object $F(A)$ of $\mathcal{D}$, and to each morphism $f : A \to B$ a morphism $F(f) : F(A) \to F(B)$, satisfying the **functor laws**:

$$F(\mathrm{id}_A) = \mathrm{id}_{F(A)}, \qquad F(g \circ f) = F(g) \circ F(f).$$

Examples:

- **Powerset** $\mathcal{P} : \mathbf{Set} \to \mathbf{Set}$: $A \mapsto \mathcal{P}(A)$, and $f : A \to B$ goes to the direct-image map $\mathcal{P}(f) : S \mapsto f[S]$. Both laws are one-line checks: $\mathrm{id}[S] = S$ and $(g \circ f)[S] = g[f[S]]$.
- **Forgetful** $U : \mathbf{Grp} \to \mathbf{Set}$: send a group to its underlying set, a homomorphism to itself as a function. Forgetting is structure-preserving — trivially but usefully, as Section 4 shows.
- **Free** $F : \mathbf{Set} \to \mathbf{Grp}$: send a set $S$ to the free group $F(S)$ of reduced words (see the construction in [proofs/10_category_theory/free_group_universal_property/](../../../proofs/10_category_theory/free_group_universal_property/paper_proof.md)), a function to its extension on words.
- **Fundamental group** $\pi_1 : \mathbf{Top}_{\ast} \to \mathbf{Grp}$: a continuous basepoint-preserving map induces a homomorphism of fundamental groups. Functoriality is the engine of algebraic topology: topological problems ride functors into algebra, where they can be settled by computation.
- **Hom-functors**: for locally small $\mathcal{C}$ and fixed $A$, the functor $\mathrm{Hom}(A, -) : \mathcal{C} \to \mathbf{Set}$ sends $B \mapsto \mathrm{Hom}(A, B)$ and $g : B \to C$ to postcomposition $g \circ -$.

**Contravariant functors** reverse arrows: a contravariant functor $\mathcal{C} \to \mathcal{D}$ is exactly a covariant functor $\mathcal{C}^{op} \to \mathcal{D}$, so it sends $f : A \to B$ to $F(f) : F(B) \to F(A)$ with $F(g \circ f) = F(f) \circ F(g)$. The paradigm is $\mathrm{Hom}(-, A)$, acting by precomposition; other examples are the preimage powerset $f \mapsto f^{-1}[-]$ and the dual space $V \mapsto V^{\ast}$ in $\mathbf{Vect}_k$.

There is a logical reading. A functor translates one categorical "theory" into another while preserving the composition structure — just as an interpretation of one first-order theory in another (Chapter 9) maps symbols to definable notions while preserving provability. Functors are interpretations; functoriality is the soundness condition of the translation.

## Natural Transformations

**Definition (Natural transformation).** Let $F, G : \mathcal{C} \to \mathcal{D}$ be functors. A **natural transformation** $\eta : F \Rightarrow G$ assigns to each object $A$ a **component** $\eta_A : F(A) \to G(A)$ in $\mathcal{D}$ such that for every $f : A \to B$ the **naturality square** commutes:

$$\begin{array}{ccc}
F(A) & \xrightarrow{\ F(f)\ } & F(B) \\
\downarrow{\scriptstyle \eta_A} & & \downarrow{\scriptstyle \eta_B} \\
G(A) & \xrightarrow{\ G(f)\ } & G(B)
\end{array}
\qquad\text{i.e.}\qquad \eta_B \circ F(f) = G(f) \circ \eta_A.$$

Naturality says the family $(\eta_A)$ is *uniform*: defined by one recipe, with no case-by-case choices depending on $A$.

**Worked example (singleton is natural).** Define $\eta : \mathrm{Id}_{\mathbf{Set}} \Rightarrow \mathcal{P}$ by $\eta_A(a) = \{a\}$. For any $f : A \to B$ and $a \in A$:

$$(\mathcal{P}(f) \circ \eta_A)(a) = f[\{a\}] = \{f(a)\} = \eta_B(f(a)) = (\eta_B \circ f)(a),$$

so the square commutes and $\eta$ is natural. $\square$

**Example (natural vs unnatural isomorphism).** For a finite-dimensional vector space $V$ there are isomorphisms $V \cong V^{\ast}$, but each requires choosing a basis, and no choice makes the squares commute for all linear maps simultaneously. By contrast the double-dual map

$$\iota_V : V \to V^{\ast\ast}, \qquad \iota_V(v) = (\varphi \mapsto \varphi(v))$$

is defined uniformly, and one checks $\iota_W \circ f = f^{\ast\ast} \circ \iota_V$ for every linear $f : V \to W$. This is *the* historical example: Eilenberg and Mac Lane's 1945 paper "General Theory of Natural Equivalences" was written precisely to state the difference between $V \cong V^{\ast}$ (true but arbitrary) and $V \cong V^{\ast\ast}$ (natural).

**Definition (Functor category).** For categories $\mathcal{C}, \mathcal{D}$ with $\mathcal{C}$ small, the **functor category** $[\mathcal{C}, \mathcal{D}]$ has functors $\mathcal{C} \to \mathcal{D}$ as objects and natural transformations as morphisms, composed componentwise. Categories of the form $[\mathcal{C}^{op}, \mathbf{Set}]$ — *presheaf categories* — return as the main examples of toposes in Section 6.

## The Yoneda Lemma

The deepest elementary theorem of category theory relates an object to the totality of morphisms involving it.

**Theorem (Yoneda).** Let $\mathcal{C}$ be locally small, $F : \mathcal{C} \to \mathbf{Set}$ a functor, and $A$ an object of $\mathcal{C}$. Then there is a bijection

$$\mathrm{Nat}(\mathrm{Hom}(A, -),\, F) \;\cong\; F(A),$$

natural in both $A$ and $F$.

*Proof.* Define $\Phi(\alpha) = \alpha_A(\mathrm{id}_A) \in F(A)$ for a natural transformation $\alpha : \mathrm{Hom}(A,-) \Rightarrow F$. In the other direction, for $x \in F(A)$ define $\Psi(x)$ by components $\Psi(x)_B(f) = F(f)(x)$ for $f : A \to B$. This is natural: for $g : B \to C$,

$$\Psi(x)_C(g \circ f) = F(g \circ f)(x) = F(g)\big(F(f)(x)\big) = F(g)\big(\Psi(x)_B(f)\big),$$

using functoriality of $F$. The two maps are mutually inverse. First, $\Phi(\Psi(x)) = \Psi(x)_A(\mathrm{id}_A) = F(\mathrm{id}_A)(x) = x$. Second, given $\alpha$ and any $f : A \to B$, the naturality square of $\alpha$ at $f$, evaluated at $\mathrm{id}_A \in \mathrm{Hom}(A,A)$, gives

$$\alpha_B(f) = \alpha_B(f \circ \mathrm{id}_A) = F(f)\big(\alpha_A(\mathrm{id}_A)\big) = \Psi(\Phi(\alpha))_B(f),$$

so $\Psi(\Phi(\alpha)) = \alpha$. $\square$

The proof is short because there is only one thing a natural transformation out of $\mathrm{Hom}(A,-)$ *can* do: everything is forced by where $\mathrm{id}_A$ goes. That rigidity is the content.

**Corollary (Yoneda embedding).** The functor $y : \mathcal{C} \to [\mathcal{C}^{op}, \mathbf{Set}]$, $A \mapsto \mathrm{Hom}(-, A)$, is full and faithful. In particular $A \cong B$ if and only if $\mathrm{Hom}(-, A) \cong \mathrm{Hom}(-, B)$.

The slogan: **an object is determined, up to isomorphism, by its relationships to all objects**. Nothing about the "internal nature" of $A$ matters beyond how morphisms probe it. This is the license behind universal properties (Section 3): describing how an object relates to everything else pins it down completely. Logically it resonates with proof-theoretic semantics — a proposition individuated by its inferential connections rather than by an intrinsic meaning.

## Exercises
See [problems/ch21_category_theory/](../../../problems/ch21_category_theory/)
