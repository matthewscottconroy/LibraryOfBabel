# 1.6 Homomorphisms and Isomorphisms

## Structure-Preserving Maps

Once you have mathematical objects (groups), the next question is: what are the right *maps* between them? The "right" maps should preserve the structure — they should take the group operation in the source to the group operation in the target.

This is a general principle throughout mathematics. For sets, the right maps are functions. For vector spaces, the right maps are linear transformations (they preserve addition and scalar multiplication). For topological spaces, the right maps are continuous functions (they preserve open sets). For groups, the right maps are *homomorphisms*.

**Definition (Group Homomorphism).** A *homomorphism* from a group $(G, \cdot)$ to a group $(H, *)$ is a function $\phi : G \to H$ satisfying:
$$\phi(a \cdot b) = \phi(a) * \phi(b) \quad \text{for all } a, b \in G$$

In words: $\phi$ takes products in $G$ to products in $H$. The operation happens either before or after applying $\phi$ — it doesn't matter, you get the same result.

## Why This Condition?

The defining condition $\phi(ab) = \phi(a)\phi(b)$ might look arbitrary, but it's the only condition that makes $\phi$ "compatible" with the group structure. Let's see what it forces.

**Lemma.** If $\phi: G \to H$ is a homomorphism, then:
1. $\phi(e_G) = e_H$ (identities map to identities)
2. $\phi(a^{-1}) = \phi(a)^{-1}$ (inverses map to inverses)
3. $\phi(a^n) = \phi(a)^n$ for all $n \in \mathbb{Z}$

*Proof of (1):* $\phi(e_G) = \phi(e_G \cdot e_G) = \phi(e_G) * \phi(e_G)$. Multiply both sides on the right by $\phi(e_G)^{-1}$: $e_H = \phi(e_G)$. $\square$

*Proof of (2):* $\phi(a) * \phi(a^{-1}) = \phi(a \cdot a^{-1}) = \phi(e_G) = e_H$. So $\phi(a^{-1})$ is a right inverse of $\phi(a)$, hence equals $\phi(a)^{-1}$. $\square$

These consequences show that a homomorphism is not just about "respecting multiplication" — it automatically respects the entire group structure: identity, inverses, powers.

## The Image and Kernel

Given a homomorphism $\phi: G \to H$, two subsets stand out:

**Definition (Image and Kernel).**
$$\text{Im}(\phi) = \{\phi(g) \mid g \in G\} \subseteq H$$
$$\ker(\phi) = \{g \in G \mid \phi(g) = e_H\} \subseteq G$$

**Theorem.** $\text{Im}(\phi) \leq H$ and $\ker(\phi) \unlhd G$.

*Proof that $\text{Im}(\phi) \leq H$:*
- $\phi(e_G) = e_H \in \text{Im}(\phi)$ (nonempty).
- If $\phi(a), \phi(b) \in \text{Im}(\phi)$, then $\phi(a)\phi(b) = \phi(ab) \in \text{Im}(\phi)$ (closed).
- If $\phi(a) \in \text{Im}(\phi)$, then $\phi(a)^{-1} = \phi(a^{-1}) \in \text{Im}(\phi)$ (inverses). $\square$

*Proof that $\ker(\phi) \unlhd G$:*
- $\phi(e) = e \in \ker(\phi)$ (nonempty).
- If $a, b \in \ker(\phi)$: $\phi(ab) = \phi(a)\phi(b) = e \cdot e = e$, so $ab \in \ker(\phi)$ (closed).
- If $a \in \ker(\phi)$: $\phi(a^{-1}) = \phi(a)^{-1} = e^{-1} = e$, so $a^{-1} \in \ker(\phi)$ (inverses).
- Normality: for $n \in \ker(\phi)$ and $g \in G$: $\phi(gng^{-1}) = \phi(g)\phi(n)\phi(g)^{-1} = \phi(g) \cdot e \cdot \phi(g)^{-1} = e$. So $gng^{-1} \in \ker(\phi)$. $\square$

The kernel being *normal* (not just a subgroup) is the crucial extra piece. Conversely:

**Theorem.** Every normal subgroup is the kernel of some homomorphism.

*Proof.* If $N \unlhd G$, then the canonical map $\pi: G \to G/N$, $\pi(g) = gN$, is a homomorphism with $\ker(\pi) = N$. $\square$

So "kernel of a homomorphism" and "normal subgroup" are two names for the same thing.

## Examples of Homomorphisms

**Inclusion:** If $H \leq G$, the inclusion $\iota: H \hookrightarrow G$ defined by $\iota(h) = h$ is a homomorphism. Its image is $H$, its kernel is $\{e\}$.

**Quotient map:** $\pi: G \to G/N$, $\pi(g) = gN$. Surjective, kernel $= N$.

**Sign map:** $\text{sgn}: S_n \to \{1, -1\}$ (where $\{1,-1\}$ is a group under multiplication). Sends even permutations to $1$ and odd permutations to $-1$. Homomorphism: sign is multiplicative (composition of permutations multiplies signs). Kernel is $A_n$, image is all of $\{1,-1\}$ for $n \geq 2$.

**Determinant:** $\det: \text{GL}_n(\mathbb{R}) \to \mathbb{R}^*$. The determinant is multiplicative: $\det(AB) = \det(A)\det(B)$. Kernel is $\text{SL}_n(\mathbb{R})$.

**Exponential:** $\exp: (\mathbb{R}, +) \to (\mathbb{R}_{>0}, \cdot)$ defined by $\exp(x) = e^x$. Check: $e^{x+y} = e^x \cdot e^y$. This is an isomorphism.

**Reduction mod n:** $\mathbb{Z} \to \mathbb{Z}/n\mathbb{Z}$, $k \mapsto [k]$. Surjective, kernel is $n\mathbb{Z}$.

**Trivial homomorphism:** $\phi: G \to H$ defined by $\phi(g) = e_H$ for all $g$. Always a homomorphism. Image is $\{e_H\}$, kernel is $G$.

## Isomorphisms and the Notion of Sameness

**Definition.** A homomorphism $\phi: G \to H$ is:
- *Injective* (monomorphism): $\phi(a) = \phi(b) \implies a = b$
- *Surjective* (epimorphism): for every $h \in H$, there exists $g \in G$ with $\phi(g) = h$
- *Bijective* (isomorphism): both injective and surjective

Two groups are *isomorphic*, written $G \cong H$, if there exists an isomorphism $\phi: G \to H$.

**Lemma.** A homomorphism $\phi: G \to H$ is injective if and only if $\ker(\phi) = \{e_G\}$.

*Proof.* ($\Rightarrow$) If $\phi$ is injective and $g \in \ker(\phi)$, then $\phi(g) = e_H = \phi(e_G)$, so $g = e_G$. ($\Leftarrow$) If $\ker(\phi) = \{e\}$ and $\phi(a) = \phi(b)$, then $\phi(a)\phi(b)^{-1} = e_H$, so $\phi(ab^{-1}) = e_H$, so $ab^{-1} \in \ker(\phi) = \{e\}$, so $a = b$. $\square$

**What isomorphism means:** Two isomorphic groups are "the same group up to relabeling." Every property expressible in the language of group theory (order, commutativity, number of elements of each order, normal subgroups, etc.) is the same in $G$ and $H$ if $G \cong H$. An isomorphism is a bijection that perfectly preserves all group structure.

This is the *right notion of equality for groups* — not literal set-equality, but structural sameness. In category theory, this is called being *isomorphic objects*, and it's the correct notion in every category.

In HoTT, this theme becomes central: the *univalence axiom* says that isomorphic mathematical objects are actually *equal* (as types). The philosophical insight of HoTT is that "sameness up to isomorphism" is the right notion of equality for mathematical objects, and this should be built into the foundations.

## Examples of Isomorphisms

$(\mathbb{R}, +) \cong (\mathbb{R}_{>0}, \cdot)$ via $\exp: x \mapsto e^x$ (inverse: $\log$).

$\mathbb{Z}/4\mathbb{Z} \not\cong \mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$: both have order 4, but $\mathbb{Z}/4\mathbb{Z}$ has an element of order 4 (namely $[1]$) while $\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$ does not (every non-identity element has order 2). Order of elements is preserved by isomorphisms.

$S_3 \cong D_3$: both have order 6 and the same structure. An explicit isomorphism: label the vertices of an equilateral triangle 1, 2, 3; each symmetry of the triangle permutes the vertices.

$\mathbb{Z}/mn\mathbb{Z} \cong \mathbb{Z}/m\mathbb{Z} \times \mathbb{Z}/n\mathbb{Z}$ when $\gcd(m,n) = 1$ (Chinese Remainder Theorem).

## Automorphisms

An *automorphism* of $G$ is an isomorphism $\phi: G \to G$ (from $G$ to itself). Automorphisms form a group under composition, denoted $\text{Aut}(G)$.

For $n \geq 3$: $\text{Aut}(\mathbb{Z}/n\mathbb{Z}) \cong (\mathbb{Z}/n\mathbb{Z})^*$, the group of units mod $n$. 

**Inner automorphisms:** For each $g \in G$, define $\text{conj}_g: G \to G$ by $\text{conj}_g(a) = gag^{-1}$ (conjugation by $g$). This is an automorphism: $\text{conj}_g(ab) = g(ab)g^{-1} = (gag^{-1})(gbg^{-1}) = \text{conj}_g(a)\text{conj}_g(b)$. The map $g \mapsto \text{conj}_g$ is a homomorphism $G \to \text{Aut}(G)$ with kernel $Z(G)$.

The image of this homomorphism is the group of *inner automorphisms* $\text{Inn}(G) \cong G/Z(G)$. Automorphisms not of this form are called *outer automorphisms*, and the *outer automorphism group* $\text{Out}(G) = \text{Aut}(G)/\text{Inn}(G)$ is an important invariant.

The most famous result: $\text{Out}(S_6) \neq 1$ — the symmetric group on 6 elements has an outer automorphism, uniquely among all $S_n$. This surprising fact has deep connections to the Mathieu groups.

## Invariants Under Isomorphism

To show two groups are *not* isomorphic, we need to find a property that differs. Properties preserved by isomorphisms include:
- Order $|G|$
- Whether the group is abelian
- The multiset of orders of elements
- The number of elements of each order
- Whether $G$ is cyclic
- The number and orders of subgroups
- The structure of the subgroup lattice
- Isomorphism class of $G/Z(G)$

To show two groups *are* isomorphic, we must exhibit an explicit isomorphism and verify it works — or use a structural theorem that guarantees it.

This classification problem — which groups of a given order are isomorphic? — is deeply studied and connects to number theory, topology, and representation theory.
