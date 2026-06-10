# 3.1 Group Actions: Definition and Examples

## Symmetry in Action

So far, we've studied groups as abstract algebraic objects. But groups originally arose as *symmetry groups* — collections of transformations of some object. Group actions are the formal framework for this: a group "acting" on a set means the group elements are transformations of that set, with the group operation corresponding to composition of transformations.

This perspective enriches both algebra and geometry. Studying how a group acts on various sets reveals the structure of the group itself. Conversely, groups are the right tools for understanding the symmetries of geometric and algebraic objects.

## The Definition

**Definition (Group Action).** A *left action* of a group $G$ on a set $X$ is a function:
$$\cdot : G \times X \to X, \quad (g, x) \mapsto g \cdot x$$
satisfying:
1. **Identity axiom:** $e \cdot x = x$ for all $x \in X$ (the identity does nothing)
2. **Compatibility axiom:** $g \cdot (h \cdot x) = (gh) \cdot x$ for all $g, h \in G$ and $x \in X$ (acting by $h$ then $g$ is the same as acting by $gh$)

We say $G$ *acts on* $X$ and write $G \curvearrowright X$. The set $X$ is called a *$G$-set*.

**Equivalent formulation.** A group action is equivalently a group homomorphism $\phi: G \to \text{Sym}(X)$, where $\phi(g)$ is the function $x \mapsto g \cdot x$.

*Why these are equivalent:* Given an action $(g, x) \mapsto g \cdot x$, define $\phi(g)(x) = g \cdot x$. The compatibility axiom says $\phi(gh) = \phi(g) \circ \phi(h)$ (it's a homomorphism). The identity axiom says $\phi(e) = \text{id}_X$.

Conversely, given $\phi: G \to \text{Sym}(X)$, define $g \cdot x = \phi(g)(x)$.

This equivalence is useful: it means acting on a set is the same as being represented by symmetries of that set.

**Right actions.** A *right action* is a function $X \times G \to X$, $(x, g) \mapsto x \cdot g$, satisfying $x \cdot e = x$ and $(x \cdot g) \cdot h = x \cdot (gh)$. We can convert between left and right actions by $g \cdot x = x \cdot g^{-1}$.

## The Zoo of Actions

Group actions arise everywhere. Here's a collection of important examples.

**1. The natural action of $S_n$ on $\{1, \ldots, n\}$.** $\sigma \cdot i = \sigma(i)$. This is the original "symmetry" action — $S_n$ is the group of all symmetries of a finite set.

**2. Left regular action.** $G$ acts on itself by left multiplication: $g \cdot h = gh$. This always works: $e \cdot h = h$ and $g \cdot (h \cdot k) = g \cdot (hk) = g(hk) = (gh)k = (gh) \cdot k$. Every group acts on itself in this way.

**3. Conjugation action.** $G$ acts on itself by $g \cdot h = ghg^{-1}$ (conjugation). Check: $e \cdot h = ehe^{-1} = h$ and $g \cdot (h \cdot k) = g \cdot (hkh^{-1}) = g(hkh^{-1})g^{-1} = (gh)k(gh)^{-1} = (gh) \cdot k$. ✓

The conjugation action knows about the normal subgroups of $G$: a subgroup $N$ is normal if and only if it is *stable* under the conjugation action (i.e., $G \cdot N \subseteq N$, where the action extends to subsets).

**4. $\text{GL}_n(\mathbb{R})$ acting on $\mathbb{R}^n$.** $(M, v) \mapsto Mv$ (matrix-vector multiplication). The axioms: $Iv = v$ and $M(Nv) = (MN)v$.

**5. $SO(3)$ acting on $S^2$.** The rotation group acts on the sphere: a rotation $R \in SO(3)$ maps a point $p \in S^2$ to $Rp$ (rotated point). This action is *transitive* (any point can be rotated to any other) and reveals the sphere as a homogeneous space.

**6. Deck transformations.** If $p: \tilde{X} \to X$ is a covering map, the *deck transformation group* $\text{Deck}(p)$ acts on $\tilde{X}$ (and on each fiber $p^{-1}(x_0)$). This is the action that connects covering space theory to group theory.

**7. The action of $\pi_1(X, x_0)$ on the fiber.** The fundamental group of $X$ acts on the fiber $p^{-1}(x_0)$ of any covering $p: \tilde{X} \to X$. A loop $[\gamma]$ in $\pi_1(X, x_0)$ sends a point $\tilde{x} \in p^{-1}(x_0)$ to the endpoint of the lift of $\gamma$ starting at $\tilde{x}$. This action classifies covering spaces.

## Faithful, Free, and Transitive Actions

Several properties of actions are important enough to have names.

**Faithful (or effective) action.** The action is *faithful* if the homomorphism $\phi: G \to \text{Sym}(X)$ is injective — if only the identity element acts as the identity on $X$. Equivalently: $g \cdot x = x$ for all $x \in X$ implies $g = e$.

Faithfulness means the action "reflects the full structure of $G$" — no non-trivial group element is hidden.

**Example.** The natural action of $S_n$ on $\{1, \ldots, n\}$ is faithful: if $\sigma(i) = i$ for all $i$, then $\sigma = \text{id}$.

**Example.** The conjugation action $G \curvearrowright G$ is not always faithful: $g$ acts trivially iff $ghg^{-1} = h$ for all $h$, iff $g \in Z(G)$. So the kernel of the conjugation action is $Z(G)$.

**Free action.** The action is *free* if every non-identity element acts without fixed points: $g \cdot x = x$ implies $g = e$ (for all $x$).

Free actions are "maximally faithful" in a point-by-point sense. Deck transformations of a universal cover act freely.

**Transitive action.** The action is *transitive* if for any $x, y \in X$, there exists $g \in G$ with $g \cdot x = y$. That is: $G$ acts as one big "blob" moving everything to everything else.

Transitivity means $X$ is "homogeneous" from the perspective of $G$ — no point is special relative to any other.

**Simply transitive (regular) action.** *Both* free and transitive: for any $x, y \in X$, there is a *unique* $g$ with $g \cdot x = y$. Regular actions arise in the theory of torsors and principal bundles.

**Example.** The left regular action $G \curvearrowright G$ is simply transitive: for any $h, k \in G$, there is a unique $g = kh^{-1}$ with $g \cdot h = k$.

## Actions on Coset Spaces

One of the most important families of actions: $G$ acts on the set of left cosets $G/H$ for any $H \leq G$.

Define $g \cdot (aH) = (ga)H$. Check: $e \cdot (aH) = aH$ ✓ and $g \cdot (h \cdot (aH)) = g \cdot (haH) = (gha)H = (gh)aH = (gh) \cdot (aH)$ ✓.

This action is *transitive*: for any cosets $aH$ and $bH$, the element $ba^{-1}$ sends $aH \mapsto (ba^{-1})aH = bH$.

The kernel (elements acting trivially on all cosets) is: $\{g \mid gaH = aH \text{ for all } a\} = \{g \mid a^{-1}ga \in H \text{ for all } a\} = \bigcap_{a \in G} aHa^{-1}$. This is the *core* of $H$ in $G$, the largest normal subgroup of $G$ contained in $H$.

**Theorem (Cayley's Theorem, first version).** Every group $G$ acts faithfully on itself by left multiplication (the left regular action).

This means every group embeds into a symmetric group. The next section makes this precise.

## Why Actions Matter

Group actions are not just examples — they're the main tool for proving things about groups. Key applications:

**Counting via actions.** When $G$ acts on $X$, we can use the action to count things. The orbit-stabilizer theorem (next section) gives a fundamental counting formula. Burnside's lemma (Chapter on combinatorics) counts equivalence classes using actions.

**Proving normality.** A subgroup $N$ is normal iff it's a union of conjugacy classes iff it's stable under the conjugation action. Many normality proofs go through actions.

**The class equation.** Applying the orbit-stabilizer theorem to the conjugation action gives the *class equation*:
$$|G| = |Z(G)| + \sum_i [G : C_G(g_i)]$$
(sum over non-central conjugacy class representatives). This has powerful consequences, including the fact that $p$-groups (groups of prime power order) have non-trivial centers.

**Sylow's theorems.** The existence and properties of Sylow subgroups are proved by cleverly choosing group actions.

**Covering space theory.** The fundamental connection between topology and algebra runs through the action of $\pi_1$ on fibers of covering spaces.

Each of these applications shows that "how $G$ acts" and "what $G$ is" are deeply intertwined. Actions are the way groups reveal themselves.
