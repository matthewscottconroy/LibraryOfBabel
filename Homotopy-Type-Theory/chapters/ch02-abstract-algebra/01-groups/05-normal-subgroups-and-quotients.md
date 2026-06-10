# 1.5 Normal Subgroups and Quotient Groups

## The Problem With Ordinary Cosets

We just saw that every subgroup $H$ of $G$ partitions $G$ into cosets. But can we turn this partition into a *group*? That is, can we define a multiplication on the set of cosets $\{gH \mid g \in G\}$ that makes it a group?

The natural thing to try is: $(aH)(bH) = (ab)H$. Multiply the representative elements and take that coset. Let's see if this is well-defined.

**The problem:** Suppose we pick different representatives. That is, suppose $aH = a'H$ and $bH = b'H$ — the cosets are the same, but we're looking at them through different representatives. For our multiplication to be well-defined, we'd need $(ab)H = (a'b')H$.

Let's see when this fails. Take $G = S_3$ and $H = \{e, (12)\}$. We computed earlier that the cosets are:
- $eH = \{e, (12)\}$
- $(13)H = \{(13), (132)\}$  
- $(23)H = \{(23), (123)\}$

Note that $(13)H = (132)H$ (they're the same coset, just described with different representatives). And $(23)H = (123)H$.

Now try: $(eH)((13)H) = (13)H$ using representative $e$ and $(13)$. But using representatives $e$ and $(132)$: $(e)(132) = (132) \in (13)H$. OK so far.

But try from the other side: $((13)H)(eH)$ using $(13)$ and $e$: $(13)e = (13) \in (13)H$.
Using $(132)$ and $(12)$: $(132)(12) = ?$. Let's compute: $1 \xrightarrow{(12)} 2 \xrightarrow{(132)} 1$; $2 \xrightarrow{(12)} 1 \xrightarrow{(132)} 3$; $3 \xrightarrow{(12)} 3 \xrightarrow{(132)} 2$. So $(132)(12) = (23)$, which is in $(23)H$, not $(13)H$.

The coset multiplication is *not* well-defined here. Cosets $aH$ and $bH$ don't have a consistent product unless we add a condition on $H$.

## The Key Condition: Normality

**Definition (Normal Subgroup).** A subgroup $N \leq G$ is *normal* in $G$, written $N \unlhd G$, if for all $g \in G$ and all $n \in N$:
$$gng^{-1} \in N$$

Equivalently: $gNg^{-1} = N$ for all $g \in G$ (where $gNg^{-1} = \{gng^{-1} \mid n \in N\}$).

The operation $n \mapsto gng^{-1}$ is called *conjugation* by $g$. So normality says: $N$ is closed under conjugation by every element of $G$.

Another way to say this: the left coset $gN$ equals the right coset $Ng$ for all $g$. (Proof: $gN = Ng \iff gNg^{-1} = N$ — multiply both sides on the right by $g$.) So $N \unlhd G$ means left and right cosets coincide.

## Examples of Normal and Non-Normal Subgroups

**Always normal:**
- The trivial subgroup $\{e\}$ and $G$ itself are always normal.
- In an abelian group, *every* subgroup is normal: $gng^{-1} = n$ (since $gn = ng$).
- The center $Z(G) = \{z \in G \mid zg = gz \text{ for all } g\}$ is always normal.
- Any subgroup of index 2 is normal. (Index 2 means only two cosets; the non-identity coset must equal both its left and right versions since there's only one option.)

**Normal in $S_n$:**
- $A_n \unlhd S_n$: index 2, so it's normal. Alternatively: $A_n = \ker(\text{sgn})$ where sgn is the sign homomorphism, and kernels of homomorphisms are always normal.

**Not normal:**
- In $S_3$, the subgroup $H = \{e, (12)\}$ is *not* normal. We saw this above: the coset multiplication wasn't well-defined.
  
  More directly: $(123)(12)(123)^{-1} = (123)(12)(132)$. Compute: apply (132), then (12), then (123). $1 \xrightarrow{(132)} 3 \xrightarrow{(12)} 3 \xrightarrow{(123)} 1$; $2 \xrightarrow{(132)} 1 \xrightarrow{(12)} 2 \xrightarrow{(123)} 3$; $3 \xrightarrow{(132)} 2 \xrightarrow{(12)} 1 \xrightarrow{(123)} 2$. So the conjugate is $(23)$.
  
  $(23) \notin \{e, (12)\}$, confirming $H$ is not normal.

## The Quotient Group

When $N \unlhd G$, we can form the quotient group.

**Definition (Quotient Group).** If $N \unlhd G$, the *quotient group* $G/N$ is the set of left cosets of $N$ in $G$:
$$G/N = \{gN \mid g \in G\}$$
with the operation $(aN)(bN) = (ab)N$.

**Theorem.** When $N \unlhd G$, this operation is well-defined, and $G/N$ is a group.

*Proof.*

*Well-definedness:* Suppose $aN = a'N$ and $bN = b'N$. We need $(ab)N = (a'b')N$. Since $aN = a'N$, we have $a' = an_1$ for some $n_1 \in N$. Since $bN = b'N$, we have $b' = bn_2$ for some $n_2 \in N$. Then:
$$a'b' = (an_1)(bn_2) = a(n_1b)n_2 = a(b \cdot b^{-1}n_1b)n_2 = ab \cdot (b^{-1}n_1b) \cdot n_2$$

Since $N \unlhd G$, $b^{-1}n_1b \in N$. So $(b^{-1}n_1b)n_2 \in N$, and $a'b' = ab \cdot (\text{something in }N)$, which means $(a'b')N = (ab)N$.

*Group axioms:*
- *Associativity:* $(aN \cdot bN) \cdot cN = (ab)N \cdot cN = (abc)N = aN \cdot (bc)N = aN \cdot (bN \cdot cN)$.
- *Identity:* $eN = N$ is the identity: $(eN)(aN) = (ea)N = aN$.
- *Inverses:* $(aN)^{-1} = a^{-1}N$: $(aN)(a^{-1}N) = (aa^{-1})N = eN = N$. $\square$

**Examples of Quotient Groups:**

$\mathbb{Z}/n\mathbb{Z}$ is literally the quotient group: $\mathbb{Z}$ mod the normal subgroup $n\mathbb{Z}$. The elements are cosets $[k] = k + n\mathbb{Z}$, and addition is $[j] + [k] = [j+k]$. Normality is automatic since $\mathbb{Z}$ is abelian.

$S_n / A_n \cong \mathbb{Z}/2\mathbb{Z}$: the quotient by the alternating group has two elements (the even coset $A_n$ and the odd coset — everything else). This is the sign map.

$G / Z(G)$: the quotient of $G$ by its center. If this quotient is cyclic, then $G$ is abelian (a useful lemma: if $G/Z(G)$ is cyclic, then $G = Z(G)$, so $G$ is abelian).

## Quotients as Collapsing

Geometrically, forming $G/N$ is like "collapsing" the subgroup $N$ to a point — all of $N$ becomes the identity in $G/N$, and all elements in the same $N$-coset become equal.

This is exactly the same as quotient constructions everywhere in mathematics:
- Quotient vector spaces: $V/W$ collapses $W$ to zero.
- Quotient topological spaces: identify points in an equivalence class.
- Quotient rings: $R/I$ collapses the ideal $I$ to zero.

The theme is always: you have a structure and a "substructure to mod out by," and the result is a new structure where that substructure has become trivial. Normality is the condition that makes this collapse respect the group structure.

## The Canonical Homomorphism

There's always a canonical surjective homomorphism $\pi: G \to G/N$ defined by $\pi(g) = gN$. Check: $\pi(ab) = (ab)N = (aN)(bN) = \pi(a)\pi(b)$. The kernel of $\pi$ is exactly $N$: $\pi(g) = eN = N \iff gN = N \iff g \in N$.

So $N$ is the kernel of the map that "quotients out by $N$." This is the first glimpse of the first isomorphism theorem.

## Commutator Subgroup and Abelianization

**Definition.** The *commutator* of $a$ and $b$ is $[a,b] = aba^{-1}b^{-1}$. Note that $[a,b] = e$ iff $ab = ba$ — commutators measure the failure of commutativity.

**Definition.** The *commutator subgroup* (or *derived subgroup*) $[G,G]$ is the subgroup generated by all commutators: $[G,G] = \langle [a,b] \mid a,b \in G \rangle$.

**Theorem.** $[G,G] \unlhd G$, and $G/[G,G]$ is abelian. Moreover, if $N \unlhd G$ and $G/N$ is abelian, then $[G,G] \leq N$.

*Proof sketch.* For normality: conjugate a commutator $[a,b] = aba^{-1}b^{-1}$ by $g$:
$$g[a,b]g^{-1} = (gag^{-1})(gbg^{-1})(ga^{-1}g^{-1})(gb^{-1}g^{-1}) = [gag^{-1}, gbg^{-1}]$$
which is still a commutator, hence in $[G,G]$.

For the abelian quotient: in $G/[G,G]$, we have $(aN)(bN) = (ab)N$ and $(bN)(aN) = (ba)N$. Are these equal? $abN = baN \iff b^{-1}a^{-1}ba \in N$, i.e., $[b^{-1}, a^{-1}] \in N = [G,G]$. Yes, since $N = [G,G]$ contains all commutators. $\square$

The quotient $G^{\text{ab}} = G/[G,G]$ is called the *abelianization* of $G$. It's the "largest abelian quotient" of $G$, in the sense that any homomorphism from $G$ to an abelian group $A$ factors through $G^{\text{ab}}$.

In algebraic topology, the abelianization of the fundamental group is the first homology group: $H_1(X) \cong \pi_1(X)^{\text{ab}}$. Abelianization is "what homology does to the fundamental group."

## Simple Groups

A group $G$ is *simple* if it has no proper nontrivial normal subgroups: the only normal subgroups are $\{e\}$ and $G$ itself. Simple groups are the "atoms" of group theory — they can't be factored further by quotient constructions.

Examples of simple groups:
- $\mathbb{Z}/p\mathbb{Z}$ for prime $p$ (abelian simple groups)
- $A_n$ for $n \geq 5$ (this is why the general quintic is not solvable by radicals)
- The 26 *sporadic groups*, including the *Monster group* of order approximately $8 \times 10^{53}$

The *Classification of Finite Simple Groups* (CFSG) says every finite simple group is one of: a cyclic group of prime order, an alternating group $A_n$ ($n \geq 5$), a group of Lie type, or one of the 26 sporadic groups. The proof spans tens of thousands of pages and was completed around 1980.

Understanding quotients and normality is the first step toward understanding this classification and the structure theory of groups generally.
