# 6.1 From Classical to Synthetic: Topology and HoTT

## The Synthetic Approach

Classical topology is *analytic*: you start with a set and specify a topology (a collection of open sets), then derive everything else from this data. A topological space is defined externally, by giving its structure.

HoTT is *synthetic*: types come with their "topology" built in — it's encoded by the identity type structure. You don't need to specify which sets are open, because the path structure already knows. Every construction in HoTT automatically respects the "continuous" structure.

This is the difference between:
- Analytic: "Let $X$ be a topological space. We need to check this function is continuous."
- Synthetic: "Everything is a type. All functions respect path structure automatically."

## The Dictionary

Here's the systematic translation between classical topology and HoTT:

| Classical Topology | HoTT |
|---|---|
| Topological space $X$ | Type $A$ |
| Point $x \in X$ | Term $a : A$ |
| Continuous path $\gamma : [0,1] \to X$ | Identity proof $p : a = b$ |
| Homotopy $H : \gamma \simeq \delta$ | 2-path $H : p = q$ |
| $n$-dimensional homotopy | $n$-fold iterated identity type |
| Homeomorphism $X \cong Y$ | Equivalence of types $A \simeq B$ |
| Equality of homeomorphic types | Path in the universe ($p : A = B$, by Univalence) |
| Contractible space | Contractible type (h-level $-2$) |
| Connected space | Connected type ($\pi_0(A)$ contractible) |
| Simply connected | 1-connected ($\pi_1(A) = 0$) |
| CW complex | Higher inductive type |
| Quotient space $X/\sim$ | Quotient type / HIT |
| Suspension $\Sigma X$ | Suspension type |
| Path space $X^{[0,1]}$ | Identity type $a = b$ |
| Loop space $\Omega X$ | Loop type $a = a$ |
| Fundamental group $\pi_1(X, x)$ | $\pi_1(A, a) := \|\Omega^1 A\|_0$ |
| Homotopy group $\pi_n(X, x)$ | Homotopy group of type |

## Why Not Just Use Classical Topology?

If classical topology and HoTT are so closely related, why bother with HoTT at all? Several reasons:

**1. Synthetic proofs are shorter.** Proving a theorem in HoTT often takes fewer steps than proving the analogous theorem in classical topology. The type theory enforces the "right" level of abstraction, ruling out non-homotopy-invariant constructions.

**2. HoTT is inherently homotopy-invariant.** In classical topology, you can write down functions or constructions that don't respect homotopy equivalence. In HoTT, every construction is automatically homotopy-invariant — because the type theory only gives you what's definable from the path structure.

**3. HoTT is constructive.** HoTT proofs carry computational content. A proof that a construction exists gives you an algorithm to compute it. Classical topology proofs often use classical logic (law of excluded middle) in ways that lose computational content.

**4. HoTT has Univalence.** The Univalence axiom (equivalent types are equal) has no direct analog in classical topology. It allows you to *substitute* one type for an equivalent one in any context, which is a powerful proof principle.

**5. HITs generalize classical constructions.** Higher inductive types let you define new spaces (types) by specifying generators and paths, without constructing an underlying point-set structure. This is more flexible than classical topology.

## The Path Space is the Identity Type

Let's work through the central example: how the path space of a topological space corresponds to the identity type.

**Classical.** For a space $X$ and points $x, y \in X$, the *path space* from $x$ to $y$ is:
$$P(X, x, y) = \{\gamma : [0,1] \to X \mid \gamma \text{ continuous}, \gamma(0) = x, \gamma(1) = y\}$$
with the subspace topology from $X^{[0,1]}$ (the function space with the compact-open topology).

**HoTT.** The identity type $x =_A y$ is the type of paths from $x$ to $y$ in $A$.

**The correspondence:** In the simplicial set model, the identity type $x =_A y$ is interpreted as the simplicial set of "simplicial paths" from $x$ to $y$ in $\llbracket A \rrbracket$. For a topological space, the singular complex $\mathsf{Sing}(X)$ has $n$-simplices that are continuous maps $\Delta^n \to X$. The "simplicial path space" has 1-simplices from $x$ to $y$ as its elements — which are continuous paths.

So the translation is: $\llbracket x =_A y \rrbracket =$ the simplicial set of paths from $x$ to $y$ in $\llbracket A \rrbracket$.

**The J rule as the path induction principle.** The classical version: any property $P$ of paths from $x$ to $y$ that holds for the constant path at $x$ (when $x = y$) holds for all paths. This is a lifting property (see Chapter 11, Section 5).

In HoTT: the J rule says exactly this. To prove $P$ for all $p : a = b$, it suffices to prove $P$ for $\mathsf{refl}_a$. This is path induction.

## The Interval in Topology and HoTT

The unit interval $[0,1]$ plays a central role in classical topology: paths are maps from $[0,1]$, homotopies are maps from $[0,1]^2$, and so on.

In HoTT, the interval doesn't need to be defined explicitly — it's abstracted away. Paths are just elements of identity types, and homotopies are paths between paths.

However, it's possible to define an *interval type* in HoTT as a HIT:

**Interval HIT $\mathbb{I}$:** Constructors:
- $0 : \mathbb{I}$ and $1 : \mathbb{I}$ (endpoints)
- $\mathsf{seg} : 0 = 1$ (the segment connecting them)

Induction principle: for $P : \mathbb{I} \to \mathsf{Type}$, to prove $\prod_{i:\mathbb{I}} P(i)$, provide:
- $p_0 : P(0)$
- $p_1 : P(1)$
- $\mathsf{ps} : \mathsf{transport}^P(\mathsf{seg}, p_0) = p_1$

The interval HIT is contractible (there's a unique "path" between any two points, namely a multiple of $\mathsf{seg}$). This matches the topological fact that $[0,1]$ is contractible.

**Function extensionality from the interval.** One payoff of the interval HIT: function extensionality (two functions are equal iff pointwise homotopic) follows from the existence of the interval. A homotopy $h : f \sim g$ (i.e., $\prod_{a:A} f(a) = g(a)$) can be "curried" to give a map $A \times \mathbb{I} \to B$ (a path in the function space), which is a path $f = g$ in $A \to B$.

## Homotopy Groups in Classical and Synthetic Topology

The homotopy groups $\pi_n(X, x_0)$ are the central invariants of algebraic topology:
- $\pi_0(X)$: connected components (a set)
- $\pi_1(X, x_0)$: fundamental group (paths from $x_0$ to $x_0$, up to homotopy)
- $\pi_n(X, x_0)$: the $n$th homotopy group (maps $S^n \to X$ sending basepoint to $x_0$, up to homotopy)

In HoTT, these are defined internally:
- $\pi_0(A) := \|A\|_0$ (the 0-truncation, a set)
- $\pi_1(A, a) := \|\Omega A\|_0 = \|(a = a)\|_0$ (the 0-truncation of the loop space)
- $\pi_n(A, a) := \|\Omega^n A\|_0$ (the 0-truncation of the iterated loop space)

The structure:
- $\pi_0(A)$ is a set
- $\pi_1(A, a)$ is a group (from path concatenation and inversion)
- $\pi_n(A, a)$ is an abelian group for $n \geq 2$ (by the Eckmann-Hilton argument)

All of this is provable in HoTT from first principles, without any topology machinery.

## Open Sets in HoTT?

A natural question: do open sets appear anywhere in HoTT?

The short answer: not directly. HoTT doesn't have a notion of "open set" built in. Instead:

- *Propositions* in HoTT correspond to open sets in sheaf toposes (Chapter 11)
- The *subobject classifier* $\Omega$ in a topos (which classifies open sets/subobjects) corresponds to the *type of propositions* $\mathsf{Prop}$ in HoTT
- *Open maps* in topology correspond to *$(-1)$-connected maps* in HoTT

But you don't need to track open sets explicitly. The type-theoretic operations automatically respect the "topology" (the path structure). Open sets are implicit.

This is the power of the synthetic approach: you work at a higher level of abstraction, and the lower-level details (open sets, epsilon-delta arguments) are handled automatically.

## What Classical Topology Can't Do

There are things HoTT can do that classical topology can't, and vice versa.

**What HoTT adds:**
- The Univalence axiom (no classical analog that works as smoothly)
- HITs as first-class citizens (no need to construct an underlying space)
- Proof-relevant reasoning: a proof that $A = B$ *is* an equivalence, and you can use it as such
- Intrinsic homotopy-invariance: everything is automatically continuous

**What classical topology has:**
- Point-set constructions (non-continuous maps, pathological spaces)
- Non-homotopy-invariant invariants (dimension as a homeomorphism invariant, not just a homotopy invariant)
- Classical logic (LEM, full choice) — useful for certain existence proofs
- Set-theoretic foundations that are broadly understood

For most of homotopy theory, HoTT is the better language. For point-set topology (homeomorphism rather than homotopy equivalence), classical topology is more appropriate.

## Summary

Classical topology provides the semantic foundation for HoTT: the simplicial set model shows that types really do behave like topological spaces. But HoTT transcends classical topology by axiomatizing the homotopy-theoretic structure directly, without needing point-set machinery.

The progression:
1. **Point-set topology** (this chapter): topological spaces, continuous maps, the language of classical analysis and geometry
2. **Homotopy theory** (next chapter): homotopy equivalences, the language of algebraic topology
3. **HoTT** (Chapters 16-26): synthetic homotopy theory, where everything is a type and homotopy structure is built-in

Each level is a refinement of the previous: homotopy theory keeps only what's "continuous" and "homotopy-invariant" from topology; HoTT axiomatizes the homotopy-invariant structure directly.
